# Cloudflare deployment guide

This guide is for the eventual hosted version. The app is **not cloud-backed yet**: the current Rust browser prototype saves to that browser's local storage. Creating this file does not create a Cloudflare account, database, or live deployment.

## Stage 1 — Publish the static Rust/WebAssembly interface

1. Sign in to Cloudflare and open **Workers & Pages**.
2. Choose **Create application → Pages → Connect to Git** and connect the GitHub account that owns `TheSaintJust/writing-space`.
3. Select the `writing-space` repository and production branch `main`.
4. Use a custom build configuration:
   - Build command: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && . "$HOME/.cargo/env" && rustup target add wasm32-unknown-unknown && cargo install trunk --locked && trunk build --release`
   - Build output directory: `dist`
5. Start with a preview deployment and test it before treating the URL as production.

Cloudflare's Git integration can redeploy after commits and create preview deployments. See [Cloudflare Pages Git integration](https://developers.cloudflare.com/pages/configuration/git-integration/) and [build configuration](https://developers.cloudflare.com/pages/configuration/build-configuration/).

**Important:** first verify that the GitHub Actions build passes. If Cloudflare's build environment or Rust installation changes, we may replace the long build command with a checked-in build script. Never deploy an unverified build as the team's only copy.

## Stage 2 — Create the database (when the API is ready)

The proposed database is Cloudflare D1 (SQLite-compatible). A starter schema is in `migrations/0001_initial_schema.sql`; it is a design artifact and has not been applied to any remote database.

After a Cloudflare account is connected and the backend configuration is ready:

1. Install Node.js LTS on your computer if it is not already installed.
2. From the project folder, run `npx wrangler@latest login`.
3. Create the database: `npx wrangler@latest d1 create writing-space-db`.
4. Put the database name and the ID returned by Cloudflare into the project's Wrangler configuration. Never guess the ID or commit authentication tokens.
5. Apply the schema to a **staging** database first: `npx wrangler@latest d1 migrations apply writing-space-db --remote`.
6. Test document creation, edits, source/citation links, deletion, and backup/restore before production data is entered.

Cloudflare documents database creation and migration commands at [D1 getting started](https://developers.cloudflare.com/d1/get-started/) and [D1 migrations](https://developers.cloudflare.com/d1/reference/migrations/).

## Stage 3 — Connect the application to the database

This requires a backend API; the browser must not connect to a database with privileged credentials. The API must authenticate the person making each request and check workspace membership on every read and write. It should validate input, use parameterized SQL, enforce size limits, and avoid logging private document text.

The browser UI currently has no such API. Do not treat the database schema alone as cloud saving.

## Stage 4 — Private access, files, and AI

- Gate early previews to invited people. Do not put real research into the app until identity checks and workspace/document access rules are tested.
- Add PDF storage separately (Cloudflare R2 is a candidate), with upload size/type checks and private object access.
- Add AI review only after the author explicitly requests it. Keep the original text unchanged; store suggestions separately and explain uncertainty.
- Keep AI provider keys in server-side secrets, never in Rust/WebAssembly or public GitHub files.
- Create a staging environment, backup/restore procedure, and a way to delete/export a workspace before onboarding a team.

## UX changes after deployment

1. Tell me the screen or workflow you want changed, in plain language (a screenshot is useful but optional).
2. I can edit the code in GitHub and describe the exact changes.
3. The Git integration creates a preview deployment for review; only merge/publish after you approve it.
4. For important changes, test a copy of the data first and keep the previous working version available.

## Cost and privacy

Check current Cloudflare plan limits before enabling services; free limits can change. The code repository's visibility does not determine whether the deployed app is private. A private repository is recommended for development, but the hosted app still needs access controls.
