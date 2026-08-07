# Space in Paths Handling Verification

Testing environment with `PM_SKILLS_DIR` set to a path containing spaces: `/tmp/tmp9ty9zj9d/pm skills spaced dir`

## Sync Script Output
- Exit Code: 0
- Stdout:
```
Tool draft-nda registered.
Using CLI path: /home/baoltb/Desktop/harness/scripts/bin/harness-cli
Scanning PM skills directory: /tmp/tmp9ty9zj9d/pm skills spaced dir
Found 1 plugins: ['pm-toolkit']
Registering tool: /home/baoltb/Desktop/harness/scripts/bin/harness-cli tool register --name draft-nda --command ./scripts/pm-skills-runner draft-nda --description Draft NDA (harness) --responsibility Tool access --force --args parties:string:optional
Sync complete. Registered: 1, Updated: 0, Skipped: 0, Removed: 0
```

## Runner Wrapper Output (for draft-nda)
- Exit Code: 0
- Stdout:
```
Description: Draft NDA

Drafting NDA...
```

**Overall Verdict**: PASSED