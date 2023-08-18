use crate::{Error, Result};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::{read_keypair_file, Keypair};
use yaml_rust::YamlLoader;

/// The schema for Shop data in program derived accounts. This is what
/// is serialized into the account and updated when hellos are sent.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ShopSchema {
    pub buyer: Pubkey,
    pub paid_amount: u8,
}

#[derive(Copy, Clone)]
pub enum ACTION {
    AddRating = 1,
    SetFirstRating = 2,
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
    // e.g. "shop1"
}

/// Derives and returns the program derived account public key for a given
/// USER, PROGRAM combination.
pub fn program_derived_account_key(user: &Pubkey, program: &Pubkey) -> Result<Pubkey> {
    Ok(Pubkey::create_with_seed(
        user,
        &seed_for_program_derived_account_creation(),
        program,
    )?)
}

/// Determines and reports the size of Shop obj.
pub fn get_shop_obj_size() -> Result<usize> {
    let encoded = ShopSchema {
            buyer: Pubkey::default(), paid_amount: 0
        }
        .try_to_vec()
        .map_err(|e| Error::SerializationError(e))?;
    Ok(encoded.len())
    // E.g.
    // Ok(4 + (3 * 4)) // vec<u32> w/ 3 elements
    // Ok(3 * 4) // array[u32, 3] = 12 bytes
}

pub fn get_shop_obj(data: &[u8]) -> Result<ShopSchema> {
    let decoded = ShopSchema::try_from_slice(data).map_err(
        |e| Error::SerializationError(e)
    )?;
    Ok(decoded)
}