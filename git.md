# Git
2025-10-01  #learn #git #tools #cli

## what is it
Version control system — tracks file changes, enables branching workflows, and syncs code with remote repositories.

## how it works

### setup & config
```bash
git config --global user.name "Your Name"
git config --global user.email "you@email.com"
git config --global init.defaultBranch main
git config --list
```

### create / clone
```bash
git init        # initialize repo in current directory
git clone URL   # clone remote repo locally
```

### status & history
```bash
git status                        # current state of working tree
git log                           # commit history
git log --oneline --graph --all   # compact visual graph
git diff                          # unstaged changes
git diff --staged                 # staged changes
```

### stage & commit
```bash
git add file.txt       # stage specific file
git add .              # stage all changes
git commit -m "msg"    # commit staged changes
git commit -am "msg"   # stage + commit tracked files (skips untracked)
```

### branching
```bash
git branch                # list branches
git branch new-branch     # create branch
git checkout branch       # switch branch
git checkout -b branch    # create + switch in one step
git switch branch         # modern alternative to checkout
git merge branch          # merge branch into current
```

### remote repos
```bash
git remote -v               # list remotes
git remote add origin URL   # add remote
git fetch                   # download changes (don't merge)
git pull                    # fetch + merge
git push origin main        # push to remote
git push -u origin main     # push + set upstream (first time)
```

### undo / fix mistakes
```bash
git restore file.txt              # discard unstaged changes
git restore --staged file.txt     # unstage a file
git reset --soft HEAD~1           # undo last commit, keep changes staged
git reset --hard HEAD~1           # undo last commit, delete changes
git revert COMMIT_HASH            # safe undo — creates a new reverting commit
```

### remove / rename
```bash
git rm file.txt
git mv old.txt new.txt
```

### stash
```bash
git stash          # save current changes temporarily
git stash list     # list stashes
git stash pop      # apply latest stash + remove it
git stash apply    # apply latest stash, keep it in list
```

### inspect
```bash
git show COMMIT_HASH    # show a specific commit's diff
git blame file.txt      # who last changed each line
git reflog              # full history of HEAD movements — recovery tool
```

## example
```bash
# typical daily workflow
git status
git add .
git commit -m "feat: add search to note list"
git push origin main
```

## gotchas
- `git reset --hard` permanently deletes uncommitted changes — no undo
- `git revert` is the safe undo for public history — creates a new commit instead of rewriting
- `git commit -am` only stages already-tracked files — new untracked files still need `git add`
- `git pull` = fetch + merge — use `git fetch` first if you want to inspect before merging
- `git reflog` can recover commits even after a hard reset — it tracks all HEAD movements

## links
