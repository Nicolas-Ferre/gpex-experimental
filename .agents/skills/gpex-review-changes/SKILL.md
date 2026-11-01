---
name: gpex-review-changes
description: Perform extensive code review.
---

## Preconditions

- Never run the skill if not explicitly referenced by another skill or the user.

## Task

Review the changes of the current branch since its merge base with `origin/main` (uncommitted and
untracked changes included).

## Workflow

The branch label is the output of `git branch --show-current`, or `(detached HEAD)` when empty.

Run these steps in order:

1. Run `git fetch origin main`.
2. If existing, delete `./tmp/review-changes-findings-accepted.md` without reading it to avoid
   bias.
3. If `./tmp/review-changes-findings-refused.md` exists, read only its first line (`head -n 1`).
   If the branch label is `(detached HEAD)` or that line is not `Branch: <branch label>`, delete
   the file without reading the rest.
4. If there is no change in the current branch (uncommitted and untracked changes included), stop.
5. Get the user request from the skill arguments or conversation, else from the GitHub issue
   matching the branch number prefix (`gh issue view <id> --json title,body`). If none is found,
   or the branch covers only part of the issue, ask the user for the scope. Run every review
   sub-agent in parallel, passing it the user request.
6. Once all sub-agents are done, write in `./tmp/review-changes-findings-accepted.md` only the found
   issues, following the finding format.
   If there is no finding, inform the user that the changes look good and stop.
7. Run the judgment step (not in a sub-agent).
8. Based on accepted findings, append in `./tmp/review-changes-improvements-delayed.md` all missing
   deterministic checkers that could have prevented the found issues (if not already present in the
   file). These checkers must:
    - be fully deterministic and not rely on AI.
    - generalize well (they are not put in place for specific edge cases).
9. If no accepted finding is left, inform the user that the changes look good. Else, point the
   user to `./tmp/review-changes-findings-accepted.md`, and let them know that they can refine the
   findings with `/gpex-confirm-findings changes`.

## Review sub-agents

Each subsection corresponds to a sub-agent to run during review.

The sub-agents must not modify any file, must not read `./tmp/review-changes-findings-*.md`, and
must return the full list of findings, following the finding format.
They report absolutely all possible findings, even minor or uncertain ones: the judgment step
refuses the invalid ones.

### Bug detection

- Find all bugs, including suspected ones.
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

## Finding format

Each finding is written as a numbered entry:

````markdown
1. [<sub-agent names>] **<short title>**
   - Location:
     - [<path>:<line>:<column>](../<path>#L<line>)
   - Problem: <what is wrong>
   - Reason: <why it is wrong>
   - Fix: <best approach to resolve it>
   - Diff:
     ```diff
     <diff snippet>
     ```
````

- Location: one sub-bullet per place where the issue occurs.
    - Link target is relative to `./tmp/` (findings files folder), link text is relative to the
      repository root.
    - Lines and columns come from the working tree. Omit the column when not meaningful
      (`<path>:<line>`).
    - Issue in removed code: line where the code was removed. Deleted file: its parent folder.
    - No precise line: most relevant file or folder, without line (e.g.
      `[tests/diagnostics/](../tests/diagnostics/)`). Omit the `Location:` bullet only when no
      path applies.
- One issue per finding. Findings describing the same issue are merged into one entry: it keeps
  the best fix, all locations and all sub-agent names.
- Fix: only the recommended approach.
- Snippet part, by order of preference:
    - `Diff:` with a `diff` block when the fix is a local text edit.
    - `Before:` and `After:` blocks when a diff would be unreadable (e.g. restructuring, diagrams).
    - Omitted when no snippet can illustrate the fix.
    - Snippet fences are longer than any backtick run inside the snippet.
- Refused entry: same entry, plus a last `- Refusal: <refusal reason>` bullet.
- The judgment step completes an incomplete entry or fixes a wrong location as a correction, and
  refuses the finding when no valid reason or fix can be stated (never only because of its
  location).

## Judgment step

This step reads `./tmp/review-changes-findings-accepted.md`, and applies a judgment on each
finding:

- If the judge accepts the finding, it is kept in `./tmp/review-changes-findings-accepted.md`.
- If the judge refuses the finding, remove it from `./tmp/review-changes-findings-accepted.md` and
  append it to `./tmp/review-changes-findings-refused.md` as a refused entry. When creating
  this file, start it with a `Branch: <branch label>` line followed by an empty line.
- If the judge corrects a finding, it is modified in `./tmp/review-changes-findings-accepted.md`.

It refuses a finding when it is:

- Already applied.
- Incorrect.
- Present in `./tmp/review-changes-findings-refused.md` and its refusal reason still applies.
- Formatting the configured checks (see `./AGENTS.md`) already accept.
- Wording that does not change meaning, unless it is a clear simplification or deduplication.
- A rename that is a preference between two clear names. Keep a rename when the current name makes
  a reader get something wrong, or miss something.
- A comment that restates the code. A comment that records a non-obvious constraint can be kept.
- An edit to code the change does not need to touch.
- A remedy whose only form is a new test runner, linter, or dependency. Append this remedy to
  `./tmp/review-changes-improvements-delayed.md` if not already there.
