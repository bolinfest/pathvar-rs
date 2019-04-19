# pathvar-rs

An executable that facilitates idempotent modifications to your `PATH` variable.

After you have done `cargo install pathvar` to add `pathvar` to your `PATH`,
you could add a line like the following to your `~/.bashrc`:

```
PATH=$(pathvar add ~/bin)
```

This will append `~/bin` (expanded) to your `$PATH` if it is not already present.
This means that if you do `source ~/.bashrc` later, your `$PATH` will remain the
same. Note that this is not the case if you followed the standard idiom:

```
PATH=$PATH:~/bin
```

as running `source ~/.bashrc` would cause `~/bin` to be added again.
