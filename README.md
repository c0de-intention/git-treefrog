# git-treefrog

[![CI](https://github.com//git-treefrog/workflows/CI/badge.svg)](https://github.com//git-treefrog/actions)

![banner](./banner.png)

Navigate your git worktrees with ease with this Rust tui.


## Motivation

I want to use the Zed editor, but still have full control over my experience of switching and managing my git worktrees, 
so I created this tui and I can use it as a task and keymap in the Zed editor to hop between trees.


## Roadmap

- [x] list and switch worktrees
- [ ] create worktrees based on config
- [ ] create worktrees based on target branch
- [ ] create worktrees with custom name
- [ ] sort worktrees by most recently used

## Installation

```sh
git clone https://github.com/c0de-intention/git-treefrog.git
cd git-treefrog
cargo install --path .
```

## Usage

```sh
git-treefrog
```

or

```sh
gt
```

## Keymaps

### Worktree selector

- `Up` and `Down` or `j` and `k` arrows to select worktree in the repo.

- `Enter` to open it in your favourite editor, it will respect `$EDITOR`.
