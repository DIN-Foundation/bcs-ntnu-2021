# did CLI

Example usage:
```
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

    Wallet:
        did messages
        did message <message id>
        did connections
        did connection <connection id>
        did credentials
        did credential <credential id>
        did presentations
        did presentation <presentation id>
```

### Build instructions

1. Make sure you have installed the latest rust toolchain on your machine.

    *Example of using rustup to install the rust toolchain. See: https://rustup.rs/*
    ```
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. Clone from github source code
    ```
    git clone git@github.com:DIN-Foundation/bcs-ntnu-2021.git
    ```

3. Build the `did-cli` using `cargo`
    ```
    cd bcs-ntnu-2021/did-cli/
    cargo build
    ```

4. Copy the built executeable into some directory in your `$PATH`.

    *Example of copying into `$HOME/bin/`*
    ```
    cp target/debug/did $HOME/bin/
    ```

5. Run `did-cli` by typing `did <command>` in your terminal.
    ```
    did help
    ```
