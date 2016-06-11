# FasterPath
[![Gem Version](https://badge.fury.io/rb/faster_path.svg)](https://badge.fury.io/rb/faster_path)
[![Build Status](https://travis-ci.org/danielpclark/faster_path.svg?branch=master)](https://travis-ci.org/danielpclark/faster_path)
[![Tweet This](https://raw.githubusercontent.com/danielpclark/faster_path/master/assets/tweet.png)](https://twitter.com/share?url=https%3A%2F%2Fgithub.com%2Fdanielpclark%2Ffaster_path&via=6ftdan&related=ruby%2Crails&hashtags=Ruby&text=You%20could%20save%2015%25%20or%20more%20on%20website%20load%20time%20by%20switching%20to%20the%20FasterPath%20gem.)

The primary **GOAL** of this project is to improve performance in the most heavily used areas of Ruby as
path relation and file lookup is currently a huge bottleneck in performance.  As this is the case the
path performance updates will likely not be limited to just changing the Pathname class but also will
be offering changes in related methods and classes.

Users will have the option to write their apps directly for this library, or they can choose to either
refine or monkeypatch the existing standard library.  Refinements are narrowed to scope and monkeypatching will
be a sledge hammer ;-)

**NOTE**: Refinements and monkeypatch methods are highly likely to be changed and renamed pre version
0.1.0 so keep that in mind!

## Why

I did a check on Rails on what methods were being called the most and where the application spend
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

## Status

* Rust compilation is working
* Methods are _most likely_ stable
* Testers and developers are most welcome!

## Installation

Ensure Rust is installed:

[Rust Downloads](https://www.rust-lang.org/downloads.html)

```
curl -sSf https://static.rust-lang.org/rustup.sh | sudo sh -s -- --channel=nightly
```

Add this line to your application's Gemfile:

```ruby
gem 'faster_path'
```

And then execute:

    $ bundle

Or install it yourself as:

    $ gem install faster_path

## Usage

Current methods implemented:

|FasterPath Rust Implementation|Ruby 2.3.1 Implementation|Performance Improvement|
|---|---|:---:|
| `FasterPath.absolute?` | `Pathname#absolute?` | 1234.6% |
| `FasterPath.chop_basename` | `Pathname#chop_basename` | 27.5% |
| `FasterPath.relative?` | `Pathname#relative?` | 908.6% |
| `FasterPath.blank?` | | |

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

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake test` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and tags, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/danielpclark/faster_path.


## License

[MIT License](http://opensource.org/licenses/MIT) or APACHE 2.0 at your pleasure.

