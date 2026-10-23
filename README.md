# Writing Space

A research and writing workspace for an individual founder and a small team. The intended workflow is sentence-by-sentence writing, with sources and citations beside claims and an argument reviewer that explains its concerns without silently changing the author's text.

## Current status — early prototype

The Rust + Leptos interface is a prototype. It supports editing sentence blocks, adding/reordering/deleting blocks, a title, word counts, in-browser saving, and Markdown export. It currently stores a single draft in the browser's local storage.

**Not yet implemented:** multi-document storage, team sign-in/sharing, a connected cloud database, PDF uploads, citation attachment workflow, and live AI assessment. The SQL schema and behavior specifications are preparation for those features, not proof that they work.

## Run locally

Install Rust and then run:

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve --open
```

Build the static site:

```sh
trunk build --release
```

The generated site is written to `dist/`.

## Project notes

- [Product plan](docs/PRODUCT_PLAN.md)
- [Technical choices](docs/TECHNICAL_CHOICES.md)
- [Cloudflare deployment guide](docs/CLOUDFLARE_DEPLOYMENT.md)
- [Database schema migration](migrations/0001_initial_schema.sql)
- [Argument review specification](docs/ARGUMENT_REVIEW_SPEC.md)

## Privacy

The repository visibility should be checked before confidential material is added. Do not commit real drafts, private sources, passwords, or API keys. A private repository does not make the deployed website private; the hosted app needs its own authentication and authorization.
