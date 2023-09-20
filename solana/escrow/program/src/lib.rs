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
    pub eth_usd_price: u32, // For development purposes
    // pub sent_to_seller: bool,
    // pub purchase_complete: bool
}

#[derive(Copy, Clone)]
enum ACTION {
    SavePurchaseData = 1,
    RefundToBuyer = 2,
    IsPostDelivered = 3,
    TransferTokenToBuyer = 4,    
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

    // ACTION SELECTOR
    // todo: move to instructions.rs
    // todo: better use match but it will be two indentations and
    //       more readable with if (see previous versions with match used)
    let fb = instruction_data[0]; // first byte
    if fb == ACTION::TransferTokenToBuyer as u8 {
        // Accounts required for token transfer
        let _user = next_account_info(accounts_iter)?;
        // 1. Token account we hold
        let source_token_account = next_account_info(accounts_iter)?;
        // 2. Token account to send to
        let destination_token_account = next_account_info(accounts_iter)?;
        // 3. Our wallet address
        let source_token_account_holder = next_account_info(accounts_iter)?;
        // 4. Token Program
        let token_program = next_account_info(accounts_iter)?;
        let token_transfer_amount = 1_000_000_000;
        msg!(
            "Transferring {} tokens from {} to {}",
            token_transfer_amount,
            source_token_account.key.to_string(),
            destination_token_account.key.to_string()
        );
        let transfer_tokens_instruction = spl_token::instruction::transfer(
            &token_program.key,
            &source_token_account.key,
            &destination_token_account.key,
            &source_token_account_holder.key,
            &[&source_token_account_holder.key],
            token_transfer_amount,
        )?;
        let required_accounts_for_transfer = [
            source_token_account.clone(),
            destination_token_account.clone(),
            source_token_account_holder.clone(),
        ];
        // Passing the TransactionInstruction to send
        solana_program::program::invoke(
            &transfer_tokens_instruction,
            &required_accounts_for_transfer,
        )?;
        msg!("Transfer successful");
    }
    else if fb == ACTION::IsPostDelivered as u8 {
        msg!("--- instruction IsPostDelivered");
        let feed_account = next_account_info(accounts_iter)?.clone();
        let chainlink_program = next_account_info(accounts_iter)?.clone();
        let (post_delivered, eth_usd_price) = 
            post_delivered(chainlink_program, feed_account);
        msg!("--- Post delivered: {}", post_delivered);
        let mut program_data = Escrow::try_from_slice(&pda.data.borrow())?;
        program_data.eth_usd_price = eth_usd_price;
        program_data.post_delivered = post_delivered;
        program_data.serialize(&mut &mut pda.data.borrow_mut()[..])?;
        msg!("--- IsPostDelivered Success");
    } 
    else if fb == ACTION::SavePurchaseData as u8 {
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
        msg!("--- SavePurchaseData Success");
        msg!("--- pda.Escrow: {:?}", Escrow::try_from_slice(&pda.data.borrow())?);
        // msg!("--- pda.data: {:?}", pda.data.borrow());
    }
    else if fb == ACTION::RefundToBuyer as u8 {
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
        msg!("--- RefundToBuyer Success");
    }
    else {
        todo!() 
    }

    Ok(())
}

// For now, instead of reading from Post Delivery Oracle program which is (todo),
// we read from Chainlink Oracle program's ETH/USD price.
// If ETH/USD price is even, post_delivered is true otherwise false
fn post_delivered<'a>(
    chainlink_program: AccountInfo<'a>, feed_account: AccountInfo<'a>
) -> (bool, u32) {
    msg!("--- Reading Chainlink Price Feed ...");
    // https://docs.chain.link/data-feeds/solana/using-data-feeds-solana#the-chainlink-data-feeds-ocr2-program
    // https://docs.chain.link/data-feeds/price-feeds/addresses?network=solana#Solana%20Devnet
    // This is the account of the price feed data to read from
    // For Solana Devnet ETH/USD: 669U43LNHx7LsVj95uYksnhXUfWKDsdzVqev3V4Jpw3P
    msg!("--- feed_account:{}", feed_account.key);
    // The Chainlink Data Feeds OCR2 Program
    // The program that owns the data feeds on both Devnet and Mainnet is 
    // HEvSKofvBgfaexv23kMabbYqxasxU3mQ4ibBMEmJWHny
    // This is the program ID that you use to retrieve Chainlink Price Data
    // on-chain in your program
    msg!("--- chainlink_program:{}", chainlink_program.key);
    let round = chainlink::latest_round_data(
        chainlink_program.clone(),
        feed_account.clone(),
    ).unwrap();
    let description = chainlink::description(
        chainlink_program.clone(),
        feed_account.clone(),
    ).unwrap();
    let decimals = chainlink::decimals(
        chainlink_program.clone(),
        feed_account.clone(),
    ).unwrap();
    let answer = round.answer.to_string();
    let dec_digits = answer.len()-decimals as usize;
    let price_str: String = answer.to_string().chars().take(dec_digits).collect();
    let eth_usd_price: u32 = price_str.parse().unwrap();
    msg!("{} price (short): {}", description, eth_usd_price);
    (eth_usd_price % 2 == 0, eth_usd_price)
}