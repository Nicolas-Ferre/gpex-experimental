---
name: gpex-review
description: Perform extensive code review.
---

## Preconditions

- Never run the skill if not explicitly referenced by another skill or the user.

## Task

Review the changes of the current branch since its merge base with `origin/main` (uncommitted and
untracked changes included).

## Workflow

Run these steps in order:

1. Run `git fetch origin main`.
2. If existing, delete `./tmp/review-findings-accepted.md` without reading it to avoid bias.
3. If there is no change in the current branch (uncommitted and untracked changes included), stop.
4. Get the user request from the skill arguments or conversation, else from the GitHub issue
   matching the branch number prefix (`gh issue view <id> --json title,body`). Run every review
   sub-agent in parallel, passing it the user request.
5. Once all sub-agents are done, write in `./tmp/review-findings-accepted.md` only the found issues
   the following way:
   ```markdown
   1. [<sub-agent name>] **<first finding>**
      <brief details and short snippet(s) of the proposed fix>
   2. [<sub-agent name>] **<second finding>**
      <brief details and short snippet(s) of the proposed fix>
      ...
   ```
   Do not mix multiple issues in the same finding.
   Merge findings describing the same issue into one entry, listing all sub-agent names.
   Illustrate the findings with snippets when possible to make it easier to understand.
   If there is no finding, stop.
6. Run the judgment step (not in a sub-agent).
7. Based on accepted findings, append in `./tmp/review-improvements-delayed.md` all missing
   deterministic checkers that could have prevented the found issues (if not already present in the
   file). These checkers must:
    - be fully deterministic and not rely on AI.
    - generalize well (they are not put in place for specific edge cases).

## Review sub-agents

Each subsection corresponds to a sub-agent to run during review.

The sub-agents must not modify any file, must not read `./tmp/review-findings-*.md`, and must
return the full list of findings.

### Bug detection

- Find all real bugs, do not miss any.
- Take into account the following ISO/IEC 25010:2023 product-quality:
    - Reliability.
    - Security.
    - Safety.

### Architecture review

Challenge the architecture and verify whether:

- the solution is future-proof and will not cause issues with future changes in the project
- the solution generalizes well and avoids uncovered edge cases
- the solution is not unnecessarily complicated

Also, review the current change against the following ISO/IEC 25010:2023 product-quality
characteristics:

- Functional suitability.
- Compatibility.
- Maintainability at the level of the architecture.
- Flexibility.

### Tests review

Check for missing tests, for example related to:

- An uncovered code branch
- An uncovered case related to the current changes
- An existing test that doesn't follow general conventions

### Code quality check

Check local code quality issues, such as:

- readability/maintainability issues
- inconsistent styling
- bad practices
- performance improvements
- not followed conventions defined in `AGENTS.md` files
- not followed conventions defined in `Conventions` section from `gpex-implement` skill (don't run
  this skill)
- not up-to-date documentation
- typos or weird formulations

Also, review the current change against the following ISO/IEC 25010:2023 product-quality
characteristics:

- Performance efficiency.
- Maintainability at the level of the code.
- Interaction capability.

## Judgment step

This step reads `./tmp/review-findings-accepted.md`, and applies a judgment on each
finding:

- If the judge accepts the finding, it is kept in `./tmp/review-findings-accepted.md`.
- If the judge refuses the finding, remove it from `./tmp/review-findings-accepted.md` and append it
  to `./tmp/review-findings-refused.md` with the refusal reason.
- If the judge corrects a finding, it is modified in `./tmp/review-findings-accepted.md`.

It refuses a finding when it is:

- Already applied.
- Incorrect.
- Present in `./tmp/review-findings-refused.md` and its refusal reason still applies.
- Formatting the configured checks (see `./AGENTS.md`) already accept, or wording that does not
  change meaning.
- A rename that is a preference between two clear names. Keep a rename when the current name makes
  a reader get something wrong, or miss something.
- A comment that restates the code. A comment that records a non-obvious constraint can be kept.
- An edit to code the change does not need to touch.
- A remedy whose only form is a new test runner, linter, or dependency. Append this remedy to
  `./tmp/review-improvements-delayed.md` if not already there.
