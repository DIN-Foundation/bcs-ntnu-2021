pub async fn run(config: Config) -> Result<String, std::io::Error> {
    match config.cmd {
        // Basic
        CMD::DID => did(),
        CMD::Doc => doc(),
        CMD::Connect{ connection_id, did } => connect(&connection_id, &did),
        CMD::Write{ connection_id, message } => write(&connection_id, &message),
        CMD::Read{ dcem } => read(&dcem),
        CMD::Help => help(),

        // Verifiable Credentials
        CMD::IssuePassport{ connection_id } => issue("Passport", &connection_id).await,
        CMD::IssueLawEnforcer{ connection_id } => issue("LawEnforcer", &connection_id).await,
        CMD::IssueTrafficAuthority{ connection_id } => issue("TrafficAuthority", &connection_id).await,
        CMD::IssueDriversLicense{ connection_id } => issue("DriversLicense", &connection_id).await,
        CMD::Hold{ dcem } => hold(&dcem),
        CMD::Present{ credential_id, connection_id } => present(&credential_id, &connection_id).await,
        CMD::Verify{ issuer_connection_id, subject_connection_id, dcem } => verify(&issuer_connection_id, &subject_connection_id, &dcem).await,

        // View wallet data
        CMD::Messages => messages(),
        CMD::Message{ message_id } => message(&message_id),
        CMD::Connections => connections(),
        CMD::Connection{ connection_id } => connection(&connection_id),
        CMD::Credentials => credentials(),
        CMD::Credential{ credential_id } => credential(&credential_id),
        CMD::Presentations => presentations(),
        CMD::Presentation{ presentation_id } => presentation(&presentation_id),
    }
}

