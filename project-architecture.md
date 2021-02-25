# Architecture

This document should not be a top-down perfect description of the architecture, because we don't know the architecture before we tried something. This is just to get started...

## One application

We need a single application, which implements all the features. The application will implement an agent, an SSI-agent. We cannot know which agent will take on which role upfront, so all agents needs all capabilities.

## The CLI

The CLI will be used to do operations such as: 
- Create new agent
- Select government
- Issue credential to another agent.
- List possible credentials for issuing.
- Present credential to other agent.
- List credentials.

## Transport

DIDComm is agnostic about the transport. It could be anything like HTTP, ZeroMQ, libp2p. For simplicity sake we will do transport over STDIN/STDOUT. This allows for transporting DIDComm-messages via files, or messages in other applications, or via the clipboard.

