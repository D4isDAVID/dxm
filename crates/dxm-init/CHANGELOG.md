# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

[dxm unreleased]

### Added

- `Template` struct to create new templates.
- `TemplateError` enum containing possible errors.
- `TemplateResult` utility type.
- `base_template` function to create a default base template.
- `git_template` function to create a default template for Git.

### Changed

- **Breaking:** all blocking functions are now asynchronous.
- **Breaking:** all functions returning `Result<_, Box<dyn Error>>` now return
  proper error types.

### Removed

- **Breaking::** `server` function - use `base_template` instead.
- **Breaking::** `vcs` module - use `git_template` instead.
- **Breaking:** constants `ENV_CFG_NAME`, `PERMISSIONS_CFG_NAME`,
  `README_NAME`, `RESOURCES_CFG_NAME`, `SECRETS_CFG_NAME`, `SERVER_CFG_NAME`,
  `TXDATA_CONFIG_NAME`, `TXDATA_DEFAULT_PROFILE` and `TXDATA_DIR`.

## 0.3.0 - 2026-05-18

[dxm 0.3.0]

### Added

- More CFG files and default convars.
- Default README.
- Default txAdmin config.

## 0.2.2 - 2026-03-07

[dxm 0.2.4]

### Changed

- Updated dependencies.

## 0.2.1 - 2026-02-05

[dxm 0.2.3]

### Changed

- Updated dependencies.

## 0.2.0 - 2026-01-29

[dxm 0.2.1]

### Added

- `SERVER_CFG_NAME` constant.

### Changed

- **Breaking:** updated `VcsOption::init` to accept a manifest instead of
  hard-coding paths.
- **Breaking:** updated `server` to use paths from the manifest instead of
  hard-coded paths.
- Updated `vcs::VcsOption` to derive from `Debug`, `PartialEq` and `Eq`.

## 0.1.2 - 2026-01-12

[dxm 0.2.0]

### Changed

- Updated dependencies.

## 0.1.1 - 2025-07-20

[dxm 0.1.2]

### Changed

- Updated dependencies.

## 0.1.0 - 2025-03-19

[dxm 0.1.0]

Initial release.

[dxm unreleased]: https://github.com/D4isDAVID/dxm/commits/main/crates/dxm-init
[dxm 0.3.0]: https://github.com/D4isDAVID/dxm/commits/v0.3.0/crates/dxm-init
[dxm 0.2.4]: https://github.com/D4isDAVID/dxm/commits/v0.2.4/crates/dxm-init
[dxm 0.2.3]: https://github.com/D4isDAVID/dxm/commits/v0.2.3/crates/dxm-init
[dxm 0.2.1]: https://github.com/D4isDAVID/dxm/commits/v0.2.1/crates/dxm-init
[dxm 0.2.0]: https://github.com/D4isDAVID/dxm/commits/v0.2.0/crates/dxm-init
[dxm 0.1.2]: https://github.com/D4isDAVID/dxm/commits/v0.1.2/crates/dxm-init
[dxm 0.1.0]: https://github.com/D4isDAVID/dxm/commits/v0.1.0/crates/dxm-init
