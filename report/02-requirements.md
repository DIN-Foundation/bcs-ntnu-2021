# 2 Requirements

## 2.x Non-functional Requirements

- The application needs to have a CLI.
- No GUI.
- Developed in the Rust programming language.
- Use existing Rust libraries wherever possible.
- Only support one DIDComm transport - stdin/stdout
- Only support one DID-method - did-key
- Only support one cryptographic-method - x25519/ed25519


## 2.x Scenario

All functional requirements are written solve the following scenario:

>The norwegian driver license issuer, Statens Vegvesen, is considering to start issuing it's driver licenses as verifiable credentials (VCs). Statens Vegvesen is not sure if verifiable credentials is the future yet, but are willing to try and dip it's toes in the water. Statens vegvesen will still issue credentials in the traditional way for the forseeable future.
>
>What Statens Veivesen want is a proof-of-concept SSI-application which will issue, hold and verify driver-licenses. They are hoping that this will bootstrap SSI in Norway, as this will enable other individuals and organizations to start experimenting with holding and verifying a serious credentials which actually are useful.
>
>The proof-of-concept may demonstrate, that a driver license as a verifiable credential could be considered on par, legally speaking, with traditional driver licenses.
>
>Statens Vegvesen wants the application to follow open standards which will enable the application to be agnostic about where and how credentials are issued, stored and verified. In other words Statens Vegvesen want to avoid the application to be locked to a specific ledger and a specific wallet.


## 2.x Functional Requirements Writing Style

The functional requirement are written with User-Stories as titles, with BDD-style tests as descriptions. BDD-style tests are written in a way that is easy to translate to a code-test. The BDD-style tests are also easy for a human to follow, if a human wants to do a manual test.

Making the bridge between requirements and manual or automated tests as small as possible, saves time later in the development process. It also makes the gap between technical and non-technical team-members as small as possible, making it possible to iterate on the requirements faster. Requirements are usually developed by non-technical people. If they can write in a way that is as structured as possible, it will be benefitial to everyone on the team.


## 2.x Functional Requirement Layers

The functional requirements are divided into 4 layers:
- Layer 1 - DID Functional Requirements
- Layer 2 - DIDComm Functional Requirements
- Layer 3 - Verifiable Credentials Functional Requirements
- Layer 4 - Scenario Functional Requirements

Each layer solves the problem on a different layer in the SSI-stack.


## 2.x Layer 1 - DID Functional Requirements

### 2.x.x As a user I want to initiate a DID-agent inside a directory on my machine

**Scenario 1:**

- **Given** I navigate to an empty directory
- **When** I run `did init`
- **Then** the sub-directory `.did/` should be created
- **And** my DID should be written to `stdout`.


**Scenario 2:**

- **Given** I navigate to a directory where a `.did/` already exists
- **When** I run `did init`
- **Then** the existing sub-directory `.did/` should be left as is
- **And** my DID should be written to `stdout`

*Note: As a user I should never have to know the contents of `.did/`. The only thing I have to care about is that `.did/`, in practice, IS my agent. If I move `.did/` to a different location, it will be like moving my agent to a different location. This should be similar to how `git`-CLI works, which creates the `.git/`-directory when running `git init`.*

### 2.x.x As a user I want to view my DID

**Scenario:**

- **Given** there is a `.did/` in my working directory
- **When** I run `did init`
- **Then** my DID should be written to `stdout`.

### 2.x.x As a user I want to view my DID document

**Scenario:**

- **Given** there is a `.did/` in my working directory
- **When** I run `did doc`
- **Then** my DID-document should be written to `stdout` as prettified JSON.

### 2.x.x As a user I want to store and refer to DIDs by name

