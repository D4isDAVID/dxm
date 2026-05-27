# dxm-init

> This crate is a part of [`dxm`], a CLI tool for managing FXServer artifacts &
> resources. See the [GitHub Repository] for more information.

Create basic project templates for FXServer. Provides default templates, and
supports creating your own or extending existing templates.

## Usage

This crate uses [`tracing`] for logging, and is built with [`tokio`] in mind. If
you require support for other runtimes, feel free to open an issue on the
[GitHub Repository].

To install the crate as a dependency, add the following to your `Cargo.toml`:

```toml
[dependencies]
dxm-init = "0.3"
```

## Example

```rs
#[tokio::main]
async fn main() {
    // create default base template
    let template = dxm_init::base_template()
        // add file on top of base template
        .file("./README.md", "# FXServer");

    // write template to ./server, creating ./server/README.md and base files
    template.write("./server").unwrap();
}
```

## Contributing

If you are interested in contributing to this project, read the
[Contributing guidelines] to learn more.

## License

This project's source code © 2024 David Malchin is licensed under the
**MIT License (MIT)** provided in the [LICENSE] file.

[`dxm`]: https://crates.io/crates/dxm
[`tracing`]: https://crates.io/crates/tracing
[`tokio`]: https://crates.io/crates/tokio
[contributing guidelines]: ./CONTRIBUTING.md
[license]: ./LICENSE
