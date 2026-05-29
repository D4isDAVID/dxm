# dxm-artifacts

> This crate is a part of [`dxm`], a CLI tool for managing FXServer artifacts &
> resources. See the [GitHub Repository] for more information.

Download & install FXServer artifacts. Supports downloading specific builds,
resolving versions such as latest and recommended, and fetching the JG
Scripts [Artifacts DB].

Downloads are supported for both [Windows] and [Linux] which FXServer
supports. Version resolution is done through Cfx.re's [Changelogs API] or
the JG Scripts [Artifacts DB].

## Usage

This crate uses [`tracing`] for logging, and is built with [`tokio`] and
[`reqwest`] in mind.

To install the crate as a dependency, add the following to your `Cargo.toml`:

```toml
[dependencies]
dxm-artifacts = "0.3"
```

## Example

```rs
use std::path::Path;
use dxm_artifacts::{Artifacts, ArtifactsPlatform, BuildVersion};

#[tokio::main]
async fn main() {
    let artifacts = Artifacts::new().unwrap();

    let build = artifacts.resolve(&BuildVersion::LatestJg).await.unwrap();
    let artifacts_path = Path::new("./artifacts/");
    let platform = ArtifactsPlatform::default();

    artifacts.install(&build, platform, artifacts_path).await.unwrap();
}
```

## Contributing

If you are interested in contributing to this project, read the
[Contributing guidelines] to learn more.

## License

This project's source code © 2024 David Malchin is licensed under the
**MIT License (MIT)** provided in the [LICENSE] file.

[`dxm`]: https://crates.io/crates/dxm
[github repository]: https://github.com/D4isDAVID/dxm
[artifacts db]: https://artifacts.jgscripts.com
[windows]: https://runtime.fivem.net/artifacts/fivem/build_server_windows/master
[linux]: https://runtime.fivem.net/artifacts/fivem/build_proot_linux/master
[changelogs api]: https://changelogs-live.fivem.net/api/changelog/versions/win32/server
[`tracing`]: https://crates.io/crates/tracing
[`tokio`]: https://crates.io/crates/tokio
[`reqwest`]: https://crates.io/crates/reqwest
[contributing guidelines]: ./CONTRIBUTING.md
[license]: ./LICENSE
