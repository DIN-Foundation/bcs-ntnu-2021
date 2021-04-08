pub fn run(config: Config) -> Result<String, std::io::Error> {
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
        CMD::IssuePassport{ to_did_name } => issue_passport(&to_did_name),
        CMD::IssueLawEnforcer{ to_did_name } => issue_law_enforcer(&to_did_name),
        CMD::IssueTrafficAuthority{ to_did_name } => issue_traffic_authority(&to_did_name),
        CMD::IssueDriversLicense{ to_did_name } => issue_drivers_license(&to_did_name),
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
    use did_key::DIDCore;

    // 1. Read jwk from file
    let jwk = std::fs::read(didkey_jwk_path())?;
    let jwkstr = String::from_utf8(jwk).unwrap();
    let (public,_) = jwkstr_to_publicprivatebytes(&jwkstr);

    // 2. Transform public key to a did-document
    let keypair = did_key::from_existing_key::<did_key::Ed25519KeyPair>(&public, None);
    let did_doc = keypair.get_did_document(did_key::CONFIG_LD_PUBLIC);

    // 3. Serialize did-document to json
    let did_doc = serde_json::to_string_pretty(&did_doc).unwrap();
    Ok(format!("{}", did_doc))
}


fn did() -> Result<String, std::io::Error> {
    use did_key::DIDCore;

    // 1. Read jwk from file
    let jwk = std::fs::read(didkey_jwk_path())?;
    let jwkstr = String::from_utf8(jwk).unwrap();
    let (public,_) = jwkstr_to_publicprivatebytes(&jwkstr);

    // 2. Transform public key to a did-document
    let keypair = did_key::from_existing_key::<did_key::Ed25519KeyPair>(&public, None);
    let did_doc = keypair.get_did_document(did_key::CONFIG_LD_PUBLIC);

    let did = did_doc.id;

    // 3. Print did
    Ok(format!("{}", did))
}

fn connect(to_did_name: &str, did: &str) -> Result<String, std::io::Error> {
    use std::io::Write;

    // 2. Create 'name' -> 'did' mapping
    let mut file = std::fs::File::create(name_path(to_did_name)).unwrap();
    file.write(did.as_bytes()).unwrap();

    // 3. Create 'did' to 'name' mapping
    let mut file = std::fs::File::create(did_path(did)).unwrap();
    file.write(to_did_name.as_bytes()).unwrap();

    Ok(format!("{}\n{}", name_path(to_did_name), did_path(did)))
}


fn write(to_did_name: &str, message: &str) -> Result<String, std::io::Error> {
    use did_key::KeyMaterial;
    use std::io::Write;

    // 1. Read from-key
    let jwk = std::fs::read(didkey_jwk_path())?;
    let jwkstr = String::from_utf8(jwk).unwrap();
    let (_, private) = jwkstr_to_publicprivatebytes(&jwkstr);
    let from_key = did_key::Ed25519KeyPair::from_seed(&private);

    // 2. Read to-key
    let to_did = std::fs::read_to_string(name_path(to_did_name)).unwrap();
    let to_key = did_key::resolve(&to_did).unwrap();
    let to_key = did_key::Ed25519KeyPair::from_public_key(&to_key.public_key_bytes());

    // 3. Encrypt message with from_key, to keep message history in local file
    let dcem = encrypt_didcomm(&from_key, &from_key, message).unwrap();
    let mut file = std::fs::File::create(message_path()).unwrap();
    file.write(dcem.as_bytes()).unwrap();

    // 4. Encrypt message with to_key, to prepare it for transmission
    let dcem = encrypt_didcomm(&from_key, &to_key, message).unwrap();

    Ok(format!("{}", &dcem))
}


fn read(dcem: &str) -> Result<String, std::io::Error> {
    use std::io::Write;
    use did_key::KeyMaterial;

    // 1. Store incomming message to file, to keep the message history
    let message_fpath = message_path();
    let message_fpath = std::path::Path::new(&message_fpath);
    let mut file = std::fs::File::create(message_fpath).unwrap();
    file.write(dcem.as_bytes()).unwrap();

    // 2. Get to-key
    let jwk = std::fs::read(didkey_jwk_path())?;
    let jwkstr = String::from_utf8(jwk).unwrap();
    let (_, private) = jwkstr_to_publicprivatebytes(&jwkstr);
    let to_key = did_key::Ed25519KeyPair::from_seed(&private);

    // 3. Get from-key
    let from_jwe: didcomm_rs::Jwe = serde_json::from_str(&dcem).unwrap();
    let from_did = from_jwe.from().as_ref().unwrap();
    let from_key = did_key::resolve(&from_did).unwrap();
    let from_key = did_key::Ed25519KeyPair::from_public_key(&from_key.public_key_bytes());

    // 4. Decrypt message
    let decrypted = decrypt_didcomm(&from_key, &to_key, dcem).unwrap();

    // 5. Format
    let from_name = std::fs::read_to_string(did_path(from_did))
        .unwrap_or(from_did.clone());
    let filename = &message_fpath.file_name().unwrap().to_str().unwrap();
    Ok(format!("[{}] {} > {}", filename, from_name, decrypted))
}


fn messages() -> Result<String, std::io::Error> {
    use did_key::KeyMaterial;

    let mut result = String::from("");

    let mut entries: Vec<std::fs::DirEntry> = std::fs::read_dir(messages_path()).unwrap().filter_map(|f| f.ok()).collect();
    entries.sort_by_key(|e| e.path());

    // 1. Get to-key
    let jwk = std::fs::read(didkey_jwk_path())?;
    let jwkstr = String::from_utf8(jwk).unwrap();
    let (_, private) = jwkstr_to_publicprivatebytes(&jwkstr);
    let to_key = did_key::Ed25519KeyPair::from_seed(&private);

    for entry in entries {
        if entry.path().is_dir() {
            continue;
        }
        let dcem = std::fs::read_to_string(entry.path())?;

        // 2. Get from-key
        let from_jwe: didcomm_rs::Jwe = serde_json::from_str(&dcem).unwrap();
        let from_did = from_jwe.from().as_ref().unwrap();
        let from_key = did_key::resolve(&from_did).unwrap();
        let from_key = did_key::Ed25519KeyPair::from_public_key(&from_key.public_key_bytes());

        // 3. Decrypt message
        let decrypted = decrypt_didcomm(&from_key, &to_key, &dcem).unwrap();

        // 4. Format
        let from_name = std::fs::read_to_string(did_path(from_did))
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
fn issue_passport(to_did_name: &str) -> Result<String, std::io::Error> { 
    Ok(String::new()) 
}
fn issue_drivers_license(to_did_name: &str) -> Result<String, std::io::Error> { 
    Ok(String::new()) 
}
fn issue_traffic_authority(to_did_name: &str) -> Result<String, std::io::Error> { 
    Ok(String::new()) 
}
fn issue_law_enforcer(to_did_name: &str) -> Result<String, std::io::Error> { 
    Ok(String::new()) 
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
    use did_key::DIDCore;

    // 1. Get dids
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
