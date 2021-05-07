# 2 Requirements

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

## 2.3 Functional requirements

### 2.3.2 As a user I want to create a DID-agent contained within a directory on my machine

- **Given** I navigate to an empty directory
- **When** I run `did init`
- **Then** the sub-directory `.did/` should be created. As a user I should never have to know the contents of `.did/`. The only thing I have to care about is that `.did/`, in practice, IS my agent. If I move `.did/` to a different location, it will be like moving my agent to a different location. This should be similar to how `git`-CLI works, which creates the `.git/`-directory when running `git init`.
---

- **Or Given** I navigate to a directory where a `.did/` already exists
- **When** I run `did init`
- **Then** nothing should happen.

### 2.3.3 As a user I want to view my DID

- **Given** there is a `.did/` in my working directory
- **When** I run `did did`
- **Then** my DID should be written to `stdout`.

### 2.3.4 As a user I want to view my DID document

- **Given** there is a `.did/` in my working directory
- **When** I run `did doc`
- **Then** my DID-document should be written to `stdout` as prettified JSON.
