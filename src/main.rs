extern crate semver;

use semver::{Version,VersionReq};
mod semverio;
mod nvm;

fn best_version (versions: Vec<String>, range: String) -> Option<Version> {
  let r = VersionReq::parse(range.as_slice()).unwrap();

  let mut sorted = versions.iter()
    .map(|v| Version::parse(v.as_slice()).unwrap())
    .filter(|version| r.matches(version))
    .collect::<Vec<Version>>();

  sorted.sort();
  sorted.pop()
}

fn main () {
  let home = nvm::home();
  println!("home is {}", home.display());
  println!("current_version is {}", nvm::current_version().unwrap());

  let stable = semverio::stable().unwrap();
  let unstable = semverio::unstable().unwrap();
  println!("stable: {}, unstable: {}", stable, unstable);

  let versions = semverio::remote_versions().unwrap();
  println!("unsorted: {}", versions);

  println!("best_version: {}", best_version(versions, "~0.10".into_string()));
}
