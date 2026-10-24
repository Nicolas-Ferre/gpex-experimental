---
name: gpex-review-specs
description: Perform a light review of a specifications file.
---

## Preconditions

- Never run the skill if not explicitly referenced by another skill or the user.

## Task

Review the specifications file, which is taken from the skill arguments or conversation, else
`./tmp/specs.md`. Never modify the reviewed file.

## Workflow

Run these steps in order, without sub-agent:

1. If existing, delete `./tmp/review-specs-findings.md` without reading it to avoid bias.
2. If the specifications file does not exist, stop and inform the user.
3. Read the specifications and explore the codebase.
4. Review the specifications against the criteria below.
5. If there is no finding, inform the user that the specifications look good and stop.
6. Write the findings in `./tmp/review-specs-findings.md` the following way:
   ```markdown
   1. **<first finding>**
      <brief details and short snippet(s) of the proposed fix>
   2. **<second finding>**
      <brief details and short snippet(s) of the proposed fix>
      ...
   ```
   Do not mix multiple issues in the same finding.
   Illustrate the findings with snippets when possible to make it easier to understand.
7. If the reviewed file is `./tmp/specs.md`, let the user know that they can refine the
   specifications with `gpex-specify`.

## Review criteria

Apply all the sub-agent definitions of the `gpex-review-changes` skill (don't run this skill) to the
specifications instead of code changes, focused on the content itself (the form is less important),
ignoring code-only checks. Also check that:

- There is no duplicated information or ambiguity.
- `### Testing` covers every case of the functional design and the edge cases.
- `### Architecture` and `### Module interactions` match real files and the existing organization.
- Listed libraries exists and present no risk or better alternative.

You can question certain decisions if it makes sense.
