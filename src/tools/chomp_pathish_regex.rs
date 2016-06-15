fn chomp_pathish_regex(string: &str, globish_string: &str) -> String {
  use regex::Regex;
  // Keep first match of Regex pattern
  // \A(?:(.*)(?:\.{1}\w+)|(.*))\z
  let mut reg_builder: String = String::with_capacity(globish_string.len()+30); // need 29 chars + gs
  reg_builder.push_str(r"\A(?:(.*)");
  if globish_string.len() != 0 {
    let mut gs = globish_string.chars();
    loop {
      match gs.next() {
        Some('.') => { reg_builder.push_str(r"(?:\.{1}") },
        Some('*') => { reg_builder.push_str(r"\w+")   },
        Some(x)   => { reg_builder.push(x)            },
        None      => {
          reg_builder.push_str(r")|(.*)");
          break
        },
      }
    }
  }
  reg_builder.push_str(r")\z");

  let re = Regex::new(&reg_builder[..]).unwrap();
  let caps = re.captures(string).unwrap();
  caps.at(1).unwrap_or(caps.at(0).unwrap_or("")).to_string()
}

#[test]
fn it_chomps_strings_correctly(){
  assert_eq!(chomp_pathish_regex("",""), "");
  assert_eq!(chomp_pathish_regex("ruby",""), "ruby");
  assert_eq!(chomp_pathish_regex("ruby.rb",".rb"), "ruby");
  assert_eq!(chomp_pathish_regex("ruby.rb",".*"), "ruby");
  assert_eq!(chomp_pathish_regex(".ruby/ruby.rb",".rb"), ".ruby/ruby");
  assert_eq!(chomp_pathish_regex(".ruby/ruby.rb.swp",".rb"), ".ruby/ruby.rb.swp");
  assert_eq!(chomp_pathish_regex(".ruby/ruby.rb.swp",".swp"), ".ruby/ruby.rb");
  assert_eq!(chomp_pathish_regex(".ruby/ruby.rb.swp",".*"), ".ruby/ruby.rb");
}
