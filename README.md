# pathvar-rs

An executable that facilitates idempotent modifications to your `PATH` variable.

## Motivation

Note that the idiomatic way to add folders to your `PATH` is to add lines like
the following to your `~/.bashrc`:

```
PATH=~/bin:$PATH
PATH=$PATH:~/bin
```

This means that if you run `source ~/.bashrc`, `~/bin` will be added to the
`PATH` again, which is not ideal. Note this also effectively happens
when you run `tmux new window`, so successive tmux windows have longer and
longer `PATH` variables under the standard idiom.

## Installation

Run `cargo install pathvar`, which will install `pathvar` on your machine
(most likely at `~/.cargo/bin/pathvar`).

## Command: insert

This will prepend `~/bin` (expanded) to the `$PATH` if it is not already present:

```
PATH=$(pathvar insert ~/bin)
```

## Command: add

This will append `~/bin` (expanded) to the `$PATH` if it is not already present:

```
PATH=$(pathvar add ~/bin)
```
