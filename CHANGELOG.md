# Changelog

All important changes for the QuikDecision Rust library will be documented
in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.10.1] - 2020-11-22
### Changed
- Improve error handling
- Update quikdecision library

## [0.10.0] - 2019-05-2
### Changed
- Update to latest version of quikdecision library

## [0.9.5] - 2019-01-13
### Added
- API support for shuffling a list of strings
- API support for drawing a card from one of 3 deck types
- UI support for drawing a card from one of 3 deck types
### Changed
- Update license for website.
- Add MIT license for code.
- Fill out more metadata for OpenAPI spec.

## [0.9.2] - 2019-01-01
### Added
- Ability to save dice expressions.
- Added ability to remove saved items
### Changed
- Combined user-saved and example lists into one drop-down menu.
- Added meta tags to the main page.
- Tweak spacing between buttons on list selection tab.
- Protect against empty strings in lists.
### Removed
- Removed the example list buttons.

## [0.9.1] - 2018-12-30
### Added
- Some example lists are provided for the select a string feature.
- The ability to save and load lists from local storage.
### Changed
- Select a string from a list icon changed

## [0.9.0] - 2018-12-30
### Added
- Support for OpenAPI description of API

### Changed
- Errors returned with status code of 400 instead of 200.
- JavaScript on page changed to handle 400 return code.
- Update attribution file
- Add privacy policy file
- Fix static file support
- Modify to allow setting IP Address from environment
- Rename /likely endpoint to /likelihood

## [0.8.0] - 2018-12-21
### Changed
- DiceRolls now support 3-sided dice

## [0.7.5] - 2018-12-07
### Changed
- Update code to 2018 edition of Rust
