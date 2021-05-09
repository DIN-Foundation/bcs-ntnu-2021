
pub async fn run(config: Config) -> Result<String, std::io::Error> {
    match config.cmd {
        CMD::Help => help(),

        // DID
        CMD::Init => init(),
        CMD::Doc => doc(),
        CMD::Connect{ didname, did } => connect(&didname, &did),
        CMD::Dids => dids(),
        CMD::Did{ didname } => did(&didname),

        // DIDComm v2
        CMD::Write{ didname, message } => write(&didname, &message),
        CMD::Read{ dcem } => read(&dcem),
        CMD::Hold{ dcem } => hold(&dcem),
        CMD::Messages => messages(),
        CMD::Message{ message_id } => message(&message_id),

        // Verifiable Credentials
        CMD::IssuePassport{ didname } => issue("Passport", &didname).await,
        CMD::IssueLawEnforcer{ didname } => issue("LawEnforcer", &didname).await,
        CMD::IssueTrafficAuthority{ didname } => issue("TrafficAuthority", &didname).await,
        CMD::IssueDriversLicense{ didname } => issue("DriversLicense", &didname).await,
        CMD::Present{ didname, dcem } => present(&didname, &dcem).await,
        CMD::Verify{ issuer_didname, subject_didname, dcem } => verify(&issuer_didname, &subject_didname, &dcem).await,
    }
}

