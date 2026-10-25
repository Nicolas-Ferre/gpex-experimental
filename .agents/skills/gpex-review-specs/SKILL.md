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
6. Write the findings in `./tmp/review-specs-findings-accepted.md`, following the finding format of
   the `gpex-review-changes` skill (don't run this skill), except that entries omit
   `[<sub-agent names>]` and locations point to the specifications file.
7. Run the judgment step (not in a sub-agent).
8. If no accepted finding is left, inform the user that the specifications look good and stop.
9. Point the user to `./tmp/review-specs-findings-accepted.md`.
10. If the specs path is `./tmp/specs.md`, let the user know that they can refine the
    specifications with `gpex-specify`.

## Review sub-agent

The sub-agent reads the specifications, explores the codebase and reviews the specifications
against all the criteria below, without starting other sub-agents. It follows the rules
of the `## Review sub-agents` intro of the `gpex-review-changes` skill, with
`./tmp/review-specs-findings-*.md` instead of `./tmp/review-changes-findings-*.md`.

## Review criteria

Apply all the sub-agent definitions of the `gpex-review-changes` skill (don't run this skill) to
the specifications instead of code changes, focused on the content itself (the form is less
important), ignoring code-only checks. Also check that:

- There is no duplicated information or ambiguity.
- Sections do not contradict each other.
- `### Testing` covers every case of the functional design and the edge cases.
- `### Architecture` and `### Module interactions` match real files and the existing organization.
- Listed libraries exist on their registry with the stated version, and present no risk or better
  alternative.

You can question certain decisions if it makes sense.

## Judgment step

Apply the judgment step of the `gpex-review-changes` skill to
`./tmp/review-specs-findings-accepted.md`, with these differences:

- Refused findings are appended to `./tmp/review-specs-findings-refused.md`. When creating this
  file, start it with a `Specs: <specs path>` line followed by an empty line.
- Refusal criteria are those of `gpex-review-changes`, ignoring code-only ones, reading
  `./tmp/review-specs-findings-refused.md` as the refused file.
- Never append to `./tmp/review-changes-improvements-delayed.md`.
