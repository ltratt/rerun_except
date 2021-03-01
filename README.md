# rerun_except

Specify which files should *not* trigger a `cargo` rebuild.

In normal operation, `cargo` rebuilds a project when any potentially relevant
file changes. One can use the
[`rerun-if-changed`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#change-detection)
instruction to tell `cargo` to only rebuild if certain files are changed.
However, it is easy to forget to add new files when using `rerun-if-changed`,
causing `cargo` not to rebuild a project when it should.

`rerun_except` inverts this logic, causing `cargo` to rebuild a project when a
file changes *unless you explicitly ignored that file*. This is safer than
`rerun-if-changed` because if you forget to explicitly ignore files, then
`cargo` will still rebuild your project.

`rerun_except` uses the [`ignore`](https://crates.io/crates/ignore) library to
specify which files to ignore in `gitignore` format. Note that explicit ignore
files in your project (e.g. `.gitignore`) are implicitly added to the list of
ignored files.

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
tell `rerun_except` to exclude `lang_tests/*.lang`. Assuming, as is common, that your
`.gitignore` file also  the `target/` directory, then `rerun_except` will
also ignore the `target` directory. 

Adding a new file such as `lang_tests/test3.lang` will not trigger a rebuild
(since it is covered by the ignore glob `lang_tests/*.lang`), but adding a new
file such as `build.rs` will trigger a rebuild (since it is not covered by an
ignore glob).

To use `rerun_except` in this manner you simply need to call
`rerun_except::rerun_except` with an array of ignore globs in [`gitignore`
format](https://git-scm.com/docs/gitignore) as part of your `build.rs` file:

```rust,ignore
use rerun_except::rerun_except;

fn main() {
    rerun_except(&["lang_tests/*.lang"]).unwrap();
}
```
