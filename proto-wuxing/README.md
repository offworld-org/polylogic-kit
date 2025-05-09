# Proto Wuxing

A Rust-based protocol implementation for the Wuxing system.

## Description

Proto Wuxing is a protocol implementation that provides core functionality for the Wuxing system. It handles the fundamental communication and data structures needed for the system's operation.

## Usage

### Prerequisites

- Rust 1.70 or higher
- Cargo (Rust's package manager)

### Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
proto-wuxing = "0.1.0"
```

### Basic Usage

```rust
use proto_wuxing;

// Initialize the protocol
let protocol = proto_wuxing::Protocol::new();

// Create a new message
let message = proto_wuxing::Message::new()
    .with_type("command")
    .with_payload("data");

// Send the message
protocol.send(message);

// Receive messages
if let Some(received) = protocol.receive() {
    match received.get_type() {
        "command" => {
            // Handle command message
            println!("Received command: {:?}", received.get_payload());
        }
        "response" => {
            // Handle response message
            println!("Received response: {:?}", received.get_payload());
        }
        _ => {
            // Handle unknown message type
            println!("Unknown message type received");
        }
    }
}

// Error handling
if let Err(e) = protocol.connect() {
    eprintln!("Connection error: {}", e);
}
```

### Advanced Usage

```rust
use proto_wuxing::{Protocol, Message, Config};

// Configure the protocol
let config = Config::default()
    .with_timeout(5000)
    .with_retry_attempts(3);

// Initialize with custom configuration
let protocol = Protocol::with_config(config);

// Batch message processing
let messages = vec![
    Message::new().with_type("command1"),
    Message::new().with_type("command2"),
];

for msg in messages {
    protocol.send(msg);
}

// Async usage
async fn handle_messages(protocol: &Protocol) {
    while let Some(msg) = protocol.receive_async().await {
        // Process message asynchronously
        println!("Processing message: {:?}", msg);
    }
}
```

## Development

To build the project:

```bash
cargo build
```

To run tests:

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.
