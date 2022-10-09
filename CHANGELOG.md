# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] – 2022-10-09

### Changed
- Bump MSRV to 1.56.

## [0.2.3] – 2022-03-12

### Changed
- Limit `gethostname()` to `cfg(unix)` and `cfg(windows)` to provide more useful build failures on other platforms (see [#7]).

[#7]: https://github.com/lunaryorn/gethostname.rs/issues/7

## [0.2.2] – 2022-01-14

## [0.2.1] – 2019-12-18
### Changed
- Consolidate documetation.
- Update crates.io metadata.

## [0.2.0] – 2019-01-22
### Added
- Add Windows implementation (see [GH-1]).

[Gh-1]: https://github.com/lunaryorn/gethostname.rs/pull/1

### Changed
- Pin supported Rust version to 1.31

## 0.1.0 – 2019-01-20
Initial release.

### Added

- `gethostname()` for non-Windows platforms.

[Unreleased]: https://github.com/lunaryorn/gethostname.rs/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/lunaryorn/gethostname.rs/compare/v0.2.3...v0.3.0
[0.2.3]: https://github.com/lunaryorn/gethostname.rs/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/lunaryorn/gethostname.rs/compare/gethostname-0.2.1...v0.2.2
[0.2.0]: https://github.com/lunaryorn/gethostname.rs/compare/gethostname-0.1.0...gethostname-0.2.0
[0.2.1]: https://github.com/lunaryorn/gethostname.rs/compare/gethostname-0.2.0...gethostname-0.2.1
