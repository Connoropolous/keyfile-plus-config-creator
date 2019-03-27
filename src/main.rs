use failure::Error;
use holochain_dpki::{key_blob::Blobbable, key_bundle::KeyBundle, seed::SeedType, SEED_SIZE};
use holochain_sodium::secbuf::SecBuf;
use std::{
    fs::File,
    io::prelude::*,
    path::PathBuf,
};

pub fn keygen(path: PathBuf, passphrase: String) -> Result<String, Error> {
    let mut seed = SecBuf::with_secure(SEED_SIZE);
    seed.randomize();
    let mut keybundle = KeyBundle::new_from_seed_buf(&mut seed, SeedType::Mock)
        .expect("Failed to generate keybundle");
    let passphrase_bytes = passphrase.as_bytes();
    let mut passphrase_buf = SecBuf::with_insecure(passphrase_bytes.len());
    passphrase_buf
        .write(0, passphrase_bytes)
        .expect("SecBuf must be writeable");
    let blob = keybundle
        .as_blob(&mut passphrase_buf, "hint".to_string(), None)
        .expect("Failed to encrypt with passphrase.");

    let mut file = File::create(path.clone())?;
    file.write_all(serde_json::to_string(&blob).unwrap().as_bytes())?;
    Ok(keybundle.get_id().to_string())
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
key_file = "./priv.key"

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
n3h_persistence_path = "/tmp"
bootstrap_nodes = ["wss://99.224.95.144:5555/?a=HcSCjrvPAa8jp64awoR673zFD3oCgYru6IogRdwut9c88cvpfpih84QE3Zv4ymr"]
"#;

pub fn main() {
    let maybe_address = keygen(PathBuf::from("./priv.key".to_string()), String::from(holochain_common::DEFAULT_PASSPHRASE));
    match maybe_address {
        Ok(address) => {
            let mut file = File::create(PathBuf::from("conductor-config.toml".to_string())).unwrap();
            let contents = format!("{}public_address = \"{}\"{}", FIRST_HALF, address, SECOND_HALF);
            file.write_all(contents.as_bytes());
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}