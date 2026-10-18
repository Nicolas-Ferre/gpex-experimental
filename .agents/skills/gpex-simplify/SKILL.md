---
name: gpex-simplify
description: Simplify the changes done on the current branch.
---

## Task

Simplify the changes of the current branch (compared to `main` branch, uncommitted changes
included).

## Workflow

1. If there is no change in the current branch, stop and return that no simplification was applied.
2. Run a sub-agent to investigate all possible simplifications and return a plan.
3. If there is no simplification to perform, stop and return that no simplification was applied.
4. Run a sub-agent to judge the plan independently and return the refined simplification plan to
   apply.
5. If there is no simplification to perform, stop and return that no simplification was applied.
6. Run a sub-agent to apply the agreed simplifications.
7. Return the number of applied simplifications.

## Rules

- Edits follow the existing local style and configured checks (see `./AGENTS.md`) the repository
  already uses.

## Simplification acceptance

A good simplification:

- doesn't change behavior.
- doesn't negatively impact maintainability and understandability.
- makes the code significantly shorter to perform the same operation.
