use std::env;
use didcomm_rs::{
    Message,
};
use didcomm_rs::crypto::{
    CryptoAlgorithm
};
use x25519_dalek::{PublicKey, StaticSecret};
use rand_core::OsRng;

fn main() {
    let jonas = "did:xyz:ulapcuhsatnpuhza930hpu34n_";
    let cecilie = vec!("did::xyz:34r3cu403hnth03r49g03");
    let args: Vec<String> = env::args().collect();
    let args_as_string = args.join("");
    let args_as_bytes = args_as_string.as_bytes();

    // println!("{:?}", &args[1..]);

    // 1. Unencrypted
    {
        let message = Message::new()
            .from(jonas)
            .to(cecilie.clone())
            .body(args_as_bytes);

        let serialized_message = message
            .clone()
            .as_raw_json()
            .unwrap();

        println!("---------UNECNRYPTED---------");
        println!("{:?}", serialized_message);
        println!("---------UNECNRYPTED---------");
    }

    // 2. Encrypted
    {
        let jonas_secret = StaticSecret::new(OsRng);
        let cecilie_secret = StaticSecret::new(OsRng);
        let _ = PublicKey::from(&jonas_secret);
        let cecilie_public = PublicKey::from(&cecilie_secret);

        let shared_secret = jonas_secret.diffie_hellman(&cecilie_public);

        let message = Message::new()
            .from(jonas)
            .to(cecilie.clone())
            .timed(Some(3600))
            .body(args_as_bytes)
            .as_jwe(&CryptoAlgorithm::XC20P);

        let serialized_message = message
            .seal(shared_secret.as_bytes())
            .unwrap();

        println!("---------ENCRYPTED---------");
        println!("{:?}", serialized_message);
        println!("---------ENCRYPTED---------");
    }
}
