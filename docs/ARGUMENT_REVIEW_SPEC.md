# Argument review — behavior specification

The reviewer should help the author reason more clearly, not pretend to be an oracle.

## Claim labels
- **Descriptive:** reports what is or was the case.
- **Normative:** says what should be done or what is good/bad.
- **Analytical:** defines, distinguishes, or interprets concepts.
- **Causal:** says that one factor produces or changes another.
- **Inferential:** draws a conclusion from other stated claims or evidence.
- **Uncertain / mixed:** use when the sentence cannot reliably be assigned one label.

Labels are working hypotheses, not facts. A sentence may combine types.

## Review dimensions
1. **Evidence:** Is a verifiable claim missing a source? Does the attached source actually support the exact claim and locator?
2. **Logic:** Are premises stated? Does the conclusion follow, or is a hidden assumption doing the work?
3. **Causality:** Does the text establish a mechanism and rule out plausible alternatives, or merely describe correlation?
4. **Consistency:** Does this claim conflict with another claim in the same document?
5. **Scope:** Is a generalization broader than the evidence or population described?
6. **Structure:** Does the sentence connect to the paragraph's purpose and the document's central thesis?

## Required output for every flag
- Exact sentence or passage being reviewed.
- Category and severity.
- Plain-language reason.
- A concrete question or next step.
- Optional suggested wording, clearly separate from the author's text.
- Confidence/uncertainty and a dismiss/resolve control.

## Guardrails
- Missing citation means "check this", not "false".
- Do not invent source titles, quotations, page numbers, or bibliographic data.
- Do not silently rewrite, delete, or reorder a writer's text.
- Distinguish source reliability from whether a source supports the claim.
- Record reviewer version and allow the author to dismiss false positives.
- AI output must be treated as a suggestion and verified by the author.
