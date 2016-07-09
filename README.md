# uninitialized-rs

[![travis-badge][]][travis] [![release-badge][]][cargo] [![docs-badge][]][docs] [![license-badge][]][license]

`uninitialized` provides `std::mem::uninitialized()` behaviour only when a
project is willing to take the risk of uninitialized data being passed to
external interfaces. By default it is equivalent to `std::mem::zeroed()`.

## Enabling uninitialized

The feature `uninitialized` may be turned on to revert to the unsafe behaviour.
Optionally, the `UNSAFE_UNINITIALIZED=1` environment variable may be set at
build time.

## [Documentation][docs]

See the [documentation][docs] for up to date information.

[travis-badge]: https://img.shields.io/travis/arcnmx/uninitialized-rs/master.svg?style=flat-square
[travis]: https://travis-ci.org/arcnmx/uninitialized-rs
[release-badge]: https://img.shields.io/crates/v/uninitialized.svg?style=flat-square
[cargo]: https://crates.io/crates/uninitialized
[docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
[docs]: http://arcnmx.github.io/uninitialized-rs/uninitialized/
[license-badge]: https://img.shields.io/badge/license-MIT-ff69b4.svg?style=flat-square
[license]: https://github.com/arcnmx/uninitialized-rs/blob/master/COPYING
