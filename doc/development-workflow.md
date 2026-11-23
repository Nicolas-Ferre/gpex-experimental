# Development workflow

This file describes the workflow performed by a human developer for each new change to perform.

## Specification

This part is done for all changes (functional or purely technical), except simple changes
implemented directly (see "Implementation").

1. Run `gpex-specify` AI skill (generally `/gpex-specify <change description>`).
    - Developer is interviewed by AI to obtain full functional and technical specifications.
    - If the change needs several PRs, the specifications end with a `## Steps` section.
2. Create a single GitHub issue with the full specifications in description, and update it after
   each specifications change (including entries recorded by `/gpex-confirm-findings specs`).
3. Add a warning on top of the description as long as specifications are not fully reviewed:
   ```markdown
   > [!WARNING]
   > TO REVIEW
   ```
4. Carefully review and refine specifications (`gpex-specify` skill can be reused if needed).
5. Run `/gpex-review-specs ./tmp/specs.md` and refine specifications until the skill reports that
   the review cycle can stop.
    - Validate accepted findings with `/gpex-confirm-findings specs`.
    - Apply them with `/gpex-specify apply accepted spec findings`.
6. Create new GitHub issues from "Out of scope" section when relevant.

## Implementation

Each step is implemented in its own PR, referencing the issue without closing it. The developer
closes the issue after the last step.

When the issue to implement or review has a `## Steps` section, `gpex-implement` and
`gpex-review-changes` ask which steps the branch covers, unless given as argument or in the
conversation.

For the moment, the implementation workflow is flexible, but always uses the following skills:

- `gpex-implement` skill. Invocation depends on the context:
    - To implement the issue matching the branch number prefix: `/gpex-implement`
    - To implement another issue: `/gpex-implement <issue URL>`
    - To implement from specification file: `/gpex-implement specifications from <file path>`
    - To implement simple change: `/gpex-implement <description of the changes>`
    - To implement given steps of the issue: `/gpex-implement step 2`
- `gpex-simplify` skill that doesn't require any argument. To be ideally run after
  implementation, and once the review cycle can stop.
- `gpex-review-changes` skill (e.g. `/gpex-review-changes step 2` to give covered steps). To be
  run after any change, until the skill reports that the review cycle can stop.
    - Validate accepted findings with `/gpex-confirm-findings changes`.
    - Apply findings by invoking
      `/gpex-implement findings in ./tmp/review-changes-findings-accepted.md`

In addition to generated code, several files must be manually reviewed after each skill run to
improve development workflow, skills and quality checks:

| File                                           | Description                                                                | Producer skill                                                  |
|------------------------------------------------|----------------------------------------------------------------------------|-----------------------------------------------------------------|
| `./tmp/implement-decisions.md`                 | Decisions taken by AI during an implementation                             | `gpex-implement` and  `gpex-simplify`                           |
| `./tmp/review-specs-findings-refused.md`       | Any specification review finding rejected for the specs file in its header | `gpex-review-specs` and `gpex-confirm-findings`                 |
| `./tmp/review-specs-findings-accepted.md`      | Any specification review finding accepted by the AI                        | `gpex-review-specs` and `gpex-confirm-findings`                 |
| `./tmp/review-changes-improvements-delayed.md` | Any deterministic check that could be enforced in the CI workflow          | `gpex-review-changes`                                           |
| `./tmp/review-changes-findings-refused.md`     | Any review finding rejected on the branch named in its header              | `gpex-review-changes` and `gpex-confirm-findings`               |
| `./tmp/review-changes-findings-accepted.md`    | Any review finding accepted by the AI                                      | `gpex-review-changes` and `gpex-confirm-findings`               |
