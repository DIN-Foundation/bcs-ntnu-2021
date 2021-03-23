# Domain experts weekly - 2021-03-23 @ 12:30

*Attendees: Jonas, Snorre, Mariusz, Abylay*
*Location: https://meet.google.com/usf-xcei-ssr*

# Log

## Agenda 

1. Review Didchat CLI -  https://github.com/DIN-Foundation/bsc-ntnu-2021/tree/main/playground/didchat2 
2. Discuss Aries DIDComm RFC's overlapping/not-overlapping with DIDComm v2 - https://github.com/DIN-Foundation/bsc-ntnu-2021/issues/27.
3. Is it time to pivot?


## Log

**Jonas 12:35 PM**
- Demo time

**Mariusz**
- Network transport? 
- Not necessary. stdin/stdout transport will be ok.
- Adding another transport does not get us closer to solve the problem statement.

**Jonas S12:56 PM**
- `didchat snorre connect jonas $(didchat jonas did)`, could be thought of as reading a QR-code to connect two traditional agents.

**Jonas S12:58 PM**
- Aries RFC investigation - https://github.com/DIN-Foundation/bsc-ntnu-2021/issues/27
- Which Aries RFC's will be deprecated after DIDComm v2?
- Which Aries RFC's will continue to get support after DIDComm v2? 
- There is not 100% overlap between Aries Didcomm and DIDComm v2. 
- DIDComm v2 has a smaller scope than the Aries RFC's.
- What will happen to the Aries RFC's which are outside of the scope of DIDComm v2?
- What will happen to the Aries RFC's which are inside the scope of DIDComm v2?

**Jonas S1:05 PM**
- Pivot: diver deeper into didcomm OR stretch towards VC's?
- Mariusz says that both are ok, but in the DIDComm-space it may be more difficult to define a problem-statement, than in the verifiable credentials space.

**Snorre Lothar von Gohren Edwin1:07 PM**
- https://identity.foundation/working-groups/did-comm.html

**Jonas S1:16 PM**
- Sam Curren may be able answer Aries RFC questions