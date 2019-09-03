// Copyright (c) 2019 King's College London created by the Software Development Team
// <http://soft-dev.org/>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, or the UPL-1.0 license <http://opensource.org/licenses/UPL>
// at your option. This file may not be copied, modified, or distributed except according to those
// terms.

//! `rerun_except` allows you to specify which files should *not* trigger a `cargo` rebuild,
//! which can significantly cut down on unnecessary rebuilds. In essence, this library inverts the
//! normal way that you tell `cargo` about dependencies: `cargo` requires you to tell it which
//! files *should* be tracked; `rerun_except` requires you to tell it which files should be
//! ignored. The latter is safer, because if you add files to your project later they will
//! automatically trigger a rebuild until, and unless, you explicitly inform `rerun_except` that
//! they should be ignored.
//!
//! `rerun_except` uses the [`ignore`](https://crates.io/crates/ignore) library to specify which
//! files to ignore. You thus need to specify one or more globs in `gitignore` format which
//! specifies which files to ignore: all other files (except those in ignore files such as
//! `.gitignore`) will be tracked.
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
//! `rerun_except` to ignore `lang_tests/*.lang`. Assuming that the `.gitignore` file ignores the
//! `target/` directory, then `rerun_except` will also ignore the `target` directory. Note that
//! adding a new file such as `lang_tests/test3.lang` will not trigger a rebuild (since it is
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
//!
//! This will automatically communicate the necessary dependencies to `cargo`.

use std::env;
use std::error::Error;

use ignore::{overrides::OverrideBuilder, WalkBuilder};

/// Specify which files should not cause `cargo` to rebuild a project. `globs` is an array of
/// ignore globs. Each entry must be in [`gitignore` format](https://git-scm.com/docs/gitignore)
/// with the minor exception that entries must not begin with a `!`.
pub fn rerun_except(globs: &[&str]) -> Result<(), Box<Error>> {
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

fn check_globs(globs: &[&str]) -> Result<(), Box<Error>> {
    for g in globs {
        if g.starts_with('!') {
            return Err(Box::<Error>::from("Glob '%s' starts with a '!'"));
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
