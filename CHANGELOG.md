# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)

## [Unreleased] - yyyy-mm-dd

### Added

### Changed

- Upgrade protobuf to v3.4.0

### Removed

### Fixed

- The protobuf version is now pinned, as the generated code is pinned to a specific version

## [0.8.0] - 2023-11-20

### Changed

- Upgrade protobuf to v3.3.0

## [0.6.x]-[0.7.x] - 2023-04

### Changed

- Change some protobuf string entry into bytes, due to them not being correct utf-8 in some dfhack versions

## [0.5.x] - 2023-04-23

### Changed

- Upgraded protobuf dependency to v3

## [0.1.x]-[0.4.x] - 2022-04

### Added

- Initial versions of `dfhack-remote`
