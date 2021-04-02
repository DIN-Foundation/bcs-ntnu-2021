pub fn run(config: Config) -> Result<String, std::io::Error> {
    match config.cmd {
        CMD::Init{ path } => init(&path),
        CMD::Doc{ path } => doc(&path),
        CMD::Did{ path } => did(&path),
        CMD::Help => help()
    }
}


fn init(path: &str) -> Result<String, std::io::Error> {
    use std::io::Write;

    // 1. Create empty folders
    if !std::fs::metadata(root_path(path)).is_ok() {
        std::fs::create_dir_all(root_path(path))?;
    }

    if !std::fs::metadata(jwk_path(path)).is_ok() {
        // 2. Generate seed
        use did_key::KeyMaterial;
        let mut csprng = rand::rngs::OsRng {};
        let private_key = ed25519_dalek::SecretKey::generate(&mut csprng).to_bytes();
        let did_key = did_key::Ed25519KeyPair::from_seed(&private_key);
        
        let jwk = publicprivatebytes_to_jwkstr(did_key.public_key_bytes(), did_key.private_key_bytes());
        
        // 3. Write seed to file
        let mut file = std::fs::File::create(jwk_path(path)).unwrap();
        file.write(jwk.as_bytes()).unwrap();
        
        Ok(format!("{} is ready", path))
    } else {
        Ok(format!("{} already exists", path))
    }
}


fn doc(path: &str) -> Result<String, std::io::Error> {
    use did_key::DIDCore;
    
    // 1. Read jwk from file
    let jwk = std::fs::read(jwk_path(path))?;
    let jwkstr = String::from_utf8(jwk).unwrap();
    let (public,_) = jwkstr_to_publicprivatebytes(&jwkstr);

    // 2. Transform public key to a did-document
    let keypair = did_key::from_existing_key::<did_key::Ed25519KeyPair>(&public, None);
    let did_doc = keypair.get_did_document(did_key::CONFIG_LD_PUBLIC);

    // 3. Serialize did-document to json
    let did_doc = serde_json::to_string_pretty(&did_doc).unwrap();
    Ok(format!("{}", did_doc))
}


fn did(path: &str) -> Result<String, std::io::Error> {
    use did_key::DIDCore;

    // 1. Read jwk from file
    let jwk = std::fs::read(jwk_path(path))?;
    let jwkstr = String::from_utf8(jwk).unwrap();
    let (public,_) = jwkstr_to_publicprivatebytes(&jwkstr);

    // 2. Transform public key to a did-document
    let keypair = did_key::from_existing_key::<did_key::Ed25519KeyPair>(&public, None);
    let did_doc = keypair.get_did_document(did_key::CONFIG_LD_PUBLIC);

    let did = did_doc.id;

    // 3. Print did
    Ok(format!("{}", did))
}

fn help() -> Result<String, std::io::Error> {
    Ok(String::from("
    Usage:
        didchat <path> <command>
        
        didchat <path> init
        didchat <path> doc    
        didchat <path> did    
"))
}


#[derive(Debug)]
enum CMD {
    Init{ path: String },
    Doc{ path: String },
    Did{ path: String },
    Help
}

pub struct Config {
    cmd: CMD,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, std::io::Error> {
        let default_cmd = String::from("help");
        
        let path = args.get(1).unwrap_or(&default_cmd).clone();
        let cmd = args.get(2).unwrap_or(&default_cmd).clone();

        let cmd = if args.len() < 3 {
            eprintln!("Command missing!");
            default_cmd.clone()
        } else {
            cmd.clone()
        };

        let cmd: CMD = match &cmd[..] {
            "did" => {
                CMD::Did{ path }
            },
            "doc" => {
                CMD::Doc{ path }
            },
            "init" => {
                CMD::Init{ path }
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

fn root_path(path: &str) -> String {
    format!("{}/.didchat", path)
}

fn jwk_path(path: &str) -> String {
    format!("{}/.didchat/me.jwk", path)
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