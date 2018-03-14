use std::iter;
use helpers::{is_same_path, to_str};
use path_parsing::SEP_STR;
use cleanpath_aggressive::cleanpath_aggressive;
use chop_basename::chop_basename;
use pathname::Pathname;
use ruru;
use ruru::{Exception as Exc, AnyException as Exception};

type MaybeString = Result<ruru::RString, ruru::result::Error>;

pub fn relative_path_from(itself: MaybeString, base_directory: MaybeString) -> Result<Pathname, Exception> {
  let dest_directory = cleanpath_aggressive(to_str(&itself));
  let base_directory = cleanpath_aggressive(to_str(&base_directory));

  let (dest_prefix, mut dest_names) = to_names(dest_directory.as_ref());
  let (base_prefix, mut base_names) = to_names(base_directory.as_ref());

  if !is_same_path(&dest_prefix, &base_prefix) {
    return Err(
      Exception::new(
        "ArgumentError",
        Some(&format!("different prefix: {} and {}", dest_prefix, base_prefix)),
      )
    );
  }

  // Remove shared tail
  {
    let num_same = dest_names.iter().rev().zip(base_names.iter().rev()).
        take_while(|&(dest, base)| dest == base).count();
    let num_dest_names = dest_names.len();
    dest_names.truncate(num_dest_names - num_same);
    let num_base_names = base_names.len();
    base_names.truncate(num_base_names - num_same);
  };

  if base_names.contains(&"..") {
    return Err(
      Exception::new(
        "ArgumentError",
        Some(&format!("base_directory has ..: {}", base_directory)),
      )
    );
  }

  if base_names.is_empty() && dest_names.is_empty() {
    Ok(Pathname::new("."))
  } else {
    Ok(Pathname::new(&iter::repeat("..").take(base_names.len()).chain(dest_names.into_iter().rev()).
        collect::<Vec<&str>>().join(&SEP_STR)))
  }
}

#[inline(always)]
fn to_names(path: &str) -> (&str, Vec<&str>) {
  let mut result: Vec<&str> = vec![];
  let mut prefix = path;
  loop {
    match chop_basename(&prefix) {
      Some((ref dest, ref basename)) => {
        prefix = dest;
        if basename != &"." {
          result.push(basename);
        }
      }
      None => return (prefix, result),
    }
  }
}
