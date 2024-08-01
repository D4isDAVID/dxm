use serde::Deserialize;

#[cfg(unix)]
const URL: &str = "https://changelogs-live.fivem.net/api/changelog/versions/linux/server";
#[cfg(windows)]
const URL: &str = "https://changelogs-live.fivem.net/api/changelog/versions/win32/server";

#[derive(Deserialize)]
pub struct CfxServerChangelogs {
    critical: String,
    recommended: String,
    optional: String,
    latest: String,
    critical_download: String,
    recommended_download: String,
    optional_download: String,
    latest_download: String,
}

impl CfxServerChangelogs {
    pub fn url() -> &'static str {
        URL
    }

    pub fn critical(&self) -> &str {
        &self.critical
    }

    pub fn recommended(&self) -> &str {
        &self.recommended
    }

    pub fn optional(&self) -> &str {
        &self.optional
    }

    pub fn latest(&self) -> &str {
        &self.latest
    }

    pub fn critical_download(&self) -> &str {
        &self.critical_download
    }

    pub fn recommended_download(&self) -> &str {
        &self.recommended_download
    }

    pub fn optional_download(&self) -> &str {
        &self.optional_download
    }

    pub fn latest_download(&self) -> &str {
        &self.latest_download
    }
}
