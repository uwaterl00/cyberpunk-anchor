# Cyberpunk Mode A Anchor

This repository contains the reference implementation of the **Cyberpunk Mode A Anchor Layer**, as specified in the technical report: *"Cyberpunk: Computation-Centric Digital Assets from Multi-Party Computation."*

## Technical Overview
Cyberpunk shifts digital asset identity from ledger state to joint private computation. This `ink!` smart contract provides the **Anchor Layer**, allowing for:
- **O(1) On-Chain Footprint**: Only 32-byte manifest commitments are stored.
- **Deterministic Gas Profile**: Verification and anchoring costs are constant.
- **Asynchronous Compatibility**: Designed to work with ABFT-ordered inputs (e.g., Aleph Zero).

## Implementation
The contract is written in Rust using the `ink!` framework for Substrate-based chains. It utilizes a `Mapping` for efficient key-value storage of manifest hashes.

## Building
To compile the contract to WASM:
```bash
cargo contract build
