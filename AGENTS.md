# Repository guidelines

The project contains a compiler and runner CLI for GPEx, a custom programming language that runs
entirely on the GPU side.

## Repository structure

Here are relevant folders and files:

```text
.
├── README.md
├── .editorconfig
├── .github/workflows/           # GitHub Actions workflows
├── doc/                         # Repository documentation
└── scripts/
    └── lint.sh                  # Run all linters
```

## Branching conventions

- Number prefix at the start of a branch corresponds to the GitHub issue ID
- Branch can either correspond to the whole issue or to a sub-part of the issue

## Writing style

Any documentation, comment or answer you write must follow these rules:

- Be as concise as possible.
- Be straight to the point.
- Be readable and easily understandable by a human.
- Be clear for anyone that has not checked repository content and code.
- Do not duplicate information.

## Misc

- Some of the `gh` commands are interactive, so use `--json` when possible.
- Run the following configured checks before finishing any change task:
    - `scripts/lint.sh`
