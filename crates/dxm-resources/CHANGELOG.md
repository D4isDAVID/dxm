# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

[dxm unreleased]

## 0.2.1 - 2026-01-29

[dxm 0.2.2]

### Fixed

- `github::resolve_download_url` incorrectly resolving
  `github.com/author/repo/archive` URLs.

## 0.2.0 - 2026-01-29

[dxm 0.2.1]

### Changed

- **Breaking:** updated `github::resolve_download_url` to return `None` given a
  non-GitHub URL.
- **Breaking:** updated `resolve_download_url` to receive `Into<String>` instead
  of `AsRef<str>`.

### Fixed

- `github::resolve_download_url` to return an error with an empty `github.com`
  string without an ending slash.
- `github::api::get_default_branch_archive_url` returning the default branch
  name rather than the archive URL.

## 0.1.0 - 2026-01-12

[dxm 0.2.0]

Initial release.

[dxm unreleased]: https://github.com/D4isDAVID/dxm/commits/main/crates/dxm-resources
[dxm 0.2.2]: https://github.com/D4isDAVID/dxm/commits/v0.2.2/crates/dxm-resources
[dxm 0.2.1]: https://github.com/D4isDAVID/dxm/commits/v0.2.1/crates/dxm-resources
[dxm 0.2.0]: https://github.com/D4isDAVID/dxm/commits/v0.2.0/crates/dxm-resources
