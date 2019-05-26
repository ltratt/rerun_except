# rerun_except

`rerun_except` allows you to easily specify which files should *not* trigger a
`cargo` rebuild, which can significantly cut down on unnecessary builds. In
essence, this library inverts the normal way that you tell `cargo` about
dependencies: `cargo` requires you to tell it which files *should* be tracked;
`rerun_except` requires you to tell it which files should be ignored. The
latter is safer, because if you add files to your project later they will
automatically trigger a rebuild until and unless you explicitly inform
`rerun_except` that they should be ignored.

`rerun_except` uses the [`ignore`](https://crates.io/crates/ignore) library to
specify which files to ignore. You thus need to specify one or more globs in
`gitignore` format which specifies which files to ignore: all other files
(except those in ignore files such as `.gitignore`) will be tracked.

For example if you have the following file layout:

```text
proj/
  .gitignore
  Cargo.toml
  src/
    lib.rs
  lang_tests/
    run.rs
    test1.lang
    test2.lang
  target/
   ...
```

and you do not want the two `.lang` files to trigger a rebuild then you would
tell `rerun_except` to ignore `lang_tests/*.lang`. Assuming that the
`.gitignore` file ignores the `target/` directory, then `rerun_except` will
also ignore the `target` directory. Note that adding a new file such as
`lang_tests/test3.lang` will not trigger a rebuild (since it is covered by the
ignore glob `lang_tests/*.lang`), but adding a new file such as `build.rs` will
trigger a rebuild (since it is not covered by an ignore glob).

To use `rerun_except` in this manner you simply need to call
`rerun_except::rerun_except` with an array of ignore globs in [`gitignore`
format](https://git-scm.com/docs/gitignore) as part of your `build.rs` file:

```rust,ignore
use rerun_except::rerun_except;

fn main() {
    rerun_except(&["lang_tests/*.lang"]).unwrap();
}
```

This will automatically communicate the necessary dependencies to `cargo`.
