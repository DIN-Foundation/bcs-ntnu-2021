# Architecture

What should be the architecture?
We have a CLI application.
We have DIDComm-rs.
We have DIDKit-rs for verifiable credentials.
We have STDIN/STDOUT.

```
Producer --> Transport --> Consumer
```

# Demo

Given 4 CLI agents.
All agents elects 1 agent to be the **GOVERNMENT**.
The **GOVERNMENT** gives all agents a name credential.
The **GOVERNMENT** elects 1 agent to be the **VEGVESEN**.
The **GOVERNMENT** elects 1 agent to be the **POLICE**.
