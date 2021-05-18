# 5 Application Architecture



## 5.1 Data Model

@Screenshot

### 5.1.x DID

A DID is the unique identifier of an agent.

### 5.1.x DIDKey

DIDKey is the only DID-method we are supporting.

### 5.1.x ed25519-JWK

The JWK DID-CLI is supporting, is a `ed25519/x25519` cryptographic public-private keypair, underpinning the DIDKey-method. The `jwk` is used for many things:
- Each unique `jwk` maps to a unique DID in the DIDKey-method.
- By holding the `jwk`, the agent is able to assert control over it's DID in the DIDKey-method.
- The `jwk` is used to encrypt DIDComm messages.
- The `jwk` is used when generating the shared key in the Elliptic-Curve-Diffie-Helmann-key-exchange (ECDH-protocol).
- The `jwk` is used to sign Verifiable Credentials and Verifiable Presentations.

*Example of an ed25519-jwk used by DID-CLI*
```json
{
	"kty":"OKP",
	"crv":"Ed25519",
	"x":"YRJAoEuAzcdc_7QdEM0NHQCd6hd-FdHkpdXl8T-RlVA",
	"d":"HOrwgKInYvPw_Wh6nN6kTNEd3wkkwYySMSuXzdr5Gec"
}
```

### 5.1.x DCEM - DIDComm Encrypted Message

All messages read and written by the agent, are serialized as DIDComm Encrypted Messages, both in transit, and when at stored at rest inside the agent, as files.

### 5.1.x Verifiable Credentials

All credentials issued by the agent are serialized as Verifiable Credentials.

### 5.1.x Verifiable Presentations

All presentations of Verifiable Credentials are serialized as Verifiable Presentations.

### 5.1.x DIDName
- DIDName is a way of refering to a DID in a local DID-CLI command, because it is impossible to remember the full DID.
- DIDName is the only part of DID-CLI's data-model which is not part of an SSI-standard. 
- Each DIDName is stored as a file `<DIDName>.did`, and the full DID as the content.
- Example: `self.did` or `jonas.did`.




## 5.2 File storage 

### 5.2.x The `.did/` directory

All the agents files are contained within the `.did/`-directory, created when initializing the agent. The DID-CLI will use whatever `.did/`-directory is inside the current terminal working directory, just like GIT-CLI uses the `.git/`-directory.

### 5.2.x Portability

The agent database, represented by the `.did/` directory, should be portable. A user should be able to move it to any other location on local machine, or to any other machine, and the agent should still work.


### 5.2.x One directory per agent

```

for i in bob lisa snorre jonas
do
	mkdir $i;
	cd $i;
	did init;
	cd ..;
done
```

### 5.2.x Communicating by sharing files

```
mkdir bob/ lisa/
cd bob/
did init
did did self > ../bob.did

cd ../lisa
did init
did did self > ../lisa.did

cd ../lisa
cat ../bob.did | did connect bob
cd ../bob
cat ../lisa.did | did connect lisa
```


