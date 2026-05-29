//! Download & install FXServer artifacts. Supports downloading specific builds,
//! resolving versions such as latest and recommended, and fetching the JG
//! Scripts [Artifacts DB].
//!
//! Downloads are supported for both [Windows] and [Linux] which FXServer
//! supports. Version resolution is done through Cfx.re's [Changelogs API] or
//! the JG Scripts [Artifacts DB].
//!
//! ## Example
//!
//! ```no_run
//! use std::path::Path;
//!
//! use dxm_artifacts::{Artifacts, ArtifactsPlatform, BuildVersion};
//!
//! #[tokio::main]
//! async fn main() {
//!     let artifacts = Artifacts::new().unwrap();
//!
//!     let build = artifacts.resolve(&BuildVersion::LatestJg).await.unwrap();
//!     let artifacts_path = Path::new("./artifacts/");
//!     let platform = ArtifactsPlatform::default();
//!
//!     artifacts.install(&build, platform, artifacts_path).await.unwrap();
//! }
//! ```
//!
//! [artifacts db]: https://artifacts.jgscripts.com
//! [windows]: https://runtime.fivem.net/artifacts/fivem/build_server_windows/master
//! [linux]: https://runtime.fivem.net/artifacts/fivem/build_proot_linux/master
//! [changelogs api]: https://changelogs-live.fivem.net/api/changelog/versions/win32/server

#![deny(clippy::all)]
#![deny(missing_docs)]

mod api;
mod archive;
mod build;
mod error;
mod resolve;

pub use crate::api::ArtifactsPlatform;
pub use crate::build::{Build, BuildIssue, BuildVersion};
pub use crate::error::{Error, Result};
pub use crate::resolve::Artifacts;
