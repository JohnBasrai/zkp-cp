# Chaum-Pedersen Zero Knowledge Proof in Rust
This project implements a Chaum–Pedersen Zero Knowledge Proof (ZKP) protocol using Rust, with gRPC networking and async communication between client and server.

Chaum–Pedersen ZKP is a cryptographic proof that allows a prover to convince a verifier they know a discrete logarithm without revealing the secret value.

## Key Rust Crates Used

| Crate          | Purpose                                                     |
|----------------|-------------------------------------------------------------|
| **tonic**      | Rust implementation of gRPC for client/server communication |
| **tokio**      | Asynchronous runtime for network applications |
| **num-bigint** | Big integer arithmetic for cryptographic operations |
| **rand**       | Cryptographically secure random number generation |
| **clap**       | Command line argument parsing |
| **anyhow**     | Improved error handling |
| **prost**      | Protocol Buffers implementation |

## Building

```bash
cargo build --release
```

## Running

In one terminal, start the server:
```bash
cargo run --release --bin server
```

In another terminal, run the client:
```bash
cargo run --release --bin client -- --user-name "alice"
```

**What happens next:**

1. **Registration Step**: You'll be prompted to enter a password to register the user
   ```
   Password: [enter any password, e.g., "mypassword123"]
   ✅ Registration was successful
   ```

2. **Authentication Step**: You'll immediately be prompted to enter the password again to login
   ```
   Please provide the password (to login):
   Password (to login): [enter the SAME password again]
   ✅Login successful! session_id: abc123xyz
   ```

**Important**: Use the same password for both steps. The first creates your account, the second proves you know the password using Zero Knowledge Proof - the server never sees your actual password!

## Testing

Run the test suite to verify the ZKP implementation:
```bash
cargo test
```

## Containerization
Work in progress.