**Scenario:**
- **Given** I have a DID `did:key:z6MkjidGmTqu3jG73hVdz5MKEGtVLCLof9ctxTXHMomNcivxA
- **When** I run `did connect doctor did:key:z6MkjidGmTqu3jG73hVdz5MKEGtVLCLof9ctxTXHMomNcivx`
- **Then** the relationship between `doctor` <--> `DID` should be stored in my agent
- **And** I should be able to refer to the DID in other commands, by the name `doctor`
- **And** the name `doctor` should be written to `stdout` (@TODO implement this requirement)

### 2.x.x As a user I want to refer to my own DID with the name `self`

**Scenario:**
- **Given** there is a `.did/` in my working directory
- **When** I run any command
- **Then** I should be able to refer to my own DID by the name `self`. Example: `did write self hello`.

### 2.x.x As a user I want to view all my DID's

**Scenario:**
- **Given** I have stored some DIDs in my agent
- **When** I run `did dids`
- **Then** a list of all my DIDs and DIDNames should be written to `stdout`.

### 2.x.x As a user I want to view the DID referred to by a DIDName

**Scenario:**
- **Given** I have stored a DIDName `police`
- **When** I run `did did police`
- **Then** the DID referred to by `police` should be written to `stdout`.


## 2.x Layer 2 - DIDComm Functional Requirements

### 2.x.x As a user I want to write a DCEM to another agent

**Scenario:**
- **Given** I have initialized two agents on my machine
- **And** I have stored the DID of the other agent by the name `other`
- **When** I run `did write other hello`
- **Then** a DCEM should be written to `stdout`
- **And** the DCEM should be addressed to the DID referred to by `other`.

*DCEM - DIDComm encrypted Message*

### 2.x.x As a user I want to read the contents of a DCEM addressed to me

**Scenario:**
- **Given** I have received a file - `hello.dcem` - containing a DCEM addressed to me
- **When** I run `did read $(cat hello.dcem)` or `cat hello.dcem | did read`
- **Then** the plaintext contents of the DCEM should be written to `stdout`.

### 2.x.x As a user I want to store (hold) DCEM's inside my agent

**Scenario:**
- **Given** I have received a file - `hello.dcem`
- **When** I run `did hold $(cat hello.dcem)` or `cat hello.dcem | did hold`
- **Then** the DCEM should be stored inside my agent, for later usage
- **And** the DCEM should be written to `stdout`.

### 2.x.x As a user I want to view a list of all my stored DCEM's

**Scenario:**
- **Given** I am holding 1 or more DCEM's
- **When** I run `did messages`
- **Then** a list of all my DCEM's together with their ids, should be written to `stdout`

### 2.x.x As a user I want to view a single DCEM I am holding

**Scenario:**
- **Given** a DCEM's id is `7497036273686508746`
- **When** I run `did message 7497036273686508746`
- **Then** the DCEM should be written to `stdout`.


## 2.x Layer 3 - Verifiable Credentials Functional Requirements

### 2.x.x As an issuer I want to issue a verifiable credentials to a subject

**Scenario:**
- **Given** I have connected a DID to the name `jonny`
- **When** I run `did issue Passport jonny`
- **Then** a DCEM wrapping will be written to `stdout`.
- **With** a Verifiable Credential of type `Passport`
- **And with** `subject.did` of `jonny`
- **And with** `issuer.did` of `self`.

### 2.x.x As a holder I want to present a verifiable credential as a verifiable presentation to a verifier

**Scenario:**
- **Given** I have connected a DID to the name `police`
- **And** I have a file with a Verifiable Credential of type `Passport` stored as `passport.vc.dcem`
- **When** I run `cat passport.vc.dcem | did present police` or `did present police $(cat passport.vc.dcem)`
- **Then** a DCEM should be written to `stdout`
- **With** a Verifiable Presentation of type `Passport`
- **With** `holder.did` of `self`
- **And with** a Verifiable Credential of type `Passport`
- **With** address to the `police`

### 2.x.x As a verifier I want to verify a verifiable presentation

**Scenario:**
- **Given** I have connected a DID to the name `jonny`
- **And** I have connected a DID to the name `police`
- **And** I have a file with a Verifiable Presentation of type `Passport` stored as `passport.vp.dcem`
- **When** I run `did verify police jonny Passport Passport` and it succeeds
- **Then** I can trust that the `vc.issuer.did` is `police`,
- **And** the `vc.subject.did` is `jonny`,
- **And** the `vp.type` is `Passport`
- **And** the `vc.type` is `Passport`. (@TODO Implement checking of vc.type and vp.type)
- **And** that `passport.vp.dcem` will be written to `stdout`


## 2.x Layer 4 - Scenario Functional Requirements

### 2.x.x As a citizen I want to publish my DID to a file

**Scenario:**
- **Given** I have an agent
- **When** I run `did init > self.did`
- **Or** `did did self > self.did`
- **Then** a file with the name `self.did` should contain my DID.

### 2.x.x As governemnt I want to connect my citizens DIDs to names

**Scenario:**
- **Given** my citizens each have their own agents
- **And** each citizens has published their DIDs as files: `snorre.did`, `abylay.did`, `jonas.did`
- **When** I run `cat jonas.did | did connect jonas`
- **And** I run `cat abylay.did | did connect abylay`
- **And** I run `cat snorre.did | did snorre snorre`
- **Then** I should be able to refer to my citizens DIDnames `jonas`,`abylay` and `snorre`, in other commands.

### 2.x.x As a citizen I want to connect to my governemnt DID

**Scenario:**
- **Given** my governemnt has a DID published as `government.did`
- **When** I run `cat governemnt.did | did connect government`
- **Then** I should be able to refer to the name `government` in other commands.

### 2.x.x As government I want issue Passports to my citizens as files

**Scenario:**
- **Given** I have stored the DIDnames of my citizens
- **When** I run `did issue Passport jonas > jonas.passport.vc.dcem`
- **and** I run `did issue Passport abylay > abylay.passport.vc.dcem`
- **and** I run `did issue Passport snorre > snorre.passport.vc.dcem`
- **Then** my citizens passports should be stored as files that can be shared
- **And** the files can only be read by their intended recipient.

### 2.x.x As a citizen I want to hold Passports issued to me

**Scenario:**
- **Given** my governemnt has issued a Passport to in a file `jonas.passport.vc.dcem`
- **When** I run `cat jonas.passport.vc.dcem | did hold`
- **Then** the Passport is stored in my agent as a DCEM
- **And** and the DCEM can be later refered to by the DCEM.id
- **And** the DCEM can be looked up by using `did messages`.

### 2.x.x As a citizen I want to view my Passport in plaintext

**Scenario:**
- **Given** I have am holding a Passport as a DCEM with id 340340032
- **When** I run `did message 340340032 | did read`
- **Then** my Passport should be written to `stdout` in cleartext.

### 2.x.x As a citizen I want to present my Passport to the Police

**Scenario:**
- **Given** I have am holding a Passport as a DCEM with id 340340032
- **And** I have connected a DID to the DIDName `police`
- **When** I run `did message 340340032 | did present police > jonas.passport.vp.dcem`
- **Then** my Passport should be stored in a file
- **And** it should only be able to be viewed and verified by the `police`.

### 2.x.x As the Police I want to verify a Passport from a citizen I am controlling

**Scenario:**
- **Given** I have approached a citizen which has a agent
- **And** the citizens DID is stored in my agent with name `jonas`
- **And** the governments DID is stored in my agent with name `government`
- **And** the citizen present his passport to me as the file `jonas.passport.vp.dcem`
- **When** I run `cat jonas.passport.vp.dcem | did verify government jonas Passport`
- **Then** I can be sure that the Passport is valid, and issued by the `government`.

## 2.x Sources

| # | Who | What                         | Where                                 | When       |
|---|-----------|------------------------|---------------------------------------|------------|
| 1 | Dan North | Introducing BDD        | https://dannorth.net/introducing-bdd/ | 2021-05-10 |
