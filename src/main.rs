use failure::Error;
use holochain_conductor_api::{
    key_loaders::mock_passphrase_manager,
    keystore::{Keystore, PRIMARY_KEYBUNDLE_ID},
};
use holochain_dpki::SEED_SIZE;
use std::{
    env,
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
[logger]
type = "debug"
[[logger.rules.rules]]
pattern = "^debug"

[[agents]]
id = "test_agent1"
name = "HoloTester1"
"#;

const SECOND_HALF : &'static str = r#"
keystore_file = "./priv.key"

[[dnas]]
id = "chat_dna"
file = "dna/holo-chat.hcpkg"

[[instances]]
id = "holo-chat"
dna = "chat_dna"
agent = "test_agent1"
[instances.logger]
type = "simple"
file = "app_spec.log"
[instances.storage]
type = "file"
path = "storage"

[[interfaces]]
id = "websocket_interface"
[interfaces.driver]
type = "websocket"
port = 3400
[[interfaces.instances]]
id = "holo-chat"

[[ui_bundles]]
id = "main"
root_dir = "./ui"
hash = "Qm000"

[[ui_interfaces]]
id = "ui-interface"
bundle = "main"
port = 3000
dna_interface = "websocket_interface"

[network]
n3h_path = "./n3h"
n3h_log_level = "i"
n3h_persistence_path = "./n3hfolder"
"#;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Generating key file, please wait...");
    let bootstrap_node = &args[1];
    let maybe_address = keygen(PathBuf::from("./priv.key".to_string()), holochain_common::DEFAULT_PASSPHRASE.to_string());
    match maybe_address {
        Ok(address) => {
            let mut file = File::create(PathBuf::from("conductor-config.toml".to_string())).unwrap();
            let contents = format!("{}public_address = \"{}\"{}bootstrap_nodes=[\"{}\"]", FIRST_HALF, address, SECOND_HALF, bootstrap_node);
            file.write_all(contents.as_bytes());
            println!("Successfully wrote priv.key and conductor-config.toml file");
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}