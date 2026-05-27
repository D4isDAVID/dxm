//! Create basic project templates for FXServer. Provides default templates, and
//! supports creating your own or extending existing templates.
//!
//! ## Example
//!
//! ```no_run
//! #[tokio::main]
//! async fn main() {
//!     // create default base template
//!     let template = dxm_init::base_template()
//!         // add file on top of base template
//!         .file("./README.md", "# FXServer\n");
//!
//!     // write template to ./server, creating ./server/README.md and base files
//!     template.write("./server").await.unwrap();
//! }
//! ```

mod defaults;
mod template;

pub use defaults::*;
pub use template::*;
