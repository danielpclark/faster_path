use helpers::is_same_path;
use path_parsing::SEP_STR;
extern crate array_tool;
use self::array_tool::vec::{Shift, Join};
use self::array_tool::iter::ZipOpt;
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
    Ok(Pathname::new(&base_names.iter().chain(dest_names.iter()).collect::<Vec<&String>>().join(&SEP_STR)))
  }
}
