//! Contains structures that represent resource data.

use std::{
    path::{Path, PathBuf, StripPrefixError},
    sync::LazyLock,
};

use serde::{Deserialize, Serialize};

use crate::util::relative_path;

static DEFAULT_CATEGORY: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("."));
static DEFAULT_NESTED_PATH: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("."));

/// Represents dxm-managed resource data.
#[derive(Serialize, Deserialize)]
pub struct Resource {
    /// The resource download URL.
    url: Option<String>,
    /// The resource category path.
    ///
    /// Default: `.`
    category: Option<PathBuf>,
    /// The path to the resource inside the download archive.
    ///
    /// Default: `.`
    nested_path: Option<PathBuf>,
}

impl Resource {
    /// Constructs and returns a new `Resource` instance.
    pub fn new<S, P, N>(url: S, category: P, nested_path: N) -> Self
    where
        S: Into<String>,
        P: Into<PathBuf>,
        N: Into<PathBuf>,
    {
        Self {
            url: Some(url.into()),
            category: Some(category.into()),
            nested_path: Some(nested_path.into()),
        }
    }

    /// Sets the resource's download URL.
    pub fn set_url<S>(&mut self, url: S)
    where
        S: Into<String>,
    {
        self.url = Some(url.into());
    }

    /// Returns the resource's download URL.
    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    /// Sets the resource's category path relative to the given server resources path.
    pub fn set_category<M, P>(
        &mut self,
        resources_path: M,
        category: P,
    ) -> Result<(), StripPrefixError>
    where
        M: AsRef<Path>,
        P: AsRef<Path>,
    {
        self.category = Some(relative_path(resources_path, category)?);

        Ok(())
    }

    /// Returns the resource's category path relative to the server resources path.
    fn relative_category(&self) -> &PathBuf {
        self.category.as_ref().unwrap_or(&*DEFAULT_CATEGORY)
    }

    /// Returns the resource's category path appended to the given server resources path.
    pub fn category<P>(&self, resources_path: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        resources_path.as_ref().join(self.relative_category())
    }

    /// Sets the resource's nested path inside its download archive.
    pub fn set_nested_path<P>(&mut self, nested_path: P) -> Result<(), StripPrefixError>
    where
        P: Into<PathBuf>,
    {
        self.category = Some(nested_path.into());

        Ok(())
    }

    /// Returns the resource's nested path inside its download archive.
    pub fn nested_path(&self) -> &PathBuf {
        self.nested_path.as_ref().unwrap_or(&*DEFAULT_NESTED_PATH)
    }

    /// Fills out information about the resource inside the given TOML document.
    pub fn fill_toml_item(&self, item: &mut toml_edit::Item) {
        let category = self.relative_category();
        if category == &*DEFAULT_CATEGORY {
            if item.get("category").is_some() {
                item["category"] = toml_edit::Item::None
            }
        } else {
            item["category"] = toml_edit::value(category.to_string_lossy().into_owned());
        }
        let nested_path = self.nested_path();
        if nested_path == &*DEFAULT_NESTED_PATH {
            if item.get("nested_path").is_some() {
                item["nested_path"] = toml_edit::Item::None
            }
        } else {
            item["nested_path"] = toml_edit::value(nested_path.to_string_lossy().into_owned());
        }
        if let Some(url) = self.url() {
            item["url"] = toml_edit::value(url);
        }
    }
}
