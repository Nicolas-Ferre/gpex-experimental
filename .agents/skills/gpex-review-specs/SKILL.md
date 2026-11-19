---
name: gpex-review-specs
description: Perform review of a specifications file.
---

## Preconditions

- Never run the skill if not explicitly referenced by another skill or the user.

## Task

Review the specifications file, which is taken from the skill arguments or conversation, else
`./tmp/specs.md`. Never modify the reviewed file.

## Workflow

The specs path is the reviewed file path relative to the repository root, prefixed with `./`
(e.g. `./tmp/specs-step1.md`).

Run these steps in order:

1. If existing, delete `./tmp/review-specs-findings-accepted.md` without reading it to avoid bias.
2. If `./tmp/review-specs-findings-refused.md` exists, read only its first line (`head -n 1`). If
   that line is not `Specs: <specs path>`, delete the file without reading the rest.
3. If the specifications file does not exist, stop and inform the user.
4. Run a single review sub-agent, passing it the specs path.
5. If there is no finding, inform the user that the specifications look good and stop.
6. Write the findings in `./tmp/review-specs-findings-accepted.md`, following the finding format and
   classes of the `gpex-review-changes` skill (don't run this skill), except that entries omit
   `[<sub-agent names>]` and locations point to the specifications file.
7. Run the judgment step (not in a sub-agent).
8. If no accepted finding is left, inform the user that the specifications look good and stop.
9. Point the user to `./tmp/review-specs-findings-accepted.md`, give the number of findings per
   class, tell them whether the review cycle can stop (see cycle rule of `gpex-review-changes`),
   and let them know that they can refine the findings with `/gpex-confirm-findings specs`.
10. If the specs path is `./tmp/specs.md`, let the user know that they can refine the
    specifications with `/gpex-specify`.

## Review sub-agent

The sub-agent reads the specifications, explores the codebase and reviews the specifications
against all the criteria below, without starting other sub-agents. It follows the rules
of the `## Review sub-agents` intro of the `gpex-review-changes` skill, with
`./tmp/review-specs-findings-*.md` instead of `./tmp/review-changes-findings-*.md`.

## Review criteria

Apply all the sub-agent definitions of the `gpex-review-changes` skill (don't run this skill) to
the specifications instead of code changes, ignoring code-only checks. Also check that:

- No statement is ambiguous or contradicts another.
- `### Testing` lists only automated tests, and covers every automatable case of the functional
  design and the edge cases.
- `### Architecture` and `### Module interactions` match real files and the existing organization.
- Listed libraries exist on their registry with the stated version, and present no risk or better
  alternative.
- The solution is the simplest possible.

You can challenge certain decisions if it makes sense. Entries of the settled sections (see
`gpex-specify`) are already agreed with the user. Only challenge an entry with a `BLOCKER`, and
only if the problem is not already covered by the entry's reason.
Template rule violations of an entry (e.g. missing reason) can still be reported.

## Finding classes

Wherever this skill or its sub-agent uses the finding classes of `gpex-review-changes`, it uses
them with these differences:

- A form-only issue (no impact on what is implemented, tested, decided or justified, e.g.
  identical duplicated statement, wording, formatting, section order) is at most `NITPICK`, even
  when it violates a written rule (e.g. the `gpex-specify` self-check). A duplicate or wording
  that diverges in meaning or is ambiguous is not form-only.
- Other `REQUIRED` findings can also cite a rule of the `gpex-specify` skill (e.g. its template
  rules).

## Judgment step

Apply the judgment step of the `gpex-review-changes` skill to
`./tmp/review-specs-findings-accepted.md`, with these differences:

- Refused findings are added to `./tmp/review-specs-findings-refused.md`. When creating this
  file, start it with a `Specs: <specs path>` line followed by an empty line.
- Refusal criteria are those of `gpex-review-changes`, ignoring code-only ones, with these
  substitutions:
    - `./tmp/review-changes-findings-refused.md` → `./tmp/review-specs-findings-refused.md`.
    - "the specifications in the user request" → the reviewed specifications file.
- Never append to `./tmp/review-changes-improvements-delayed.md`.
