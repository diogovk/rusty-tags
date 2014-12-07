use std::io::fs;
use std::io::fs::PathExtensions;
use std::io;
use std::os;
use glob::{glob, Paths};

use app_result::{AppResult, app_err};

/// where `rusty-tags` caches its tag files
pub fn rusty_tags_cache_dir() -> AppResult<Path>
{
   let dir = try!(
      rusty_tags_dir().map(|mut d| {
         d.push("cache");
         d
      })
   );

   if ! dir.is_dir() {
      try!(fs::mkdir_recursive(&dir, io::USER_RWX));
   }

   Ok(dir)
}

/// where rusty-tags puts all of its stuff
pub fn rusty_tags_dir() -> AppResult<Path>
{
   let dir = try!(
      homedir().map(|mut d| {
         d.push(".rusty-tags");
         d 
      })
   );

   if ! dir.is_dir() {
      try!(fs::mkdir_recursive(&dir, io::USER_RWX));
   }

   Ok(dir)
}

/// where cargo puts its git checkouts
pub fn cargo_git_src_dir() -> AppResult<Path>
{
   cargo_dir().map(|mut d| {
      d.push("git");
      d.push("checkouts");
      d
   })
}

/// where cargo puts the source code of crates.io
pub fn cargo_crates_io_src_dir() -> AppResult<Path>
{
   let src_dir = try!(
      cargo_dir().map(|mut d| {
         d.push("registry");
         d.push("src");
         d.push("github.com-*");
         d
      })
   );

   let paths = glob_path(&src_dir);
   if paths.count() != 1 {
      return Err(app_err(format!("Expected one matching path for '{}'!", src_dir.display())));
   }

   let mut paths = glob_path(&src_dir);
   Ok(paths.nth(0).unwrap())
}

/// where cargo puts all of its stuff
fn cargo_dir() -> AppResult<Path>
{
   homedir().map(|mut d| { d.push(".cargo"); d })
}

pub fn glob_path(pattern: &Path) -> Paths
{
   glob(pattern.as_str().unwrap())
}

fn homedir() -> AppResult<Path>
{
   os::homedir().ok_or(app_err("Couldn't read home directory!".to_string()))
}
