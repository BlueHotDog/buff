# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [0.3.15] - 2019-07-25

### Changed
- Changes minimum documented rust version to 1.28 (#76)

### Fixed
- Fix Travis CI badge url (#78)
- Fix project name in README.md (#81)

### Added
- Support specifying range of versions (#75)
- Allow cross-compilation if pkg-config is customized (#44, #86)

## [0.3.14] - 2018-08-28

### Fixed
- Don't append .lib suffix on MSVC builds (#72)

## [0.3.13] - 2018-08-06

### Fixed
- Fix MSVC support to actually work and consider library paths too (#71)

## [0.3.12] - 2018-06-18

### Added
- Support for MSVC (#70)
- Document and test Rust 1.13 as minimally supported version (#66)

## [0.3.11] - 2018-04-24

### Fixed
- Re-added AsciiExt import (#65)

## [0.3.10] - 2018-04-23

### Added
- Allow static linking of /usr/ on macOS (#42)
- Add support for parsing `-Wl,` style framework flags (#48)
- Parse defines in `pkg-config` output (#49)
- Rerun on `PKG_CONFIG_PATH` changes (#50)
- Introduce target-scoped variables (#58)
- Respect pkg-config escaping rules used with --cflags and --libs (#61)

### Changed
- Use `?` instead of `try!()` in the codebase (#63)
