fn main() {
    println!("{}", ed25519_keypair_to_jwk());
    println!("{}", didkey_keypair_to_jwk());
}

fn ed25519_keypair_to_jwk() -> String {
    use did_key::KeyMaterial;
    let mut csprng = rand::rngs::OsRng {};
    let private_key = ed25519_dalek::SecretKey::generate(&mut csprng).to_bytes();
    let did_key = did_key::Ed25519KeyPair::from_seed(&private_key);
    
    bytes_to_jwk(did_key.public_key_bytes(), did_key.private_key_bytes())
}

fn didkey_keypair_to_jwk() -> String {
    let mut csprng = rand::rngs::OsRng {};
    let keypair = ed25519_dalek::Keypair::generate(&mut csprng);

    bytes_to_jwk(keypair.public.to_bytes().to_vec(), keypair.secret.to_bytes().to_vec())
}

fn bytes_to_jwk(public: Vec<u8>, private: Vec<u8>) -> String {
    let jwk = ssi::jwk::JWK {
        params: ssi::jwk::Params::OKP(ssi::jwk::OctetParams {
            curve: "Ed25519".to_string(),
            public_key: ssi::jwk::Base64urlUInt(public),
            private_key: Some(ssi::jwk::Base64urlUInt(private)),
        }),
        public_key_use: None,
        key_operations: None,
        algorithm: None,
        key_id: None,
        x509_url: None,
        x509_certificate_chain: None,
        x509_thumbprint_sha1: None,
        x509_thumbprint_sha256: None
    };

    let _okp = (if let ssi::jwk::Params::OKP(o) = jwk.params.clone() {
        Some(o)
    } else {
        None
    }).unwrap();
    
    
    serde_json::to_string(&jwk).unwrap()
}

