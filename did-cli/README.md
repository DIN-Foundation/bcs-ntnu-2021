# did CLI

Example usage:
```
$ did help

    Basic:
        did init
        did doc
        did connect <name> <did>

    DIDComm v2 messaging:
        did write  <subject name> <message>  -->  <dcem>
        did hold   <dcem>                    -->  <dcem>
        did read   <dcem>                    -->  <plaintext message>

    DIDComm v2 + Verifiable Credentials:
        did issue   Passport         <subject name>      -->  <dcem>
        did issue   DriversLicense   <subject name>      -->  <dcem>
        did issue   TrafficAuthority <subject name>      -->  <dcem>
        did issue   LawEnforcer      <subject name>      -->  <dcem>

        did present <verifier name>              <dcem>  -->  <dcem>
        did verify  <issuer name> <subject name> <dcem>  -->  <dcem>

    Wallet:
        did messages
        did message <message id>
        did connections
        did connection <name>

```

### Build instructions

1. Make sure you have installed the latest rust toolchain on your machine.

    *Example of using rustup to install the rust toolchain. See: https://rustup.rs/*
    ```shell
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. Clone from github source code
    ```shell
    git clone git@github.com:DIN-Foundation/bcs-ntnu-2021.git
    git submodule update --init --recursive
    ```

3. Build the `did-cli` using `cargo`
    ```shell
    cd bcs-ntnu-2021/did-cli/
    cargo build
    ```

4. Copy the built executeable into some directory in your `$PATH`.

    *Example of copying into `$HOME/bin/`*
    ```shell
    cp target/debug/did $HOME/bin/
    ```

5. Run `did-cli` by typing `did <command>` in your terminal.
    ```shell
    did help
    ```
