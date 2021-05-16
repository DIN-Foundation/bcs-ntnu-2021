# 3 Requirements

The requirements are developed in a collaborative fashion, between Jonas, Snorre, Mariusz and Abylay, during the weekly technical supervisor meeting.

## 3.1 Non-functional Requirements

- The application needs to have a CLI.
- No GUI.
- Developed in the Rust programming language.
- Use existing Rust libraries wherever possible.
- Only support one DIDComm transport - stdin/stdout
- Only support one DID-method - did-key
- Only support one cryptographic-method - x25519/ed25519


## 3.2 Functional Requirements Writing Style

The functional requirement are written with User-Stories as titles and with BDD-style tests as descriptions. BDD-style tests are written in a way that is easy to translate to a code-test. The BDD-style tests are also easy for a human to follow, if a human wants to do a manual test. For an introduction to BDD (Behaviour Driven Development), read Dan North's original post about BDD here: https://dannorth.net/introducing-bdd/.

BDD makes the bridge between requirements, implementation and testing as small as possible, saving time later in the development process. It also makes the gap between technical and non-technical team-members as small as possible, making it possible to iterate on the requirements faster. The people developing requirements may have a wide range of technical skill. BDD is about talking in a "common-denominator" that everyone can understand, and at the same is easy to implement.

## 3.3 Functional Requirement Layers

The functional requirements are grouped in the following manner:
- Layer 1 - DID Functional Requirements
- Layer 2 - DIDComm Functional Requirements
- Layer 3 - Verifiable Credentials Functional Requirements
- Layer 4 - Scenario Functional Requirements

Each group represents a layer in the SSI-stack.

## 3.4 Layer 1 - DID Functional Requirements

### 3.4.1 As a user I want to initiate a DID-agent inside a directory on my machine

```
Scenario 1

GIVEN I am in an empty directory
WHEN  I run did init
THEN  my DID should be written to stdout.
AND   the sub-directory .did/ should be created
AND   I should be able to refer to my own DID by the name self
```

```
Scenario 2

GIVEN I run `did init`
WHEN  I run did init again
THEN  nothing should happen to .did/
AND   my DID should be written to stdout.
```

*Note: As a user I should never have to know the contents of `.did/`. The only thing I have to care about is that `.did/`, in practice, IS my agent. If I move `.did/` to a different location, it will be like moving my agent to a different location. This should be similar to how `git`-CLI works, which creates the `.git/`-directory when running `git init`.*

### 3.4.2 As a user I want to view my DID

```
GIVEN I run `did init`
WHEN  I run did init
OR    I run did did self
THEN  my DID should be written to stdout.
```

### 3.4.3 As a user I want to view my DID document

```
GIVEN I run `did init`
WHEN  I run did doc
THEN  my DID-document should be written to stdout as prettified JSON.
```

### 3.4.4 As a user I want to store and refer to DIDs by name

```
GIVEN I run `did init`
GIVEN I have a DID `did:key:z6MkjidGmTqu3jG73hVdz5MKEGtVLCLof9ctxTXHMomNcivxA`
WHEN  I run `did connect doctor did:key:z6MkjidGmTqu3jG73hVdz5MKEGtVLCLof9ctxTXHMomNcivx`
THEN  the relationship between `doctor` <--> `DID` should be stored in my agent
AND   I should be able to refer to the DID in other commands, by the name `doctor`
AND   the name `doctor` should be written to `stdout`
```

### 3.4.5 As a user I want to refer to my own DID with the name `self`

```
GIVEN I run `did init`
WHEN  I run any command
THEN  I should be able to refer to my own DID by the name `self`. Example: `did write self hello`.
```

### 3.4.6 As a user I want to view all my DID's

```
GIVEN I run `did init`
AND   I store some DIDNames in my agent
WHEN  I run `did dids`
THEN  a list of all my DIDs and DIDNames should be written to `stdout`.
```

### 3.4.7 As a user I want to view the DID referred to by a DIDName

```
GIVEN I run `did init`
AND   I store a DIDName `police`
WHEN  I run `did did police`
THEN  the DID referred to by `police` should be written to `stdout`.
```

## 3.5 Layer 2 - DIDComm Functional Requirements

### 3.5.1 As a user I want to write a DCEM to another agent

```
GIVEN I run `did init` in two different directories on my machine
AND   I have stored the DID of the other agent by the name `other`
WHEN  I run `did write other hello`
THEN  a DCEM should be written to `stdout`
AND   the DCEM should be addressed to the DID referred to by `other`.
```

*DCEM - DIDComm encrypted Message*

### 3.5.2 As a user I want to read the contents of a DCEM addressed to me

```
GIVEN I receive a DCEM-file `hello.dcem`, addressed to my agent
WHEN  I run `did read $(cat hello.dcem)` or `cat hello.dcem | did read`
THEN  the plaintext contents of the DCEM should be written to `stdout`.
```

### 3.5.3 As a user I want to store (hold) DCEM's inside my agent

```
GIVEN I receive a DCEM-file `hello.dcem`, addressed to my agent
WHEN  I run `did hold $(cat hello.dcem)` or `cat hello.dcem | did hold`
THEN  the DCEM should be stored inside my agent, for later usage
AND   the DCEM should be written to `stdout`.
```

