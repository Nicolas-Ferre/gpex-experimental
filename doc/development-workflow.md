# Development workflow

This file describes the workflow performed by a human developer for each new change to perform.

## Functional specification

This part is done when a change is not purely technical.

- Run `gpex-specify` AI skill (generally `/gpex-specify <change description>`).
    - Developer is interviewed by AI to obtain full functional specifications.
- If the task is complex and should be split into multiple issues:
    - Run `gpex-split` AI skill (generally `/gpex-split`).
        - A specification file is obtained for each agreed step.
- For each step (or the full specifications if not split):
    - Create a GitHub issue with functional specifications in description
    - Add a warning on top of the description as long as specifications are not fully reviewed:
      ```markdown
      > [!WARNING]
      > TO REVIEW
      ```
    - Carefully review and refine specifications (`gpex-specify` skill can be reused if needed).
    - Create new GitHub issues from "Out of scope" section when relevant.
