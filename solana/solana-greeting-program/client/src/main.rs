use solana_sdk::signer::Signer;
use solana_program::native_token::lamports_to_sol;
use zeke_contract as zc;
use zc::utils::get_greeting_public_key;

fn main() {
    let pretty_print = |num: u64| { // e.g. 10000 -> 10_000
        num.to_string().as_bytes().rchunks(3).rev().map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>().unwrap().join("_")  // separator
    };
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!(
            "usage: {} <path to solana hello world example program keypair>",
            args[0]
        );
        std::process::exit(-1);
    }
    let keypair_path = &args[1];

    let connection = zc::client::establish_connection().unwrap();
    println!(
        "\n1. Connected to remote solana node running version ({}).\n",
        connection.get_version().unwrap()
    );

    let balance_requirement = zc::client::get_balance_requirement(&connection).unwrap();
    println!(
        "({}) lamports are required for this transaction.",
        pretty_print(balance_requirement)
    );

    let user = zc::utils::get_player().unwrap();
    let user_balance = zc::client::get_player_balance(&user, &connection).unwrap();
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
    println!("Greeting Program: {:?}\n", program.pubkey());
    let key = get_greeting_public_key(&user.pubkey(), &program.pubkey()).unwrap();
    println!("Data account of the program to read: {:?}", key);
    println!("(derived addr for a given user and program combination)");

    // 2. Optional - Create account for greeting program to write its data 
    // (Fee: 5000)
    // (a new addr for a given user and program combination)
    // zc::client::create_greeting_account(&user, &program, &connection).unwrap();

    // 3. write 
    println!("3. Write to chain: Sending greeting ... (sending tx) \n");
    zc::client::greet(&user, &program, &connection).unwrap();

    // 4. read
    println!("4. Read from chain:");
    // println!(
    //     "> greeting count: {}",
    //     zc::client::count_greetings(&user, &program, &connection).unwrap()
    // );
    println!(
        "> greeting obj: {:?}",
        zc::client::get_greeting_obj(&user, &program, &connection)//.unwrap()
    );
    println!("\nEnd\n");
}
