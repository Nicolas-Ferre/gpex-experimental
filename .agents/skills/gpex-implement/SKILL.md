---
name: gpex-implement
description: Implement a change with the best possible quality.
---

## Preconditions

- Never run the skill if not explicitly referenced by another skill or the user.

## Task

Implement a change while focusing as much as possible on quality.

## Workflow

1. If no change is provided by the user, stop.
2. If there are uncommitted changes, ask confirmation that the skill can continue.
3. Run a sub-agent to plan changes and return the plan.
4. Run a sub-agent to apply the plan, including configured checks defined in `./AGENTS.md`.
5. Run 2 times the following:
    1. Run `gpex-simplify` skill.
    2. Stop this loop if the skill didn't apply any simplification.
6. Run 3 times the following:
    1. Run `gpex-review` skill and provide the request provided by the user as a reference.
    2. Stop this loop if `./tmp/review-findings-accepted.md` doesn't contain any finding.
    3. Run a sub-agent to apply findings in `./tmp/review-findings-accepted.md`.
    4. Run `gpex-simplify` skill.
7. If the previous loop is not stopped because no finding is found:
    1. Run `gpex-review` skill and provide the request provided by the user as a reference.
    2. Make `./tmp/review-findings-accepted.md` high level enough so that it is not needed to read
       the code to understand the findings and their implications.
    3. Don't apply findings and let user review the file `./tmp/review-findings-accepted.md`.

## Rules

- Any decision taken by this skill or any inner skill that is not provided by the user should be
  documented in `tmp/implement-decisions.md`, and user should be notified at the end of the skill
  running that the file is not empty. Only decisions impacting the following should be documented:
    - behavior
    - public interface
    - security
    - safety
