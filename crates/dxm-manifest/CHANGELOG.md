# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

[dxm unreleased]

### Changed

- **Breaking:** use `BTreeMap` instead of `HashMap` for `Lockfile` resources to
  prevent rewriting them in different order.
- Updated dependencies.

## 0.3.0 - 2026-01-29

[dxm 0.2.1]

### Added

- `MANIFEST_NAME` constant.
- `lockfile::LOCKFILE_NAME` constant.
- `sourcefile::SOURCEFILE_NAME` constant.

## 0.2.0 - 2026-01-12

[dxm 0.2.0]

### Added

- Lockfile structures.
- Sourcefile functions.

### Changed

- **Breaking:** updated the default value for `Artifact::version` from an empty
  string to `latest-jg`.
- **Breaking:** updated `Artifact::channel` to parse the `version` field from
  the source TOML, and return the value only if the version is a valid update
  channel.
- Updated dependencies.
- TOML categories are now written individiually instead of overwriting the entire file.

### Removed

- **Breaking:** `Artifact::set_channel` and the `channel` field in
  `Artifact::fill_toml_item` - use `Artifact::set_version` and the `version`
  field instead, with a stringified channel.

### Fixed

- TOML writing erroring for new documents.

## 0.1.1 - 2025-07-20

[dxm 0.1.2]

### Changed

- Updated dependencies.
- Migrated from `toml` to `toml_edit` to preserve the manifest file format and support partial manifest files.

## 0.1.0 - 2025-03-19

[dxm 0.1.0]

Initial release.

[dxm unreleased]: https://github.com/D4isDAVID/dxm/commits/main/crates/dxm-manifest
[dxm 0.2.1]: https://github.com/D4isDAVID/dxm/commits/v0.2.1/crates/dxm-manifest
[dxm 0.2.0]: https://github.com/D4isDAVID/dxm/commits/v0.2.0/crates/dxm-manifest
[dxm 0.1.2]: https://github.com/D4isDAVID/dxm/commits/v0.1.2/crates/dxm-manifest
[dxm 0.1.0]: https://github.com/D4isDAVID/dxm/commits/v0.1.0/crates/dxm-manifest
