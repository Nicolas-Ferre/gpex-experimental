# CI specifications

This file contains all specifications related to the CI workflows that cannot be deduced from their
definition.

CI workflows are the following:

- `.github/workflows/ci-main.yml`
- `.github/workflows/ci-pr-size.yml`

## Decisions

| Description                                                                       | Reason                                                                              |
|-----------------------------------------------------------------------------------|-------------------------------------------------------------------------------------|
| Pin every action to a full commit SHA, with the version tag in a trailing comment | Safer in case of retagging, and Dependabot version updates still follow the comment |
| Actions must not modify the PR, so they shouldn't request `write` permission      | `read` permission is always safer                                                   |
