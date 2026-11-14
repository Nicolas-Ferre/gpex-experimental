---
name: gpex-simplify
description: Simplify the changes done on the current branch.
---

## Preconditions

- Never run the skill if not explicitly referenced by another skill or the user.

## Task

Simplify the changes of the current branch since its merge base with `origin/main` (uncommitted
and untracked changes included).

## Workflow

1. Run `git fetch origin main`.
2. Get the specifications from the skill arguments or conversation, else from the GitHub issue
   matching the branch number prefix (`gh issue view <id> --json body`), if any.
3. List the simplifications meeting the acceptance criteria below, limited to the changes of the
   current branch. If none qualifies, stop.
4. Run `gpex-implement` skill with this list as the change, and the specifications as
   constraints only (not part of the change).

## Simplification acceptance

A good simplification:

- doesn't change behavior.
- doesn't contradict the settled sections (see `gpex-specify`) of the specifications.
- doesn't negatively impact maintainability and understandability.
- makes the code significantly shorter to perform the same operation.