fn help() -> Result<String, std::io::Error> {
    Ok(String::from("
    Basic:
        did
        did doc
        did connect <connection id> <did>

    DIDComm v2 messaging:
        did write  <connection id> <message>  -->  <dcem>
        did read   <dcem>                     -->  <message id>

    DIDComm v2 + Verifiable Credentials:
        did issue   Passport         <connection id>  -->  <dcem>
        did issue   DriversLicense   <connection id>  -->  <dcem>
        did issue   TrafficAuthority <connection id>  -->  <dcem>
        did issue   LawEnforcer      <connection id>  -->  <dcem>
        did hold    <dcem>                            -->  <credential id>
        did present <credential id>  <connection id>  -->  <dcem>
        did verify  <issuer connection id> <subject connection id> <dcem>  -->  <presentation id>

    View wallet data:
        did messages
        did message <message id>
        did connections
        did connection <connection id>
        did credentials
        did credential <credential id>
        did presentations
        did presentation <presentation id>
"))
}

//
// Commands: Basic
//
fn did() -> Result<String, std::io::Error> {
    use std::io::Write;

    // 1. Create empty folders, if not exists
    if !std::fs::metadata(root_path()).is_ok() {
        std::fs::create_dir_all(root_path()).unwrap();
    }
    if !std::fs::metadata(connections_path()).is_ok() {
        std::fs::create_dir_all(connections_path()).unwrap();
    }
    if !std::fs::metadata(dids_path()).is_ok() {
        std::fs::create_dir_all(dids_path()).unwrap();
    }
    if !std::fs::metadata(messages_path()).is_ok() {
        std::fs::create_dir_all(messages_path()).unwrap();
    }
    if !std::fs::metadata(credentials_path()).is_ok() {
        std::fs::create_dir_all(credentials_path()).unwrap();
    }
    if !std::fs::metadata(presentations_path()).is_ok() {
        std::fs::create_dir_all(presentations_path()).unwrap();
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

fn connect(connection_id: &str, did: &str) -> Result<String, std::io::Error> {
    use std::io::Write;

    // 1. Create 'name' -> 'did'-mapping
    let mut file = std::fs::File::create(connection_path(connection_id)).unwrap();
    file.write(did.as_bytes()).unwrap();

    // 2. Create 'did' -> 'name'-mapping
    let mut file = std::fs::File::create(did_path(did)).unwrap();
    file.write(connection_id.as_bytes()).unwrap();

    Ok(format!("{}\n{}", connection_path(connection_id), did_path(did)))
}


fn write(to_did_name: &str, message: &str) -> Result<String, std::io::Error> {
    use std::io::Write;

    // 1. Get did:keys
    let from_key = get_self_didkey();
    let to_key = get_other_didkey(to_did_name);

    // 2. Encrypt message with from_key, to keep message history in local file
    let (dcem, dcem_id) = encrypt_didcomm(&from_key, &from_key, message);
    let mut file = std::fs::File::create(message_path(&dcem_id)).unwrap();
    file.write(dcem.as_bytes()).unwrap();

    // 3. Encrypt message with to_key, to prepare it for transmission
    let (dcem, _) = encrypt_didcomm(&from_key, &to_key, message);

    Ok(format!("{}", &dcem))
}


fn read(dcem: &str) -> Result<String, std::io::Error> {
    use std::io::Write;

    // 1. Get did:keys
    let to_key = get_self_didkey();
    let from_key = get_from_key_from_didcomm_message(dcem);

    // 2. Decrypt message
    let (decrypted, id) = decrypt_didcomm(&from_key, &to_key, dcem);

    // 3. Store incomming message to file, to keep the message history
    let path = message_path(&id);
    let path = std::path::Path::new(&path);
    let mut file = std::fs::File::create(path).unwrap();
    file.write(dcem.as_bytes()).unwrap();

    // 4. Format message
    use did_key::DIDCore;
    let from_did = from_key.get_did_document(did_key::CONFIG_LD_PUBLIC).id;
    let from_name = std::fs::read_to_string(did_path(&from_did))
        .unwrap_or(from_did.clone());
    let filename = &path.file_name().unwrap().to_str().unwrap();

    Ok(format!("[{}] {} > {}", filename, from_name, decrypted))
}


//
// Commands: Verifiable credentials
//
async fn issue(credential_type: &str, to_did_name: &str) -> Result<String, std::io::Error> {
    // 1. Get did docs
    let (issuer_didkey, issuer_jwk) = get_self_jwk_and_didkey();
    let subject_didkey = get_other_didkey(to_did_name);

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

    // 5. Serialize and encrypt with issuer_didkey
    let vc = serde_json::to_string_pretty(&vc).unwrap();
    let (dcem, dcem_id) = encrypt_didcomm(&issuer_didkey, &issuer_didkey, &vc.clone());

    // 6. Store vc to file
    let mut file = std::fs::File::create(credential_path(&dcem_id)).unwrap();
    use std::io::Write;
    file.write(dcem.as_bytes()).unwrap();

    // 7. Serialize and encrypt with subject_didkey
    let (dcem,_) = encrypt_didcomm(&issuer_didkey, &subject_didkey, &vc.clone());

    Ok(dcem)
}

fn hold(dcem: &str) -> Result<String, std::io::Error> {
    // 1. Get did:keys
    let to_key = get_self_didkey();
    let from_key = get_from_key_from_didcomm_message(dcem);

    // 2. Decrypt message, to check if it is intendend for me
    let (_,credential_id) = decrypt_didcomm(&from_key, &to_key, dcem);

    // 3. Store incomming credential to file
    let mut file = std::fs::File::create(credential_path(&credential_id)).unwrap();
    use std::io::Write;
    file.write(dcem.as_bytes()).unwrap();

    Ok(credential_id)
}

async fn present(credential_id: &str, to_did_name: &str) -> Result<String, std::io::Error> {
    // 0. Read from file
    let credential_path = credential_path(&credential_id);
    let credential_path = std::path::Path::new(&credential_path);
    let dcem = std::fs::read_to_string(credential_path).unwrap();

    // 1. Un-encrypt
    let (holder_key, holder_jwk) = get_self_jwk_and_didkey();

    use did_key::DIDCore;
    let holder_doc = holder_key.get_did_document(did_key::CONFIG_LD_PUBLIC);
    let from_key = get_from_key_from_didcomm_message(&dcem);
    let (vc,_) = decrypt_didcomm(&from_key, &holder_key, &dcem);

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

    // 3. Re-encrypt to holder_key
    let vp = serde_json::to_string_pretty(&vp).unwrap();
    let (dcem, dcem_id) = encrypt_didcomm(&holder_key, &holder_key, &vp);

    // 4. Store outgoing presentation to file
    let mut file = std::fs::File::create(presentation_path(&dcem_id)).unwrap();
    use std::io::Write;
    file.write(dcem.as_bytes()).unwrap();

    // 5. Re-encrypt to to_key
    let to_key = get_other_didkey(&to_did_name);
    let (dcem,_) = encrypt_didcomm(&holder_key, &to_key, &vp);

    Ok(dcem)
}

async fn verify(issuer_connection_id: &str, subject_connection_id: &str, dcem: &str) -> Result<String, std::io::Error> {
    // 0. Get keys
    let subject_key = get_other_didkey(subject_connection_id);
    let issuer_key = get_other_didkey(issuer_connection_id);
    let holder_key = get_from_key_from_didcomm_message(dcem);
    let verifier_key = get_self_didkey();

    // 1. Get dids
    use did_key::DIDCore;
    let expected_issuer_did = issuer_key.get_did_document(did_key::CONFIG_LD_PUBLIC).id;
    let expected_subject_did = subject_key.get_did_document(did_key::CONFIG_LD_PUBLIC).id;

    // 2. Decrypt vp
    let (vp, vp_id) = decrypt_didcomm(&holder_key, &verifier_key, dcem);

    // 3. Store vp to file
    let mut file = std::fs::File::create(presentation_path(&vp_id)).unwrap();
    use std::io::Write;
    file.write(dcem.as_bytes()).unwrap();

    // 4. Verify VP
    let vp: ssi::vc::Presentation = serde_json::from_str(&vp).unwrap();
    let result = vp.verify(None, &ssi_did_key::DIDKey).await;

    if result.errors.len() > 0 {
        return Ok(format!("Failed to verify VP: {}: {:#?}", vp_id, result))
    }

    // 5. Verify VC
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
                vp_id, issuer_connection_id, expected_issuer_did, actual_issuer_did));
        }

        let actual_subject: &ssi::vc::CredentialSubject = vc.credential_subject.to_single().unwrap();
        let actual_subject_did = match actual_subject.id.clone().unwrap() {
            ssi::vc::URI::String(s) => s
        };
        if expected_subject_did != actual_subject_did {
            return Ok(format!(
                "Failed to verify VP: {}: vc.subject.did, did not match the did of {}: Expected did: {}: Actual did: {}",
                vp_id, subject_connection_id, expected_issuer_did, actual_issuer_did));
            }
    }

    Ok(vp_id)
}

