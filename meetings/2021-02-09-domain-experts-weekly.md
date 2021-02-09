# Domain experts weekly - 2021-02-09 @ 12:30

*Attendees: Jonas, Snorre, Abylay, Mariusz*
*Location: https://meet.google.com/usf-xcei-ssr*

## Agenda
- Discuss architecture/tech-stack decisions
- Discuss how to get into the DIF Foundation. 
- Kanban board: https://github.com/DIN-Foundation/bsc-ntnu-2021/projects/1

## Chat log

**Jonas 12:39**
- Meeting start
- Goes through each task in-review column
- Goes through each task in-progress column 

**Mariusz 12:45**
- Mariusz is attempting to get blanket agreement for the NTNU organization in DIF
- Better to get 1 agreement for all NTNU employees, than 1 per employee

**Jonas 12:46**
- Jonas, not employed by NTNU, is going ahead with individual agreement.

**Mariusz 12:49**
- Discussion: Why choose to build on top of DIDComm vs Aries?
- Thinner, more focused.
- Gives more flexibility.
- Aries too fat, too unflexible.
- Aries has stated they want to align to be compatible with DIDComm.

**Jonas 12:58**
- Discussion: Why do we ever need something other than did:key?
- Different use cases require different did:methods.

**Snorre 13:00** 
- Markus Sabadello - Father of DID methods, creator of DID resolver. Worth a conversation with him

**Jonas 13:02**
- DIDKit wasm https://github.com/spruceid/didkit/pull/52

**Jonas 13:05**
- Should we go Web or CLI?
- Prototype using web-tech?
- Stable/long-term solutions needs to go native/Rust.
- Web will currently need WASM-compilation of Rust libraries, which is a risky build-dependency....
- CLI is safe, but boring
- Web is cool, and mobile, but risky.

**Jonas 13:07** 
- No meeting with Deepti today.

**Jonas 13:15**
- Meeting ends
