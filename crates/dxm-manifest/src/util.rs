use std::path::{Path, PathBuf, StripPrefixError};

use toml_edit::{InlineTable, Item, Value};

/// Strips the given base from the given path to create a relative path.
///
/// For example, given a `/test/dir` base and a `/test/dir/nested` path, the
/// function returns `nested`.
pub fn relative_path<B, P>(base: B, path: P) -> Result<PathBuf, StripPrefixError>
where
    B: AsRef<Path>,
    P: AsRef<Path>,
{
    path.as_ref().strip_prefix(base).map(ToOwned::to_owned)
}

/// Uses the given function to fill out information inside the document table
/// with the given key. If a table with the given key does not exist, creates it
/// first.
pub fn add_and_fill_missing_table<S>(item: &mut Item, key: S, fill: impl Fn(&mut Item))
where
    S: AsRef<str>,
{
    let key = key.as_ref();

    if item.get(key).is_none() {
        item[key] = toml_edit::table();
    }

    fill(&mut item[key]);
}

/// Uses the given function to fill out information inside the document table
/// with the given key. If a table with the given key does not exist, creates it
/// first.
pub fn add_and_fill_inline_table<S>(item: &mut Item, key: S, fill: impl Fn(&mut Item))
where
    S: AsRef<str>,
{
    let key = key.as_ref();

    if item.get(key).is_none() {
        item[key] = Item::Value(Value::InlineTable(InlineTable::new()));
    }

    fill(&mut item[key]);
}
