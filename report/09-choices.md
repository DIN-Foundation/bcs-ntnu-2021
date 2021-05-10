# 9.x Why BDD?

- Own experience from working with BDD out in the field

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

# 9.x Why Rust?

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


