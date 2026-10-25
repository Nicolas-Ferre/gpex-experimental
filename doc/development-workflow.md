# Development workflow

This file describes the workflow performed by a human developer for each new change to perform.

## Specification

This part is done for all changes (functional or purely technical), except simple changes
implemented directly (see "Implementation").

1. Run `gpex-specify` AI skill (generally `/gpex-specify <change description>`).
    - Developer is interviewed by AI to obtain full functional and technical specifications.
2. If the task is complex and should be split into multiple issues:
    - Run `gpex-split` AI skill (generally `/gpex-split`).
        - A specification file is obtained for each agreed step.
3. For each step (or the full specifications if not split):
    - Create a GitHub issue with specifications in description
    - Add a warning on top of the description as long as specifications are not fully reviewed:
      ```markdown
      > [!WARNING]
      > TO REVIEW
      ```
    - Carefully review and refine specifications (`gpex-specify` skill can be reused if needed).
    - Run `/gpex-review-specs <step specs file>` and refine specifications until
      `./tmp/review-specs-findings-accepted.md` has no relevant finding left.
    - Create new GitHub issues from "Out of scope" section when relevant.

## Implementation

For the moment, the implementation workflow is flexible, but always uses the following skills:

- `gpex-implement` skill. Invocation depends on the context:
    - To implement an issue: `/gpex-implement <issue URL>`
    - To implement from specification file: `/gpex-implement specifications from <file path>`
    - To implement simple change: `/gpex-implement <description of the changes>`
- `gpex-simplify` skill that doesn't require any argument. To be ideally run after
  implementation, and after having fixed all review findings.
- `gpex-review-changes` skill that doesn't require any argument. To be run after any change.
    - Apply findings by invoking
      `/gpex-implement findings in ./tmp/review-changes-findings-accepted.md`

In addition to generated code, several files must be manually reviewed after each skill run to
improve development workflow, skills and quality checks:

| File                                           | Description                                                                          | Producer skill                        |
|------------------------------------------------|--------------------------------------------------------------------------------------|---------------------------------------|
| `./tmp/implement-decisions.md`                 | Decisions taken by AI during an implementation                                       | `gpex-implement` and  `gpex-simplify` |
| `./tmp/review-specs-findings-refused.md`       | Any specification review finding rejected by the AI for the specs file in its header | `gpex-review-specs`                   |
| `./tmp/review-specs-findings-accepted.md`      | Any specification review finding accepted by the AI                                  | `gpex-review-specs`                   |
| `./tmp/review-changes-improvements-delayed.md` | Any deterministic check that could be enforced in the CI workflow                    | `gpex-review-changes`                 |
| `./tmp/review-changes-findings-refused.md`     | Any review finding rejected by the AI on the branch named in its header              | `gpex-review-changes`                 |
| `./tmp/review-changes-findings-accepted.md`    | Any review finding accepted by the AI                                                | `gpex-review-changes`                 |