//
// Commands: For viewing data at rest
//
fn messages() -> Result<String, std::io::Error> {
    let mut list = format!("{:24.24}{:16.16}{}\n", "ID", "From", "Text");

    let mut entries: Vec<std::fs::DirEntry> = std::fs::read_dir(messages_path()).unwrap().filter_map(|f| f.ok()).collect();
    entries.sort_by_key(|e| e.path());

    // 1. Get to-key
    let to_key = get_self_didkey();

    for entry in entries {
        if entry.path().is_dir() {
            continue;
        }
        let dcem = std::fs::read_to_string(entry.path()).unwrap();

        // 2. Get from-didkey
        let from_key = get_from_key_from_didcomm_message(&dcem);
        use did_key::DIDCore;
        let from_did = from_key.get_did_document(did_key::CONFIG_LD_PUBLIC).id;

        // 3. Decrypt message
        let (message, message_id) = decrypt_didcomm(&from_key, &to_key, &dcem);

        // 4. Format
        let from_name = std::fs::read_to_string(did_path(&from_did))
            .unwrap_or("".to_string());

        list.push_str(&format!("{:24.24}{:16.16}{}\n", message_id, from_name, message));
    }

    Ok(list)
}

fn message(message_id: &str) -> Result<String, std::io::Error> {
    let dcem = std::fs::read_to_string(message_path(message_id)).unwrap();
    let from_key = get_from_key_from_didcomm_message(&dcem);
    let self_key = get_self_didkey();
    let (message,_) = decrypt_didcomm(&from_key, &self_key, &dcem);

    Ok(message)
}

