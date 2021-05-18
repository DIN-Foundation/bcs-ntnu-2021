# 7 Decisions

## 7.x Why behaviour-driven-development BDD?

- Own experience from working with BDD out in the field.

- Dan North, Introduction to BDD, https://dannorth.net/introducing-bdd/
    + >If we could develop a consistent vocabulary for analysts, testers, developers, and the business, then we would be well on the way to eliminating some of the ambiguity and miscommunication that occur when technical people talk to business people.
    + >A story’s behaviour is simply its acceptance criteria – if the system fulfills all the acceptance criteria, it’s behaving correctly; if it doesn’t, it isn’t. So we created a template to capture a story’s acceptance criteria.

- As a, I want, so what
    ```
    As a [X]
    I want [Y]
    so that [Z]
    ```

- given, when, then
    ```
    Given some initial context (the givens),
    When an event occurs,
    then ensure some outcomes.
    ```

## 7.x Why Rust?

- Multiple libraries in the SSI community written in Rust already
- The only official implementation of DIDComm v2 is in Rust
- Strong typing
- Safety/security features eliminates whole classes of bugs:
    - Memory safety - allocating/freeing
    - Boundary safety
    - Null pointer safety
- Compiler catches many bugs before run-time
- Modern language from this century
- Performance like C
- Backed by industry champs like Mozilla, Microsoft, Linux Foundation



## 7.x Why re-use existing work as much as possible

Very often, a specific programming language is chosen, because we want to re-use as much as possible of existing code. This is the main reason Rust was chosen. It has an existing library for all the standards we want to comply with.

Using existing libraries, makes it easier to comply with standards:
- did-key-rs - Implements the `did`-standards.
- didcomm-rs - Implements the `didcomm-v2`-standards.
- spruceid/ssi - Implements the `vc`-standards.

## 7.x Why a command line interface - no GUI?

- GUI does not solve our problem statement any better.
- A degree in Programming, does not get extra points for design.
- Time considered better spent on lower-level stuff.

## 7.x Why only support a single cryptographic toolkit?

- Why only support ed25519/x25519 for signing and encryption?
- To limit scope.
- Supporting multiple toolkits, does not solve our problem-statement any better.
- One is enough to prove the point.
- One could rewrite the software to support multiple cryptographic toolkits later, using a plugin-based architecture.

## 7.x Why only support a single DIDComm transport?

- To limit scope.
