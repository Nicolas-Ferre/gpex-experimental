---
name: gpex-review-specs
description: Perform a light review of a specifications file.
---

## Preconditions

- Never run the skill if not explicitly referenced by another skill or the user.

## Task

Review the specifications in `./tmp/specs.md`, or in the file stated by the user. Never modify the
reviewed file.

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
7. Let the user know that they can refine the specifications with `gpex-specify`.

## Review criteria

Apply the sub-agent definitions of the `gpex-review-changes` skill (don't run this skill) to the
specifications instead of code changes, ignoring code-only checks. Also check that:

- `### Testing` covers every case of the functional design and the edge cases
- the template defined in the `gpex-specify` skill is followed
- there is no duplicated information or ambiguity
- `### Architecture` and `### Module interactions` match real files and the existing organization
- listed libraries exist on their package registry with the stated version

You can question certain decisions if it makes sense.
