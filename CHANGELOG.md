# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2025-06-20

### Added
- Added comprehensive CHANGELOG.md following Keep a Changelog format
- Added table format for Rust crates documentation in README
- Added author information to Cargo.toml

### Changed
- Updated dependencies to compatible stable versions:
  - tonic: 0.11 → 0.12
  - prost: 0.12 → 0.13  
  - rand: 0.8 (pinned for ecosystem compatibility, rand 0.9 ecosystem not fully mature)
  - hex: 0.4 (explicit version, was "0")
  - tonic-build: 0.12 (explicit version, was "0")
- Completely rewrote README with clearer instructions and step-by-step examples
- Improved documentation comments in lib.rs
- Enhanced build and usage instructions

### Fixed
- Resolved compilation errors with tonic/prost version compatibility
- Fixed missing imports and deprecated function warnings
- Corrected `generate_random_number_below` implementation and documentation

## [0.1.0] - 2024-2025 (Historical)

### Added
- Initial Chaum-Pedersen Zero Knowledge Proof implementation
- gRPC client/server architecture using tonic
- Command line argument parsing with clap
- Secure password input handling with rpassword
- Big integer cryptographic operations with num-bigint
- Comprehensive test suite covering various bit sizes (toy, 1024-bit, 2048-bit)
- Support for RFC 5114 cryptographic constants
- Error handling with anyhow
- Async/await support with tokio
- GitHub Actions workflow with cargo fmt and clippy checks
- Code formatting standardization using rustfmt

### Security
- Zero-knowledge password authentication (passwords never transmitted)
- Cryptographically secure random number generation
- Implementation follows Chaum-Pedersen protocol specifications

### Documentation
- Initial README with project description
- Build and usage instructions
- Protocol explanation and security notes

[Unreleased]: https://github.com/JohnBasrai/zkp-cp/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/JohnBasrai/zkp-cp/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/JohnBasrai/zkp-cp/releases/tag/v0.1.0