# 2 Background

This chapter will give a summary of theory required to understand how this project fits into a the broader SSI-context.

## 2.1 Core SSI standards

Consider our 3 areas of investigation - DID's, DIDComm and VC's. When looking at all three from a birds-eye view, it is useful to think about them in the context message-passing, similar to email. This enables us to see how these seemingly independent standards each solve their piece of a larger puzzle:

- **DID's** - Who is passing messages? (An individual, a bank, the police, my doctor, etc..)
- **DIDComm** - How are messages sent, mediated and received? (Envelope, address, correspondance, etc..)
- **VC's** - What does messages contain? (Contracts, certificates, money, cat-videos, etc..)

### 2.1.1 Decentralized identifiers (DID)

### 2.1.2 DID-comm (v2)

### 2.1.3 Verifiable Credentials (VC)




## 2.2 DID methods

### 2.2.1 did:key
- self-resolving did

### 2.2.2 did:peer
- self-resolving did.
- One did per connection.

### 2.2.3 did:uni
- Universal
- Keri project

### 2.2.4 Other


## 2.3 DID-comm transports

### 2.3.1 HTTP

### 2.3.2 Bluetooth

### 2.3.3 QR codes

### 2.3.4 STDIN/STDOUT

### 2.3.5 Other



## 2.4 Cryptography

### 2.4.1 RSA

### 2.4.2 ECDSA(x25519)

### 2.4.3 Ed25519

### 2.4.4 ECDSA vs ECDH vs Ed25519 vs Curve25519

- https://security.stackexchange.com/a/211484



>@marni: Problem statement and background (you may want to split these two into individual chapters too, depending how fat thatt chapter gets). This is where you describe the background, why the problem is what it is, and what existing solutions exist, how they try/fail to solve everything, and where your work fits in. You only PLACE it, where it fits, not describe it yet. Only research questions here.
