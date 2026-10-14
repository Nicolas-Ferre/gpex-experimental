# Development workflow

This file describes the workflow performed by a human developer for each new change to perform.

## Functional specification

- Run `gpex-specify` AI skill (generally `/gpex-specify <change description>`).
    - Developer is interviewed by AI to obtain full functional specifications.
- If the task is complex and should be split in multiple issues:
    - Run `gpex-split` AI skill (generally `/gpex-split`).
        - A specification file is obtained for each agreed step.
- Create necessary GitHub issues with the functional specifications in description (and add a
  warning at the top of the description if the issue is not reviewed yet).
- For each step:
    - Create a GitHub issue with functional specifications in description
    - Add a warning on top of tge description as long as specifications are not fully reviewed.
    - Carefully review and refine specifications (`gpex-specify` skill can be reused if needed).
    - Create new GitHub issues for good candidates from "Out of scope" section.
