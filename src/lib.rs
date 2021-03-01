//! Specify which files should *not* trigger a `cargo` rebuild.
//!
//! In normal operation, `cargo` rebuilds a project when any potentially relevant file changes. One
//! can use the
//! [`rerun-if-changed`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#change-detection)
//! instruction to tell `cargo` to only rebuild if certain files are changed. However, it is easy
//! to forget to add new files when using `rerun-if-changed`, causing `cargo` not to rebuild a
//! project when it should.
//!
//! `rerun_except` inverts this logic, causing `cargo` to rebuild a project when a file changes
//! *unless you explicitly ignored that file*. This is safer than `rerun-if-changed` because if you
//! forget to explicitly ignore files, then `cargo` will still rebuild your project.
//!
//! `rerun_except` uses the [`ignore`](https://crates.io/crates/ignore) library to specify which
//! files to ignore in `gitignore` format. Note that explicit ignore files in your project (e.g.
//! `.gitignore`) are implicitly added to the list of ignored files.
//!
//! For example if you have the following file layout:
//!
//! ```text
//! proj/
//!   .gitignore
//!   Cargo.toml
//!   src/
//!     lib.rs
//!   lang_tests/
//!     run.rs
//!     test1.lang
//!     test2.lang
//!   target/
//!     ...
//! ```
//!
//! and you do not want the two `.lang` files to trigger a rebuild then you would tell
//! `rerun_except` to exclude `lang_tests/*.lang`. Assuming, as is common, that your `.gitignore`
//! file also  the `target/` directory, then `rerun_except` will also ignore the `target`
//! directory.
//!
//! Adding a new file such as `lang_tests/test3.lang` will not trigger a rebuild (since it is
//! covered by the ignore glob `lang_tests/*.lang`), but adding a new file such as `build.rs` will
//! trigger a rebuild (since it is not covered by an ignore glob).
//!
//! To use `rerun_except` in this manner you simply need to call `rerun_except::rerun_except` with
//! an array of ignore globs in [`gitignore` format](https://git-scm.com/docs/gitignore) as part of
//! your `build.rs` file:
//!
//! ```rust,ignore
//! use rerun_except::rerun_except;
//!
//! fn main() {
//!     rerun_except(&["lang_tests/*.lang"]).unwrap();
//! }
//! ```

#![allow(clippy::needless_doctest_main)]

use std::env;
use std::error::Error;

use ignore::{overrides::OverrideBuilder, WalkBuilder};

/// Specify which files should not cause `cargo` to rebuild a project. `globs` is an array of
/// ignore globs. Each entry must be in [`gitignore` format](https://git-scm.com/docs/gitignore)
/// with the minor exception that entries must not begin with a `!`.
pub fn rerun_except(globs: &[&str]) -> Result<(), Box<dyn Error>> {
    check_globs(globs)?;

    let mdir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut overb = OverrideBuilder::new(&mdir);
    for g in globs {
        overb.add(&format!("!{}", g))?;
    }
    for e in WalkBuilder::new(&mdir)
        .overrides(overb.build()?)
        .build()
        .filter(|x| x.is_ok())
    {
        let e_uw = e?;
        let path = e_uw.path();
        if path.is_dir() {
            continue;
        }
        if let Some(path_str) = path.to_str() {
            if path_str == mdir {
                continue;
            }
            println!("cargo:rerun-if-changed={}", path_str);
        }
    }

    Ok(())
}

fn check_globs(globs: &[&str]) -> Result<(), Box<dyn Error>> {
    for g in globs {
        if g.starts_with('!') {
            return Err(Box::<dyn Error>::from("Glob '%s' starts with a '!'"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_globs() {
        assert!(check_globs(&["a"]).is_ok());
        assert!(check_globs(&["^a"]).is_ok());
        assert!(check_globs(&["!a"]).is_err());
    }
}
