use crate::{Error, Result};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::{read_keypair_file, Keypair};
use yaml_rust::YamlLoader;

/// The schema for Escrow data in program derived accounts. This is what
/// is serialized into the account and updated when hellos are sent.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct EscrowSchema {
    pub buyer: Pubkey,
    pub paid_amount: u8,
    pub refunded: bool,
    pub post_delivered: bool,
    pub eth_usd_price: u32, // For development purposes
}

#[derive(Copy, Clone)]
pub enum ACTION {
    AddRating = 1,
    SetFirstRating = 2,
}

/// pretty_print
pub fn pp(num: u64) -> String {
    num.to_string().as_bytes().rchunks(3).rev().map(std::str::from_utf8)
       .collect::<std::result::Result<Vec<&str>, _>>().unwrap().join("_")  
       // _ is separator
}

pub fn get_args() -> Vec<String> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 4 { return args }    
    eprintln!(
    "\nError: Wrong number of args.
       Usage:
       // transfer token to buyer
       cargo r ../program/target/deploy/escrow-keypair.json ttb buyer1
       // check if post delivered
       cargo r ../program/target/deploy/escrow-keypair.json pd buyer1
       // Refund to buyer story
       cargo r ../program/target/deploy/escrow-keypair.json w buyer1
    ",
    );
    std::process::exit(-1);
}

/// Parses and returns the Solana yaml config on the system.
pub fn get_config() -> Result<yaml_rust::Yaml> {
    let path = match home::home_dir() {
        Some(mut path) => {
            path.push(".config/solana/cli/config.yml");
            path
        }
        None => {
            return Err(Error::ConfigReadError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "failed to locate homedir and thus can not locoate solana config",
            )));
        }
    };
    let config = std::fs::read_to_string(path).map_err(|e| Error::ConfigReadError(e))?;
    let mut config = YamlLoader::load_from_str(&config)?;
    match config.len() {
        1 => Ok(config.remove(0)),
        l => Err(Error::InvalidConfig(format!(
            "expected one yaml document got ({})",
            l
        ))),
    }
}

/// Gets the RPC url for the cluster that this machine is configured
/// to communicate with.
pub fn get_rpc_url() -> Result<String> {
    let config = get_config()?;
    match config["json_rpc_url"].as_str() {
        Some(s) => Ok(s.to_string()),
        None => Err(Error::InvalidConfig(
            "missing `json_rpc_url` field".to_string(),
        )),
    }
}

pub fn get_devnet_2_user() -> Result<Keypair> {
    let key_path = "/Users/gsimsek/.config/solana/devnet-2.json";
    read_keypair_file(key_path).map_err(|err| {
        Error::InvalidConfig (
            format!("failed to read keypair file ({}): ({})", key_path, err)
        )
    })
}

/// Gets the "user" or local solana wallet that has been configured
/// on the machine.
pub fn get_user() -> Result<Keypair> {
    let config = get_config()?;
    let path = match config["keypair_path"].as_str() {
        Some(s) => s,
        None => {
            return Err(Error::InvalidConfig(
                "missing `keypair_path` field".to_string(),
            ))
        }
    };
    read_keypair_file(path).map_err(|e| {
        Error::InvalidConfig(format!("failed to read keypair file ({}): ({})", path, e))
    })
}

/// Gets the seed used to generate program derived account. If you'd like to
/// force this program to generate a new program derived account (= new Shop obj)
pub fn seed_for_program_derived_account_creation() -> String {
    let str = std::env::args().collect::<Vec<_>>()[3].clone();
    str
    // e.g. "buyer1"
}

/// Derives and returns the program derived account public key for a given
/// USER, PROGRAM combination.
pub fn pda_key(user: &Pubkey, program: &Pubkey) -> Result<Pubkey> {
    Ok(Pubkey::create_with_seed(
        user,
        &seed_for_program_derived_account_creation(),
        program,
    )?)
}

/// Determines and reports the size of Program's obj.
pub fn get_program_obj_size() -> Result<usize> {
    let encoded = EscrowSchema {
            buyer: Pubkey::default(), paid_amount: 0, refunded: false,
            post_delivered: false, eth_usd_price: 0
        }
        .try_to_vec()
        .map_err(|e| Error::SerializationError(e))?;
    Ok(encoded.len())
    // E.g.
    // Ok(4 + (3 * 4)) // vec<u32> w/ 3 elements
    // Ok(3 * 4) // array[u32, 3] = 12 bytes
}

pub fn get_program_obj(data: &[u8]) -> Result<EscrowSchema> {
    let decoded = EscrowSchema::try_from_slice(data).map_err(
        |e| Error::SerializationError(e)
    )?;
    Ok(decoded)
}