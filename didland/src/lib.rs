pub async fn run(config: Config) -> Result<String, std::io::Error> {
    match config.cmd {
        // Basic
        CMD::Init{} => init(),
        CMD::Doc{} => doc(),
        CMD::Did{} => did(),
        CMD::Messages{} => messages(),
        CMD::Connect{ to_did_name, did } => connect(&to_did_name, &did),
        CMD::Write{ to_did_name, message } => write(&to_did_name, &message),
        CMD::Read{ dcem } => read(&dcem),
        CMD::Help => help(),

        // Verifiable Credentials
        CMD::IssuePassport{ to_did_name } => issue_passport(&to_did_name).await,
        CMD::IssueLawEnforcer{ to_did_name } => issue_law_enforcer(&to_did_name).await,
        CMD::IssueTrafficAuthority{ to_did_name } => issue_traffic_authority(&to_did_name).await,
        CMD::IssueDriversLicense{ to_did_name } => issue_drivers_license(&to_did_name).await,
    }
}

//
// Commands: Basic
//
fn init() -> Result<String, std::io::Error> {
    use std::io::Write;

    // 1. Create empty folders
    if !std::fs::metadata(root_path()).is_ok() {
        std::fs::create_dir_all(root_path())?;
    }
    if !std::fs::metadata(names_path()).is_ok() {
        std::fs::create_dir_all(names_path())?;
    }
    if !std::fs::metadata(dids_path()).is_ok() {
        std::fs::create_dir_all(dids_path())?;
    }
    if !std::fs::metadata(messages_path()).is_ok() {
        std::fs::create_dir_all(messages_path())?;
    }

    if !std::fs::metadata(didkey_jwk_path()).is_ok() {

        // 2. Generate jwk
        let mut csprng = rand::rngs::OsRng {};
        let private_key = ed25519_dalek::SecretKey::generate(&mut csprng).to_bytes();
        let did_key = did_key::Ed25519KeyPair::from_seed(&private_key);

        use did_key::KeyMaterial;
        let jwk = publicprivatebytes_to_jwkstr(did_key.public_key_bytes(), did_key.private_key_bytes());

        // 3. Write jwk to file
        let mut file = std::fs::File::create(didkey_jwk_path()).unwrap();
        file.write(jwk.as_bytes()).unwrap();

        // 4. Connect to self
        use did_key::DIDCore;
        let did_doc = did_key.get_did_document(did_key::CONFIG_LD_PUBLIC);

        let _ = connect("self", &did_doc.id);

        Ok(format!("Is ready"))
    } else {
        Ok(format!("Already Ready"))
    }
}


fn doc() -> Result<String, std::io::Error> {
    let self_didkey = get_self_didkey()?;

    use did_key::DIDCore;
    let did_doc = self_didkey.get_did_document(did_key::CONFIG_LD_PUBLIC);
    let did_doc = serde_json::to_string_pretty(&did_doc).unwrap();

    Ok(format!("{}", did_doc))
}


fn did() -> Result<String, std::io::Error> {
    let self_didkey = get_self_didkey()?;

    use did_key::DIDCore;
    let diddoc = self_didkey.get_did_document(did_key::CONFIG_LD_PUBLIC);
    let did = diddoc.id;

    Ok(format!("{}", did))
}

fn connect(to_did_name: &str, did: &str) -> Result<String, std::io::Error> {
    use std::io::Write;

    // 1. Create 'name' -> 'did'-mapping
    let mut file = std::fs::File::create(name_path(to_did_name))?;
    file.write(did.as_bytes())?;

    // 2. Create 'did' -> 'name'-mapping
    let mut file = std::fs::File::create(did_path(did))?;
    file.write(to_did_name.as_bytes())?;

    Ok(format!("{}\n{}", name_path(to_did_name), did_path(did)))
}


fn write(to_did_name: &str, message: &str) -> Result<String, std::io::Error> {
    use std::io::Write;

    // 1. Get did:keys
    let from_key = get_self_didkey()?;
    let to_key = get_other_didkey(to_did_name)?;
    
    // 2. Encrypt message with from_key, to keep message history in local file
    let dcem = encrypt_didcomm(&from_key, &from_key, message).unwrap();
    let mut file = std::fs::File::create(message_path())?;
    file.write(dcem.as_bytes())?;

    // 3. Encrypt message with to_key, to prepare it for transmission
    let dcem = encrypt_didcomm(&from_key, &to_key, message).unwrap();

    Ok(format!("{}", &dcem))
}


