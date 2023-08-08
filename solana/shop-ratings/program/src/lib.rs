use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg,
};

// The type of state managed by this program. The type defined here
// must match the `Shop` type defined by the client.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Shop {
    pub ratings: [u32; 3],
}

#[derive(Copy, Clone)]
enum OPCODE {
    AddRating = 1,
    SetFirstRating = 2,
}

// Declare the programs entrypoint. The entrypoint is the function
// that will get run when the program is executed.
#[cfg(not(feature = "exclude_entrypoint"))]
entrypoint!(process_instruction);

// Logic that runs when the program is executed. This program expects
// a single account that is owned by the program as an argument and
// no instructions.
//
// The account passed must contain a `Shop` obj
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> entrypoint::ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order for the
    // program to write to it. If that is not the case then the
    // program has been invoked incorrectly and we report as much.
    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    msg!("--- account.data before: {:?}", account.data.borrow());

    // Deserialize the Shop obj. from the account, modify
    // it, and then write it back.
    let mut shop_data = Shop::try_from_slice(&account.data.borrow())?;

    let fb = instruction_data[0]; // first byte
    match fb {
        fb if fb == OPCODE::AddRating as u8 => {
            msg!("--- instruction 1: Add new rating");
            let index = shop_data.ratings.iter().position(|&e| e == 0);
            if index.is_none() { 
                msg!("--- Not enough space, not adding the new rating");
                return Ok(())
            }
            shop_data.ratings[index.unwrap()] = instruction_data[1] as u32;
            shop_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
            msg!("--- success");
        } ,
        fb if fb == OPCODE::SetFirstRating as u8 => {
            msg!("--- instruction 2: Set the first rating of a shop");
            shop_data.ratings[0] = instruction_data[1] as u32;
            shop_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
            msg!("--- success");
        },
        _ => todo!()
    } 

    msg!("--- account.data after: {:?}", account.data.borrow());

    Ok(())
}
