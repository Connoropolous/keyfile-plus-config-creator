# keyfile-plus-config-creator

This binary will create two files

It uses the null/default passphrase that the Conductor expects, as a temporary hack to quicken this process

"priv.key", the newly generated key for your user
"conductor-config.toml" which is configured for that new user

The files can be changed in `src/main.rs`

Stick this executable in the main directory of an app that you want people to download, and start a new user / node