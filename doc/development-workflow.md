# Development workflow

This file describes the workflow performed by a human developer for each new change to perform.

## Functional specification

This part is done when a change is not purely technical.

1. Run `gpex-specify` AI skill (generally `/gpex-specify <change description>`).
    - Developer is interviewed by AI to obtain full functional specifications.
2. If the task is complex and should be split into multiple issues:
    - Run `gpex-split` AI skill (generally `/gpex-split`).
        - A specification file is obtained for each agreed step.
3. For each step (or the full specifications if not split):
    - Create a GitHub issue with functional specifications in description
    - Add a warning on top of the description as long as specifications are not fully reviewed:
      ```markdown
      > [!WARNING]
      > TO REVIEW
      ```
    - Carefully review and refine specifications (`gpex-specify` skill can be reused if needed).
    - Create new GitHub issues from "Out of scope" section when relevant.

## Implementation

1. Run `gpex-implement` AI skill. Depending on the context:
    - To implement an issue: `/gpex-implement <issue URL>`
    - To implement from specification file: `/gpex-implement specifications from <file path>`
    - To implement simple change: `/gpex-implement <description of the changes>`
2. Review `./tmp/implement-decisions.md` to ensure taken decisions are correct.
    - If not, go back to step 1 and provide modifications to perform.
3. Review `./tmp/review-improvements-delayed.md` and create necessary GitHub issues.
4. Review `./tmp/review-findings-refused.md` to ensure all refusals make sense.
5. If `./tmp/review-findings-accepted.md` is not empty:
    - Review the findings.
    - If the findings are relevant:
        - Go back to step 1 (`/gpex-implement findings in ./tmp/review-findings-accepted.md`)
    - If the findings are not relevant:
        - Improve skills or documentation to avoid these findings in the future.
