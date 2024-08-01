use changelogs::CfxServerChangelogs;

pub mod changelogs;

pub struct Cfx;

impl Cfx {
    pub fn server_changelogs() -> anyhow::Result<CfxServerChangelogs> {
        let changelogs =
            reqwest::blocking::get(CfxServerChangelogs::url())?.json::<CfxServerChangelogs>()?;

        Ok(changelogs)
    }
}
