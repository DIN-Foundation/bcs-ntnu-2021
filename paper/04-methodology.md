# 4. Methodology and Requirements

>@marni: Methodology - HOW you want to solve/implement/address the challenges/problems.

## 4.1 Use a community supported programming language

- Based on existing libraries from the community, 3 languages were considered: Rust, Golang and Python

**Rust**
- Why was Rust considered?

**Golang**
- Why was Golang considered?

**Python**
- Why was Python considered?

In the end, which of them did we go for, and why?

## 4.2 Re-using existing work as much as possible (didcomm-rs, spruceid/ssi)

Very often, a specific programming language is chosen, because we want to re-use as much as possible of existing code. This is the main reason Rust was chosen. It has an existing library for all the standards we want to comply with.

## 4.3 Comply with standards (did, didcomm, vc)

Using existing libraries, makes it easier to comply with standards:
- did-key-rs - Implements the `did`-standards.
- didcomm-rs - Implements the `didcomm-v2`-standards.
- spruceid/ssi - Implements the `vc`-standards.

## 4.4 A command line interface (Instead of a GUI)

- GUI does not solve our problem statement any better.
- A degree in Programming, does not get extra points for design.
- Time considered better spent on lower-level stuff.

## 4.5 Only support a single cryptographic toolkit for signing and encryption (ed25519/x25519)

- To limit scope.
- Supporting multiple toolkits, does not solve our problem-statement any better.
- One is enough to prove the point.
- One could rewrite the software to support multiple cryptographic toolkits later, using a plugin-based architecture.

## 4.6 Only support a single transport (stdin/stdout)

- To limit scope.
- Supporting multiple transports, does not solve our problem-statement any better.
- One is enough to prove the point.
- One could rewrite the software to support multiple transports later, using a plugin-based architecture.

## 4.7 Only support a single did-method (did-key)

- To limit scope.
- Supporting multiple did-methods, does not solve our problem-statement any better.
- One is enough to prove the point.
- One could rewrite the software to support multiple did-methods later, using a plugin-based architecture.

## 4.8 Source control (git + Github)

## 4.9 Issue tracking and Kanban board (Github projects)

## 4.10 Communication (DIN Discord, Google Meet, MS Teams, DIF Slack)

## 4.11 Weekly meetings (Domain expert tuesdays, Supervisor wednesdays)

