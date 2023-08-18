use crate::utils;
use crate::utils::program_derived_account_key;
use crate::{Error, Result};
use solana_program::pubkey::Pubkey;
use utils::ACTION;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::message::Message;
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::{read_keypair_file, Keypair};
use solana_sdk::transaction::Transaction;

/// Establishes a RPC connection with the solana cluster configured by
/// `solana config set --url <URL>`. Information about what cluster
/// has been configured is gleened from the solana config file
/// `~/.config/solana/cli/config.yml`.
pub fn establish_connection() -> Result<RpcClient> {
    let rpc_url = utils::get_rpc_url()?;
    Ok(RpcClient::new_with_commitment(
        rpc_url,
        CommitmentConfig::confirmed(),
    ))
}

/// Determines the amount of lamports that will be required to execute
/// this smart contract. The minimum balance is calculated assuming
/// that the user would like to make their account rent exempt.
///
/// For more information about rent see the Solana documentation
/// [here](https://docs.solana.com/implemented-proposals/rent#two-tiered-rent-regime)
pub fn get_balance_requirement(connection: &RpcClient) -> Result<u64> {
    let account_fee =
        connection.get_minimum_balance_for_rent_exemption(utils::get_shop_obj_size()?)?;

    let (_, fee_calculator) = connection.get_recent_blockhash()?;
    let transaction_fee = fee_calculator.lamports_per_signature * 100;

    Ok(transaction_fee + account_fee)
}

/// Gets the balance of USER in lamports via a RPC call over
/// CONNECTION.
pub fn get_user_balance(user: &Keypair, connection: &RpcClient) -> Result<u64> {
    Ok(connection.get_balance(&user.pubkey())?)
}

/// Requests that AMOUNT lamports are transfered to USER via a RPC
/// call over CONNECTION.
///
/// Airdrops are only avaliable on test networks.
pub fn request_airdrop(user: &Keypair, connection: &RpcClient, amount: u64) -> Result<()> {
    let sig = connection.request_airdrop(&user.pubkey(), amount)?;
    loop {
        let confirmed = connection.confirm_transaction(&sig)?;
        if confirmed {
            break;
        }
    }
    Ok(())
}

/// Loads keypair information from the file located at KEYPAIR_PATH
/// and then verifies that the loaded keypair information corresponds
/// to an executable account via CONNECTION. Failure to read the
/// keypair or the loaded keypair corresponding to an executable
/// account will result in an error being returned.
pub fn get_program(keypair_path: &str, connection: &RpcClient) -> Result<Keypair> {
    let program_keypair = read_keypair_file(keypair_path).map_err(|e| {
        Error::InvalidConfig(format!(
            "failed to read program keypair file ({}): ({})",
            keypair_path, e
        ))
    })?;

    let program_info = connection.get_account(&program_keypair.pubkey())?;
    if !program_info.executable {
        return Err(Error::InvalidConfig(format!(
            "program with keypair ({}) is not executable",
            keypair_path
        )));
    }

    Ok(program_keypair)
}

/// On Solana accounts are ways to store data. In order to use our
/// smart contract we need some way to store the
/// number of times we have said hello to the contract. To do this we
/// create a program derived account which we subsequentally transfer
/// ownership of to the program. This allows the program to write to
/// that account as it deems fit.
///
/// The program derived account has a [derived address]
/// (https://docs.solana.com/developing/programming-model/calling-between-programs#program-derived-addresses)
/// which allows it to own and manage the account. Additionally the
/// address being derived means that we can regenerate it when we'd
/// like to find the program derived account again later.
pub fn create_program_derived_account(
    user: &Keypair,
    program: &Keypair,
    connection: &RpcClient,
) -> Result<()> {
    let program_derived_account = 
        program_derived_account_key(&user.pubkey(), &program.pubkey())?;

    let shop_obj_size = utils::get_shop_obj_size().unwrap();
    println!("--- shop_obj_size: {}", shop_obj_size);
    let lamport_requirement = connection.get_minimum_balance_for_rent_exemption(
        shop_obj_size
    )?;
    println!("--- min_balance_for_rent_exemption: {}", lamport_requirement);

    let mut success = false;
    if let Err(_) = connection.get_account(&program_derived_account) {
        println!("... creating program derived account");

        // This instruction creates an account with the key
        // "program_derived_account". The created account is owned by the
        // program. The account is loaded with enough lamports to stop
        // it from needing to pay rent. The lamports to fund this are
        // paid by the user.
        //
        // It is important that the program owns the created account
        // because it needs to be able to modify its contents.
        //
        // The address of the account created by
        // create_account_with_seed is the same as the address
        // generated by utils::program_derived_account. We do this as
        // opposed to create_account because create account doesn't
        // derive that address like that.
        let instruction = solana_sdk::system_instruction::create_account_with_seed(
            &user.pubkey(),
            &program_derived_account,
            &user.pubkey(),
            &utils::seed_for_program_derived_account_creation(),
            lamport_requirement,
            shop_obj_size as u64,
            &program.pubkey(),
        );
        let message = Message::new(&[instruction], Some(&user.pubkey()));
        let transaction =
            Transaction::new(&[user], message, connection.get_recent_blockhash()?.0);

        let signature = connection.send_and_confirm_transaction(&transaction)?;
        success = true;
        println!("Signature: {}", signature);
    }

    if !success { println!("... not created, account may already exist "); }

    Ok(())
}

