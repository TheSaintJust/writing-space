# First screen: Writing Space

## Purpose
Help a writer focus on a draft while keeping each sentence independently editable.

## Layout
- **Left sidebar:** document list, New document action, and current document title.
- **Main column:** document title and sentence blocks in reading order.
- **Right panel:** initially a quiet placeholder for future Sources and Argument feedback. It must not distract from writing.
- **Top bar:** save status and basic document actions.

## Sentence block behaviour
- Each block contains one editable sentence or an unfinished sentence fragment while drafting.
- Entering text must not unexpectedly split or discard text.
- An explicit **Add sentence** action adds a new block after the current block.
- Each block can be moved up or down.
- Each block can be deleted with a clear affordance; avoid accidental deletion.
- Blank blocks are allowed while drafting.
- Selecting a block may later reveal citation and argument-analysis controls.

## First milestone boundaries
Include:
- Local in-browser prototype interactions.
- Sample document content.
- Add, edit, reorder and remove sentence blocks.
- Responsive layout.

Do not include yet:
- Login and team permissions.
- Cloud database or PDF uploads.
- AI API integration.
- Billing or user tracking.
- Claim that data is safely saved to the cloud before persistence is implemented and tested.

## Acceptance criteria
A nontechnical user can open the app, recognize where to write, edit individual sentences, add another sentence, move a sentence, and remove a sentence without losing surrounding text.
