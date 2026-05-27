use crate::Template;

/// Creates and returns a [`Template`] containing the following files:
///
/// - `./data/env.cfg`: Environment configuration.
/// - `./data/permissions.cfg`: ACE permissions configuration.
/// - `./data/resources.cfg`: Resources configuration.
/// - `./data/secrets.cfg`: Credentials configuration.
/// - `./data/server.cfg`: General server configuration.
pub fn base_template() -> Template {
    Template::new()
        .file(
            "./data/env.cfg",
            include_bytes!("../templates/base/data/env.cfg"),
        )
        .file(
            "./data/permissions.cfg",
            include_bytes!("../templates/base/data/permissions.cfg"),
        )
        .file(
            "./data/resources.cfg",
            include_bytes!("../templates/base/data/resources.cfg"),
        )
        .file(
            "./data/secrets.cfg",
            include_bytes!("../templates/base/data/secrets.cfg"),
        )
        .file(
            "./data/server.cfg",
            include_bytes!("../templates/base/data/server.cfg"),
        )
}

/// Creates and returns a [`Template`] containing the files from
/// [`base_template`], and the following files:
///
/// - `./data/.gitignore`: Data gitignore for cache and environment files.
/// - `./data/env.cfg.template`: Template for the gitignored `./data/env.cfg`.
/// - `./data/secrets.cfg.template`: Template for the gitignored
///     `./data/secrets.cfg`.
/// - `./.gitignore`: Root gitignore for artifacts and txData.
pub fn git_template() -> Template {
    base_template()
        .file(
            "./data/.gitignore",
            include_bytes!("../templates/git/data/.gitignore"),
        )
        .file(
            "./data/env.cfg.template",
            include_bytes!("../templates/base/data/env.cfg"),
        )
        .file(
            "./data/secrets.cfg.template",
            include_bytes!("../templates/base/data/secrets.cfg"),
        )
        .file(
            "./.gitignore",
            include_bytes!("../templates/git/.gitignore"),
        )
}
