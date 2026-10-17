# CI specifications

This file contains all specifications related to the CI workflow located at
`.github/workflows/ci.yml`.

## Decisions

| Description                                          | Reason                                                    |
|------------------------------------------------------|-----------------------------------------------------------|
| Pin third-party actions using their full version tag | Commit SHAs don't trigger security alerts with Dependabot |
