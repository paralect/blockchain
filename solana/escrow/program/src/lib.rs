use borsh::{BorshDeserialize, BorshSerialize};
use chainlink_solana as chainlink;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

// The type of state managed by this program. The type defined here
// must match the `Escrow` type defined by the client.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Escrow {
    pub buyer: Pubkey,
    pub paid_amount: u8, // Lamports
    pub refunded: bool,
    pub post_delivered: bool,
    // pub sent_to_seller: bool,
    // pub purchase_complete: bool
}

#[derive(Copy, Clone)]
enum ACTION {
    SavePurchaseData = 1,
    RefundToBuyer = 2,
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let pda = next_account_info(accounts_iter)?;
    msg!("--- instruction_data: {:?}", instruction_data);
    msg!("--- pda: {}", pda.key);
    msg!("--- pda.Escrow: {:?}", Escrow::try_from_slice(&pda.data.borrow())?);
    // msg!("--- pda.data: {:?}", pda.data.borrow());
    msg!("--- accounts.len {}", accounts.len());

    if 33 > 0 {
        msg!("--- Reading Chainlink Price Feed ...");
        // https://docs.chain.link/data-feeds/solana/using-data-feeds-solana#the-chainlink-data-feeds-ocr2-program
        // https://docs.chain.link/data-feeds/price-feeds/addresses?network=solana#Solana%20Devnet
        // This is the account of the price feed data to read from
        // For Solana Devnet ETH/USD: 669U43LNHx7LsVj95uYksnhXUfWKDsdzVqev3V4Jpw3P
        let feed_account = next_account_info(accounts_iter)?;
        msg!("--- feed_account:{}", feed_account.key);
        // The Chainlink Data Feeds OCR2 Program
        // The program that owns the data feeds on both Devnet and Mainnet is 
        // HEvSKofvBgfaexv23kMabbYqxasxU3mQ4ibBMEmJWHny
        // This is the program ID that you use to retrieve Chainlink Price Data
        // on-chain in your program
        let chainlink_program = next_account_info(accounts_iter)?;
        msg!("--- chainlink_program:{}", chainlink_program.key);
        let round = chainlink::latest_round_data(
            chainlink_program.clone(),
            feed_account.clone(),
        )?;
        let description = chainlink::description(
            chainlink_program.clone(),
            feed_account.clone(),
        )?;
        let decimals = chainlink::decimals(
            chainlink_program.clone(),
            feed_account.clone(),
        )?;
        let answer = round.answer.to_string();
        let dec_digits = answer.len()-decimals as usize;
        let price_str: String = answer.to_string().chars().take(dec_digits).collect();
        let price: u32 = price_str.parse().unwrap();
        msg!("{} price (short): {}", description, price);
        return Ok(());
    }

    let fb = instruction_data[0]; // first byte
    match fb { // todo: write with if
        fb if fb == ACTION::SavePurchaseData as u8 => {
            msg!("--- instruction SavePurchaseData");
            let buyer = next_account_info(accounts_iter)?;
            let paid_amount = instruction_data[1];
            msg!("--- buyer: {}, paid_amount: {}", buyer.key, paid_amount);
            // let seller = next_account_info(accounts_iter)?;
            // msg!("--- seller: {}", seller.key);            
            let mut program_data = Escrow::try_from_slice(&pda.data.borrow())?;
            program_data.buyer = *buyer.key;
            program_data.paid_amount = paid_amount;
            program_data.refunded = false;
            program_data.serialize(&mut &mut pda.data.borrow_mut()[..])?;
            msg!("--- Success. Saved:");
            msg!("--- pda.Escrow: {:?}", Escrow::try_from_slice(&pda.data.borrow())?);
            // msg!("--- pda.data: {:?}", pda.data.borrow());
        },
        fb if fb == ACTION::RefundToBuyer as u8 => {
            msg!("--- instruction RefundToBuyer");
            let buyer = next_account_info(accounts_iter)?;
            let program_data = Escrow::try_from_slice(&pda.data.borrow())?;
            // assert buyer.key program_data.paid_amount.buyer // todo
            let paid_amount =  program_data.paid_amount;
            msg!("--- Refunding {} lamports from {} to buyer {}...",
                    paid_amount, pda.key, buyer.key);
            msg!("--- pda.balance before: {:?}", pda.lamports);
            msg!("--- buyer.balance before: {:?}", buyer.lamports);
            **pda.try_borrow_mut_lamports()? -= paid_amount as u64;
            **buyer.try_borrow_mut_lamports()? += paid_amount as u64;
            msg!("--- pda.balance after: {:?}", pda.lamports);
            msg!("--- buyer.balance after: {:?}", buyer.lamports);
            // assert
            let mut program_data = Escrow::try_from_slice(&pda.data.borrow())?;
            program_data.refunded = true;
            program_data.serialize(&mut &mut pda.data.borrow_mut()[..])?;
        }
        _ => todo!()
    }

    Ok(())
}