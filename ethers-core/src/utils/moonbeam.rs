//! Moonbeam utilities

use std::collections::BTreeMap;

use k256::SecretKey;

/// Returns the private developer keys https://docs.moonbeam.network/snippets/code/setting-up-node/dev-accounts/
pub fn dev_keys() -> Vec<SecretKey> {
    MoonbeamDev::default().into_keys().collect()
}

/// Holds private developer keys with their names
#[derive(Debug, Clone)]
pub struct MoonbeamDev {
    keys: BTreeMap<&'static str, SecretKey>,
}

impl MoonbeamDev {
    pub fn keys(&self) -> impl Iterator<Item = &SecretKey> {
        self.keys.values()
    }

    pub fn into_keys(self) -> impl Iterator<Item = SecretKey> {
        self.keys.into_values()
    }

    /// Get a key by then, like `Alith`
    pub fn get(&self, name: impl AsRef<str>) -> Option<&SecretKey> {
        self.keys.get(name.as_ref())
    }

    pub fn alith(&self) -> &SecretKey {
        self.get("Alith").unwrap()
    }

    pub fn baltathar(&self) -> &SecretKey {
        self.get("Baltathar").unwrap()
    }

    pub fn charleth(&self) -> &SecretKey {
        self.get("Charleth").unwrap()
    }

    pub fn ethan(&self) -> &SecretKey {
        self.get("Ethan").unwrap()
    }
}

impl Default for MoonbeamDev {
    fn default() -> Self {
        Self {
            keys: BTreeMap::from([
                (
                    "Alith",
                    SecretKey::from_bytes(
                        hex::decode(
                            "5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133",
                        )
                        .unwrap(),
                    )
                    .unwrap(),
                ),
                (
                    "Baltathar",
                    SecretKey::from_bytes(
                        hex::decode(
                            "8075991ce870b93a8870eca0c0f91913d12f47948ca0fd25b49c6fa7cdbeee8b",
                        )
                        .unwrap(),
                    )
                    .unwrap(),
                ),
                (
                    "Charleth",
                    SecretKey::from_bytes(
                        hex::decode(
                            "0b6e18cafb6ed99687ec547bd28139cafdd2bffe70e6b688025de6b445aa5c5b",
                        )
                        .unwrap(),
                    )
                    .unwrap(),
                ),
                (
                    "Dorothy",
                    SecretKey::from_bytes(
                        hex::decode(
                            "39539ab1876910bbf3a223d84a29e28f1cb4e2e456503e7e91ed39b2e7223d68",
                        )
                        .unwrap(),
                    )
                    .unwrap(),
                ),
                (
                    "Faith",
                    SecretKey::from_bytes(
                        hex::decode(
                            "b9d2ea9a615f3165812e8d44de0d24da9bbd164b65c4f0573e1ce2c8dbd9c8df",
                        )
                        .unwrap(),
                    )
                    .unwrap(),
                ),
                (
                    "Goliath",
                    SecretKey::from_bytes(
                        hex::decode(
                            "96b8a38e12e1a31dee1eab2fffdf9d9990045f5b37e44d8cc27766ef294acf18",
                        )
                        .unwrap(),
                    )
                    .unwrap(),
                ),
                (
                    "Heath",
                    SecretKey::from_bytes(
                        hex::decode(
                            "0d6dcaaef49272a5411896be8ad16c01c35d6f8c18873387b71fbc734759b0ab",
                        )
                        .unwrap(),
                    )
                    .unwrap(),
                ),
                (
                    "Ida",
                    SecretKey::from_bytes(
                        hex::decode(
                            "4c42532034540267bf568198ccec4cb822a025da542861fcb146a5fab6433ff8",
                        )
                        .unwrap(),
                    )
                    .unwrap(),
                ),
                (
                    "Judith",
                    SecretKey::from_bytes(
                        hex::decode(
                            "94c49300a58d576011096bcb006aa06f5a91b34b4383891e8029c21dc39fbb8b",
                        )
                        .unwrap(),
                    )
                    .unwrap(),
                ),
            ]),
        }
    }
}
