---
name: gpex-review
description: Perform extensive code review of the current branch.
---

## Preconditions

- Never run the skill if not explicitly referenced by another skill or the user.

## Task

Review the changes of the current branch (compared to `main` branch, uncommitted changes included).

## Workflow

Run these steps in order:

1. If there is no change in the current branch, stop.
2. If existing, delete `./tmp/review-findings-accepted.md` without reading it to avoid bias.
3. Run the review sub-agents defined later.
4. Once all sub-agents are done, write in `./tmp/review-findings-accepted.md` only the found issues
   the following way:
   ```markdown
   1. [<sub-agent name>] **<first finding>**
      <brief details and short snippet(s) of the proposed fix>
   2. [<sub-agent name>] **<second finding>**
      <brief details and short snippet(s) of the proposed fix>
      ...
   ```
   Don't hesitate to illustrate the findings with snippets to make it easier to understand.
   Pass user request if present to each sub-agent so that they have more context.
5. Based on found issues, append in `./tmp/review-improvements-delayed.md` all missing deterministic
   checks and tools that could have prevented the found issues (if not already present in the file).
6. Run the judge sub-agent.

## Review sub-agents

Each subsection corresponds to a sub-agent to run during review.

The sub-agents must return the full list of findings.

### Bug detection

- Check all bugs related to the current branch changes.
- Do not miss any bug.

### Architecture review

Challenge the architecture and verify whether:

- the solution is future-proof and will not cause issues with future changes in the project
- the solution generalizes well and avoids uncovered edge cases
- the solution is not unnecessarily complicated

### Tests review

Check for missing tests, for example related to:

- An uncovered code branch
- An uncovered case related to the current changes
- An existing test that doesn't follow general conventions

### Code quality check

Check local code quality issues, such as:

- Single Responsibility Principle that is not correctly applied
- Code comments that explain the "what" (which is already clarified by the code itself) instead of
  the "why". "Code as documentation" principle is favored.
- readability/maintainability issues
- inconsistent styling
- bad practices
- performance improvements
- not followed conventions defined in `AGENTS.md` files
- not up-to-date documentation
- typos or weird formulations

### ISO/IEC 25010:2023 compliance

Review the current change against every ISO/IEC 25010:2023 product-quality characteristic:

- Functional suitability.
- Performance efficiency.
- Compatibility.
- Interaction capability.
- Reliability.
- Security.
- Maintainability.
- Flexibility.
- Safety.

## Judge sub-agent

The independent judge reads `./tmp/review-findings-accepted.md`, and applies a judgment on each
finding:

- If the judge accepts the finding, it is kept in `./tmp/review-findings-accepted.md`.
- If the judge refuses the finding, it is appended in `./tmp/review-findings-refused.md` (if not
  already present in the file) with the refusal reason.
- If the judge corrects a finding, it is modified in `./tmp/review-findings-accepted.md`.

It refuses a finding when it is:

- Already applied.
- Present in `./tmp/review-findings-refused.md`.
- Formatting the configured checks (see `./AGENTS.md`) already accept, or wording that does not
  change meaning.
- A rename that is a preference between two clear names. Keep a rename when the current name makes
  a reader get something wrong, or miss something.
- A comment that restates the code. A comment that records a non-obvious constraint can be kept.
- An edit to code the change does not need to touch.
- A remedy whose only form is a new test runner, linter, or dependency. They should be appended in
  `./tmp/review-improvements-delayed.md` if not already there.
