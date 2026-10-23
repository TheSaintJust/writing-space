# Writing Space — Product Plan

## Vision
A collaborative writing and research workspace for an individual founder and a small team. Writing Space helps authors write carefully, keep sources attached to claims, and improve reasoning without taking control away from the author.

## First milestone: sentence-by-sentence writing interface
Build a simple, usable writing screen before adding complex AI features.

### Core experience
- Create and name a document.
- Write one sentence per editable block.
- Add a new sentence, delete a block, and move blocks up or down.
- Reorder blocks without losing their text.
- Keep a document title and a clear save state.
- Start with a small sample document so the interface is understandable immediately.
- Use a calm, readable, multi-column layout that leaves room for writing and later source/feedback panels.
- Make the interface accessible and usable on common laptop screens.

### Product principles
- The writer remains in control; AI suggestions never silently rewrite or delete text.
- Preserve authors' work and communicate whether changes are saved.
- Do not add paid services, external API keys, or public deployment without explaining the choice first.
- Keep private research private; verify repository visibility and access settings before storing real writing.
- Build in small steps and explain technical decisions in plain language.

## Planned milestones
1. **Writing interface:** sentence blocks, add/remove/reorder, title, save indicator, responsive layout.
2. **Document persistence:** save and reopen documents reliably.
3. **Team collaboration:** individual accounts and controlled document sharing.
4. **Source manager:** web links, bibliographic details, PDFs, notes, and page-level references.
5. **Citation manager:** connect individual claims to source passages and export citations.
6. **AI argument assessor:** classify sentences (descriptive, normative, analytical, causal, inferential); evaluate support, logical gaps, contradictions, and links to prior arguments; provide explanations and suggested revisions.
7. **Export and history:** recover earlier versions and export drafts.

## Argument assessment principles
- Distinguish factual accuracy from logical form.
- Distinguish a descriptive claim from a normative claim and from an analytical or causal inference.
- Treat absence of a citation as a prompt to review, not proof a claim is false.
- Show the reason for each flag and offer actionable feedback.
- Make uncertainty visible and allow the author to dismiss incorrect suggestions.
- Do not fabricate sources, quotations, page numbers, or evidence.

## Initial technical direction (subject to validation)
- Frontend: React + TypeScript.
- Hosting and application API: Cloudflare Workers.
- Database: Cloudflare D1 for the first version, if its limits and data model fit.
- File storage: Cloudflare R2 when PDF uploads are introduced.
- AI: no AI provider until the assessment milestone; provider and costs must be explained before enabling it.
- Use GitHub for source code and change history.

These are proposed choices, not a promise that every service is free in all circumstances. Check current plan limits and pricing before deployment.

## First acceptance checklist
- [ ] A writer can create and rename a document.
- [ ] Each sentence appears in its own editable block.
- [ ] A writer can add, delete, and reorder sentence blocks.
- [ ] Text does not disappear when a block moves.
- [ ] The screen gives clear feedback about unsaved and saved changes.
- [ ] Layout is understandable without technical knowledge.
- [ ] No API key or secret is committed to the repository.
- [ ] A test/deployment flow is documented before public launch.
