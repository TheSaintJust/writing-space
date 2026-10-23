# Writing Space

A research and writing workspace for an individual founder and a small team.

## Current milestone
A Rust-based sentence-by-sentence writing interface compiled to WebAssembly for the browser.

## Run locally
You need the Rust toolchain, the WebAssembly target, and Trunk.

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve --open
```

Trunk builds the Rust browser application and serves it locally. The current prototype stores the draft in the browser's local storage. It does not yet provide team accounts, cloud sync, a server database, PDF uploads, or AI assessment.

## Build
```sh
trunk build --release
```
The generated static site is written to `dist/`. A deployment workflow should be configured and tested before sharing the app with the team.

## Project notes
- [Product plan](docs/PRODUCT_PLAN.md)
- [First screen](docs/FIRST_SCREEN.md)
- [Technical choices](docs/TECHNICAL_CHOICES.md)

## Privacy
The repository is currently public. Do not commit real drafts, confidential sources, passwords, or API keys. Change repository visibility to private before treating this as an internal team project.
