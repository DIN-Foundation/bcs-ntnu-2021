use std::io::Write;

fn main() {
    let jonas = "did:xyz:ulapcuhsatnpuhza930hpu34n_";
    let cecilie = vec!("did::xyz:34r3cu403hnth03r49g03");
    let args: Vec<String> = std::env::args().collect();
    let args_as_string = args.join("");
    let args_as_bytes = args_as_string.as_bytes();

    // println!("{:?}", &args[1..]);

    // 1. Unencrypted
    {
        let message = didcomm_rs::Message::new()
            .from(jonas)
            .to(cecilie.clone())
            .body(args_as_bytes);

        let serialized_message = message
            .clone()
            .as_raw_json()
            .unwrap();

        println!("---------UNECNRYPTED---------");
        println!("{}", serialized_message);
        println!("---------UNECNRYPTED---------");
    }
    
    // 2. Encrypted
    {
        let jonas_secret   = x25519_dalek::StaticSecret::new(rand_core::OsRng);
        let cecilie_secret = x25519_dalek::StaticSecret::new(rand_core::OsRng);
        let _jonas_public  = x25519_dalek::PublicKey::from(&jonas_secret);
        let cecilie_public = x25519_dalek::PublicKey::from(&cecilie_secret);

        std::fs::create_dir(".didchat").unwrap_or_default();

        let mut _file = std::fs::File::create(".didchat/key.private").unwrap();
        _file.write_all(&jonas_secret.to_bytes()).unwrap();
        
        let shared_secret = jonas_secret.diffie_hellman(&cecilie_public);

        let message = didcomm_rs::Message::new()
            .from(jonas)
            .to(cecilie.clone())
            .timed(Some(3600))
            .body(args_as_bytes)
            .as_jwe(&didcomm_rs::crypto::CryptoAlgorithm::XC20P);

        let serialized_message = message
            .seal(shared_secret.as_bytes())
            .unwrap();

        println!("---------ENCRYPTED---------");
        println!("{:?}", serialized_message);
        println!("---------ENCRYPTED---------");
    }
}