fn credentials() -> Result<String, std::io::Error> {
    let mut result = format!("{:24.24}{:24.24}{:16.16}{:16.16}\n", "ID", "Type", "Issuer", "Subject");

    let mut entries: Vec<std::fs::DirEntry> = std::fs::read_dir(credentials_path())
        .unwrap()
        .filter_map(|f| f.ok()).collect();

    entries.sort_by_key(|e| e.path());

    // 1. Get to-key
    let to_key = get_self_didkey();

    for entry in entries {
        if entry.path().is_dir() {
            continue;
        }
        let dcem = std::fs::read_to_string(entry.path()).unwrap();

        // 2. Get from-didkey
        let from_key = get_from_key_from_didcomm_message(&dcem);

        // 3. Decrypt message
        let (vc, vc_id) = decrypt_didcomm(&from_key, &to_key, &dcem);
        let vc: ssi::vc::Credential = serde_json::from_str(&vc).unwrap();

        // 4. Get issuer and subject name
        let issuer_did: String = match vc.issuer.unwrap() {
            ssi::vc::Issuer::URI(s) => match s {
                ssi::vc::URI::String(s) => s
            },
            ssi::vc::Issuer::Object(s) => match s.id {
                ssi::vc::URI::String(s) => s
            },
        };
        let issuer_name = std::fs::read_to_string(did_path(&issuer_did))
            .unwrap_or(issuer_did.clone());

        let subject: &ssi::vc::CredentialSubject = vc.credential_subject.to_single().unwrap();
        let subject_did = match subject.id.clone().unwrap() { ssi::vc::URI::String(s) => s };
        let subject_name = std::fs::read_to_string(did_path(&subject_did))
            .unwrap_or(subject_did.clone());

        // 5. Format
        result.push_str(&format!("{:24.24}{:24.24}{:16.16}{:16.16}\n",
            vc_id,
            vc.type_.into_iter().last().unwrap(),
            issuer_name,
            subject_name
        ));
    }

    Ok(result)
}

fn credential(credential_name: &str) -> Result<String, std::io::Error> {
    let path = credential_path(credential_name);
    let dcem = std::fs::read_to_string(path).unwrap();
    let from_key = get_from_key_from_didcomm_message(&dcem);
    let self_key = get_self_didkey();
    let (vc,_) = decrypt_didcomm(&from_key, &self_key, &dcem);

    Ok(vc)
}

