use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

// The type of state managed by this program. The type defined here
// must match the `Shop` type defined by the client.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Shop {
    pub buyer: Pubkey,
    pub paid_amount: u8, // Lamports
    // refunded // todo
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
    msg!("--- pda.Shop: {:?}", Shop::try_from_slice(&pda.data.borrow())?);
    // msg!("--- pda.data: {:?}", pda.data.borrow());
    msg!("--- accounts.len {}", accounts.len());

    let fb = instruction_data[0]; // first byte
    match fb { // todo: write with if
        fb if fb == ACTION::SavePurchaseData as u8 => {
            msg!("--- instruction SavePurchaseData");
            let buyer = next_account_info(accounts_iter)?;
            let paid_amount = instruction_data[1];
            msg!("--- buyer: {}, paid_amount: {}", buyer.key, paid_amount);
            // let seller = next_account_info(accounts_iter)?;
            // msg!("--- seller: {}", seller.key);            
            let mut shop_data = Shop::try_from_slice(&pda.data.borrow())?;
            shop_data.buyer = *buyer.key;
            shop_data.paid_amount = paid_amount;
            shop_data.serialize(&mut &mut pda.data.borrow_mut()[..])?;
            msg!("--- Success. Saved:");
            msg!("--- pda.Shop: {:?}", Shop::try_from_slice(&pda.data.borrow())?);
            // msg!("--- pda.data: {:?}", pda.data.borrow());
        },
        fb if fb == ACTION::RefundToBuyer as u8 => {
            msg!("--- instruction RefundToBuyer");
            let buyer = next_account_info(accounts_iter)?;
            let shop_data = Shop::try_from_slice(&pda.data.borrow())?;
            // assert buyer.key shop_data.paid_amount.buyer // todo
            let paid_amount =  shop_data.paid_amount;
            msg!("--- Refunding {} lamports from {} to buyer {}...",
                    paid_amount, pda.key, buyer.key);
            msg!("--- pda.balance before: {:?}", pda.lamports);
            msg!("--- buyer.balance before: {:?}", buyer.lamports);
            **pda.try_borrow_mut_lamports()? -= paid_amount as u64;
            **buyer.try_borrow_mut_lamports()? += paid_amount as u64;
            msg!("--- pda.balance after: {:?}", pda.lamports);
            msg!("--- buyer.balance after: {:?}", buyer.lamports);
        }
        _ => todo!()
    }

    Ok(())
}