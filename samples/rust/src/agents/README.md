# A2A Rust Agents

This directory contains sample agent implementations for the A2A protocol in Rust.

## Available Agents

- [OpenAI Agent](./openai_agent/README.md): An agent that uses OpenAI's API to process natural language requests

## Common Features

All agents implement the A2A protocol and provide:

- Task handling
- Message processing
- Artifact generation
- A server for handling A2A requests

## Running an Agent

Each agent has its own binary that can be run using Cargo:

```bash
# Run the OpenAI agent
cargo run --bin openai_agent
```

## Interacting with Agents

You can interact with any agent using the A2A client:

```bash
cargo run --bin a2a-client -- --url http://localhost:3000
```

## Creating Your Own Agent

To create your own agent:

1. Create a new directory under `src/agents/`
2. Implement the agent logic
3. Create a server implementation
4. Create a binary for running the agent
5. Add documentation

See the existing agents for examples of how to structure your code.
