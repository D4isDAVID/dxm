# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

[unreleased diff]

### Changed

- Changed the Linux `env.sh` to export `DXM_HOME`.
- Updated `nothing to uninstall` message to specify `DXM_HOME`.

## [0.3.0] - 2026-05-18

[0.3.0 diff]

### Added

- `dxm monitor install` to install a third-party monitor.
- `dxm monitor remove` to remove the third-party monitor if exists.
- `dxm update --monitor` to update the third-party monitor if exists.
- `dxm add --git` to install third-party resources through any Git repository.
- Profiles to use with `dxm start [profile]` to predefine startup arguments and
  environment variables.
- `dxm patch prepare` to prepare a third-party resource for persistent patching.
- `dxm patch commit` to create a persistent patch for a third-party resource.
- `dxm patch remove` to remove a persistent patch for a third-party resource.

### Changed

- Updated dependencies.
- Changed `dxm update` without arguments to update everything by default.
- Updated `dxm install` to install the third-party monitor if exists.
- Updated `dxm update` to update the third-party monitor if exists.
- **Breaking:** renamed `dxm run` to `dxm start`.
- Changed Linux release target to `x86_64-unknown-linux-musl` to support more
  Linux versions.
- **Breaking:** Updated `dxm update -a` to correlate to `dxm update --artifacts`
  since `dxm update --all` has been removed.
- Updated the `dxm init` template.

### Removed

- **Breaking:** `dxm update --all` - use `dxm update` instead.
- Colors from help and usage messages for uniform coloring.
- **Breaking:** `dxm artifacts update` - use `dxm update --artifacts` instead.
- **Breaking:** resolving most GitHub branch and tag names from most blob URLs -
  pass a non-blob URL instead.

### Fixed

- Resolving GitHub branch and tag names containing forward-slashes (`/`).

## [0.2.4] - 2026-03-07

[0.2.4 diff]

### Changed

- Updated dependencies.

## [0.2.3] - 2026-02-05

[0.2.3 diff]

### Changed

- Updated dependencies.

### Fixed

- Lockfile resources changing order.
- `dxm run` not accepting more than one server argument.

## [0.2.2] - 2026-01-29

[0.2.2 diff]

### Fixed

- Installing resources from `github.com/author/repo/archive` URLs.

## [0.2.1] - 2026-01-29

[0.2.1 diff]

### Fixed

- Installing resources from their default GitHub branch.

## [0.2.0] - 2026-01-12

[0.2.0 diff]

### Added

- `dxm add` to install resources - supports direct downloads, and GitHub URLs.
- `dxm remove` to uninstall resources.
- Lockfile to lock and keep track of download URLs.
- `dxm install` to install artifacts & servers.
- `dxm update` to update artifacts & servers.

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

[unreleased diff]: https://github.com/D4isDAVID/dxm/compare/v0.3.0...main
[0.3.0]: https://github.com/D4isDAVID/dxm/releases/tag/v0.3.0
[0.3.0 diff]: https://github.com/D4isDAVID/dxm/compare/v0.2.4...v0.3.0
[0.2.4]: https://github.com/D4isDAVID/dxm/releases/tag/v0.2.4
[0.2.4 diff]: https://github.com/D4isDAVID/dxm/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/D4isDAVID/dxm/releases/tag/v0.2.3
[0.2.3 diff]: https://github.com/D4isDAVID/dxm/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/D4isDAVID/dxm/releases/tag/v0.2.2
[0.2.2 diff]: https://github.com/D4isDAVID/dxm/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/D4isDAVID/dxm/releases/tag/v0.2.1
[0.2.1 diff]: https://github.com/D4isDAVID/dxm/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/D4isDAVID/dxm/releases/tag/v0.2.0
[0.2.0 diff]: https://github.com/D4isDAVID/dxm/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/D4isDAVID/dxm/releases/tag/v0.1.2
[0.1.2 diff]: https://github.com/D4isDAVID/dxm/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/D4isDAVID/dxm/releases/tag/v0.1.1
[0.1.1 diff]: https://github.com/D4isDAVID/dxm/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/D4isDAVID/dxm/releases/tag/v0.1.0
[0.1.0 commits]: https://github.com/D4isDAVID/dxm/commits/v0.1.0
