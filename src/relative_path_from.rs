use helpers::{is_same_path, to_str};
use path_parsing::SEP_STR;
use cleanpath_aggressive::cleanpath_aggressive;
extern crate array_tool;
use self::array_tool::iter::ZipOpt;
use ruru;
use chop_basename;
use pathname::Pathname;
use ruru::{Exception as Exc, AnyException as Exception};

type MaybeString = Result<ruru::RString, ruru::result::Error>;

pub fn relative_path_from(itself: MaybeString, base_directory: MaybeString) -> Result<Pathname, Exception> {
  let dest_directory = cleanpath_aggressive(to_str(&itself));
  let base_directory = cleanpath_aggressive(to_str(&base_directory));

  let mut dest_prefix = dest_directory.as_ref();
  let mut dest_names: Vec<&str> = vec![];
  loop {
    match chop_basename::chop_basename(&dest_prefix.clone()) {
      Some((ref dest, ref basename)) => {
        dest_prefix = dest;
        if basename != &"." {
          dest_names.push(basename)
        }
      },
      None => break,
    }
  }
  dest_names.reverse();

  let mut base_prefix = base_directory.as_ref();
  let mut base_names: Vec<&str> = vec![];
  loop {
    match chop_basename::chop_basename(&base_prefix) {
      Some((ref base, ref basename)) => {
        base_prefix = base;
        if basename != &"." {
          base_names.push(basename)
        }
      },
      None => break,
    }
  }
  base_names.reverse();

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
    zip_option(base_names.iter()).
    skip_while(|&(a, b)| {
      match (a, b) {
        (Some(x), Some(y)) => is_same_path(x,y),
        _ => false,
      }
    }).
    fold((vec![], vec![]), |mut acc, (a, b)| {
      if let Some(v) = a {
        acc.0.push(v.to_string());
      }
      if let Some(v) = b {
        acc.1.push(v.to_string());
      }
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
    Ok(Pathname::new(&base_names.iter().chain(dest_names.iter()).map(String::as_str).
        collect::<Vec<&str>>().join(&SEP_STR)))
  }
}
