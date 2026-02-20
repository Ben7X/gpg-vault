use anyhow::Context;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub gpg_key: String,
    pub groups: Vec<Group>,
}

impl Configuration {
    pub fn new() -> Self {
        let item = Item {
            path: String::from("$HOME/.ssh/id_ed25519"),
        };

        let group = Group {
            name: String::from("ssh"),
            items: vec![item],
        };

        Configuration {
            gpg_key: String::from("gpg-key-id"),
            groups: vec![group],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub name: String,
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub path: String,
}

impl Item {
    pub fn get_path(self) -> anyhow::Result<String> {
        let home = std::env::var("HOME").context("$HOME variable not set")?;
        let normalized = self.path.replace("$HOME", &home);
        Ok(normalized)
    }
}
