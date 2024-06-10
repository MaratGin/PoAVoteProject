use serde::{Serialize, Deserialize};
// use sha2::{Sha256, Digest};
// use rsa::{RsaPrivateKey, RsaPublicKey, PaddingScheme};
// use base64;
// use rand::rngs::OsRng;
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenesisBlock {
    pub index: u64,
    pub hash: String,
    pub timestamp: u128,
    pub election_name: String,
    pub election_description: String,
    pub users: Vec<String>,
    pub options: Vec<OptionInfo>,
    pub validators: Vec<String>,
    pub is_open: bool,
    pub is_multi: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionInfo {
    pub index: u64,
    pub value: String
}


impl GenesisBlock {
    pub fn new(
        index: u64,
        hash: String,
        election_name: String,
        election_description: String,
        users: Vec<String>,
        options: Vec<OptionInfo>,
        validators: Vec<String>,
        is_open: bool,
        is_multi: bool
    ) -> Self {
        // println!(index.to_string());
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let timestamp = since_the_epoch.as_millis();
        let block = GenesisBlock {
            index: 0,
            hash: String::from("0"),
            timestamp,
            election_name,
            election_description,
            users,
            options,
            validators,
            is_open,
            is_multi
        };
        block
    }
}

// fn main() {
//     let mut rng = OsRng;
//     let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("failed to generate a key");
//     let public_key = RsaPublicKey::from(&private_key);

//     let config_parameters = ConfigParameters {
//         consensus_threshold: 2,
//         hash_algorithm: String::from("SHA-256"),
//         other_parameters: vec![String::from("param1"), String::from("param2")],
//     };

//     let genesis_block = GenesisBlock::new(
//         String::from("Election 2024"),
//         String::from("This is a test election"),
//         1625078400000, // Пример даты начала
//         1625164800000, // Пример даты окончания
//         vec![String::from("Alice"), String::from("Bob")],
//         vec![String::from("Validator1PublicKey"), String::from("Validator2PublicKey")],
//         config_parameters,
//         &private_key,
//     );

//     println!("Genesis Block: {:?}", genesis_block);
//     assert!(genesis_block.verify_signature(&public_key));
//     println!("Signature verified successfully!");
// }
