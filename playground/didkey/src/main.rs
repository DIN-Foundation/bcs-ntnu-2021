use did_key::*;

fn main() {
    let key = DIDKey::new(DIDKeyType::Ed25519);
    println!("{}", key.fingerprint());

    let did_doc = key.to_did_document(CONFIG_LD_PUBLIC);
    println!("{:?}", did_doc);
}
