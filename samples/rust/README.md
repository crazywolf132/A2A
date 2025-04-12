# A2A Rust Implementation

This directory contains a Rust implementation of the Agent2Agent (A2A) protocol. It provides a simple client and server that demonstrate the core functionality of the A2A protocol.

## Features

- **A2A Types**: Rust structs and enums that match the A2A protocol specification
- **A2A Client**: A simple client for sending requests to an A2A server
- **A2A Server**: A basic server implementation that can handle A2A requests
- **In-Memory Task Store**: A simple store for managing tasks
- **Sample Agents**: Example agent implementations
  - **OpenAI Agent**: An agent that uses OpenAI's API to process natural language requests

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.75.0 or later)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (comes with Rust)

## Building

```bash
cd samples/rust
cargo build
```

## Testing

```bash
cd samples/rust
cargo test
```

The tests cover:
- Serialization and deserialization of A2A types
- Task store operations (create, get, update, cancel)

## Running the Server

```bash
cargo run --bin a2a-server
```

By default, the server listens on `http://localhost:3000`. You can change the host and port using command-line arguments:

```bash
cargo run --bin a2a-server -- --host 0.0.0.0 --port 8080
```

## Running the Client

```bash
cargo run --bin a2a-client
```

By default, the client connects to `http://localhost:3000`. You can specify a different server URL:

```bash
cargo run --bin a2a-client -- --url http://example.com/a2a
```

## Running the OpenAI Agent

The OpenAI agent requires an API key to be set as an environment variable:

```bash
export OPENAI_API_KEY=your_api_key_here
cargo run --bin openai_agent
```

You can also create a `.env` file in the root directory with your API key:

```
OPENAI_API_KEY=your_api_key_here
```

See the [OpenAI Agent README](src/agents/openai_agent/README.md) for more details.

## Using the Client

The client provides a simple command-line interface for interacting with an A2A server:

1. Type a message and press Enter to send it to the server
2. The server will respond with a message and possibly artifacts
3. Type `exit` to quit the client

## Implementation Details

### Types

The `types.rs` file contains Rust structs and enums that correspond to the A2A protocol types defined in the JSON schema. These include:

- `TaskState`: An enum representing the possible states of a task
- `Message`: A struct representing a message exchanged between a user and an agent
- `Task`: A struct representing a task being processed by an agent
- Various JSON-RPC request and response types

### Client

The `client.rs` file implements a simple A2A client that can:

- Send a message to a task
- Get a task by ID
- Cancel a task

### Server

The `server.rs` file implements a basic A2A server using the [Axum](https://github.com/tokio-rs/axum) web framework. It can:

- Handle JSON-RPC requests
- Create and manage tasks
- Process messages and generate responses

### Task Store

The `store.rs` file implements a simple in-memory store for tasks. In a production environment, you would likely replace this with a persistent database.

## Extending the Implementation

This implementation provides a starting point for building A2A applications in Rust. Here are some ways you might extend it:

- Add support for streaming responses using Server-Sent Events (SSE)
- Implement push notifications
- Add authentication and authorization
- Replace the in-memory task store with a persistent database
- Integrate with AI models for more sophisticated message processing

## License

This implementation is licensed under the Apache License 2.0, the same as the A2A protocol itself.
