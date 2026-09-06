# Release Process

This document describes the recommended release process for this experimental, AI-assisted project. The goal is to ensure releases are transparent about model usage and validation.

1. Prepare release candidate
   - Ensure CHANGELOG.md is updated with a comprehensive entry describing the release (use the template in CHANGELOG.md).
   - Run any available tests, static analysis, and CI jobs. Record CI run IDs and artifacts.

2. Draft release notes
   - Use RELEASE_NOTES.md as the template. Include model/tooling used and the AI-generated vs manually edited components.

3. Publish release
   - Create a GitHub Release with the release notes. Attach any CI artifacts or logs used for validation.

4. Post-release follow-up
   - Monitor issues and collect feedback. If a release introduces a high-severity issue, publish a hotfix and clearly document it in the changelog and release notes.


Auditability
- Keep a record (in the changelog or release notes) of what prompts or generation pipelines were used when feasible. This increases reproducibility and helps future reviewers understand context.
