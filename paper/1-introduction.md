# Exploring our Self-Sovereign-Identity(SSI) future, by developing a Command-Line-Interface(CLI) in Rust, using DID-Key, DIDComm V2 and Verifiable Credentials.

# 1 Introduction

>@marni: Introduction - this should be relatively compact and short, stating the problem, why it is a problem, what this research/project is all about, etc. Do not deep-dive into anything, but it should be a quick introduction to everything surface-level, short and compact. Max 2-5 pages. After reading introduction the reader should be able to answer a question: "Do I want to read the WHOLE thing, PARTS of it, or NOTHING". So, you need to effectively "introduce" the reader to the domain, touch on all the main points, but, without DEEP dive.

SSI is an evolving ecosystem which - although still in it's infancy - already has established a solid foundation of technologies, which when implemented will seed a system-wide disruption of what we know today as "The Internet".

## 1.1 A theoretical investigation

This paper investigates 3 major innovations related to SSI:

1. **Decentralized identifiers or DID's** - Facilitates the creation of decentralized, persistent, verifiable and resolvable agents in any network.
2. **DIDComm Messaging** - Establishes protocols for the exchange of information between networked DID-agents.
3. **Verifiable Credentials** - Specifies how to express any piece of information as a "cryptographically secure, privacy respecting and machine verifiable" data-structure.

All 3 innovations are big on their own, and fairly independent of one another. It is when combined, they truly realize their full potential, and become a whole, that is much more than the sum of it's parts. This whole, is referred to as "Self-Sovereign Identity" - SSI for short.

## 1.2 A practical experiment

This paper also includes details about a practical experiment which was conducted. The experiment implements an SSI-agent as a CLI application, in the Rust programming language.

The experiment was conducted to better understand how `didcomm-rs` - which is a novel implementation of the `DIDComm v2`-spec hosted by DIF - could interoperate with existing `Aries`-hosted implementations - which tradiationally have been `"DIDComm v1"`-implementations, but should now support `DIDComm v2` as well.

To learn more about DIF, Aries, DIDComm v1 and v2, and much more, read the next chapter: [2 Background](./2-background.md).


## 1.3 Refs
* [did-core]: https://www.w3.org/TR/did-core/ - 2021-03-30
* [did-use-cases]: https://www.w3.org/TR/did-use-cases/ - 2021-03-30
* [didcomm-messaging-v1]: https://github.com/hyperledger/aries-rfcs/tree/master/concepts/0005-didcomm - 2021-03-30
* [didcomm-messaging-v2]: https://identity.foundation/didcomm-messaging/spec/ - 2021-03-30
* [vc-data-model]: https://www.w3.org/TR/vc-data-model/ - 2021-03-30
