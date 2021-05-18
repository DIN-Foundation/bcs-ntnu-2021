# 1 Introduction

## 1.1 Problem domain - Self-Sovereign Identity

Self-Sovereign-Identity (SSI) consist of technologies, hosted by well-known standards-organizations such as World Wide Web Consortium ([W3C](https://www.w3.org/)) and Decentralized Identity Foundation ([DIF](https://github.com/decentralized-identity)).

SSI attempts to solve many problems present in today's internet. Decentralized identifiers (DID's) is a core SSI-technology, and it's design goals gives us a hint about which problems SSI is trying to solve:

*The three first design goals from [DID - 1.2 Design Goals](https://www.w3.org/TR/did-core/#design-goals)*:
>- **Decentralization** - Eliminate the requirement for centralized authorities or single point failure in identifier management, including the registration of globally unique identifiers, public verification keys, services, and other information.
>
>- **Control** - Give entities, both human and non-human, the power to directly control their digital identifiers without the need to rely on external authorities.
>
>- **Privacy** - Enable entities to control the privacy of their information, including minimal, selective, and progressive disclosure of attributes or other data.

The standards developed within the realm of SSI, are developed as open-source collaborative documents. The documents, when finalized, become specifications which is intended to be used by developers whenementing SSI-applications.

To read on, it is highly recommended that you atleast have a surface-level understanding of the following three core SSI-specifications, and how they relate to one another:


- **Descentralized Identifiers (DIDs) v1.0** - How to create and control decentralized, digital agents, represented by a globally unique identifier called a `DID` - https://www.w3.org/TR/did-core/
- **DIDComm messaging v2** - How `DID`-agents should communicate with each other - https://identity.foundation/didcomm-messaging/spec/ 
- **Verifiable Credentials Data Model 1.0** - How `DID`-agents issue verifiable claims about the attributes of each other and their relationships - https://www.w3.org/TR/vc-data-model/ 

All of these specifications are work-in-progress as of today - 2021-05-06 - and thus are subject to change, though they are considered stable enough for the SSI-community to have started developing serious applications based on them.

## 1.2 Problem Scope - Agent software

- Within the SSI space, there is a need to develop a specific type of software, called an agent.
- An agent is a piece of software which implements protocols defined by the different SSI-specifications.
- If an agent follows the specifications correctly, it should be able to communicate interoperably with other agents.
- There are many similarities between SSI-agents and web-browser-agents. All web-browsers are interoperable with each other (most of the time), because they are using the same underlying open-source, standardized web-technologies.
- SSI-agents and web-browser-agents differ in their problem domain, and are not intended to be interoperable with each other.
- Currently there are SSI-agent-implementations in Python, C#, Javascript, Rust, Golang, and probably more, and they should in theory be interoperable with each other. (@TODO link to source of implementations)
- From now on, the term "agent" strictly used to refer to an SSI-agent, NOT web-browser-agent.
- Each type of agent implements a different subset of SSI protocols. This is because different agents are made to solve different problems within the SSI problem domain.
- For instance a "Wallet", is a specific kind of agent, used to store and manage personal credentials.




## 1.3 Product Owner - Decentralized Identity Norden

Decentalized Identity Norden, or DIN, is a non-profit organization devoted to promote decentralized identity in a nordic context, and highlight the consequences this may have for individuals and society as a whole. See: https://www.din.foundation/om. 

## 1.3.1 Project motivation

DIN wants the project to implement a proof-of-concept agent. The proof-of-concept agent should solve a specific scenario which is relevant to DIN's agenda. The demonstration of solving a specific scenario should inspire and educate people about real-world use-cases of decentralized identity (aka self-sovereign identity).

## 1.3.2 Project Scenario

The proof-of-concept agent should be able to demonstrate the following scneario:

>The norwegian driver license issuer, Statens Vegvesen, is considering to start issuing it's driver licenses as verifiable credentials (VCs). Statens Vegvesen is not sure if verifiable credentials is the future yet, but are willing to try and dip it's toes in the water. Statens vegvesen will still issue credentials in the traditional way for the forseeable future.
>
>What Statens Veivesen want is a proof-of-concept SSI-application which will issue, hold and verify driver-licenses. They are hoping that this will bootstrap SSI in Norway, as this will enable other individuals and organizations to start experimenting with holding and verifying a serious credentials which actually are useful.
>
>The proof-of-concept may demonstrate, that a driver license as a verifiable credential could be considered on par, legally speaking, with traditional driver licenses.
>
>Statens Vegvesen wants the application to follow open standards which will enable the application to be agnostic about where and how credentials are issued, stored and verified. In other words Statens Vegvesen want to avoid the application to be locked to a specific ledger and a specific wallet.



## 1.4 Goals

## 1.4.x Resultatmål

- Deliver a high-level design document.
- Develop the application as an open-source project inside DIN-Foundation's Github organization, which solves the scenario.
- Present and demonstrate that the application implemented solves the scenario.
- Develop a discussion about interoperability in the SSI ecosystem.

## 1.4.x Effektmål

- Engage with the broader DIF-community during development.
- Educate people about a practical application of SSI technology.

## 1.4.x Læringsmål

- Gain deep understand SSI's layered tech stack - Layer 1 (DID), Layer2 (DIDComm), Layer 3 (VC), Layer 4 (Applications)
- Gain hands-on experience with developing a SSI agent.
- Learn a new programming language like Rust.
- Learn how to work on a project 100% remotely from start to finish.




## 1.5 Target audience

### 1.5.x Government institutions
- The proof-of-concepts is developed specifically to solve the problem of a government institution.
- The demonstration could inspire lawmakers and government officials to see the true potential of how SSI could make a real change for the better.

### 1.5.x Developers who are new into SSI
- When getting into SSI for the first time, the landscape can be a bit difficult to navigate.
- This project navigates the SSI landscape from a beginners perspective, and by doing so hopes to make the "Getting started"-part for other developers a bit easier.

### 1.5.x Specification writers
- Specification writers - people who write the documents that dictate implementations of SSI - are concerned with how beginner-friendly their specifications are.
- This work may shed some light on how easy it is for new-comers to get started, and may as a result lead to suggested improvements to current specs.




## 1.6 Team Roles

The team working on this project consists of a 1 student, 1 product owner, 2 tech supervisors and 1 academic supervisor.

| Name        | Role                                |
|-------------|-------------------------------------|
| Jonas       | Leader, Developer, Student          |
| Snorre      | Product Owner, Tech supervisor      |
| Mariusz     | Tech supervisor                     |
| Abylay      | Tech supervisor                     |
| Deepti      | Academic supervisor                 |




## 1.7 Report Overview

- **Chapter 2: Requirements** - A list of functional and non-functional requirements of the agent.
- **Chapter 3: Command-Line Interface** - A description of the user-interface of the agent.
- **Chapter 4: Architecture** - A description of the high-level design of the agent.
- **Chapter 5: Implementation** - A summary of the agents low-level source-code.
- **Chapter 6: Testing** - How testing of the agent is done.
- **Chapter 7,8,9,10,11,12** - Discussion. Chapter layout is @WIP @TODO.

