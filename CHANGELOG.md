# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

[unreleased diff]

### Added

- `dxm add` to install resources - supports direct downloads, and GitHub URLs.
- `dxm remove` to uninstall resources.
- Lockfile to lock and keep track of download URLs.

### Changed

- **Breaking:** updated the default value of the `version` field in the manifest
  from an empty string to the `latest-jg` update channel.
- Updated dependencies.
- Optimized the binary file size.

### Removed

- **Breaking:** the `channel` field from the manifest - it has been merged with
  the `version` field.

## [0.1.2] - 2025-07-20

[0.1.2 diff]

### Changed

- Updated dependencies.

### Fixed

- Writing manifest file when using commands outside the manifest's directory.

## [0.1.1] - 2025-07-14

[0.1.1 diff]

### Fixed

- `dxm run` sometimes not finding the FXServer executable.

## [0.1.0] - 2025-03-19

[0.1.0 commits]

Initial release.

[unreleased diff]: https://github.com/D4isDAVID/dxm/compare/v0.1.2...main
[0.1.2]: https://github.com/D4isDAVID/dxm/releases/tag/v0.1.2
[0.1.2 diff]: https://github.com/D4isDAVID/dxm/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/D4isDAVID/dxm/releases/tag/v0.1.1
[0.1.1 diff]: https://github.com/D4isDAVID/dxm/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/D4isDAVID/dxm/releases/tag/v0.1.0
[0.1.0 commits]: https://github.com/D4isDAVID/dxm/commits/v0.1.0
