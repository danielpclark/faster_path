# FasterPath
[![Gem Version](https://badge.fury.io/rb/faster_path.svg)](https://badge.fury.io/rb/faster_path)
[![TravisCI Build Status](https://travis-ci.org/danielpclark/faster_path.svg?branch=master)](https://travis-ci.org/danielpclark/faster_path)
[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/10ul0gk3cwhlt2lj/branch/master?svg=true)](https://ci.appveyor.com/project/danielpclark/faster-path/branch/master)
[![Latest Tag](https://img.shields.io/github/tag/danielpclark/faster_path.svg)](https://github.com/danielpclark/faster_path/tags)
[![Commits Since Last Release](https://img.shields.io/github/commits-since/danielpclark/faster_path/v0.3.8.svg)](https://github.com/danielpclark/faster_path/pulse)
[![Binary Release](https://img.shields.io/github/release/danielpclark/faster_path.svg)](https://github.com/danielpclark/faster_path/releases)
[![Coverage Status](https://coveralls.io/repos/github/danielpclark/faster_path/badge.svg?branch=master)](https://coveralls.io/github/danielpclark/faster_path?branch=master)
[![Inline docs](http://inch-ci.org/github/danielpclark/faster_path.svg?branch=master)](http://inch-ci.org/github/danielpclark/faster_path)
[![Code Triagers Badge](https://www.codetriage.com/danielpclark/faster_path/badges/users.svg)](https://www.codetriage.com/danielpclark/faster_path)
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

Here are some additional stats.  From Rails loading to my home page, these methods are called _(not directly, Rails & gems call them)_ this many times.  And the home page has minimal content.
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
* Methods are stable
* Thoroughly tested
* Testers and developers are most welcome
* Windows & encoding support is underway!

## Installation

Ensure Rust is installed:

[Rust Downloads](https://www.rust-lang.org/downloads.html)

```
curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

Add this line to your application's Gemfile:

```ruby
gem 'faster_path', '~> 0.3.8'
```

And then execute:

    $ bundle

Or install it yourself as:

    $ gem install faster_path

## Visual Benchmarks

Benchmarks in Faster Path now produce visual graph charts of performance improvements.
When you run `export GRAPH=1; bundle && rake bench` the graph art will be placed in `doc/graph/`.  Here's the performance
improvement result  for the `chop_basename` method.

![Visual Benchmark](https://raw.githubusercontent.com/danielpclark/faster_path/master/assets/chop_basename_benchmark.jpg "Visual Benchmark")

## Usage

Current methods implemented:

|FasterPath Rust Implementation|Ruby 2.5.0 Implementation|Time Shaved Off|
|---|---|:---:|
| `FasterPath.absolute?` | `Pathname#absolute?` | 95.8% |
| `FasterPath.add_trailing_separator` | `Pathname#add_trailing_separator` | 68.7% |
| `FasterPath.basename` | `File.basename` | 49.4% |
| `FasterPath.children` | `Pathname#children` | 39.2% |
| `FasterPath.chop_basename` | `Pathname#chop_basename` | 76.6% |
| `FasterPath.cleanpath_aggressive` | `Pathname#cleanpath_aggressive` | 78.2% |
| `FasterPath.cleanpath_conservative` | `Pathname#cleanpath_conservative` | 74.1% |
| `FasterPath.del_trailing_separator` | `Pathname#del_trailing_separator` | 87.7% |
| `FasterPath.directory?` | `Pathname#directory?` | 16.8% |
| `FasterPath.dirname` | `File.dirname` | 28.9% |
| `FasterPath.entries` | `Pathname#entries` | 36.1% |
| `FasterPath.extname` | `File.extname` | 66.2% |
| `FasterPath.has_trailing_separator?` | `Pathname#has_trailing_separator` | 86.3% |
| `FasterPath.plus` | `Pathname#join` | 67.9% |
| `FasterPath.plus` | `Pathname#plus` | 86.6% |
| `FasterPath.relative?` | `Pathname#relative?` | 87.2% |
| `FasterPath.relative_path_from` | `Pathname#relative_path_from` | 74.0% |

You may choose to use the methods directly, or scope change to rewrite behavior on the
standard library with the included refinements, or even call a method to monkeypatch
everything everywhere.

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

## Optional Rust implementations

**These are stable, not performant, and not included in `Pathname` by default.**

These will **not** be included by default in monkey-patches.  To try them with monkeypatching use the environment flag of `WITH_REGRESSION`.  These methods are here to be improved upon.

|FasterPath Implementation|Ruby Implementation|
|---|---|
| `FasterPath.entries_compat` | `Pathname.entries` |
| `FasterPath.children_compat` | `Pathname.children` |

It's been my observation (and some others) that the Rust implementation of the C code for `File` has similar results but
performance seems to vary based on CPU cache on possibly 64bit/32bit system environments.  These are not included by default when the monkey patch method `FasterPath.sledgehammer_everything!` is executed.

## Getting Started with Development

The primary methods to target are mostly listed in the **Why** section above.  You may find the Ruby
source code useful for Pathname's [Ruby source](https://github.com/ruby/ruby/blob/32674b167bddc0d737c38f84722986b0f228b44b/ext/pathname/lib/pathname.rb),
[C source](https://github.com/ruby/ruby/blob/32674b167bddc0d737c38f84722986b0f228b44b/ext/pathname/pathname.c),
[tests](https://github.com/ruby/ruby/blob/32674b167bddc0d737c38f84722986b0f228b44b/test/pathname/test_pathname.rb),
and checkout the [documentation](http://ruby-doc.org/stdlib-2.3.1/libdoc/pathname/rdoc/Pathname.html).

Methods will be written as exclusively in Rust as possible.  Even just writing a **not** in Ruby with a
Rust method like `!absolute?` _(not absolute)_ drops 39% of the performance already gained in Rust.
Whenever feasible implement it in Rust.

After checking out the repo, make sure you have Rust installed, then run `bundle`.
Run `rake test` to run the tests, and `rake bench` for benchmarks.

Learn and share performance tips on the [wiki](https://github.com/danielpclark/faster_path/wiki)!

### Building and running tests

First, bundle the gem's development dependencies by running `bundle`.  Rust compilation is included in the current rake commands.

FasterPath is tested with [The Ruby Spec Suite](https://github.com/ruby/spec) to ensure it is compatible with the
native implementation, and also has its own test suite testing its monkey-patching and refinements functionality.

To run all the tests at once, simply run `rake`.
To run all the ruby spec tests, run `mspec`.

To run an individual test or benchmark from FasterPath's own suite:

```sh
# An individual test file:
ruby -I lib:test test/benches/absolute_benchmark.rb
# All tests:
rake minitest
```

To run an individual ruby spec test, run `mspec` with a path relative to `spec/ruby_spec`, e.g.:

```sh
# A path to a file or a directory:
mspec core/file/basename_spec.rb
# Tests most relevant to FasterPath:
mspec core/file library/pathname
# All tests:
mspec
```

### Rust Benchmarking

Ruby's benchmarks are pretty well covered here.  So here's the pattern for running Rust benchmarks
to see whether your new implementations or modifications are faster.  Look at
[`basename_benchmark.rs`](benches/basename_benchmark.rs) for an example on how to implement Rust
benchmarks.  You'll need to create a benchmark for the current implementation and an identical
benchmark for your new version.  The versions with be benchmark method names ending in `_version#`.

To have the project included natively you will need to use the include macro: `include!("../src/lib.rs")`.

Technically we can place individual benchmarks in the source file of the original methods just like we do
with some unit tests.  But I'm against that.  I'd much prefer having benchmarks for the method that's being
re-implemented to be grouped together.

When running benchmarks it will require Rust nightly.  A simple way to run the command is as follows.

```
rake libruby_release; cargo +nightly bench 
```


## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/danielpclark/faster_path.


## License

[MIT License](http://opensource.org/licenses/MIT) or APACHE 2.0 at your pleasure.

