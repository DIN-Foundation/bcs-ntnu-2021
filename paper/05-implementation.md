
# 5. Implementation

## 5.1 Getting started

### 5.1.1 Source code

- `did-cli` is the name of the rust-project, where the implementation was developed.
- The implementation can be found in the `did-cli/` sub-folder in the Github project hosted here: https://github.com/DIN-Foundation/bcs-ntnu-2021/tree/main/did-cli.

### 5.1.2 Build instructions

```
cd did-cli/
cargo build
cp target/debug/did $HOME/bin/
did help
```

## 5.2 The CLI - Command Line Interface

- The `did-cli`'s CLI follows principles laid out in the book `The Unix Programming environment` by `Brian W. Kernighan` and `Rob Pike`, 1984.
- Each command follows the same pattern `did <command> <...args>`.

### 5.2.1 Command: `did help`

List all commands together with their command-signature. The commands are grouped together in 4 groups:
* Basic
* DIDComm V2-messaging
* DIDComm V2 + Verifiable Credentials
* Wallet.

*Example of running `did help`:*

![](./images/cmd-did-help.png)

### 5.2.2 Command: `did`

- Initializes a did agent in the current directory.
- Run this command before running any other commands.
- The command creates a new `.did/`-directory, inside your working directory.
- A secret/private key is stored inside `.did/`.
- All your agents wallet-data will be stored inside `.did/`.
- Your agents `did` will be returned to `stdout` when running this command.
- If a `.did/` already exists, this commands has no side-effects - the command is idempotent.

### 5.2.3 Command: `did doc`

### 5.2.4 Command: `did connect <connection id> <did>`

### 5.2.5 Command: `did write <connection id> <message>`

### 5.2.6 Command: `did read <dcem>`

### 5.2.7 Command: `did issue <CredentialType> <connection id>`

### 5.2.8 Command: `did hold <dcem>`

### 5.2.9 Command: `did present <credential id> <connection id>`

### 5.2.10 Command: `did verify <issuer connection id> <subject connection id> <dcem>`

### 5.2.11 Command: `did messages`

- List all didcomm messages stored in the wallet.
- Messages are added to the wallet when using the `did write` and `did read` commands.
### 5.2.12 Command: `did message <message id>`

- Show the contents of a single didcomm message based on the given `<message id>`.

### 5.2.13 Command: `did connections`

- List all did connections stored in the wallet.
- Connections are added to the wallet when using the `did connect` command.

### 5.2.14 Command: `did connection <connection id>`

- Show the did of a single did connection based on `<connection id>`.
### 5.2.15 Command: `did credentials`

- List all verifiable credentials stored in the wallet.
- Credentials are added to the wallet when using the `did issue` and `did hold` commands.

### 5.2.16 Command: `did credential <credential id>`

- Show a single verifiable credential based on the given `<credential id>`.

### 5.2.17 Command: `did presentations`

- List all verifiable presentations stored in the wallet.
- Presentations are added to the wallet when using the `did present` and `did verify` commands.

### 5.2.18 Command: `did presentation <presentation id>`

- Show a single verifiable presentation based on the given `<presentation id>`.

### 5.2.19 Intentional limitations of the CLI

- None of the commands have any optional-arguments - e.g `--option=<arg>`. This is to keep program logic as simple as possible. If the CLI was intended for a broader audicene with multiple use-cases, options may be added. This CLI is a special purpose CLI, intended to solve a specific use-case, namely the specific proof-of-concept from the problem statement. This is why optional-arguments was not prioritized.
- Options are much harder to parse correctly than fixed size positional arguments.
- None of the commands required variable length arguments, which made the implementation easier.
- None of the commands have filepath arguments. The user is expected to use `cat <filepath>` to read the contents of a file, which is then fed into a positional argument of one of the commands. Example: `did read $(cat ../message.dcem)` vs `did read ../message.dcem`. This was done to simplify implementation.
- None of the commands support pipes. This could have been useful as an alternative to the example from the previous point. Example: `cat ../message.dcem | did read`. Since positional arguments + `cat` already solves the problem of reading from file, support for pipes was not prioritized.


## 5.3 Code organization
### 5.3.1 Rust book guidelines

The implementation is following the "guidelines for binary projects", given by the Rust-book, quoted in full below:
>### Separation of Concerns for Binary Projects
>
>The organizational problem of allocating responsibility for multiple tasks to the main function is common to many binary projects. As a result, the Rust community has developed a process to use as a guideline for splitting the separate concerns of a binary program when main starts getting large. The process has the following steps:
>
>* Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
>* As long as your command line parsing logic is small, it can remain in main.rs.
>* When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.
>
>The responsibilities that remain in the main function after this process should be limited to the following:
>
>* Calling the command line parsing logic with the argument values
>* Setting up any other configuration
>* Calling a run function in lib.rs
>* Handling the error if run returns an error
>
>This pattern is about separating concerns: main.rs handles running the program, and lib.rs handles all the logic of the task at hand. Because you can’t test the main function directly, this structure lets you test all of your program’s logic by moving it into functions in lib.rs. The only code that remains in main.rs will be small enough to verify its correctness by reading it. Let’s rework our program by following this process.

Ref: https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#separation-of-concerns-for-binary-projects

### 5.3.2 File structure

Here is a screenshot of the file-structure, which follows from the guidelines:

![](./images/code-organization.png)


## 5.4 Usage of existing Rust libraries

### decentralized-identity/didcomm-rs

- github.com: https://github.com/decentralized-identity/didcomm-rs
- crates.io: https://crates.io/crates/didcomm-rs
- docs.rs: https://docs.rs/didcomm-rs/0.2.4/didcomm_rs/

### spruceid/didkit

- github.com: https://github.com/spruceid/didkit
- docs: https://spruceid.dev/docs/didkit/

### trinsic-id/did-key.rs

- github.com: https://github.com/trinsic-id/did-key.rs
- crates.io: https://crates.io/crates/did-key
- docs.rs: https://docs.rs/did-key/0.0.11/did_key/

### dalek-cryptography/ed25519-dalek

- github.com: https://github.com/dalek-cryptography/ed25519-dalek
- docs.rs: https://docs.rs/ed25519-dalek/1.0.1/ed25519_dalek/
- crates.io: https://crates.io/crates/ed25519-dalek


### dalek-cryptography/x25519-dalek

- github.com: https://github.com/dalek-cryptography/x25519-dalek
- docs.rs: https://docs.rs/x25519-dalek/1.1.0/x25519_dalek/
- crates.io: https://crates.io/crates/x25519-dalek


>@marni: Implementation - WHAT you have done, and how you have done it...