fn presentations() -> Result<String, std::io::Error> {
    let mut list = format!("{:24.24}{:24.24}{:16.16}{:16.16}{:16.16}\n", "ID", "Type", "Holder", "Issuer", "Subject");

    let mut entries: Vec<std::fs::DirEntry> = std::fs::read_dir(presentations_path())
        .unwrap()
        .filter_map(|f| f.ok()).collect();

    entries.sort_by_key(|e| e.path());

    // 1. Get to-key
    let self_key = get_self_didkey();

    for entry in entries {
        if entry.path().is_dir() {
            continue;
        }
        let dcem = std::fs::read_to_string(entry.path()).unwrap();

        // 2. Get from-didkey
        let from_key = get_from_key_from_didcomm_message(&dcem);

        // 3. Decrypt message
        let (vp,_) = decrypt_didcomm(&from_key, &self_key, &dcem);
        let vp: ssi::vc::Presentation = serde_json::from_str(&vp).unwrap();
        let holder_did: String = match vp.holder.unwrap() {
            ssi::vc::URI::String(did) => did
        };
        let vc = vp.verifiable_credential.unwrap();
        let vc: &ssi::vc::Credential = match vc.to_single().unwrap() {
            ssi::vc::CredentialOrJWT::Credential(vc) => vc,
            ssi::vc::CredentialOrJWT::JWT(_) => panic!("presentations(): ssi::vc::CredentialOrJWT::JWT not supported")
        };

        // 4. Get issuer and subject name
        let issuer_did: String = match vc.issuer.clone().unwrap() {
            ssi::vc::Issuer::URI(s) => match s {
                ssi::vc::URI::String(s) => s
            },
            ssi::vc::Issuer::Object(s) => match s.id {
                ssi::vc::URI::String(s) => s
            },
        };
        let issuer_name = std::fs::read_to_string(did_path(&issuer_did))
            .unwrap_or(issuer_did.clone());

        let subject: &ssi::vc::CredentialSubject = vc.credential_subject.to_single().unwrap();
        let subject_did = match subject.id.clone().unwrap() { ssi::vc::URI::String(s) => s };
        let subject_name = std::fs::read_to_string(did_path(&subject_did))
            .unwrap_or(subject_did.clone());

        // 5. Format
        let path = did_path(&holder_did);
        let holder_name = std::fs::read_to_string(path)
            .unwrap_or(holder_did.clone());
        let file_name = String::from(entry.file_name().to_str().unwrap());
        let file_name = file_name.replace(".dcem", "");

        list.push_str(&format!("{:24.24}{:24.24}{:16.16}{:16.16}{:16.16}\n",
            file_name,
            vp.type_.into_iter().last().unwrap(),
            holder_name,
            issuer_name,
            subject_name
        ));
    }

    Ok(list)
}

fn presentation(presentation_name: &str) -> Result<String, std::io::Error> {
    let path = presentation_path(presentation_name);
    let dcem = std::fs::read_to_string(path).unwrap();
    let from_key = get_from_key_from_didcomm_message(&dcem);
    let self_key = get_self_didkey();
    let (vp,_) = decrypt_didcomm(&from_key, &self_key, &dcem);

    Ok(vp)
}

fn connections() -> Result<String, std::io::Error> {
    let mut list = format!("{:16}{}\n", "ID", "DID");
    let mut entries: Vec<std::fs::DirEntry> = std::fs::read_dir(connections_path())
        .unwrap()
        .filter_map(|f| f.ok()).collect();
    entries.sort_by_key(|e| e.path());

    for entry in entries {
        if entry.path().is_dir() {
            continue;
        }
        let connection_did = std::fs::read_to_string(entry.path()).unwrap();
        let connection_id = String::from(entry.file_name().to_str().unwrap()).replace(".did", "");
        list.push_str(&format!("{:16}{}\n", connection_id, connection_did));
    }

    Ok(list)
}

