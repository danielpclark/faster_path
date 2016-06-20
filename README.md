# FasterPath
[![Gem Version](https://badge.fury.io/rb/faster_path.svg)](https://badge.fury.io/rb/faster_path)
[![Build Status](https://travis-ci.org/danielpclark/faster_path.svg?branch=master)](https://travis-ci.org/danielpclark/faster_path)
[![Tweet This](https://raw.githubusercontent.com/danielpclark/faster_path/master/assets/tweet.png)](https://twitter.com/share?url=https%3A%2F%2Fgithub.com%2Fdanielpclark%2Ffaster_path&via=6ftdan&hashtags=Ruby&text=You%20could%20save%2015%25%20or%20more%20on%20website%20page%20load%20time%20by%20switching%20to%20the%20FasterPath%20gem.)

#### This gem shaves off more than 30% of my Rails application page load time.

The primary **GOAL** of this project is to improve performance in the most heavily used areas of Ruby as
path relation and file lookup is currently a huge bottleneck in performance.  As this is the case the
path performance updates will likely not be limited to just changing the Pathname class but also will
be offering changes in related methods and classes.

Users will have the option to write their apps directly for this library, or they can choose to either
refine or monkeypatch the existing standard library.  Refinements are narrowed to scope and monkeypatching will
be a sledge hammer ;-)

## Why

I read a blog post about the new Sprockets 3.0 series being faster than the 2.0 series so I tried it out.  It was not faster but rather it made my website take 31.8% longer to load.  So I reverted back to the 2.0 series and I did a check on Rails on what methods were being called the most and where the application spends
most of its time.  It turns out roughly 80% _(as far as I can tell)_ of the time spent and calls made
are file Path handling.  This is shocking, but it only gets worse when handling assets.  **That is
why we need to deal with these load heavy methods in the most efficient manner!**

Here's a snippet of a Rails stack profile with some of the most used and time expensive methods.

```
Booting: development
Endpoint: "/"
       user     system      total        real
100 requests 26.830000   1.780000  28.610000 ( 28.866952)
Running `stackprof tmp/2016-06-09T00:42:10-04:00-stackprof-cpu-myapp.dump`. Execute `stackprof --help` for more info
==================================
  Mode: cpu(1000)
  Samples: 7184 (0.03% miss rate)
  GC: 1013 (14.10%)
==================================
     TOTAL    (pct)     SAMPLES    (pct)     FRAME
      1894  (26.4%)        1894  (26.4%)     Pathname#chop_basename
      1466  (20.4%)         305   (4.2%)     Pathname#plus
      1628  (22.7%)         162   (2.3%)     Pathname#+
       234   (3.3%)         117   (1.6%)     ActionView::PathResolver#find_template_paths
      2454  (34.2%)          62   (0.9%)     Pathname#join
        57   (0.8%)          52   (0.7%)     ActiveSupport::FileUpdateChecker#watched
       760  (10.6%)          47   (0.7%)     Pathname#relative?
       131   (1.8%)          25   (0.3%)     ActiveSupport::FileUpdateChecker#max_mtime
        88   (1.2%)          21   (0.3%)     Sprockets::Asset#dependency_fresh?
        18   (0.3%)          18   (0.3%)     ActionView::Helpers::AssetUrlHelper#compute_asset_extname
       108   (1.5%)          14   (0.2%)     ActionView::Helpers::AssetUrlHelper#asset_path
```

Here are some addtional stats.  From Rails loading to my home page, these methods are called _(not directly, Rails & gems call them)_ this many times.  And the home page has minimal content.
```ruby
Pathname#to_s called 29172 times.
Pathname#<=> called 24963 times.
Pathname#chop_basename called 24456 times
Pathname#initialize called 23103 times.
File#initialize called 23102 times.
Pathname#absolute? called 4840 times.
Pathname#+ called 4606 times.
Pathname#plus called 4606 times.
Pathname#join called 4600 times.
Pathname#extname called 4291 times.
Pathname#hash called 4207 times.
Pathname#to_path called 2706 times.
Pathname#directory? called 2396 times.
Pathname#entries called 966 times.
Dir#each called 966 times.
Pathname#basename called 424 times.
Pathname#prepend_prefix called 392 times.
Pathname#cleanpath called 392 times.
Pathname#cleanpath_aggressive called 392 times.
Pathname#split called 161 times.
Pathname#open called 153 times.
Pathname#exist? called 152 times.
Pathname#sub called 142 times.
```

After digging further I've found that Pathname is heavily used in Sprockets 2 but in Sprockets 3 they switched to calling Ruby's faster methods from `File#initialize` and `Dir#each`.  It appears they've written all of the path handling on top of these themselves in Ruby.  They achieved some performance gain by switching to rawer code methods, but then they lost more than that in performance by the **many** method calls built on top of that.

If you want to see the best results in Rails with this gem you will likely need to be using the Sprockets 2.0 series.  Otherwise this library would need to rewrite Sprockets itself.

I've said this about Sprockets but this required two other gems to be updated as well.  These are the gems and versions I upgraded and consider group 1 (Sprockets 2) and group 2 (Sprockets 3).  My data is based on method calls rather than source code.

|Sprockets 2 Group|Sprockets 3 Group|
|:---:|:---:|
|sprockets 2.12.4|sprockets 3.6|
|sass 3.2.19|sass 5.0.4|
|bootstrap-sass 3.3.4.1|bootstrap-sass 3.3.6|

## Status

* Rust compilation is working
* Methods are _most likely_ stable
* Testers and developers are most welcome!

## Installation

Ensure Rust is installed:

[Rust Downloads](https://www.rust-lang.org/downloads.html)

```
curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

Add this line to your application's Gemfile:

```ruby
gem 'faster_path', '~> 0.1'
```

And then execute:

    $ bundle

Or install it yourself as:

    $ gem install faster_path
    
**MAC USERS:** At the moment Mac users need to install the extension manualy.  Go to the gem directory and run `cargo build --release` .  There is an issue opened for this and I'm looking for people who have Macs to help on this.

## Usage

Current methods implemented:

|FasterPath Rust Implementation|Ruby 2.3.1 Implementation|Performance Improvement|
|---|---|:---:|
| `FasterPath.absolute?` | `Pathname#absolute?` | 93.9% |
| `FasterPath.basename` | `File.basename` | 75% |
| `FasterPath.chop_basename` | `Pathname#chop_basename` | 50.6% |
| `FasterPath.relative?` | `Pathname#relative?` | 93.2% |
| `FasterPath.blank?` | | |
| `FasterPath.directory?` | `Pathname#directory?` | 25.5% |
| `FasterPath.add_trailing_separator` | `Pathname#add_trailing_separator` | 46.8% |

You may choose to use the methods directly, or scope change to rewrite behavior on the
standard library with the included refinements, or even call a method to monkeypatch 
everything everywhere.

**Note:** `Pathname#chop_basename` in Ruby STDLIB has a bug with blank strings, that is the
only difference in behavior against FasterPath's implementation.

For the scoped **refinements** you will need to

```
require "faster_path/optional/refinements"
using FasterPath::RefinePathname
```

And for the sledgehammer of monkey patching you can do

```
require "faster_path/optional/monkeypatches"
FasterPath.sledgehammer_everything!
```

## Getting Started with Development

The primary methods to target are mostly listed in the **Why** section above.  You may find the Ruby
source code useful for Pathname's [Ruby source](https://github.com/ruby/ruby/blob/32674b167bddc0d737c38f84722986b0f228b44b/ext/pathname/lib/pathname.rb),
[C source](https://github.com/ruby/ruby/blob/32674b167bddc0d737c38f84722986b0f228b44b/ext/pathname/pathname.c),
[tests](https://github.com/ruby/ruby/blob/32674b167bddc0d737c38f84722986b0f228b44b/test/pathname/test_pathname.rb),
and checkout the [documentation](http://ruby-doc.org/stdlib-2.3.1/libdoc/pathname/rdoc/Pathname.html).

Methods will be written as exclusively in Rust as possible.  Even just writing a **not** in Ruby with a
Rust method like `!absolute?` _(not absolute)_ drops 39% of the performance already gained in Rust.
Whenever feasible implement it in Rust.

After checking out the repo, make sure you have Rust installed.  Then run `bundle && rake build_lib` .
Then, run `rake test` to run the tests, and `rake bench` for benchmarks.

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/danielpclark/faster_path.


## License

[MIT License](http://opensource.org/licenses/MIT) or APACHE 2.0 at your pleasure.

