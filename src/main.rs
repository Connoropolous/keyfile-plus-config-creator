use failure::Error;
use holochain_conductor_lib::{
    key_loaders::mock_passphrase_manager,
    keystore::{Keystore, PRIMARY_KEYBUNDLE_ID},
};
use holochain_dpki::SEED_SIZE;

use std::{
    fs::File,
    io::prelude::*,
    path::PathBuf,
};

pub fn keygen(path: PathBuf, passphrase: String) -> Result<String, Error> {
    let mut keystore = Keystore::new(mock_passphrase_manager(passphrase), None)?;
    keystore.add_random_seed("root_seed", SEED_SIZE)?;

    let (pub_key, _) = keystore.add_keybundle_from_seed("root_seed", PRIMARY_KEYBUNDLE_ID)?;

    keystore.save(path.clone())?;
    Ok(pub_key)
}

const FIRST_HALF : &'static str = r#"
bridges = []
persistence_dir = '.'
ui_bundles = []
ui_interfaces = []

[[agents]]
id = 'hc-run-agent'
keystore_file = './keystore.key'
name = 'Agent 1'
"#;

const SECOND_HALF : &'static str = r#"
[[dnas]]
file = './dna/acorn-hc.dna.json'
hash = 'QmPBJ3EUDJypp2ySz9ooY9dhzCRa44EFv5EhuPLbg16WpJ'
id = 'hc-run-dna'

[[instances]]
agent = 'hc-run-agent'
dna = 'hc-run-dna'
id = 'acorn'

[instances.storage]
type = 'memory'

[[interfaces]]
admin = true
id = 'websocket-interface'

[[interfaces.instances]]
id = 'acorn'

[interfaces.driver]
port = 8888
type = 'websocket'

[logger]
state_dump = false
type = 'info'

[logger.rules]
rules = []

[passphrase_service]
type = 'cmd'

[signals]
consistency = false
trace = false

[network]
type = 'sim2h'
sim2h_url = 'wss://sim2h.holochain.org:9000'
"#;

pub fn main() {
    println!("Generating key file, please wait...");
    let maybe_address = keygen(PathBuf::from("./keystore.key".to_string()), holochain_common::DEFAULT_PASSPHRASE.to_string());
    match maybe_address {
        Ok(address) => {
            let mut file = File::create(PathBuf::from("conductor-config.toml".to_string())).unwrap();
            let contents = format!("{}public_address = \"{}\"{}", FIRST_HALF, address, SECOND_HALF);
            let _ = file.write_all(contents.as_bytes());
            println!("Successfully wrote keystore.key and conductor-config.toml file");
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
