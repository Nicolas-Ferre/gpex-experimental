# CI specifications

This file contains all specifications related to the CI workflow located at
`.github/workflows/ci.yml`.

## Decisions

| Description                                                                       | Reason                                                                              |
|-----------------------------------------------------------------------------------|-------------------------------------------------------------------------------------|
| Pin every action to a full commit SHA, with the version tag in a trailing comment | Safer in case of retagging, and Dependabot version updates still follow the comment |
