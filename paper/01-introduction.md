# Exploring our Self-Sovereign Identity future, by developing a Command-Line-Interface in Rust, using DID-Key, DIDComm V2 and Verifiable Credentials

# 1 Introduction
## 1.1 Problem domain

Self-Sovereign-Identity (SSI) is an umbrella-term for a bunch of novel, evolving technologies, hosted by well-known standards-organizations such as World Wide Web Consortium ([W3C](https://www.w3.org/)) and Decentralized Identity Foundation ([DIF](https://github.com/decentralized-identity)).

SSI tries to solve many problems of the internet today. Decentralized identifiers (DID's) is a core SSI-technology, and it's design goals gives us a hint about which problems SSI as a whole is a solution to:

*The three first design goals from [DID - 1.2 Design Goals](https://www.w3.org/TR/did-core/#design-goals)*:
>- **Decentralization** - Eliminate the requirement for centralized authorities or single point failure in identifier management, including the registration of globally unique identifiers, public verification keys, services, and other information.
>
>- **Control** - Give entities, both human and non-human, the power to directly control their digital identifiers without the need to rely on external authorities.
>
>- **Privacy** - Enable entities to control the privacy of their information, including minimal, selective, and progressive disclosure of attributes or other data.

The standards developed within the realm of SSI, are developed as open-source collaborative documents. The documents, when finalized, become specifications which is intended to be used by developers for implementing SSI-applications.

To read on, it is highly recommended that you have a surface-level understanding of the following three core SSI-specifications, and how they relate to one another:

- Decentralized Identifiers (DIDs) v1.0 - https://www.w3.org/TR/did-core/
- DIDComm messaging (v2) - https://identity.foundation/didcomm-messaging/spec/
- Verifiable Credentials Data Model 1.0 - https://www.w3.org/TR/vc-data-model/

## 1.2 Scope

O

## Sources
* [did-core]: https://www.w3.org/TR/did-core/ - 2021-03-30
* [did-use-cases]: https://www.w3.org/TR/did-use-cases/ - 2021-03-30
* [didcomm-messaging-v1]: https://github.com/hyperledger/aries-rfcs/tree/master/concepts/0005-didcomm - 2021-03-30
* [didcomm-messaging-v2]: https://identity.foundation/didcomm-messaging/spec/ - 2021-03-30
* [vc-data-model]: https://www.w3.org/TR/vc-data-model/ - 2021-03-30


>@marni: Introduction - this should be relatively compact and short, stating the problem, why it is a problem, what this research/project is all about, etc. Do not deep-dive into anything, but it should be a quick introduction to everything surface-level, short and compact. Max 2-5 pages. After reading introduction the reader should be able to answer a question: "Do I want to read the WHOLE thing, PARTS of it, or NOTHING". So, you need to effectively "introduce" the reader to the domain, touch on all the main points, but, without DEEP dive.
