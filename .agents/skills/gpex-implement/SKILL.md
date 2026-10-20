---
name: gpex-implement
description: Implement a change.
---

## Preconditions

- Never run the skill if not explicitly referenced by another skill or the user.

## Task

Implement a change autonomously.

## Workflow

1. If no change is provided, stop and inform the user.
2. Delete `./tmp/implement-decisions.md` if existing.
3. Delete `./tmp/implement-plan.md` if existing.
4. Explore the codebase and create a detailed plan of the requested change in `./tmp/implement-plan.md`.
5. Apply the plan from `./tmp/implement-plan.md`.

## Taken decisions

Any decision taken by this skill or an inner sub-agent that is not provided by the user should be
documented in `./tmp/implement-decisions.md`, and user should be notified at the end of the skill
running that the file is not empty.

Only decisions impacting the following should be documented:

- behavior
- public interface
- security
- safety

## Conventions

Always apply the following conventions:

- Always implement the simplest possible solution, don't over-engineer.
- Follow existing coding style and naming in the repository.
- Apply Single Responsibility Principle.
- Add or update tests covering every new or changed behavior, including edge cases.
- Make configured checks pass at the end of the implementation (see `./AGENTS.md`)
- Add code comments only when strictly required: document only the "why" and details that cannot be
  understood from the code only, never "what" the code is doing. Always favor code readability and
  improvements over adding a comment. Only exception is for public interfaces such as libraries.
