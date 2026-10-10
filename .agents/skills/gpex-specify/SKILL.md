---
name: gpex-specify
description: Define all necessary open points to specify a change.
---

## Task

Create functional specifications for a given change.

## Workflow

1. Ask all open questions one by one on topics defined below. Make them easy to read and as visual
   as possible (diagrams, GPEx code snippets, ...).
2. Save specifications in `tmp/SPECS.md` following the template defined below.
3. Ask user to verify.
    1. If user agrees, continue.
    2. If user disagrees, retrieve what needs to be changed and go back to 1.
4. Propose a PR split: each PR should be as granular as possible, but adding value to final user and
   testable end to end. It is common to create an initial PR for the minimal scope and then other
   PRs to incrementally reach the full scope of the change, but a list of independent PRs is also
   possible if more relevant.
    1. If user agrees, stop and don't implement anything.
    2. If user disagrees, retrieve what needs to be changed and go back to 4.
5. Stop and don't implement anything.

## Questions

Ask all necessary questions to fully align with user on the change to perform, especially on the
following topics:

- Scope.
- Expected behavior.
- All edge cases.
- Security when necessary.

The change should be thought with ISO/IEC 25019:2023 standard in mind to maximize quality,
especially the following categories:

- Functional efficiency.
- Compatibility.
- Interaction capability.
- Security.
- Flexibility.
- Safety.

## Specification template

```markdown
## Summary

Short description of the change to perform.

## Details

Detailed specifications in an easy-to-read and visual way, for example:

- Short bullet points.
- Mermaid diagrams.
- GPEx code snippets.
- ...

This section can be split in subsections for each major aspect of the change.

## Security

Any note regarding security risks with the new change. Remove the section if not relevant.

## Steps

- List of PR titles necessary to implement the change. 
```
