---
name: gpex-implement
description: Implement a change.
---

## Preconditions

- Never run the skill if not explicitly referenced by another skill or the user.

## Task

Implement a change autonomously.

## Workflow

1. If no change is provided, the change is the GitHub issue matching the branch number prefix
   (`gh issue view <id> --json body`): stop and inform the user if there is none, and ask the
   user for the scope if the branch covers only part of it.
2. If the change is a GitHub issue, use its body as change and specifications. Else, if no
   specifications are provided, get them from the issue matching the branch number prefix, if
   any, using only their settled sections (see `gpex-specify`), never as part of the change.
3. If an issue body used above still contains the `TO REVIEW` warning, ask the user before using
   it.
4. Delete `./tmp/implement-decisions.md` if existing.
5. Delete `./tmp/implement-plan.md` if existing.
6. Explore the codebase and create a detailed plan of the requested change in
   `./tmp/implement-plan.md`.
7. Apply the plan from `./tmp/implement-plan.md`.

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

Respect the settled sections (see `gpex-specify`) of the provided specifications, if any. Always
apply the following conventions, except those waived in `## Decisions` of the specifications:

- Always implement the simplest possible solution, don't over-engineer.
- Follow existing coding style and naming in the repository.
- Apply Single Responsibility Principle.
- Add or update tests covering every new or changed behavior, including edge cases.
- Update the documentation impacted by the change.
- Make configured checks pass at the end of the implementation (see `./AGENTS.md`)
- Add code comments only when strictly required: document only the "why" and details that cannot be
  understood from the code only, never "what" the code is doing. Always favor code readability and
  improvements over adding a comment. Only exception is for public interfaces such as libraries.
- Avoid long sentences with a lot of commas.