fn connection(connection_name: &str) -> Result<String, std::io::Error> {
    let path = connection_path(connection_name);
    let connection_did = std::fs::read_to_string(path).unwrap();

    Ok(connection_did)
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

fn connections_path() -> String {
    std::path::Path::new(ROOT_PATH)
        .join("connections/")
        .to_str().unwrap().to_string()
}

fn connection_path(connection_id: &str) -> String {
    std::path::Path::new(ROOT_PATH)
        .join("connections/")
        .join(format!("{}.did", connection_id))
        .to_str().unwrap().to_string()
}

fn dids_path() -> String {
    std::path::Path::new(ROOT_PATH)
        .join("dids/")
        .to_str().unwrap().to_string()
}

fn did_path(did: &str) -> String {
    std::path::Path::new(ROOT_PATH)
        .join("dids/")
        .join(did)
        .to_str().unwrap().to_string()
}

fn credentials_path() -> String {
    std::path::Path::new(ROOT_PATH)
        .join("credentials/")
        .to_str().unwrap().to_string()
}

fn credential_path(credential_id: &str) -> String {
    std::path::Path::new(ROOT_PATH)
        .join("credentials/")
        .join(format!("{}.dcem", credential_id))
        .to_str().unwrap().to_string()
}

fn presentations_path() -> String {
    std::path::Path::new(ROOT_PATH)
        .join("presentations/")
        .to_str().unwrap().to_string()
}

fn presentation_path(presentation_name: &str) -> String {
    std::path::Path::new(ROOT_PATH)
        .join("presentations/")
        .join(format!("{}.dcem", presentation_name))
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
    let path = connection_path(other_did_name);
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
    // Basic commands
    DID,
    Doc,
    Connect{ connection_id: String, did: String },
    Write{ connection_id: String, message: String },
    Read{ dcem: String },
    Help,

    // Verifiable Credentials commands
    IssuePassport{ connection_id: String },
    IssueDriversLicense{ connection_id: String },
    IssueTrafficAuthority{ connection_id: String },
    IssueLawEnforcer{ connection_id: String },
    Hold{ dcem: String },
    Present{ credential_id: String, connection_id: String },
    Verify{ issuer_connection_id: String, subject_connection_id: String, dcem: String },

    // View wallet data
    Messages,
    Message{ message_id: String },
    Connections,
    Connection{ connection_id: String },
    Credentials,
    Credential{ credential_id: String },
    Presentations,
    Presentation{ presentation_id: String }
}

pub struct Config {
    cmd: CMD,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, std::io::Error> {
        let default_cmd = String::from("help");

        let cmd = args.get(1).unwrap_or(&default_cmd).clone();

        let cmd = if args.len() < 2 {
            "did".to_string()
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
                CMD::DID
            },
            "doc" => {
                CMD::Doc
            },
            "connect" => {
                let connection_id = get_arg_or_return_help!(2);
                let did = get_arg_or_return_help!(3);

                CMD::Connect{ connection_id, did }
            },
            "write" => {
                let connection_id = get_arg_or_return_help!(2);
                let message = get_arg_or_return_help!(3);

                CMD::Write{ connection_id, message }
            },
            "read" => {
                let dcem = get_arg_or_return_help!(2);

                CMD::Read{ dcem }
            },
            "issue" => {
                let credential_type = get_arg_or_return_help!(2);

                match &credential_type[..] {
                    "Passport" => {
                        let connection_id = get_arg_or_return_help!(3);
                        CMD::IssuePassport{ connection_id }
                    },
                    "TrafficAuthority" => {
                        let connection_id = get_arg_or_return_help!(3);
                        CMD::IssueTrafficAuthority{ connection_id }
                    },
                    "LawEnforcer" => {
                        let connection_id = get_arg_or_return_help!(3);
                        CMD::IssueLawEnforcer{ connection_id }
                    },
                    "DriversLicense" => {
                        let connection_id = get_arg_or_return_help!(3);
                        CMD::IssueDriversLicense{ connection_id }
                    },
                    &_ => {
                        CMD::Help
                    }
                }
            },
            "hold" => {
                let dcem = get_arg_or_return_help!(2);

                CMD::Hold{ dcem }
            },
            "present" => {
                let credential_id = get_arg_or_return_help!(2);
                let connection_id = get_arg_or_return_help!(3);

                CMD::Present{ credential_id, connection_id }
            },
            "verify" => {
                let issuer_connection_id = get_arg_or_return_help!(2);
                let subject_connection_id = get_arg_or_return_help!(3);
                let dcem = get_arg_or_return_help!(4);

                CMD::Verify{ issuer_connection_id, subject_connection_id, dcem }
            },
            "messages" => {
                CMD::Messages
            },
            "message" => {
                let message_id = get_arg_or_return_help!(2);
                CMD::Message{ message_id }
            },
            "credentials" => {
                CMD::Credentials
            },
            "credential" => {
                let credential_id = get_arg_or_return_help!(2);
                CMD::Credential{ credential_id }
            },
            "presentations" => {
                CMD::Presentations
            },
            "presentation" => {
                let presentation_id = get_arg_or_return_help!(2);
                CMD::Presentation{ presentation_id }
            },
            "connections" => {
                CMD::Connections
            },
            "connection" => {
                let connection_id = get_arg_or_return_help!(2);
                CMD::Connection{ connection_id }
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
