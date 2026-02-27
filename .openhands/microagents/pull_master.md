---
name: pull_master
type: knowledge
version: 1.0.0
agent: CodeActAgent
triggers:
  - pull_master
---

# Pull Master

This microagent handles pulling changes from the master/main branch of a specific repository.

## Instructions

When triggered with `pull_master`, perform a git pull from the repository at https://github.com/just-every/code.

The task is to:
1. Add the remote repository https://github.com/just-every/code if not already added
2. Fetch the latest changes from that repository
3. Pull from the appropriate branch (typically main or master)

## Usage

When the user mentions "pull_master" or asks to "делать git pull с ветки https://github.com/just-every/code", execute:

```bash
# Add remote if needed
git remote add just-every https://github.com/just-every/code.git

# Or if remote already exists
git fetch just-every

# Pull from the main/master branch
git pull just-every main
# or
git pull just-every master
```

## Notes

- This microagent is specifically for pulling from https://github.com/just-every/code
- Handle any merge conflicts that may arise
- Provide feedback to the user about the result of the pull operation
