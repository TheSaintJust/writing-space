# Technical choices — explained plainly

This file records decisions for the early version of Writing Space. Choices can change as the product is tested.

## What each part means
- **GitHub:** the project's code folder and change history.
- **Frontend:** the screen people use to write and edit.
- **Backend:** the behind-the-scenes logic that saves and processes work.
- **Database:** structured storage for documents, sentence blocks, citations, and source details.
- **File storage:** storage for uploaded files such as PDFs.
- **Deployment/hosting:** publishing the application so invited users can access it online.

## Proposed first stack
- React + TypeScript for the interface.
- Cloudflare Workers for hosting and server-side functions.
- Cloudflare D1 for structured data, if suitable after checking current limits.
- Cloudflare R2 for uploaded PDFs when file uploads are implemented.
- A separate AI provider only when we implement AI assessment.

## Privacy and safety rules
- Keep the GitHub repository private before putting real product data or secrets into it.
- Never commit passwords, tokens, API keys, or real user documents.
- Do not treat a hosted app as private merely because the code repository is private.
- Add authentication and document-level access controls before using real team writing.
- Test with fictional/sample writing before uploading sensitive research.
- Explain paid services and any unexpected costs before enabling them.
- Verify current provider pricing and limits at the point of setup; free tiers can change.
- Make backups and test recovery before relying on the app for important work.

## Important limitation of the first interface
If it uses only browser memory or local state, it is a prototype. Closing or refreshing the page may lose unsaved content. Cloud saving should only be promised after database persistence is implemented and tested.

## Decision log
- Audience: founder plus a small team.
- First priority: sentence-by-sentence writing interface.
- Initial storage/hosting direction: evaluate Cloudflare-first hosting.
- Status: planning files created; application implementation and deployment have not yet been completed.
