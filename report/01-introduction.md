# Exploring our Self-Sovereign Identity future, by developing a Command-Line-Interface in Rust, using DID-Key, DIDComm V2 and Verifiable Credentials

# 1 Introduction

## 1.x Problem domain

Self-Sovereign-Identity (SSI) is an umbrella-term for a bunch of novel, evolving technologies, hosted by well-known standards-organizations such as World Wide Web Consortium ([W3C](https://www.w3.org/)) and Decentralized Identity Foundation ([DIF](https://github.com/decentralized-identity)).

SSI attempts to solve many problems present in today's internet. Decentralized identifiers (DID's) is a core SSI-technology, and it's design goals gives us a hint about which problems SSI is trying to solve:

*The three first design goals from [DID - 1.2 Design Goals](https://www.w3.org/TR/did-core/#design-goals)*:
>- **Decentralization** - Eliminate the requirement for centralized authorities or single point failure in identifier management, including the registration of globally unique identifiers, public verification keys, services, and other information.
>
>- **Control** - Give entities, both human and non-human, the power to directly control their digital identifiers without the need to rely on external authorities.
>
>- **Privacy** - Enable entities to control the privacy of their information, including minimal, selective, and progressive disclosure of attributes or other data.

The standards developed within the realm of SSI, are developed as open-source collaborative documents. The documents, when finalized, become specifications which is intended to be used by developers for implementing SSI-applications.

To read on, it is highly recommended that you atleast have a surface-level understanding of the following three core SSI-specifications, and how they relate to one another:

| Title | Description | Living Document |
|-------|-------------|---------------|
| **Descentralized Identifiers (DIDs) v1.0** | How to create and control decentralized, digital agents, represented by a globally unique identifier called a `DID`. | https://www.w3.org/TR/did-core/ |
| **DIDComm messaging v2** | How `DID`-agents should communicate with each other. | https://identity.foundation/didcomm-messaging/spec/ |
| **Verifiable Credentials Data Model 1.0** | How `DID`-agents issue verifiable claims about the attributes of each other and their relationships. | https://www.w3.org/TR/vc-data-model/ |

All of these specifications are work-in-progress as of today - 2021-05-06 - and thus are subject to change, though they are considered stable enough for the SSI-community to have started developing serious applications based on them.

## 1.x Scope

- Within the SSI space, there is a need to develop a specific type of software, called an agent.
- An agent is a piece of software which implements protocols defined by the different SSI-specifications.
- If an agent follows the specifications correctly, it should be able to interoperate with other agents, written by other teams, in other programming languages.
- Currently there are implementations in Python, C#, Javascript, Rust, Golang, and probably more. (@TODO link to source of implementations)
- Each type of agent implements a different subset of protocols. This is because different agents are made to solve different problems.
- For instance a "Wallet", is a specific kind of agent, used to store and manage personal credentials.

## 1.x Problem statement

This project will develop a proof-of-concept agent application. The proof-of-concept application should solve the following scenario:

### 1.x.x Scenario

The norwegian driver license issuer, Statens Vegvesen, is considering to start issuing it's driver licenses as verifiable credentials (VCs). Statens Vegvesen is not sure if verifiable credentials is the future yet, but are willing to try and dip it's toes in the water. Statens vegvesen will still issue credentials in the traditional way for the forseeable future.

What Statens Veivesen want is a proof-of-concept SSI-application which will issue, hold and verify driver-licenses. They are hoping that this will bootstrap SSI in Norway, as this will enable other individuals and organizations to start experimenting with holding and verifying a serious credentials which actually are useful.

The proof-of-concept may demonstrate, that a driver license as a verifiable credential could be considered on par, legally speaking, with traditional driver licenses.

Statens Vegvesen wants the application to follow open standards which will enable the application to be agnostic about where and how credentials are issued, stored and verified. In other words Statens Vegvesen want to avoid the application to be locked to a specific ledger and a specific wallet.


### 1.x.x Deliverables

- High-level design document.
- Source code of a proof-of-concept SSI application.
- Demonstration of a proof-of-concept SSI application.
- A table giving an overview of interoperability findings.

## 1.x Motivation and goals

## 1.x.x Motivation
- Inspire people and organizations to see the potential of SSI, by solving a real world scenario.
- Create an example codebase for developers who are getting started with SSI-application development.
- Give specification writers some feedback about how easy it is to adopt their specifications.
- Prove and reflect on interoperability in the SSI-space.

## 1.x.x Effektmål

- Engage with the broader DIF-community during development.

## 1.x.x Resultatmål

- Present and demonstrate an application which solves a real-world SSI-problem in a Nordic Context.
- Develop the application as an open-source project, on behalf, and in the name of, the DIN-Foundation.

## 1.x.x Læringsmål

@TODO

## 1.x Target audience

### 1.x.x Government institutions
- The proof-of-concepts is developed specifically to solve the problem of a government institution.
- The demonstration could inspire lawmakers and government officials to see the true potential of how SSI could make a real change for the better.

### 1.x.x Developers who are new into SSI
- When getting into SSI for the first time, the landscape can be a bit difficult to navigate.
- This project navigates the SSI landscape from a beginners perspective, and by doing so hopes to make the "Getting started"-part for other developers a bit easier.

### 1.x.x Specification writers
- Specification writers - people who write the documents that dictate implementations of SSI - are concerned with how beginner-friendly their specifications are.
- This work may shed some light on how easy it is for new-comers to get started, and may as a result lead to suggested improvements to current specs.

## 1.x Roles

| Name        | Role                                                            |
|-------------|-----------------------------------------------------------------|
| Jonas       | Developer                  (Service Delivery Manager in Kanban) |
| Snorre      | Client and SSI expert 1 (Service Request Manager in Kanban)  |
| Mariusz     | SSI expert 2                                                 |
| Abylay      | SSI expert 3                                                 |
| Deepti      | Academic supervisor

## 1.x The Report

@TODO

## 1.x Sources

| # | Who | What                         | Where              | When        |
|---|-----|------------------------------|--------------------|------------|
| 1 | W3C | Home Page                    | https://www.w3.org | 2021-05-06 |
| 2 | DIF | Home Page                    | https://identity.foundation/ | 2021-05-06 |
| 3 | W3C | DID Core                     | https://www.w3.org/TR/did-core/ | 2021-03-30 |
| 4 | W3C | DID Core 1.2 design goals    | https://www.w3.org/TR/did-core/#design-goals | 2021-05-06 |
| 5 | W3C | did-use-cases                | https://www.w3.org/TR/did-use-cases/ | 2021-03-30 |
| 6 | Hyperledger | didcomm-messaging-v1 | https://github.com/hyperledger/aries-rfcs/tree/master/concepts/0005-didcomm | 2021-03-30 |
| 7 | DIF         | didcomm-messaging-v2 | https://identity.foundation/didcomm-messaging/spec/ | 2021-03-30 |
| 8 | W3C | vc-data-model                | https://www.w3.org/TR/vc-data-model/ | 2021-03-30 |

