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
4. Run every review sub-agent in parallel, passing it the specs path.
5. Once all sub-agents are done, if there is no finding, inform the user that the specifications
   look good and stop.
6. Write the findings in `./tmp/review-specs-findings-accepted.md`, following the finding format and
   finding classes of the `gpex-review-changes` skill (don't run this skill), with locations
   pointing to the specifications file.
7. Run the judgment step in a dedicated sub-agent, passing it the specs path, then wait for it.
8. If no accepted finding is left, inform the user that the specifications look good and stop.
9. Point the user to `./tmp/review-specs-findings-accepted.md`, give the number of findings per
   class, tell them whether the review cycle can stop (see cycle rule of `gpex-review-changes`),
   and let them know that they can refine the findings with `/gpex-confirm-findings specs`.
10. If the specs path is `./tmp/specs.md`, let the user know that they can refine the
    specifications with `/gpex-specify`.

## Review sub-agents

Run one sub-agent per sub-agent definition of the `gpex-review-changes` skill (don't run this
skill), with the same name. Each sub-agent reads the specifications, explores the codebase and
applies its definition to the specifications instead of code changes, focused on the content
itself (the form is less important), ignoring code-only checks, without starting other
sub-agents. It follows the rules of the `## Review sub-agents` intro of `gpex-review-changes`,
with `./tmp/review-specs-findings-*.md` instead of `./tmp/review-changes-findings-*.md`.

Additional checks per sub-agent:

- Architecture review:
    - `### Architecture` and `### Module interactions` match real files and the existing
      organization.
    - Listed libraries exist on their registry with the stated version, and present no risk or
      better alternative.
    - The solution is the simplest possible.
- Tests review: `### Testing` lists only automated tests, and covers every automatable case of the
  functional design and the edge cases.
- Code quality check:
    - There is no duplicated information or ambiguity.
    - Sections do not contradict each other.

Rules shared by all sub-agents:

A `REQUIRED` finding can also cite a rule of the `gpex-specify` skill (e.g. its template rules).

You can challenge certain decisions if it makes sense. Entries of the settled sections (see
`gpex-specify`) are already agreed with the user. Only challenge an entry with a `BLOCKER`, and
only if the problem is not already covered by the entry's reason.
Template rule violations of an entry (e.g. missing reason) can still be reported.

## Judgment step

Its sub-agent reads the specifications and explores the codebase, without starting other
sub-agents, so that the agent that wrote the specifications does not judge findings on them.

Apply the judgment step of the `gpex-review-changes` skill to
`./tmp/review-specs-findings-accepted.md`, with these differences:

- Refused findings are added to `./tmp/review-specs-findings-refused.md`. When creating this
  file, start it with a `Specs: <specs path>` line followed by an empty line.
- Refusal criteria are those of `gpex-review-changes`, ignoring code-only ones, with these
  substitutions:
    - `./tmp/review-changes-findings-refused.md` → `./tmp/review-specs-findings-refused.md`.
    - "the specifications in the user request" → the reviewed specifications file.
- Never append to `./tmp/review-changes-improvements-delayed.md`.
