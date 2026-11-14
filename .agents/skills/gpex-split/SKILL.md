---
name: gpex-split
description: Split specifications into more granular steps.
---

## Task

Split a specification file into more granular PR steps.

## Workflow

1. If any `./tmp/specs-step*.md` file exists, ask if they can be deleted.
    1. If yes, delete the files.
    2. If no, stop.
2. Read `./tmp/specs.md` file.
    1. If it doesn't exist, stop and inform the user.
3. Propose steps as described below.
    1. If user agrees with steps, then generate `./tmp/specs-step<step number starting from "1">.md`
       files, each adapting the specifications to its step. No requirement should be lost across the
       set of step files. Each step file keeps the settled section entries (see `gpex-specify`)
       relevant to it, and adds an `## Out of scope` row with reason `Covered by step <N>` for
       each point left to another step.
    2. If user disagrees with steps, then rework with them until they agree.
4. Stop and don't implement anything.

## Steps

A step corresponds to a PR that should be as granular as possible, but also add value (to the
final user, or to the codebase for purely technical changes) and be testable end to end.

It is common to create an initial PR for the minimal scope and then other
PRs to incrementally reach the full scope of the change, but a list of independent PRs is also
possible if more relevant.
