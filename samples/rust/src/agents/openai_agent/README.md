# OpenAI Agent for A2A Protocol

This directory contains a Rust implementation of an A2A agent that uses OpenAI's API to process natural language requests.

## Features

- Integrates with OpenAI's API to process messages
- Implements the A2A protocol for task handling
- Provides a simple server for handling A2A requests

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.75.0 or later)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (comes with Rust)
- An OpenAI API key

## Configuration

The agent requires the following environment variables:

- `OPENAI_API_KEY`: Your OpenAI API key
- `OPENAI_MODEL` (optional): The OpenAI model to use (defaults to "gpt-3.5-turbo")

You can set these environment variables directly or create a `.env` file in the root of the project with the following content:

```
OPENAI_API_KEY=your_api_key_here
OPENAI_MODEL=gpt-3.5-turbo
```

## Running the Agent

```bash
cd samples/rust
cargo run --bin openai_agent
```

By default, the agent listens on `http://localhost:3000`. You can change the host and port using command-line arguments:

```bash
cargo run --bin openai_agent -- --host 0.0.0.0 --port 8080
```

## Using the Agent

You can interact with the agent using the A2A client:

```bash
cargo run --bin a2a-client -- --url http://localhost:3000
```

The agent will process your messages using OpenAI's API and respond accordingly.

## Implementation Details

The OpenAI agent consists of the following components:

- `OpenAIAgent`: A struct that handles communication with OpenAI's API
- `server.rs`: A server implementation that handles A2A requests and routes them to the agent
- `openai_agent.rs`: A binary that runs the agent server

## Example Interaction

```
> Tell me about the A2A protocol
Agent: The A2A (Agent-to-Agent) protocol is an open protocol developed by Google that enables communication and interoperability between AI agents built on different frameworks or by different vendors. It provides a standardized way for agents to discover each other's capabilities, negotiate interactions, and collaborate securely on tasks.

Key features of the A2A protocol include:

1. Agent Discovery: Agents can expose their capabilities through an "Agent Card" (typically at /.well-known/agent.json) that describes their skills, endpoint URL, and authentication requirements.

2. Task Lifecycle: Tasks progress through defined states (submitted, working, input-required, completed, canceled, failed, etc.) allowing for structured interactions.

3. Messaging Format: Communication happens via JSON-RPC 2.0 over HTTP(S), with messages containing "Parts" that can be text, files, or structured data.

4. Streaming Updates: Long-running tasks can provide real-time updates using Server-Sent Events (SSE).

5. Artifacts: Agents can exchange outputs as structured artifacts.

The protocol is designed to be framework-agnostic, allowing agents built with different technologies (like LangGraph, CrewAI, ADK, etc.) to work together seamlessly. This interoperability is crucial for enterprise AI adoption, as it enables organizations to combine specialized agents from different vendors into cohesive systems.

A2A complements other protocols like Anthropic's Model Context Protocol (MCP), focusing on agent-to-agent communication rather than model-to-tool interactions.
```

## License

This implementation is licensed under the Apache License 2.0, the same as the A2A protocol itself.
