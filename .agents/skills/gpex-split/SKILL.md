---
name: gpex-split
description: Split specifications in more granular steps.
---

## Task

Split a specification file in more granular PR steps.

## Workflow

1. Read `./tmp/specs.md` file.
2. Propose the most possible granular steps respecting next section.
    1. If user agrees with steps, then generate `./tmp/specs-step<step number>.md` files which
       specializes function specifications to each step.
    2. If user disagrees with steps, then rework with them until they agree.
3. Stop and don't implement anything.

## Steps

A step corresponds to a PR that should be as granular as possible, but adding value to final user
and testable end to end.

It is common to create an initial PR for the minimal scope and then other
PRs to incrementally reach the full scope of the change, but a list of independent PRs is also
possible if more relevant.