### 3.5.4 As a user I want to view a list of all my stored DCEM's

```
GIVEN my agent is holding 1 or more DCEM's
WHEN  I run `did messages`
THEN  a list of all my DCEM's together with their ids, should be written to `stdout`
```

### 3.5.5 As a user I want to view a single DCEM I am holding

```
GIVEN a DCEM's id my agent is holding id is: `7497036273686508746`
WHEN  I run `did message 7497036273686508746`
THEN  the DCEM should be written to `stdout`.
```


## 3.6 Layer 3 - Verifiable Credentials Functional Requirements

### 3.6.1 As an issuer I want to issue a verifiable credentials to a subject

```
GIVEN my agent has connected a DID to the name `jonny`
WHEN  I run `did issue Passport jonny`
THEN  a DCEM will be written to `stdout`.
    WITH a Verifiable Credential of type `Passport`
    WITH `subject.did` of `jonny`
    WITH `issuer.did` of `self`.
```

### 3.6.2 As a holder I want to present a verifiable credential as a verifiable presentation to a verifier

```
GIVEN my agent has connected a DID to the name `police`
AND   I have a file with a Verifiable Credential of type `Passport` stored as `passport.vc.dcem`
WHEN  I run `cat passport.vc.dcem | did present police` or `did present police $(cat passport.vc.dcem)`
THEN  a DCEM should be written to `stdout`
    WITH a Verifiable Presentation of type `Passport`
    WITH `holder.did` of `self`
    WITH a Verifiable Credential of type `Passport` addressed to the `police`
```

### 3.6.3 As a verifier I want to verify a verifiable presentation

```
GIVEN my agent has connected a DID to the name `jonny`
AND   I have connected a DID to the name `police`
AND   I have a file with a Verifiable Presentation of type `Passport` stored as `passport.vp.dcem`
WHEN  I run `did verify police jonny Passport Passport` and it succeeds
THEN  I can trust that the `vc.issuer.did` is `police`,
AND   the `vc.subject.did` is `jonny`,
AND   the `vp.type` is `Passport`
AND   the `vc.type` is `Passport`.
AND   that `passport.vp.dcem` will be written to `stdout`
```


## 3.7 Layer 4 - Scenario Functional Requirements

### 3.7.1 As a citizen I want to publish my DID to a file

```
GIVEN I have an agent
WHEN  I run `did init > self.did`
OR    I run `did did self > self.did`
THEN  a file with the name `self.did` should contain my DID.
```

### 3.7.2 As governemnt I want to connect my citizens DIDs to names

```
GIVEN my citizens each have their own agents
AND   each citizens has published their DIDs as files: `snorre.did`, `abylay.did`, `jonas.did`
WHEN  I run `cat jonas.did | did connect jonas`
AND   I run `cat abylay.did | did connect abylay`
AND   I run `cat snorre.did | did snorre snorre`
THEN  I should be able to refer to my citizens DIDnames `jonas`,`abylay` and `snorre`, in other commands.
```

### 3.7.3 As a citizen I want to connect to my governemnt DID

```
GIVEN my governemnt has a DID published as `government.did`
WHEN  I run `cat governemnt.did | did connect government`
THEN  I should be able to refer to the name `government` in other commands.
```

### 3.7.4 As government I want issue Passports to my citizens as files

```
GIVEN I have stored the DIDnames of my citizens
WHEN  I run `did issue Passport jonas > jonas.passport.vc.dcem`
AND   I run `did issue Passport abylay > abylay.passport.vc.dcem`
AND   I run `did issue Passport snorre > snorre.passport.vc.dcem`
THEN  my citizens passports should be stored as files that can be shared
AND   the files can only be read by their intended recipient.
```

### 3.7.5 As a citizen I want to hold Passports issued to me

```
GIVEN my governemnt has issued a Passport to in a file `jonas.passport.vc.dcem`
WHEN  I run `cat jonas.passport.vc.dcem | did hold`
THEN  the Passport is stored in my agent as a DCEM
AND   and the DCEM can be later refered to by the DCEM.id
AND   the DCEM can be looked up by using `did messages`.
```

### 3.7.6 As a citizen I want to view my Passport in plaintext

```
GIVEN I have am holding a Passport as a DCEM with id 340340032
WHEN  I run `did message 340340032 | did read`
THEN  my Passport should be written to `stdout` in cleartext.
```

### 3.7.7 As a citizen I want to present my Passport to the Police

```
GIVEN I have am holding a Passport as a DCEM with id 340340032
AND   I have connected a DID to the DIDName `police`
WHEN  I run `did message 340340032 | did present police > jonas.passport.vp.dcem`
THEN  my Passport should be stored in a file
AND   it should only be able to be viewed and verified by the `police`.
```

### 3.7.8 As the Police I want to verify a Passport from a citizen I am controlling

```
GIVEN I have approached a citizen which has a agent
AND   the citizens DID is stored in my agent with name `jonas`
AND   the governments DID is stored in my agent with name `government`
AND   the citizen present his passport to me as the file `jonas.passport.vp.dcem`
WHEN  I run `cat jonas.passport.vp.dcem | did verify government jonas Passport`
THEN  I can be sure that the Passport is valid, and issued by the `government`.
```
