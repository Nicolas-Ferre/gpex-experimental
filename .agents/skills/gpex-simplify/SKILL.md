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
2. Run `gpex-implement` skill to apply only simplifications meeting the acceptance criteria below.
   If none qualifies, make no change.

## Simplification acceptance

A good simplification:

- doesn't change behavior.
- doesn't negatively impact maintainability and understandability.
- makes the code significantly shorter to perform the same operation.
