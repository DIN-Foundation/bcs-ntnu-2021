# 2 Requirements

## 2.1 Non-functional requirements

- The application needs to have a CLI.
- No GUI.
- Developed in the Rust programming language.
- Use existing Rust libraries wherever possible.
- Only support one DIDComm transport - stdin/stdout
- Only support one DID-method - did-key
- Only support one cryptographic-method - x25519/ed25519

## 2.2 Functional requirements

### 2.2.1 Writing style

The functional requirement are written with User-Stories as titles, with BDD-style tests as descriptions. BDD-style tests are written in a way that is easy to translate to a code-test. The BDD-style tests are also easy for a human to follow, if a human wants to do a manual test.

Making the bridge between requirements and manual or automated tests as small as possible, saves time later in the development process. It also makes the gap between technical and non-technical team-members as small as possible, making it possible to iterate on the requirements faster. Requirements are usually developed by non-technical people. If they can write in a way that is as structured as possible, it will be benefitial to everyone on the team.

Without further ado, here is the list of functional requirements:

### 2.2.2 As a user I want to create a DID-agent contained within a directory on my machine

- **Given** I navigate to an empty directory
- **When** I ask the CLI to initialize
- **Then** the sub-directory `.did/` should be created, containing the minimum files required for running all other CLI commands. This should be similar to how `git`-CLI works, when creating the `.git/`-directory after running `git init`.
---

- **Or Given** I have navigated to a directory where `.did/` already exists
- **When** I ask the CLI to initialize
- **Then** nothing should happen.

### 2.2.3 As a user I want to view my DID

- **Given** there is a `.did/` in my working directory
- **When** I ask the CLI display the DID
- **Then** the DID should be printed to `stdout`.

### 2.2.4 As a user I want to view my DID document

- **Given** there is a `.did/` in my working directory
- **When** I ask the CLI display my DID-document
- **Then** the DID-document should be printed to `stdout` as prettified JSON.
