# Twist

> Work in progress

## A dotfile manager with a twist

Twist is a dotfile manager for Linux and macOS, inspired by [Dotdrop](https://github.com/deadc0de6/dotdrop).
The goal of Twist is to make dotfiles management easier and more fun.
Twist is written in Rust, so no additional runtime dependencies are required.

Out of the box, Twist will create and manage a Git repository for your dotfiles.

The CLI was modeled after the Git CLI. All options are documented with `--help`.

| Command        | Info                                                                                            |
| -------------- | ----------------------------------------------------------------------------------------------- |
| `twist git`    | Executes a Git command inside of the Twist Git repository. An escape hatch.                     |
| `twist init`   | Initializes a new Twist repository.                                                             |
| `twist add`    | Adds or updates the specified files or directories into the repository and commits the changes. |
| `twist rm`     | Removes the specified files or directories from the repository and commits the changes.         |
| `twist update` | Updates the tracked files and directories into the repository and commits the changes.          |
| `twist apply`  | Restores the tracked files and directories into their original locations.                       |
| `twist push`   | Pushes the changes to the remote repository.                                                    |
| `twist pull`   | Pulls the changes from the remote repository.                                                   |

## Roadmap

### v1

- Implement all basic commands
- Profile support
- Improved testing

### vNext

- Plugin or package support
  - Generate files for dynamic resources (as well as restoring these)
  - Think like tracking installed Homebrew packages
