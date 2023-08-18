use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::transfer;
use solana_program::native_token::lamports_to_sol;
use zeke_contract as zc;
use zc::client::{
    create_program_derived_account,
    get_shop_obj, get_balance_requirement, get_user_balance,
    refund_to_buyer, save_new_purchase_data, send_instruction};
use zc::utils::{
    get_devnet_2_user,
    program_derived_account_key, 
    seed_for_program_derived_account_creation,
};

fn main() {
    let pretty_print = |num: u64| { // e.g. 10000 -> 10_000
        num.to_string().as_bytes().rchunks(3).rev().map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>().unwrap().join("_")  // separator
    };
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 4 {
        eprintln!(
            "\nError: Wrong number of args.
            usage: e.g. \
            cargo r ../program/target/deploy/helloworld-keypair.json r shop1
            (w: write, r: read)
            ",
        );
        std::process::exit(-1);
    }
    let keypair_path = &args[1];

    // 1. Connect to chain
    let connection = zc::client::establish_connection().unwrap();
    println!(
        "\n1. Connected to remote solana node running version ({}).\n",
        connection.get_version().unwrap()
    );

    let balance_requirement = get_balance_requirement(&connection).unwrap();
    println!(
        "({}) lamports are required for this transaction.",
        pretty_print(balance_requirement)
    );

    let user = zc::utils::get_user().unwrap();
    let user_balance = get_user_balance(&user, &connection).unwrap();
    println!("User: {:?}",user.pubkey());
    println!("Balance: {} Sol ({} lamports)", 
        lamports_to_sol(user_balance), pretty_print(user_balance)
    );
    // println!("User {:?}: {} lamports",user.pubkey(), user_balance);

    if user_balance < balance_requirement {
        let request = balance_requirement - user_balance;
        println!(
            "User does not own sufficent lamports. Airdropping ({}) lamports.",
            request
        );
        zc::client::request_airdrop(&user, &connection, request).unwrap();
    }

    let program = zc::client::get_program(keypair_path, &connection).unwrap();

    // 2. Optional - Create account for program to write its data 
    // (Fee: 5000) (a new addr for a given user and program combination)
    println!("\n2. Create account for program to read/write its data...");
    let res = create_program_derived_account(&user, &program, &connection).unwrap();
    println!("--- res : {:?}", res);

    // 3. Print some info
    println!("\n3. Info");
    println!("Program: {:?}", program.pubkey());
    let pda = program_derived_account_key(&user.pubkey(), &program.pubkey()).unwrap();
    println!("PDA: {:?}", pda);
    println!("  (aka Program's data account to read/write)");
    println!("  (aka Derived addr for a given user and program combination)");
    println!("PDA name: {}\n", seed_for_program_derived_account_creation());

    // Todo: Use FAUCET_PDA as buyer ?
    // use std::str::FromStr;
    // use solana_program::pubkey::Pubkey;
    // const FAUCET_PDA: &str = "4uHT4GBdZPimek4Zv2PquUtup7J8wNkwuU9Lna2pS6zQ";
    // let buyer = Pubkey::from_str(FAUCET_PDA).unwrap();
    // let buyer_keypair = &user;
    //
    let buyer_keypair = get_devnet_2_user().unwrap();
    let buyer = buyer_keypair.pubkey();
    let seller = user.pubkey();
    println!("--- buyer : {}", &buyer);
    println!("--- seller: {}", &seller);
    let pda_balance_0 = connection.get_balance(&pda).unwrap();
    let buyer_balance_0 = connection.get_balance(&buyer).unwrap();
    println!("--- PDA bal  : {}", pda_balance_0);
    println!("--- buyer bal: {}", buyer_balance_0);

    // 4. write
    if args[2] == "w" {
        const TEN_LAMPORTS: u64 = 10;
        println!("\n4. Write to chain: Sending transaction(s) ...");

        // a. Buyer sends lamports to pda
        println!("\na. Buyer sending {} lamports to pda ...", TEN_LAMPORTS);
        let ins = transfer(&buyer, &pda, TEN_LAMPORTS);
        let res = send_instruction(ins, &buyer_keypair, &buyer, &connection);
        println!("res: {:?}", res);
        let pda_balance_1 = connection.get_balance(&pda).unwrap();
        let buyer_balance_1 = connection.get_balance(&buyer).unwrap();
        println!("--- PDA bal  : {}", pda_balance_1);
        const TX_COST: u64 = 5000; // todo: get this from chain
        println!("--- buyer bal: {} (tx fee: {})", buyer_balance_1, TX_COST);
        assert_eq!(pda_balance_1, pda_balance_0 + TEN_LAMPORTS);
        assert_eq!(buyer_balance_1, buyer_balance_0 - TEN_LAMPORTS - TX_COST);

        // b. save_new_purchase_data
        let seller = buyer.clone(); // For now, todo
        println!("\nb. Saving new purchase data ...");
        println!(
            "> before: {:#?}",
            get_shop_obj(&user, &program, &connection).unwrap()
        );
        let res = save_new_purchase_data(
            &user, &program, &connection,
            buyer, TEN_LAMPORTS as u8, seller
        );
        println!("res: {:?}", res);
        let purchase_data = get_shop_obj(&user, &program, &connection).unwrap();
        println!("> after: {:#?}", purchase_data);
        assert_eq!(purchase_data.paid_amount, TEN_LAMPORTS as u8);
        assert_eq!(purchase_data.buyer, buyer);

        // c. refund_to_buyer
        println!("\nc. Refunding {} to buyer ...", TEN_LAMPORTS);
        let res = refund_to_buyer(&user, &program, &connection, buyer);
        println!("res: {:?}", res);
        let pda_balance_2 = connection.get_balance(&pda).unwrap();
        let buyer_balance_2 = connection.get_balance(&buyer).unwrap();
        println!("--- PDA bal  : {}", pda_balance_2);
        println!("--- buyer bal: {}", buyer_balance_2);
        assert_eq!(pda_balance_2, pda_balance_0);
        assert_eq!(buyer_balance_2, buyer_balance_0 - TX_COST);
    } else { 
        println!("\n4. Read from chain:");
        println!("--- PDA bal   : {}", connection.get_balance(&pda).unwrap());
        println!("--- buyer bal : {}", connection.get_balance(&buyer).unwrap());
        println!("--- seller bal: {}", connection.get_balance(&seller).unwrap());        
    }

    println!("\nEnd\n");
}
