# 2 Requirements

## 2.1 Scenario

All functional requirements are written solve the following scenario:

>The norwegian driver license issuer, Statens Vegvesen, is considering to start issuing it's driver licenses as verifiable credentials (VCs). Statens Vegvesen is not sure if verifiable credentials is the future yet, but are willing to try and dip it's toes in the water. Statens vegvesen will still issue credentials in the traditional way for the forseeable future.
>
>What Statens Veivesen want is a proof-of-concept SSI-application which will issue, hold and verify driver-licenses. They are hoping that this will bootstrap SSI in Norway, as this will enable other individuals and organizations to start experimenting with holding and verifying a serious credentials which actually are useful.
>
>The proof-of-concept may demonstrate, that a driver license as a verifiable credential could be considered on par, legally speaking, with traditional driver licenses.
>
>Statens Vegvesen wants the application to follow open standards which will enable the application to be agnostic about where and how credentials are issued, stored and verified. In other words Statens Vegvesen want to avoid the application to be locked to a specific ledger and a specific wallet.


## 2.1 Non-functional requirements

- The application needs to have a CLI.
- No GUI.
- Developed in the Rust programming language.
- Use existing Rust libraries wherever possible.
- Only support one DIDComm transport - stdin/stdout
- Only support one DID-method - did-key
- Only support one cryptographic-method - x25519/ed25519

## 2.2 Functional requirements writing style

The functional requirement are written with User-Stories as titles, with BDD-style tests as descriptions. BDD-style tests are written in a way that is easy to translate to a code-test. The BDD-style tests are also easy for a human to follow, if a human wants to do a manual test.

Making the bridge between requirements and manual or automated tests as small as possible, saves time later in the development process. It also makes the gap between technical and non-technical team-members as small as possible, making it possible to iterate on the requirements faster. Requirements are usually developed by non-technical people. If they can write in a way that is as structured as possible, it will be benefitial to everyone on the team.

Without further ado, here is the list of functional requirements:

## 2.3 DID functional requirements

### 2.3.2 As a user I want to create a DID-agent contained within a directory on my machine


- **Given** I navigate to an empty directory
- **When** I run `did init`
- **Then** the sub-directory `.did/` should be created
- **And** my DID should be written to `stdout`.

---

- **Or Given** I navigate to a directory where a `.did/` already exists
- **When** I run `did init`
- **Then** my DID should be written to `stdout`.

Note: As a user I should never have to know the contents of `.did/`. The only thing I have to care about is that `.did/`, in practice, IS my agent. If I move `.did/` to a different location, it will be like moving my agent to a different location. This should be similar to how `git`-CLI works, which creates the `.git/`-directory when running `git init`.

### 2.3.3 As a user I want to view my DID

- **Given** there is a `.did/` in my working directory
- **When** I run `did init`
- **Then** my DID should be written to `stdout`.

### 2.3.4 As a user I want to view my DID document

- **Given** there is a `.did/` in my working directory
- **When** I run `did doc`
- **Then** my DID-document should be written to `stdout` as prettified JSON.

### 2.3.x As a user I want to store and refer to DIDs by name

- **Given** I have a DID `did:key:z6MkjidGmTqu3jG73hVdz5MKEGtVLCLof9ctxTXHMomNcivxA
- **When** I run `did connect doctor did:key:z6MkjidGmTqu3jG73hVdz5MKEGtVLCLof9ctxTXHMomNcivx`
- **Then** the relationship between `doctor` <--> `DID` should be stored in my agent
- **And** I should be able to refer to the DID in other commands, by the name `doctor`
- **And** the name `doctor` should be written to `stdout` (@TODO implement this requirement)

### 2.3.x As a user I want to refer to my own DID with the name `self`


- **Given** there is a `.did/` in my working directory
- **When** I run any command
- **Then** I should be able to refer to my own DID by the name `self`. Example: `did write self hello`.
### 2.3.x As a user I want to view all my DID's

- **Given** I have stored some DIDs in my agent
- **When** I run `did dids`
- **Then** a list of all my DIDs and DIDNames should be written to `stdout`.

### 2.3.x As a user I want to view the DID referred to by a DIDName

- **Given** I have stored a DIDName `police`
- **When** I run `did did police`
- **Then** the DID referred to by `police` should be written to `stdout`.


## 2.4 DIDComm v2 functional requirements

### 2.4.1 As a user I want to write a DCEM to another agent

- **Given** I have initialized two agents on my machine
- **And** I have stored the DID of the other agent by the name `other`
- **When** I run `did write other hello`
- **Then** a DCEM should be written to `stdout`
- **And** the DCEM should be addressed to the DID referred to by `other`.

*DCEM - DIDComm encrypted Message*

### 2.4.2 As a user I want to read the contents of a DCEM addressed to me

- **Given** I have received a file - `hello.dcem` - containing a DCEM addressed to me
- **When** I run `did read $(cat hello.dcem)` or `cat hello.dcem | did read`
- **Then** the plaintext contents of the DCEM should be written to `stdout`.

### 2.4.3 As a user I want to store (hold) DCEM's inside my agent

- **Given** I have received a file - `hello.dcem`
- **When** I run `did hold $(cat hello.dcem)` or `cat hello.dcem | did hold`
- **Then** the DCEM should be stored inside my agent, for later usage
- **And** the DCEM should be written to `stdout`.

### 2.4.4 As a user I want to view a list of all my stored DCEM's

- **Given** I am holding 1 or more DCEM's
- **When** I run `did messages`
- **Then** a list of all my DCEM's together with their ids, should be written to `stdout`

### 2.4.5 As a user I want to view a single DCEM I am holding

- **Given** a DCEM's id is `7497036273686508746`
- **When** I run `did message 7497036273686508746`
- **Then** the DCEM should be written to `stdout`.

## 2.5 Verifiable Credentials functional requirements

### 2.5.x As an issuer I want to issue a verifiable credentials of a specific type to a subject

- **Given** I have connected a DID to the name `jonny`
- **When** I run `did issue Passport jonny`
- **Then** a DCEM wrapping will be written to `stdout`.
- **With** a Verifiable Credential of type `Passport`
- **And with** `subject.did` of `jonny`
- **And with** `issuer.did` of `self`.

### 2.5.x As a holder I want to present a verifiable credential I am holding as a verifiable presentation to a verifier

- **Given** I have connected a DID to the name `police`
- **And** I have a file with a Verifiable Credential of type `Passport` stored as `passport.vc.dcem`
- **When** I run `cat passport.vc.dcem | did present police` or `did present police $(cat passport.vc.dcem)`
- **Then** a DCEM should be written to `stdout`
- **With** a Verifiable Presentation of type `Passport`
- **With** `holder.did` of `self`
- **And with** a Verifiable Credential of type `Passport`
- **With** address to the `police`

### 2.5.x As a verifier I want to verify that a verifiable presentation I have received from a holder, has been issued by the correct issuer, and was issued to the correct subject

- **Given** I have connected a DID to the name `jonny`
- **And** I have connected a DID to the name `police`
- **And** I have a file with a Verifiable Presentation of type `Passport` stored as `passport.vp.dcem`
- **When** I run `did verify police jonny Passport Passport` and it succeeds
- **Then** I can trust that the `vc.issuer.did` is `police`,
- **And** the `vc.subject.did` is `jonny`,
- **And** the `vp.type` is `Passport`
- **And** the `vc.type` is `Passport`. (@TODO Implement checking of vc.type and vp.type)
- **And** that `passport.vp.dcem` will be written to `stdout`

## 2.6 Scenario functional requirements

