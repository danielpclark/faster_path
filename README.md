# FasterPath
[![Gem Version](https://badge.fury.io/rb/faster_path.svg)](https://badge.fury.io/rb/faster_path)
[![CI](https://github.com/danielpclark/faster_path/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/danielpclark/faster_path/actions/workflows/ci.yml)
[![Latest Tag](https://img.shields.io/github/tag/danielpclark/faster_path.svg)](https://github.com/danielpclark/faster_path/tags)
[![Commits Since Last Release](https://img.shields.io/github/commits-since/danielpclark/faster_path/v0.3.10.svg)](https://github.com/danielpclark/faster_path/pulse)
[![Binary Release](https://img.shields.io/github/release/danielpclark/faster_path.svg)](https://github.com/danielpclark/faster_path/releases)
[![Inline docs](http://inch-ci.org/github/danielpclark/faster_path.svg?branch=master)](http://inch-ci.org/github/danielpclark/faster_path)
[![Code Triagers Badge](https://www.codetriage.com/danielpclark/faster_path/badges/users.svg)](https://www.codetriage.com/danielpclark/faster_path)

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

## Performance Specifics

The headline for the amount for improvement on this library is specific to only the improvement made with the method `chop_basename`.  Just so you know; in my initial release I had a bug in which that method immediately returned nothing. Now the good thing about this is that it gave me some very valuable information.  First I found that all my Rails site tests still passed.  Second I found that all my assets no longer loaded in the website. And third, and most importantly, I found my Rails web pages loaded just more than 66% faster without the cost of time that `chop_basename` took.

**That's right; the path handling for assets in your website \*consumes more than 2/3rds of your websites page load time.**

So now we have some real numbers to work with  We can be generoues and use 66% as our margin of area to improve over _(for `chop_basename` specifically, not counting the benefit from improving the performance in other file path related methods)_.  That means we want to remove as much of that percentage from the overall systems page load time.  The original headline boasts over 33% performance improvement — that was when `chop_basename` was improved by just over 50%.  Now `chop_basename` is improved by 83.4%.  That alone should make your site run 55.044% faster now _(given your performance profile stats are similar to mine)_.

## What Rails Versions Will This Apply To?

As mentioned earlier Sprockets, which handles assets, changed away from using `Pathname` at all when moving from major version 2 to 3.  So if you're using Sprockets 3 or later you won't reap the biggest performance rewards from using this gem for now _(it's my goal to have this project become a core feature that Rails depends on and yes… that's a big ask)_.  That is unless you write you're own implementation to re-integrate the use of `Pathname` and `FasterPath` into your asset handling library.  For now just know that the Sprockets 2 series primarily works with Rails 4.1 and earlier.  It may work in later Rails versions but I have not investigated this.

## Status

* Rust compilation is working
* Methods are stable
* Thoroughly tested
* Testers and developers are most welcome
* Windows: paths follow Ruby's Windows rules (`\` and `/` separators, drive letters, UNC paths)
* Paths keep their encoding, as with Ruby's own methods

## Requirements

* Ruby 3.2, 3.3 or 3.4 (the Rust extension is built with [Rutie](https://github.com/danielpclark/rutie) 0.13, which supports these)
* Rust 1.71 or later

## Installation

Ensure Rust is installed:

[Rust Downloads](https://www.rust-lang.org/downloads.html)

```
curl -sSf https://sh.rustup.rs | sh
```

Add this line to your application's Gemfile:

```ruby
gem 'faster_path', '~> 0.5.0'
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

Add the proper require to your project.

```ruby
require "faster_path"
```

Current methods implemented:

|FasterPath Rust Implementation|Ruby Implementation|Time Shaved Off (Ruby 2.7)|Time Shaved Off (Ruby 2.5)|
|---|---|:---:|:---:|
| `FasterPath.absolute?` | `Pathname#absolute?` | 95.7% | 96.7% |
| `FasterPath.add_trailing_separator` | `Pathname#add_trailing_separator` | 85.4% | 83.0% |
| `FasterPath.basename` | `File.basename` | 42.4% | 39.1% |
| `FasterPath.children` | `Pathname#children` | 54.7% | 47.8% |
| `FasterPath.chop_basename` | `Pathname#chop_basename` | 70.4% | 75.1% |
| `FasterPath.cleanpath_aggressive` | `Pathname#cleanpath_aggressive` | 90.3% | 90.6% |
| `FasterPath.cleanpath_conservative` | `Pathname#cleanpath_conservative` | 89.6% | 91.6% |
| `FasterPath.del_trailing_separator` | `Pathname#del_trailing_separator` | 85.3% | 87.6% |
| `FasterPath.directory?` | `Pathname#directory?` | 34.3% | 37.5% |
| `FasterPath.dirname` | `File.dirname` | 56.8% | 54.7% |
| `FasterPath.entries` | `Pathname#entries` | 38.0% | 33.9% |
| `FasterPath.extname` | `File.extname` | 74.0% | 73.1% |
| `FasterPath.has_trailing_separator?` | `Pathname#has_trailing_separator` | 88.5% | 89.1% |
| `FasterPath.join` | `Pathname#join` | 88.1% | 90.3% |
| `FasterPath.plus` | `Pathname#plus` | 91.7% | 93.4% |
| `FasterPath.relative?` | `Pathname#relative?` | 92.8% | 95.2% |
| `FasterPath.relative_path_from` | `Pathname#relative_path_from` | 92.5% | 93.6% |

See [Benchmarks](#benchmarks) for how these are measured.

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

|FasterPath Implementation|Ruby Implementation|Time Shaved Off (Ruby 2.7)|Time Shaved Off (Ruby 2.5)|
|---|---|:---:|:---:|
| `FasterPath.entries_compat` | `Pathname.entries` | 16.8% | 9.2% |
| `FasterPath.children_compat` | `Pathname.children` | 32.1% | 26.6% |

It's been my observation (and some others) that the Rust implementation of the C code for `File` has similar results but
performance seems to vary based on CPU cache on possibly 64bit/32bit system environments.  These are not included by default when the monkey patch method `FasterPath.sledgehammer_everything!` is executed.

## Benchmarks

The "Time Shaved Off" figures come from the project's Pinch-bench (`rake pbench`, in
`test/pbench`): each method runs against Ruby's own implementation, alternating with it round by
round, and the result is how much less time FasterPath's best round took than Ruby's best round.
They are the median of 3 runs of `LONG_RUN=10 rake pbench` (500,000 calls of each method, 5
rounds) on Linux x86_64 with Ruby 3.4.6 and 3.2.9, built with Rust 1.97. Expect a few points of
difference from run to run.

The figures are smaller than they were on Ruby 2 for the cheapest methods, `absolute?` and
`relative?` above all: Ruby 3's own `Pathname#absolute?` is a single regexp match, where Ruby 2's
walked the path with `chop_basename`, so there is less left to shave off.

`rake pbench` is also the performance regression test: each method has a floor in
`test/pbench/pbench_suite.rb` (about half of the improvement in the table above) and the task
fails when a method measures below it, so a change that makes a method slower shows up in
`rake test`. On Windows, where `join` is currently slower than Ruby's
([#185](https://github.com/danielpclark/faster_path/issues/185)), the floors are reported but not
enforced.

### Before and after the Rutie 0.10 upgrade

The last figures on Ruby 2, from the Rutie 0.10 upgrade (faster_path 0.4.0): Ruby 2.7.8 and
2.5.7, alternating runs of the code before the upgrade (built with Rutie 0.6) and after it, with
the benchmark then measuring the average of 5 rounds run one implementation after the other.
Higher is better; the change is in percentage points.

|Method|Ruby 2.7 before|Ruby 2.7 after|Change|Ruby 2.5 before|Ruby 2.5 after|Change|
|---|:---:|:---:|:---:|:---:|:---:|:---:|
| `absolute?` | 93.3% | 95.7% | +2.4 | 96.1% | 96.7% | +0.6 |
| `add_trailing_separator` | 66.6% | 85.4% | **+18.8** | 64.3% | 83.0% | **+18.7** |
| `basename` | 42.4% | 42.4% | 0.0 | 32.1% | 39.1% | **+7.0** |
| `children` | 52.8% | 54.7% | +1.9 | 46.5% | 47.8% | +1.3 |
| `children_compat` | −1.2% | 32.1% | **+33.3** | 1.1% | 26.6% | **+25.5** |
| `chop_basename` | 71.2% | 70.4% | −0.8 | 74.7% | 75.1% | +0.4 |
| `cleanpath_aggressive` | 90.3% | 90.3% | 0.0 | 92.7% | 90.6% | −2.1 |
| `cleanpath_conservative` | 89.8% | 89.6% | −0.2 | 91.7% | 91.6% | −0.1 |
| `del_trailing_separator` | 86.5% | 85.3% | −1.2 | 87.0% | 87.6% | +0.6 |
| `directory?` | 33.2% | 34.3% | +1.1 | 37.2% | 37.5% | +0.3 |
| `dirname` | 49.8% | 56.8% | **+7.0** | 51.2% | 54.7% | +3.5 |
| `entries` | 40.4% | 38.0% | −2.4 | 33.6% | 33.9% | +0.3 |
| `entries_compat` | −29.8% | 16.8% | **+46.6** | −23.3% | 9.2% | **+32.5** |
| `extname` | 70.3% | 74.0% | +3.7 | 74.0% | 73.1% | −0.9 |
| `has_trailing_separator?` | 87.5% | 88.5% | +1.0 | 87.1% | 89.1% | +2.0 |
| `join` | crashes | 88.1% | | crashes | 90.3% | |
| `plus` | 92.3% | 91.7% | −0.6 | 93.0% | 93.4% | +0.4 |
| `relative?` | 90.7% | 92.8% | +2.1 | 93.6% | 95.2% | +1.6 |
| `relative_path_from` | 92.7% | 92.5% | −0.2 | 94.0% | 93.6% | −0.4 |

Before the upgrade, `FasterPath.join` segfaulted, so its benchmark can't be measured.
`add_trailing_separator`, `dirname` and the `_compat` methods gained the most:
`add_trailing_separator` no longer builds a temporary string, no method checks its arguments
for valid UTF-8 any more, and the `_compat` methods build `Pathname` objects without calling
`Pathname.new`. `basename` was re-measured after it was made to take a shorter path on Unix
and skip an empty extension; it was the one method that first measured slower on Ruby 2.5.
Its timings vary a lot from run to run (Ruby 2.5 before: 40.0%, 32.1%, 28.7%; after: 29.6%,
39.1%, 39.2%), so its instruction counts from `valgrind --tool=callgrind` are a steadier
measure. They went from 2439 to 2331 per call on Ruby 2.5 and from 2689 to 2561 on Ruby 2.7.

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

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/danielpclark/faster_path.


## License

[MIT License](http://opensource.org/licenses/MIT) or APACHE 2.0 at your pleasure.