pub fn create_instruction(
    action: ACTION,
    data: u8,
    program: &Keypair,
    program_derived_account: Pubkey,
) -> Instruction {
    Instruction::new_with_bytes(
        program.pubkey(),
        &[action as u8, data],
        vec![AccountMeta::new(program_derived_account, false)],
    )    
}

/// Sends an instruction from USER to PROGRAM via CONNECTION. The
/// instruction contains no data but does contain the address of our
/// previously generated program derived account. The program will use that
/// passed in address to update its program derived account data after verifying
/// that it owns the account that we have passed in.
pub fn send_instruction(
    instruction: Instruction,
    tx_account: &Keypair,
    payer: &Pubkey,
    connection: &RpcClient,
) -> Result<()> {
    // Submit an instruction to the chain which tells the program to
    // run. We pass the account that we want the results to be stored
    // in as one of the accounts arguments which the program will
    // handle.

    let message = Message::new(&[instruction], Some(&payer));
    let transaction = Transaction::new(
        &[tx_account], message, connection.get_recent_blockhash()?.0
    );
    connection.send_and_confirm_transaction(&transaction)?;

    Ok(())
}

pub fn get_shop_obj(
    user: &Keypair, program: &Keypair, connection: &RpcClient
) -> Result<utils::ShopSchema> {
    let account_key = 
        program_derived_account_key(&user.pubkey(), &program.pubkey())?;
    let account = connection.get_account(&account_key)?;
    // println!("--- program derived account: {:?}", &account.data);
    Ok(utils::get_shop_obj(&account.data)?)
}

pub fn send_lamports(
    user: &Keypair,
    program: &Keypair,
    connection: &RpcClient,
    from: Pubkey,
    to: Pubkey,
    lamports: u8,
) -> Result<()> {
    println!("--- sending {} lamports from {} to {} ...", lamports, &from, &to);
    let instruction = Instruction::new_with_bytes(
        program.pubkey(),
        &[lamports],
        vec![
            AccountMeta::new(from, false), 
            AccountMeta::new(to, false),
        ],
    );
    let message = Message::new(&[instruction], Some(&user.pubkey()));
    let transaction = Transaction::new(&[user], message, connection.get_recent_blockhash()?.0);

    let _sig = connection.send_and_confirm_transaction(&transaction)?;
    // println!("sig: {}", sig);


    Ok(())
}

pub fn save_new_purchase_data(
    user: &Keypair,
    program: &Keypair,
    connection: &RpcClient,
    buyer: Pubkey,
    paid_amount: u8, // lamports
    seller: Pubkey,
) -> Result<()> {
    // println!("--- sending {} lamports from {} to {} ...", lamports, &from, &to);
    let pda = program_derived_account_key(&user.pubkey(), &program.pubkey())?;
    let instruction = Instruction::new_with_bytes(
        program.pubkey(),
        &[1, paid_amount],
        vec![
            AccountMeta::new(pda, false),
            AccountMeta::new(buyer, false),
            AccountMeta::new(seller, false),
        ],
    );
    let message = Message::new(&[instruction], Some(&user.pubkey()));
    let transaction = Transaction::new(
        &[user], message, connection.get_recent_blockhash()?.0
    );

    let _sig = connection.send_and_confirm_transaction(&transaction)?;
    // println!("sig: {}", sig);

    Ok(())
}

pub fn refund_to_buyer(
    user: &Keypair,
    program: &Keypair,
    connection: &RpcClient,
    buyer: Pubkey,
) -> Result<()> {
    println!("--- refund_to_buyer() {} ...", buyer);
    let pda = program_derived_account_key(&user.pubkey(), &program.pubkey())?;
    let instruction = Instruction::new_with_bytes(
        program.pubkey(),
        &[2],
        vec![
            AccountMeta::new(pda, false),
            AccountMeta::new(buyer, false),
        ],
    );
    let message = Message::new(&[instruction], Some(&user.pubkey()));
    let transaction = Transaction::new(
        &[user], message, connection.get_recent_blockhash()?.0
    );

    let _sig = connection.send_and_confirm_transaction(&transaction)?;
    // println!("sig: {}", sig);

    Ok(())
}