fn read(dcem: &str) -> Result<String, std::io::Error> {
    use std::io::Write;

    // 1. Store incomming message to file, to keep the message history
    let message_fpath = message_path();
    let message_fpath = std::path::Path::new(&message_fpath);
    let mut file = std::fs::File::create(message_fpath)?;
    file.write(dcem.as_bytes())?;

    // 2. Get did:keys
    let to_key = get_self_didkey()?;
    let from_key = get_from_didkey(dcem);

    // 3. Decrypt message
    let decrypted = decrypt_didcomm(&from_key, &to_key, dcem).unwrap();

    // 4. Format
    use did_key::DIDCore;
    let from_did = from_key.get_did_document(did_key::CONFIG_LD_PUBLIC).id;
    let from_name = std::fs::read_to_string(did_path(&from_did))
        .unwrap_or(from_did.clone());
    let filename = &message_fpath.file_name().unwrap().to_str().unwrap();

    Ok(format!("[{}] {} > {}", filename, from_name, decrypted))
}


fn messages() -> Result<String, std::io::Error> {
    let mut result = String::from("");

    let mut entries: Vec<std::fs::DirEntry> = std::fs::read_dir(messages_path()).unwrap().filter_map(|f| f.ok()).collect();
    entries.sort_by_key(|e| e.path());

    // 1. Get to-key
    let to_key = get_self_didkey()?;

    for entry in entries {
        if entry.path().is_dir() {
            continue;
        }
        let dcem = std::fs::read_to_string(entry.path())?;

        // 2. Get from-didkey
        let from_key = get_from_didkey(&dcem);
        use did_key::DIDCore;
        let from_did = from_key.get_did_document(did_key::CONFIG_LD_PUBLIC).id;

        // 3. Decrypt message
        let decrypted = decrypt_didcomm(&from_key, &to_key, &dcem).unwrap();

        // 4. Format
        let from_name = std::fs::read_to_string(did_path(&from_did))
            .unwrap_or(from_did.clone());
        let file_name = String::from(entry.file_name().to_str().unwrap());

        result.push_str(&format!("[{}] {} > {}\n", file_name, from_name, decrypted));
    }

    Ok(result)
}

