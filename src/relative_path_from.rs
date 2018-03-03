use helpers::is_same_path;
use path_parsing::SEP_STR;
extern crate array_tool;
use self::array_tool::vec::{Shift, Join};
use pathname;
use ruru;
use chop_basename;
use pathname::Pathname;
use ruru::{Exception as Exc, AnyException as Exception};

type MaybeString = Result<ruru::RString, ruru::result::Error>;

pub fn relative_path_from(itself: MaybeString, base_directory: MaybeString) -> Result<Pathname, Exception> {
  let dest_directory = pathname::pn_cleanpath_aggressive(itself).to_string();
  let base_directory = pathname::pn_cleanpath_aggressive(base_directory).to_string();

  let mut dest_prefix = dest_directory.clone();
  let mut dest_names: Vec<String> = vec![];
  loop {
    match chop_basename::chop_basename(&dest_prefix.clone()) {
      Some((ref dest, ref basename)) => {
        dest_prefix = dest.to_string();
        if basename != &"." {
          dest_names.unshift(basename.to_string())
        }
      },
      None => break,
    }
  }

  let mut base_prefix = base_directory.clone();
  let mut base_names: Vec<String> = vec![];
  loop {
    match chop_basename::chop_basename(&base_prefix.clone()) {
      Some((ref base, ref basename)) => {
        base_prefix = base.to_string();
        if basename != &"." {
          base_names.unshift(basename.to_string())
        }
      },
      None => break,
    }
  }

  if !is_same_path(&dest_prefix, &base_prefix) {
    return Err(
      Exception::new(
        "ArgumentError",
        Some(&format!("different prefix: {} and {}", dest_prefix, base_prefix)[..])
      )
    );
  }

  let (dest_names, base_names): (Vec<String>, Vec<String>) = dest_names.
    iter().
    zip(base_names.iter()).
    skip_while(|&(a, b)| is_same_path(a,b) ).
    fold((vec![], vec![]), |mut acc, (a, b)| {
      acc.0.push(a.to_string());
      acc.1.push(b.to_string());
      acc
    });

  if base_names.contains(&"..".to_string()) {
    return Err(
      Exception::new(
        "ArgumentError",
        Some(&format!("base_directory has ..: {}", base_directory)[..])
      )
    );
  }

  let base_names: Vec<String> = base_names.into_iter().map(|_| "..".to_string()).collect();

  if base_names.is_empty() && dest_names.is_empty() {
    Ok(Pathname::new("."))
  } else {
    Ok(Pathname::new(&base_names.iter().chain(dest_names.iter()).collect::<Vec<&String>>().join(&SEP_STR)))
  }
}

// #[allow(dead_code)]
// fn rpf(a: &str, b: &str) -> String {
//   relative_path_from(a.to_string(), b.to_string()).
//     unwrap_or(Pathname::new("WRONG_ANSWER")).
//     send("to_s", None).
//     try_convert_to::<RString>().
//     unwrap().
//     to_string()
// }
// 
// #[allow(dead_code)]
// fn rpfe(a: &str, b: &str) -> Result<Pathname, Exception> {
//   relative_path_from(a.to_string(), b.to_string())
// }
// 
// #[test]
// fn it_checks_relative_path() {
//   assert_eq!(rpf("a",          "b"),        "../a"         );
//   assert_eq!(rpf("a",          "b/"),       "../a"         );
//   assert_eq!(rpf("a/",         "b"),        "../a"         );
//   assert_eq!(rpf("a/",         "b/"),       "../a"         );
//   assert_eq!(rpf("/a",         "/b"),       "../a"         );
//   assert_eq!(rpf("/a",         "/b/"),      "../a"         );
//   assert_eq!(rpf("/a/",        "/b"),       "../a"         );
//   assert_eq!(rpf("/a/",        "/b/"),      "../a"         );
//   assert_eq!(rpf("a/b",        "a/c"),      "../b"         );
//   assert_eq!(rpf("../a",       "../b"),     "../a"         );
//   assert_eq!(rpf("a",          "."),        "a"            );
//   assert_eq!(rpf(".",          "a"),        ".."           );
//   assert_eq!(rpf(".",          "."),        "."            );
//   assert_eq!(rpf("..",         ".."),       "."            );
//   assert_eq!(rpf("..",         "."),        ".."           );
//   assert_eq!(rpf("/a/b/c/d",   "/a/b"),     "c/d"          );
//   assert_eq!(rpf("/a/b",       "/a/b/c/d"), "../.."        );
//   assert_eq!(rpf("/e",         "/a/b/c/d"), "../../../../e");
//   assert_eq!(rpf("a/b/c",      "a/d"),      "../b/c"       );
//   assert_eq!(rpf("/../a",      "/b"),       "../a"         );
//   assert_eq!(rpf("../a",       "b"),        "../../a"      );
//   assert_eq!(rpf("/a/../../b", "/b"),       "."            );
//   assert_eq!(rpf("a/..",       "a"),        ".."           );
//   assert_eq!(rpf("a/../b",     "b"),        "."            );
//   assert_eq!(rpf("a",          "b/.."),     "a"            );
//   assert_eq!(rpf("b/c",        "b/.."),     "b/c"          );
// }
// 
// #[test]
// fn relative_path_can_raise_exceptions() {
//   assert!(rpfe("/", ".").is_err());
//   assert!(rpfe(".", "/").is_err());
//   assert!(rpfe("a", "..").is_err());
//   assert!(rpfe(".", "..").is_err());
// }
