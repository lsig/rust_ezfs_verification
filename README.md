Setup instructions

Prerequisite:
- install Rust: ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
- install kani: ```cargo install --locked kani-verifier```
- setup kani: ```cargo kani setup```

To run (from the root directory of the project):
```bash
cargo kani
```
