---
name: gpex-specify
description: Define all necessary open points to specify a change.
---

## Task

Create functional specifications for a given change.

## Workflow

1. If `./tmp/specs.md` exists (don't read it), ask user if they want to refine the existing
   specifications or start over.
    1. If user wants to start over, then delete the file without reading its content to avoid being
       biased.
    2. Else, read the file.
2. Ask all open questions one by one on topics defined below. Make them easy to read and as visual
   as possible (diagrams, GPEx code snippets, ...).
3. Save specifications in `./tmp/specs.md` following the template defined below.
4. Let user know that they can refine the specifications if needed.
5. Stop and don't implement anything.

## Questions

Ask all necessary questions to fully align with user on the change to perform, especially on the
following topics:

- Scope.
- Expected behavior.
- All edge cases.
- Error handling.
- Security when necessary.

The change should be thought with ISO/IEC 25010:2023 standard in mind to maximize quality,
especially the following categories:

- Functional suitability.
- Compatibility.
- Interaction capability.
- Security.
- Flexibility.
- Safety.

## Specification template

```markdown
## Description

Short description of the change to perform, without entering into details.

## Details

Detailed specifications in an easy-to-read and visual way, for example:

- Short bullet points.
- Mermaid diagrams.
- GPEx code snippets.
- ...

This section can be split in subsections for each major aspect of the change.

## Diagnostics

Remove the section if not relevant.

One subsection per error case introduced by the change, with for each case:

- Trigger condition
- Result on user side

Code or/and diagnostic snippets can be used to illustrate each case when possible.

## Security

Remove the section if not relevant.

Any real security risks identified for the new change in a table with the following columns:

- Risk
- Reproduction
- Mitigation

## Out of scope

- List of related points that are out of the change scope. Avoid mentioning these points in the rest
  of the specifications.
```

Avoid duplicating information across the specifications as much as possible, and avoid unnecessary
verbosity without losing information.
