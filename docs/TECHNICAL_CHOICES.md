# Technical choices — explained plainly

This file records the current decisions for Writing Space. Choices can change after a working prototype is tested.

## What each part means
- **GitHub:** the project's code folder and change history.
- **Frontend:** the screen people use to write and edit.
- **Backend/API:** the protected service that checks permissions and saves or retrieves work.
- **Database:** structured storage for workspaces, documents, sentence blocks, sources, citations, reviews, and versions.
- **File storage:** private storage for uploaded PDFs and other research files.
- **Deployment/hosting:** publishing the app so invited users can access it online.

## Proposed stack
- **Browser interface:** Rust + Leptos compiled to WebAssembly.
- **Static hosting:** Cloudflare Pages, after the build is verified.
- **Structured database:** Cloudflare D1, once a backend API is implemented.
- **PDF storage:** Cloudflare R2 candidate for a later milestone.
- **Authentication:** must be chosen and tested before team data is stored. A private repo or hidden URL is not authentication.
- **AI:** a server-side provider may be added after the source/citation workflow exists. Never put API keys in browser code.

## Current status
- Rust/Leptos interface and GitHub Actions build workflow are in the repository.
- The interface currently stores one draft in this browser's local storage. It is not multi-document cloud storage and does not sync between devices.
- A starter D1 SQL migration is in `migrations/0001_initial_schema.sql`; it is not applied to a live database and is not connected to the app.
- Cloudflare deployment steps are documented in `docs/CLOUDFLARE_DEPLOYMENT.md`.
- Argument-review behavior is specified in `docs/ARGUMENT_REVIEW_SPEC.md`; that specification is not a substitute for an implemented, tested assessor.

## Privacy and reliability rules
- Keep the GitHub repository private before putting real product plans or sensitive code in it.
- Never commit passwords, tokens, API keys, or real user documents.
- Before using real team writing, add authentication and check workspace permissions on every server request.
- Do not claim a document is saved to the cloud until the backend confirms the write.
- Test with fictional writing before uploading sensitive research.
- Explain paid services and possible costs before enabling them.
- Keep backups and test recovery before relying on the app for important work.
