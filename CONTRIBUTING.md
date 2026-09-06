## Contributing guidelines

Thank you for your interest in contributing to this project. This repository contains large portions of AI-generated code and therefore requires additional transparency and review practices.

Before contributing
- Read docs/ai_attribution.md and README.md to understand which parts of the repository are AI-generated and what validation has been done.
- If you plan to run or test kernel-level code, do so only in isolated VMs, containers, or non-production hardware.

How to contribute
1. Open an issue describing the proposed change or problem.
2. For code changes, create a branch prefixed with `contrib/` or `fix/` (e.g., `contrib/fix-memory-init`).
3. Make small, focused commits. Each commit should compile and, where applicable, include tests.
4. Open a pull request with a clear description that includes:
   - What changed
   - Why it changed
   - Which files were AI-generated and which were manually edited
   - Validation steps (how the change was tested)

Reviewer checklist for PRs
- Does the PR clearly state which files are AI-generated and which are manual edits?
- Are changes limited in scope and well-tested where applicable?
- For modifications that touch safety- or security-sensitive code (memory management, device drivers, IPC, privileges), require additional review and testing and consider tagging maintainers with domain expertise.
- Ensure commit messages and PR description contain Signed-off-by if required by downstream consumers.

Security and responsible disclosure
- Treat security-sensitive patches with care. If you discover a vulnerability, open a private issue or contact the maintainer for responsible disclosure rather than posting full exploit details publicly.

Licensing and attribution
- If you add new files, include appropriate copyright and license headers consistent with the repository's LICENSE file.
- When modifying AI-generated files, document edits and attribute that the original file was AI-assisted.

Support and communication
- Use Issues for bug reports and feature requests.
- For discussion or design feedback, use GitHub Discussions if enabled or open a design doc in the repo and link to it from an issue.

If you'd like, maintainers can provide a PR template and more detailed reviewer checklists for specific subsystems.