fn help() -> Result<String, std::io::Error> {
    Ok(String::from("
    Usage:
        didland <command> <args>
        didland init
        didland did
        didland doc
        didland connect  <did name> <did>
    
    Basic Didcomm Messaging:
        didland write    <to did name> <message>  -->  <dcem>
        didland read     <dcem>                   -->  <from did name> <message>
        didland messages

    Verifiable Credentials:
        didland issue-passport          <to did name>  -->  <dcem>
        didland issue-traffic-authority <to did name>  -->  <dcem>
        didland issue-law-enforcer      <to did name>  -->  <dcem>
        didland issue-drivers-license   <to did name>  -->  <dcem>

        didland hold <credential name> <dcem>
        didland credentials
"))
}

//
// Commands: Verifiable credentials
//
async fn issue_passport(to_did_name: &str) -> Result<String, std::io::Error> {
    let (from_didkey, from_jwk) = get_self_jwk_and_didkey()?;
    let to_didkey = get_other_didkey(to_did_name)?;

    let credential = issue_verifiable_credential("Passport", &from_jwk, &from_didkey, &to_didkey);
    Ok(credential.await) 
}

async fn issue_drivers_license(to_did_name: &str) -> Result<String, std::io::Error> { 
    let (from_didkey, from_jwk) = get_self_jwk_and_didkey()?;
    let to_didkey = get_other_didkey(to_did_name)?;
    
    let credential = issue_verifiable_credential("DriversLicense", &from_jwk, &from_didkey, &to_didkey);
    Ok(credential.await) 
}

async fn issue_traffic_authority(to_did_name: &str) -> Result<String, std::io::Error> { 
    let (from_didkey, from_jwk) = get_self_jwk_and_didkey()?;
    let to_didkey = get_other_didkey(to_did_name)?;
    
    let credential = issue_verifiable_credential("TrafficAuthority", &from_jwk, &from_didkey, &to_didkey);
    Ok(credential.await) 
}

async fn issue_law_enforcer(to_did_name: &str) -> Result<String, std::io::Error> { 
    let (from_didkey, from_jwk) = get_self_jwk_and_didkey()?;
    let to_didkey = get_other_didkey(to_did_name)?;
    
    let credential = issue_verifiable_credential("LawEnforcer", &from_jwk, &from_didkey, &to_didkey);
    Ok(credential.await) 
}

async fn issue_verifiable_credential(
    credential_type: &str, 
    issuer_jwk: &ssi::jwk::JWK,
    issuer_didkey: &did_key::Ed25519KeyPair,
    subject_didkey: &did_key::Ed25519KeyPair, 
) -> String {
    use did_key::DIDCore;
    // 1. Get did docs
    let issuer_doc = issuer_didkey.get_did_document(did_key::CONFIG_LD_PUBLIC);
    let subject_doc = subject_didkey.get_did_document(did_key::CONFIG_LD_PUBLIC);

    // 2. Construct unsigned vc
    let vc = serde_json::json!({
        "@context": [
            "https://www.w3.org/2018/credentials/v1", 
        ],
        "type": credential_type,
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

    // 4. Generate proof, using issuer jwk + proof options
    let proof = vc.generate_proof(issuer_jwk, &proof_options).await.unwrap();
    vc.add_proof(proof);

    // 5. Make pretty
    let vc = serde_json::to_string_pretty(&vc).unwrap();

    vc
}

//
// Util
//
const ROOT_PATH: &str = "./.didland/";

fn root_path() -> String {
    String::from(ROOT_PATH)
}


fn didkey_jwk_path() -> String {
    let path = std::path::Path::new(ROOT_PATH)
        .join("didkey.jwk");

    match path.to_str() {
        None => panic!("didkey_jwk_path({:?}) is not a valid UTF-8 sequence", path),
        Some(s) => s.to_string(),
    }
}


fn names_path() -> String {
    let path = std::path::Path::new(ROOT_PATH)
        .join("names/");

    match path.to_str() {
        None => panic!("names_path({:?}) is not a valid UTF-8 sequence", path),
        Some(s) => s.to_string(),
    }
}


fn name_path(name: &str) -> String {
    let path = std::path::Path::new(ROOT_PATH)
        .join("names/")
        .join(name);

    match path.to_str() {
        None => panic!("name_path({:?}, {}) is not a valid UTF-8 sequence", path, name),
        Some(s) => s.to_string(),
    }
}


fn dids_path() -> String {
    let path = std::path::Path::new(ROOT_PATH)
        .join("dids/");

    match path.to_str() {
        None => panic!("dids_paths({:?}) is not a valid UTF-8 sequence", path),
        Some(s) => s.to_string(),
    }
}


fn did_path(did: &str) -> String {
    let path = std::path::Path::new(ROOT_PATH)
        .join("dids/")
        .join(did);

    match path.to_str() {
        None => panic!("did_path({:?}, {}) is not a valid UTF-8 sequence", path, did),
        Some(s) => s.to_string(),
    }
}


fn messages_path() -> String {
    let path = std::path::Path::new(ROOT_PATH)
        .join("messages/");

    match path.to_str() {
        None => panic!("messages_path({:?}) is not a valid UTF-8 sequence", path),
        Some(s) => s.to_string(),
    }
}


fn message_path() -> String {
    let start = std::time::SystemTime::now();
    let since_epoch = start
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards").as_secs();

    let path = std::path::Path::new(ROOT_PATH)
        .join(format!("messages/{}.dcem", since_epoch));

    match path.to_str() {
        None => panic!("message_path({:?}, {}) is not a valid UTF-8 sequence", path, since_epoch),
        Some(s) => s.to_string(),
    }
}


fn encrypt_didcomm(from_key: &did_key::Ed25519KeyPair, to_key: &did_key::Ed25519KeyPair, message: &str) -> Result<String, didcomm_rs::Error> {
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

    // 5. Seal message using shared secret
    let dcem = message
        .seal(&shared_secret)
        .unwrap();

    Ok(dcem)
}


fn decrypt_didcomm(from_key: &did_key::Ed25519KeyPair, to_key: &did_key::Ed25519KeyPair, dcem: &str)-> Result<String, didcomm_rs::Error> {
    use did_key::Ecdh;

    // 1. Map Ed25519 -> x25519
    let to_key = to_key.get_x25519();
    let from_key = from_key.get_x25519();

    // 2. Make shared secret (to -> from)
    let shared_secret = to_key.key_exchange(&from_key);

    // 3. Decrypt message
    let decrypted = didcomm_rs::Message::receive(dcem, Some(&shared_secret), None);
    let decrypted = decrypted.unwrap();
    let decrypted = String::from_utf8(decrypted.body).unwrap();

    Ok(decrypted)
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

fn get_self_jwk_and_didkey() -> Result<(did_key::Ed25519KeyPair, ssi::jwk::JWK), std::io::Error> {
    let jwk = didkey_jwk_path();
    let jwk = std::fs::read(jwk)?;
    let jwk = String::from_utf8(jwk).unwrap();

    let (_, private) = jwkstr_to_publicprivatebytes(&jwk);
    let didkey = did_key::Ed25519KeyPair::from_seed(&private);
    
    let jwk: ssi::jwk::JWK = serde_json::from_str(&jwk).unwrap();

    Ok((didkey, jwk))
}

fn get_self_didkey() -> Result<did_key::Ed25519KeyPair, std::io::Error> {
    let jwk = didkey_jwk_path();
    let jwk = std::fs::read(jwk)?;
    let jwk = String::from_utf8(jwk).unwrap();

    let (_, private) = jwkstr_to_publicprivatebytes(&jwk);
    let self_didkey = did_key::Ed25519KeyPair::from_seed(&private);
    
    Ok(self_didkey)
}

fn get_other_didkey(other_did_name: &str) -> Result<did_key::Ed25519KeyPair, std::io::Error> {
    let to_did = std::fs::read_to_string(name_path(other_did_name))?;
    let to_key = did_key::resolve(&to_did).unwrap();

    use did_key::KeyMaterial;
    let to_didkey = did_key::Ed25519KeyPair::from_public_key(&to_key.public_key_bytes());

    Ok(to_didkey)
}

fn get_from_didkey(dcem: &str) -> did_key::Ed25519KeyPair {

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
    // Basic
    Init{  },
    Doc{  },
    Did{  },
    Messages{  },
    Connect{ to_did_name: String, did: String },
    Write{ to_did_name: String, message: String },
    Read{ dcem: String },
    Help,

    // Verifiable Credentials
    IssuePassport{ to_did_name: String },
    IssueDriversLicense{ to_did_name: String },
    IssueTrafficAuthority{ to_did_name: String },
    IssueLawEnforcer{ to_did_name: String },
}

pub struct Config {
    cmd: CMD,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, std::io::Error> {
        let default_cmd = String::from("help");

        let cmd = args.get(1).unwrap_or(&default_cmd).clone();

        let cmd = if args.len() < 2 {
            eprintln!("Command missing!");
            default_cmd.clone()
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

        let cmd: CMD = match &cmd[..] {
            "did" => {
                CMD::Did{ }
            },
            "doc" => {
                CMD::Doc{ }
            },
            "init" => {
                CMD::Init{ }
            },
            "messages" => {
                CMD::Messages{ }
            },
            "connect" => {
                let to_did_name = get_arg_or_return_help!(2);
                let did = get_arg_or_return_help!(3);

                CMD::Connect{ to_did_name, did }
            },
            "write" => {
                let to_did_name = get_arg_or_return_help!(2);
                let message = get_arg_or_return_help!(3);

                CMD::Write{ to_did_name, message }
            },
            "read" => {
                let dcem = get_arg_or_return_help!(2);

                CMD::Read{ dcem }
            },
            "issue-passport" => {
                let to_did_name = get_arg_or_return_help!(2);
                CMD::IssuePassport{ to_did_name }
            },
            "issue-traffic-authority" => {
                let to_did_name = get_arg_or_return_help!(2);
                CMD::IssueTrafficAuthority{ to_did_name }
            },
            "issue-law-enforcer" => {
                let to_did_name = get_arg_or_return_help!(2);
                CMD::IssueLawEnforcer{ to_did_name }
            },
            "issue-drivers-license" => {
                let to_did_name = get_arg_or_return_help!(2);
                CMD::IssueDriversLicense{ to_did_name }
            },
            "help" => CMD::Help,
            &_ => {
                eprintln!("{} not a valid command!", cmd);
                CMD::Help
            },
        };

        Ok(Config { cmd })
    }
}
