# Repository guidelines

The project contains a compiler and runner CLI for GPEx, a custom programming language that runs
entirely on the GPU side.

## Repository structure

Here are relevant folders and files:

```text
.
├── README.md
├── .editorconfig
├── .github/workflows/ci.yml     # CI workflow run for each PR
├── doc/
│   └── specs/                   # Specifications for the different parts of the repository
└── scripts/
    └── lint.sh                  # Run all linters
```

## Branching conventions

- Number prefix at the start of a branch corresponds to the GitHub issue ID
- Branch can either correspond to the whole issue or to a sub-part of the issue

## Misc

- Some of the `gh` commands are interactive, so use `--json` when possible.
- Run `scripts/lint.sh` before finishing any change task.