fn help() -> Result<String, std::io::Error> {
    Ok(String::from("
    DID:
        did init
        did doc
        did connect <didname> <did>
        did dids
        did did <didname>

    DIDComm v2:
        did write  <subject didname> <message>  -->  <dcem>
        did hold   <dcem>                       -->  <dcem>
        did read   <dcem>                       -->  <plaintext message>
        did messages
        did message <message id>

    Verifiable Credentials over DIDComm v2:
        did issue   Passport         <subject didname>      -->  <dcem>
        did issue   DriversLicense   <subject didname>      -->  <dcem>
        did issue   TrafficAuthority <subject didname>      -->  <dcem>
        did issue   LawEnforcer      <subject didname>      -->  <dcem>

        did present <verifier didname>                 <dcem>  -->  <dcem>
        did verify  <issuer didname> <subject didname> <dcem>  -->  <dcem>
"))
}

//
// Commands: DID
//
fn init() -> Result<String, std::io::Error> {
    use std::io::Write;

    // 1. Create empty folders, if not exists
    if !std::fs::metadata(root_path()).is_ok() {
        std::fs::create_dir_all(root_path()).unwrap();
    }
    if !std::fs::metadata(dids_path()).is_ok() {
        std::fs::create_dir_all(dids_path()).unwrap();
    }
    if !std::fs::metadata(did_names_path()).is_ok() {
        std::fs::create_dir_all(did_names_path()).unwrap();
    }
    if !std::fs::metadata(messages_path()).is_ok() {
        std::fs::create_dir_all(messages_path()).unwrap();
    }

    let did_doc = if !std::fs::metadata(key_jwk_path()).is_ok() {
        // 2. Generate jwk, if not exists
        let mut csprng = rand::rngs::OsRng {};
        let private_key = ed25519_dalek::SecretKey::generate(&mut csprng).to_bytes();
        let did_key = did_key::Ed25519KeyPair::from_seed(&private_key);

        use did_key::KeyMaterial;
        let jwk = publicprivatebytes_to_jwkstr(did_key.public_key_bytes(), did_key.private_key_bytes());

        // 3. Write jwk to file
        let mut file = std::fs::File::create(key_jwk_path()).unwrap();
        file.write(jwk.as_bytes()).unwrap();

        // 4. Connect to self
        use did_key::DIDCore;
        let did_doc = did_key.get_did_document(did_key::CONFIG_LD_PUBLIC);

        let _ = connect("self", &did_doc.id);

        did_doc
    } else {
        let self_didkey = get_self_didkey();

        use did_key::DIDCore;
        let did_doc = self_didkey.get_did_document(did_key::CONFIG_LD_PUBLIC);

        did_doc
    };

    // 5. Return self did
    let did = did_doc.id;
    Ok(format!("{}", did))
}

fn doc() -> Result<String, std::io::Error> {
    let self_didkey = get_self_didkey();

    use did_key::DIDCore;
    let did_doc = self_didkey.get_did_document(did_key::CONFIG_LD_PUBLIC);
    let did_doc = serde_json::to_string_pretty(&did_doc).unwrap();

    Ok(format!("{}", did_doc))
}

fn connect(did_name: &str, did: &str) -> Result<String, std::io::Error> {
    use std::io::Write;

    // 1. Create 'name' -> 'did'-mapping
    let mut file = std::fs::File::create(did_path(did_name)).unwrap();
    file.write(did.as_bytes()).unwrap();

    // 2. Create 'did' -> 'name'-mapping
    let mut file = std::fs::File::create(did_name_path(did)).unwrap();
    file.write(did_name.as_bytes()).unwrap();

    Ok(format!("{}\n{}", did_path(did_name), did_name_path(did)))
}

fn dids() -> Result<String, std::io::Error> {
    let mut list = format!("{:16}{}\n", "ID", "DID");
    let mut entries: Vec<std::fs::DirEntry> = std::fs::read_dir(dids_path())
        .unwrap()
        .filter_map(|f| f.ok()).collect();
    entries.sort_by_key(|e| e.path());

    for entry in entries {
        if entry.path().is_dir() {
            continue;
        }
        let did = std::fs::read_to_string(entry.path()).unwrap();
        let did_name = String::from(entry.file_name().to_str().unwrap()).replace(".did", "");
        list.push_str(&format!("{:16}{}\n", did_name, did));
    }

    Ok(list)
}

fn did(did_name: &str) -> Result<String, std::io::Error> {
    let path = did_path(did_name);
    let did = std::fs::read_to_string(path).unwrap();

    Ok(did)
}


//
// Commands: DIDComm v2
//
fn write(subject_didname: &str, message: &str) -> Result<String, std::io::Error> {
    // 1. Get did:keys
    let from_key = get_self_didkey();
    let to_key = get_other_didkey(subject_didname);

    // 2. Encrypt message with to_key, to prepare it for transmission
    let (dcem, _) = encrypt_didcomm(&from_key, &to_key, message);

    Ok(format!("{}", &dcem))
}

fn hold(dcem: &str) -> Result<String, std::io::Error> {
    use std::io::Write;

    // 1. Deserialize message
    let message: DIDCommEncryptedMessage = serde_json::from_str(dcem).unwrap();

    // 2. Store incomming message to file with didcomm_header.id as filename.
    let message_id = message.didcomm_header.id.to_string();
    let path = message_path(&message_id);
    let path = std::path::Path::new(&path);
    let mut file = std::fs::File::create(path).unwrap();
    file.write(dcem.as_bytes()).unwrap();

    // 3. Print message to stdout, to support piping commands together
    //
    //    Example: did write self "Hello" | did hold | did read
    //
    Ok(format!("{}", dcem))
}

fn read(dcem: &str) -> Result<String, std::io::Error> {
    // 1. Get did:keys
    let to_key = get_self_didkey();
    let from_key = get_from_key_from_didcomm_message(dcem);

    // 2. Decrypt message, to get the contents of the message-body
    let (body, _) = decrypt_didcomm(&from_key, &to_key, dcem);

    Ok(format!("{}", body))
}

fn messages() -> Result<String, std::io::Error> {
    let mut list = format!(
        "{:16}\t{:14}\t{:14}\t{:>12}\t{:>9}",
        "ID", "From", "To", "Created", "Length");

    // 1. Get messages from message directory
    let mut messages: Vec<DIDCommEncryptedMessage> = std::fs::read_dir(messages_path())
        .unwrap()
        .filter_map(|f| f.ok())
        .filter(|f| !f.path().is_dir())
        .map(|entry| {
            let dcem = std::fs::read_to_string(entry.path()).unwrap();
            let dcem: DIDCommEncryptedMessage = serde_json::from_str(&dcem).unwrap();

            dcem
        })
        .collect();

    // 2. Sort by created time
    messages.sort_by_key(|dcem| dcem.didcomm_header.created_time.unwrap());

    for dcem in messages {
        let message_id = dcem.didcomm_header.id.to_string();
        let from_did = dcem.didcomm_header.from.clone().unwrap();
        let to_did = dcem.didcomm_header.to.first().unwrap().clone();

        // 3. Map dids to names, if exists
        let from_name = std::fs::read_to_string(did_name_path(&from_did))
            .unwrap_or(from_did);

        let to_name = std::fs::read_to_string(did_name_path(&to_did))
            .unwrap_or(to_did);

        list.push_str(&format!(
            "\n{:16}\t{:14}\t{:14}\t{:>12}\t{:>9}",
            message_id,
            from_name,
            to_name,
            dcem.didcomm_header.created_time.unwrap(),
            dcem.ciphertext.len()));
    }

    Ok(list)
}

fn message(message_id: &str) -> Result<String, std::io::Error> {
    let dcem = std::fs::read_to_string(message_path(message_id)).unwrap();
    Ok(dcem)
}


//
// Commands: Verifiable credentials
//
async fn issue(credential_type: &str, subject_didname: &str) -> Result<String, std::io::Error> {
    // 1. Get did docs
    let (issuer_didkey, issuer_jwk) = get_self_jwk_and_didkey();
    let subject_didkey = get_other_didkey(subject_didname);

    use did_key::DIDCore;
    let issuer_doc = issuer_didkey.get_did_document(did_key::CONFIG_LD_PUBLIC);
    let subject_doc = subject_didkey.get_did_document(did_key::CONFIG_LD_PUBLIC);

    // 2. Construct unsigned vc
    let vc = serde_json::json!({
        "@context": [
            "https://www.w3.org/2018/credentials/v1",
        ],
        "type": ["VerifiableCredential", credential_type],
        "issuer": issuer_doc.id,
        "issuanceDate": ssi::ldp::now_ms(),
        "credentialSubject": {
            "id": subject_doc.id
        }
    });

    // 3. Setup proof options with verification method from issuer did doc
    let mut vc: ssi::vc::Credential = serde_json::from_value(vc).unwrap();
    let mut proof_options = ssi::vc::LinkedDataProofOptions::default();

    // https://www.w3.org/TR/did-core/#assertion
    let verification_method = issuer_doc.assertion_method.unwrap()[0].clone();
    proof_options.verification_method = Some(verification_method);
    proof_options.proof_purpose = Some(ssi::vc::ProofPurpose::AssertionMethod);

    // 4. Generate proof, using issuer jwk + proof options
    let proof = vc.generate_proof(&issuer_jwk, &proof_options).await.unwrap();
    vc.add_proof(proof);

    // 5. Serialize and encrypt with subject_didkey
    let vc = serde_json::to_string_pretty(&vc).unwrap();
    let (dcem,_) = encrypt_didcomm(&issuer_didkey, &subject_didkey, &vc.clone());

    Ok(dcem)
}

async fn present(verifier_didname: &str, dcem: &str) -> Result<String, std::io::Error> {
    // 1. Un-e ncrypt vc
    let (holder_key, holder_jwk) = get_self_jwk_and_didkey();
    use did_key::DIDCore;
    let holder_doc = holder_key.get_did_document(did_key::CONFIG_LD_PUBLIC);
    let from_key = get_from_key_from_didcomm_message(&dcem);
    let (vc,_) = decrypt_didcomm(&from_key, &holder_key, &dcem);

    // 2. De-serialize an create verifiable presentation - vp
    let vc: ssi::vc::Credential = serde_json::from_str(&vc).unwrap();
    let vc_type = vc.type_.clone().into_iter().last().unwrap();

    let vp = serde_json::json!({
        "@context": ["https://www.w3.org/2018/credentials/v1"],
        "type": ["VerifiablePresentation", vc_type],
        "holder": holder_doc.id,
        "verifiableCredential": vc
    });

    // 2. Sign vp with holder signature
    let mut vp: ssi::vc::Presentation = serde_json::from_value(vp).unwrap();
    let mut proof_options = ssi::vc::LinkedDataProofOptions::default();
    let verification_method = holder_doc.assertion_method.unwrap()[0].clone();
    proof_options.verification_method = Some(verification_method);
    proof_options.proof_purpose = Some(ssi::vc::ProofPurpose::AssertionMethod);
    let proof = vp.generate_proof(&holder_jwk, &proof_options).await.unwrap();
    vp.add_proof(proof);

    // 3. Re-encrypt to to_key
    let vp = serde_json::to_string_pretty(&vp).unwrap();
    let verifier_key = get_other_didkey(&verifier_didname);
    let (dcem,_) = encrypt_didcomm(&holder_key, &verifier_key, &vp);

    Ok(dcem)
}

async fn verify(issuer_didname: &str, subject_didname: &str, dcem: &str) -> Result<String, std::io::Error> {
    // 0. Get keys
    let subject_key = get_other_didkey(subject_didname);
    let issuer_key = get_other_didkey(issuer_didname);
    let holder_key = get_from_key_from_didcomm_message(dcem);
    let verifier_key = get_self_didkey();

    // 1. Get dids
    use did_key::DIDCore;
    let expected_issuer_did = issuer_key.get_did_document(did_key::CONFIG_LD_PUBLIC).id;
    let expected_subject_did = subject_key.get_did_document(did_key::CONFIG_LD_PUBLIC).id;

    // 2. Decrypt vp
    let (vp, vp_id) = decrypt_didcomm(&holder_key, &verifier_key, dcem);

    // 3. Verify VP
    let vp: ssi::vc::Presentation = serde_json::from_str(&vp).unwrap();
    let result = vp.verify(None, &ssi_did_key::DIDKey).await;

    if result.errors.len() > 0 {
        return Ok(format!("Failed to verify VP: {}: {:#?}", vp_id, result))
    }

    // 4. Verify VC
    for vc in vp.verifiable_credential.unwrap().into_iter() {
        let vc: ssi::vc::Credential = match vc {
            ssi::vc::CredentialOrJWT::Credential(vc) => vc,
            ssi::vc::CredentialOrJWT::JWT(_) => panic!("verify(): Not credential. Was JWT")
        };

        let result = vc.verify(None, &ssi_did_key::DIDKey).await;
        if result.errors.len() > 0 {
            return Ok(format!("Failed to verify VP: {}: Verify credential failed: {:#?}",
                vp_id, result))
        }

        let actual_issuer_did: String = match vc.issuer.unwrap() {
            ssi::vc::Issuer::URI(s) => match s {
                ssi::vc::URI::String(s) => s
            },
            ssi::vc::Issuer::Object(s) => match s.id {
                ssi::vc::URI::String(s) => s
            },
        };
        if expected_issuer_did != actual_issuer_did {
            return Ok(format!(
                "Failed to verify VP: {}: vc.issuer.did, did not match the did of {}: Expected did: {}: Actual did: {}",
                vp_id, issuer_didname, expected_issuer_did, actual_issuer_did));
        }

        let actual_subject: &ssi::vc::CredentialSubject = vc.credential_subject.to_single().unwrap();
        let actual_subject_did = match actual_subject.id.clone().unwrap() {
            ssi::vc::URI::String(s) => s
        };
        if expected_subject_did != actual_subject_did {
            return Ok(format!(
                "Failed to verify VP: {}: vc.subject.did, did not match the did of {}: Expected did: {}: Actual did: {}",
                vp_id, subject_didname, expected_issuer_did, actual_issuer_did));
            }
    }

    Ok(dcem.to_string())
}


//
// Util
//
const ROOT_PATH: &str = "./.did/";

fn root_path() -> String {
    String::from(ROOT_PATH)
}

fn key_jwk_path() -> String {
    std::path::Path::new(ROOT_PATH)
        .join("key.jwk")
        .to_str().unwrap().to_string()
}

fn dids_path() -> String {
    std::path::Path::new(ROOT_PATH)
        .join("dids/")
        .to_str().unwrap().to_string()
}

fn did_path(did_name: &str) -> String {
    std::path::Path::new(ROOT_PATH)
        .join("dids/")
        .join(format!("{}.did", did_name))
        .to_str().unwrap().to_string()
}

fn did_names_path() -> String {
    std::path::Path::new(ROOT_PATH)
        .join("did-names/")
        .to_str().unwrap().to_string()
}

fn did_name_path(did: &str) -> String {
    std::path::Path::new(ROOT_PATH)
        .join("did-names/")
        .join(did)
        .to_str().unwrap().to_string()
}

fn messages_path() -> String {
    std::path::Path::new(ROOT_PATH)
        .join("messages/")
        .to_str().unwrap().to_string()
}

fn message_path(message_id: &str) -> String {
    std::path::Path::new(ROOT_PATH)
        .join("messages/")
        .join(format!("{}.dcem", message_id))
        .to_str().unwrap().to_string()
}

type DIDCommID = String;

/**
 * @returns (String, String) which is (didcomm encrypted message, didcomm header id)
 */
fn encrypt_didcomm(from_key: &did_key::Ed25519KeyPair, to_key: &did_key::Ed25519KeyPair, message: &str) -> (String, DIDCommID) {
    use did_key::Ecdh;

    // 1. Get dids
    use did_key::DIDCore;
    let from_did = from_key.get_did_document(did_key::CONFIG_LD_PUBLIC).id;
    let to_did = to_key.get_did_document(did_key::CONFIG_LD_PUBLIC).id;

    // 2. Map Ed25519 -> x25519
    let from_key = from_key.get_x25519();
    let to_key = to_key.get_x25519();

    // 3. Make shared secret (from -> to)
    let shared_secret = from_key.key_exchange(&to_key);

    // 4. Make didcomm message
    let to_vec = vec![&to_did[..]];

    let message = didcomm_rs::Message::new()
        .from(&from_did)
        .to(&to_vec[..])
        .timed(Some(3600))
        .body(message.as_bytes())
        .as_jwe(&didcomm_rs::crypto::CryptoAlgorithm::XC20P);

    let id = message.get_didcomm_header().id.to_string();

    // 5. Seal message using shared secret
    let dcem = message
        .seal(&shared_secret)
        .unwrap();

    (dcem, id)
}


fn decrypt_didcomm(from_key: &did_key::Ed25519KeyPair, to_key: &did_key::Ed25519KeyPair, dcem: &str)-> (String, DIDCommID) {
    use did_key::Ecdh;

    // 1. Map Ed25519 -> x25519
    let to_key = to_key.get_x25519();
    let from_key = from_key.get_x25519();

    // 2. Make shared secret (to -> from)
    let shared_secret = to_key.key_exchange(&from_key);

    // 3. Decrypt message
    let message = didcomm_rs::Message::receive(dcem, Some(&shared_secret), None);
    let message = message.unwrap();
    let id = message.get_didcomm_header().id.to_string();
    let body = String::from_utf8(message.body).unwrap();

    (body, id)
}


fn publicprivatebytes_to_jwkstr(public: Vec<u8>, private: Vec<u8>) -> String {
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


fn jwkstr_to_publicprivatebytes(jwkstr: &str) -> (Vec<u8>, Vec<u8>) {// -> (public: Vec<u8>, private: Vec<u8>)

    let jwk: ssi::jwk::JWK = serde_json::from_str(jwkstr).unwrap();
    let okp = (if let ssi::jwk::Params::OKP(o) = jwk.params.clone() {
        Some(o)
    } else {
        panic!("okp == None")
    }).unwrap();


    let privkey: Vec<u8> = if let Some(key) = okp.private_key {
        key.0
    } else {
        panic!("privkey == None")
    };

    (okp.public_key.0, privkey)
}

fn get_self_jwk_and_didkey() -> (did_key::Ed25519KeyPair, ssi::jwk::JWK) {
    let jwk = key_jwk_path();
    let jwk = std::fs::read(jwk).unwrap();
    let jwk = String::from_utf8(jwk).unwrap();

    let (_, private) = jwkstr_to_publicprivatebytes(&jwk);
    let didkey = did_key::Ed25519KeyPair::from_seed(&private);

    let jwk: ssi::jwk::JWK = serde_json::from_str(&jwk).unwrap();

    (didkey, jwk)
}

fn get_self_didkey() -> did_key::Ed25519KeyPair {
    let jwk = key_jwk_path();
    let jwk = std::fs::read(jwk).unwrap();
    let jwk = String::from_utf8(jwk).unwrap();

    let (_, private) = jwkstr_to_publicprivatebytes(&jwk);
    let self_didkey = did_key::Ed25519KeyPair::from_seed(&private);

    self_didkey
}

fn get_other_didkey(other_did_name: &str) -> did_key::Ed25519KeyPair {
    let path = did_path(other_did_name);
    let other_did = std::fs::read_to_string(path).unwrap();
    let other_didkey = did_key::resolve(&other_did).unwrap();

    use did_key::KeyMaterial;
    let other_didkey = did_key::Ed25519KeyPair::from_public_key(&other_didkey.public_key_bytes());

    other_didkey
}

fn get_from_key_from_didcomm_message(dcem: &str) -> did_key::Ed25519KeyPair {
    let from_jwe: didcomm_rs::Jwe = serde_json::from_str(&dcem).unwrap();
    let from_did = from_jwe.from().as_ref().unwrap();
    let from_key = did_key::resolve(&from_did).unwrap();

    use did_key::KeyMaterial;
    let from_key = did_key::Ed25519KeyPair::from_public_key(&from_key.public_key_bytes());

    from_key
}


//
// Config
//

#[derive(Debug)]
enum CMD {
    Help,

    // DID
    Init,
    Doc,
    Connect{ didname: String, did: String },
    Dids,
    Did{ didname: String },

    // DIDComm v2 messaging
    Write{ didname: String, message: String },
    Read{ dcem: String },
    Hold{ dcem: String },
    Messages,
    Message{ message_id: String },

    // DIDComm v2 + Verifiable Credentials
    IssuePassport{ didname: String },
    IssueDriversLicense{ didname: String },
    IssueTrafficAuthority{ didname: String },
    IssueLawEnforcer{ didname: String },
    Present{ didname: String, dcem: String },
    Verify{ issuer_didname: String, subject_didname: String, dcem: String },
}

pub struct Config {
    cmd: CMD,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, std::io::Error> {
        let default_cmd = String::from("help");

        let cmd = args.get(1).unwrap_or(&default_cmd).clone();

        let cmd = if args.len() < 2 {
            "help".to_string()
        } else {
            cmd.clone()
        };

        macro_rules! get_arg_or_return_help {
            ( $arg_number: expr ) => {
                (match args.get($arg_number) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone()
            };
        }

        let get_arg_or_read_from_stdin = |arg_number: usize| -> String {
            let arg = match args.get(arg_number) {
                Some(arg) => arg.clone(),
                None => {
                    use std::io::{Read};
                    let mut buffer = String::new();
                    let _ = std::io::stdin().read_to_string(&mut buffer);

                    buffer.trim().to_string()
                },
            };

            arg
        };

        let cmd: CMD = match &cmd[..] {
            "help" => CMD::Help,
            "init" => {
                CMD::Init
            },
            "doc" => {
                CMD::Doc
            },
            "connect" => {
                let didname = get_arg_or_return_help!(2);
                let did = get_arg_or_read_from_stdin(3);

                CMD::Connect{ didname, did }
            },
            "write" => {
                let didname = get_arg_or_return_help!(2);
                let message = get_arg_or_read_from_stdin(3);

                CMD::Write{ didname, message }
            },
            "read" => {
                let dcem = get_arg_or_read_from_stdin(2);
                CMD::Read{ dcem }
            },
            "hold" => {
                let dcem = get_arg_or_read_from_stdin(2);
                CMD::Hold{ dcem }
            },
            "issue" => {
                let credential_type = get_arg_or_return_help!(2);

                match &credential_type[..] {
                    "Passport" => {
                        let didname = get_arg_or_return_help!(3);
                        CMD::IssuePassport{ didname }
                    },
                    "TrafficAuthority" => {
                        let didname = get_arg_or_return_help!(3);
                        CMD::IssueTrafficAuthority{ didname }
                    },
                    "LawEnforcer" => {
                        let didname = get_arg_or_return_help!(3);
                        CMD::IssueLawEnforcer{ didname }
                    },
                    "DriversLicense" => {
                        let didname = get_arg_or_return_help!(3);
                        CMD::IssueDriversLicense{ didname }
                    },
                    &_ => {
                        CMD::Help
                    }
                }
            },
            "present" => {
                let didname = get_arg_or_return_help!(2);
                let dcem = get_arg_or_read_from_stdin(3);

                CMD::Present{ didname, dcem }
            },
            "verify" => {
                let issuer_didname = get_arg_or_return_help!(2);
                let subject_didname = get_arg_or_return_help!(3);
                let dcem = get_arg_or_read_from_stdin(4);

                CMD::Verify{ issuer_didname, subject_didname, dcem }
            },
            "messages" => {
                CMD::Messages
            },
            "message" => {
                let message_id = get_arg_or_read_from_stdin(2);
                CMD::Message{ message_id }
            },
            "dids" => {
                CMD::Dids
            },
            "did" => {
                let didname = get_arg_or_read_from_stdin(2);
                CMD::Did{ didname }
            },
            &_ => {
                eprintln!("{} not a valid command!", cmd);
                CMD::Help
            },
        };

        Ok(Config { cmd })
    }
}

// A DIDComm v2 encrypted message as specified by: https://identity.foundation/didcomm-messaging/spec/#didcomm-encrypted-message
#[derive(serde::Serialize, serde::Deserialize)]
struct DIDCommEncryptedMessage {
    /// JOSE header, which is sent as public part with JWE.
    #[serde(flatten)]
    pub jwm_header: didcomm_rs::JwmHeader,
    /// DIDComm headers part, sent as part of encrypted message in JWE.
    #[serde(flatten)]
    pub didcomm_header: didcomm_rs::DidcommHeader,
    /// Message payload, which can be basically anything (JSON, text, file, etc.) represented
    ///     as bytes of data.
    pub ciphertext: Vec<u8>,
}
