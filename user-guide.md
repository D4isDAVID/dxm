# User Guide

dxm is a CLI (Command-Line Interface) tool that empowers developers who want to
use Git to manage their servers. Basic knowledge of Git and the command line is
expected.

## Installing dxm

To install dxm, follow these steps:

1. Install the appropriate archive for your operating system from the latest
    [GitHub release].
2. Unpack the binary file from the downloaded archive.
3. Open a terminal in the directory containing the binary.
4. On Windows, run:
    ```
    .\dxm.exe self setup
    ```
    On Linux, run:
    ```
    ./dxm self setup
    ```

This will set up a `.dxm` directory in your home directory and add the dxm
binary to the environment `PATH`. After restarting the terminal, you will be
able to use `dxm` from any directory.

## Updating dxm

To update dxm, run:

```
dxm self update
```

This will automatically install the latest [GitHub release].

## Uninstalling dxm

To uninstall dxm, run:

```
dxm self uninstall
```

This removes `.dxm` from your home directory and the binary from the
environment `PATH`.

## Creating a dxm-Managed Server

Use `dxm init` to create a new dxm-managed server in the current working
directory, or `dxm new <path>` to create one at a specified path.

This will create an empty server with basic files, a Git repository, and the
latest stable artifacts.

### About the Manifest

dxm primarily uses a `dxm.toml` file in the root of your server to determine
where resources and artifacts are located. You may modify this file directly;
however, dxm provides commands to manage it for you.

dxm also uses a `dxm-lock.toml` file to track the exact download URLs for
resources and artifacts. This file should **not** be modified manually.

## Managing Artifacts

To install a new artifacts version, use:

```
dxm artifacts install [version] --path <path>
```

You may use specific version numbers such as `17000`, or aliases such as
`latest-jg`, `latest`, `recommended`, etc.

Both the version and path parameters are optional, so you may update the
artifacts version and path separately.

To update the artifacts, use:

```
dxm artifacts update
```

This command only works if the artifacts were installed using an alias such as
`latest-jg`, `latest`, `recommended`, etc.

## Managing Resources

To install a new resource, use:

```
dxm add <name> <url> -c <category>
```

This will install the resource into the specified directory and add it to the
manifest file.

To update a resource, use:

```
dxm update -r <resource>
```

This will update the given resource if it is installed.

To uninstall a resource, use:

```
dxm remove <name>
```

This will delete the resource if it is installed and remove it from the manifest
file.

## Installing and Updating a dxm-Managed Server

dxm creates `.gitignore` files for resources and artifacts to prevent them from
being pushed to your own repository.

To install all resources and artifacts at once after cloning your server
repository, use:

```
dxm install
```

To update all resources and artifacts at once, use:

```
dxm update --all
```

[github release]: https://github.com/D4isDAVID/dxm/releases
