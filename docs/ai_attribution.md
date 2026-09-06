# AI Attribution Policy

This repository contains content generated with the assistance of Google Gemini 3.8. This document explains how we record, disclose, and manage AI-generated contributions.

What "AI-generated" means here
- "AI-generated" indicates that code, documentation, or other artifacts were produced primarily by a generative model (Gemini 3.8) using curated prompts and toolchains.
- Human contributors may have edited, validated, or reorganized generated content, but the original creation was model-assisted.

How we track AI-generated files
- Files created by the generation pipeline SHOULD include a header comment noting the model used and the date of generation when feasible.
- The repository keeps a best-effort mapping between generated components and manual edits in docs/ai_attribution.md and changelog entries.

Transparency requirements
- Releases and changelogs MUST list model/tooling used, AI-generated subsystems, human edits, and validation performed.
- PRs that modify AI-generated files MUST document which parts were changed and why.

Limitations and disclaimers
- AI-generated code may contain subtle correctness, safety, or licensing issues. We explicitly do not claim this code is production-ready.
- Users should perform independent review and testing before reusing or running any code from this repository.

Contact
- For questions about provenance or to report attribution metadata errors, open an issue or contact the repository owner.
