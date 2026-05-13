# Scripts

No executable scripts exist yet.

This directory is reserved for future harness automation. Do not add fake
validation commands. Add scripts only when a story creates a real implementation
surface that can be checked.

## Future Command Contract

Expected future checks:

```text
validate:quick
  format, lint, typecheck, unit tests, architecture check

test:integration
  backend contract and integration checks

test:e2e
  user-visible end-to-end flows

test:platform
  platform shell smoke checks, if the project has a native shell

test:release
  full suite, log checks, and performance smoke
```
