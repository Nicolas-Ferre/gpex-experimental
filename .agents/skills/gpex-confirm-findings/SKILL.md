---
name: gpex-confirm-findings
description: Validate gpex-review-changes or gpex-review-specs findings.
---

## Preconditions

- Never run the skill if not explicitly referenced by another skill or the user.

## Task

Validate accepted findings of a review.

## Workflow

`<type>` is the skill argument: `changes` for `gpex-review-changes` findings, or `specs` for
`gpex-review-specs` findings. If it is anything else or missing, ask the user.
For `specs`, the specs path (see `gpex-review-specs`) is the file named in the link text of the
finding locations, prefixed with `./`.
For `changes`, the specifications are the body of the GitHub issue matching the branch number
prefix, if any.

1. If `./tmp/review-<type>-findings-accepted.md` does not exist or has no finding, stop and inform
   the user.
2. If the review cycle can stop (see cycle rule of `gpex-review-changes`), inform the user that
   this is the last review cycle.
3. For each finding in `./tmp/review-<type>-findings-accepted.md`:
    1. Read the finding location and the related code.
    2. If the finding is a `QUESTION`, ask the user, with the most likely answer as recommended
       option, then replace the entry with the resulting finding (following the finding format and
       classes of `gpex-review-changes`, or the classes of `gpex-review-specs` for `specs`) and
       validate it as below. If no finding results, move it to the refused file (see below) with
       `Answered: <answer>` as refusal reason.
    3. If it is a `NITPICK`, leave it to top-level step 4. Else, if the finding modifies neither a
       settled section (see `gpex-specify`) nor an earlier decision:
        - Keep it without asking when its fix is objectively recommended, has no relevant
          alternative and changes neither behavior nor design. This includes:
            - a template, format or written-rule violation with a single possible fix.
            - a duplication removed without changing meaning.
            - a typo, line wrap, alignment or formatting fix.
            - a wording made more precise when the rest of the reviewed content already implies
              it.
        - Keep it without asking when it is a `BLOCKER` whose fix is necessary, obvious and has no
          relevant alternative.
        - Keep it without asking when its fix is the only way to apply a settled section entry as
          written, even if it changes behavior or design.

       In any other case, display its details and ask user which solution to apply: the finding
       solution, viable alternatives, refusing the finding, or a solution they write themselves.
        1. If the finding is refused, ask for the reason required by its class (see the finding
           classes of `gpex-review-changes`) unless the user already gave it, then move it to
           `./tmp/review-<type>-findings-refused.md` as a refused entry (see the finding format of
           `gpex-review-changes`). When creating this file, start it with the following line,
           followed by an empty line:
            - `changes`: `Branch: <branch label>` (see `gpex-review-changes`).
            - `specs`: `Specs: <specs path>` (see `gpex-review-specs`).
        2. If the finding is accepted, replace its `Fix:` and snippet in
           `./tmp/review-<type>-findings-accepted.md` with the selected solution.
    4. For `specs`, when the user's refusal, answer or selected solution settles a point a
       reviewer could raise again, ask for its reason if not given, then record it in the specs
       path file, in the matching settled section (see `gpex-specify`). If the specs path is a
       step file, record it in `./tmp/specs.md` and in every other `./tmp/specs-step*.md` it is
       relevant to.
4. Ask which `NITPICK` findings left by step 3 to keep, in one go with multi-select questions
   (split if needed). Use one option per finding: number, title and fix summary. Recommend keeping
   all, or none if the review cycle can now stop (see step 2). Move unselected ones to the refused
   file (see step 3) with refusal reason `Not selected by user`.
5. List the findings kept without asking, with the reason or category applied, so the user can
   still review them.
6. Display the recommended command to apply the findings for `<type>` (see
   `doc/development-workflow.md`), then stop and don't apply any fix.

## Rules

- Always provide enough information, consider the user has not read the code nor the findings.
- Each question includes a recommended option.
- Merge questions when possible:
    - Ask closely related findings (same root cause, location or decision) as one question
      applying one solution to all of them.
    - Group findings of the same topic (same area or kind of fix) whose choice is keep or refuse
      into one multi-select question, one option per finding, then ask the reasons of all
      unselected ones in a single question. Never group unrelated findings.
    - Never merge when it hides a finding's details, alternatives or recommended option.
    - Top-level step 4 is exempt from these merge rules.
- Take earlier decisions into account, and flag findings they make obsolete or conflicting.
- Other skills are only referenced for their definitions: never run them.
