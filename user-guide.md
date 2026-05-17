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
    ```pwsh
    .\dxm.exe self setup
    ```
    On Linux, run:
    ```sh
    ./dxm self setup
    ```

This will set up a `.dxm` directory in your home directory and add the dxm
binary to the environment `PATH`. After restarting the terminal, you will be
able to use `dxm` from any directory.

## Updating dxm

To update dxm, run:

```sh
dxm self update
```

This will automatically install the latest [GitHub release].

## Uninstalling dxm

To uninstall dxm, run:

```sh
dxm self uninstall
```

This removes `.dxm` from your home directory and the binary from the
environment `PATH`.

## The Manifest File

dxm primarily uses a `dxm.toml` file in the root of your server to determine
where resources and artifacts are located. You may modify this file directly;
however, dxm mostly provides commands to manage it for you.

dxm also uses a `dxm-lock.toml` file to track the exact download URLs for
resources and artifacts. This file should **not** be modified manually.

## Creating a dxm-Managed Server

Use `dxm init` to create a new dxm-managed server in the current working
directory, or `dxm new <path>` to create one at a specified path.

This will create an empty server with basic files, a Git repository, and the
latest stable artifacts.

## Migrating a Server to dxm

To migrate a server that doesn't use dxm to use it, you can simply create a
minimal `dxm.toml` file in the root of the server:

```toml
[artifact]
path = "path/to/artifacts"
version = "latest-jg"

[server]
data = "path/to/data" # the directory that contains the resources directory
```

## Managing Artifacts

To install a new artifacts version, use:

```
dxm artifacts install [version] [--path <path>]
```

You may use specific version numbers such as `17000`, or aliases such as
`latest-jg`, `latest`, `recommended`, etc.

Both the version and path parameters are optional, so you may update the
artifacts version and path separately.

To update artifacts, use:

```sh
dxm update --artifacts
```

> [!NOTE]
> Using specific versions such as `17000` will lock your artifacts to that
> version until you install a different one. This will prevent `dxm update` from
> changing the artifacts version.

### Examples

```sh
dxm artifacts install latest-jg --path artifacts
```

```sh
dxm artifacts install 29199
```

## Managing Resources

To install a new resource, use:

```
dxm add <name> <url> [-c <category>]
```

This will install the resource into the specified directory and add it to the
manifest file.

If you want to install a specific resource from a set of resources such as
[`cfx-server-data`], use:

```
dxm add <name> <url> -n <nested path> [-c <category>]
```

This will download the resource from the specified URL, but install the
specified nested directory inside of it.

To update resources, use:

```
dxm update --resources [<resources>...]
```

You can specify which resources you want to update, or not specify any to update
all resources.

> [!NOTE]
> Installing resources with a specific commit or release will lock you to that
> version until you install a different one. This will prevent `dxm update` from
> changing the resource version.

To uninstall a resource, use:

```
dxm remove <resource>
```

### Examples

```sh
dxm add loadscreen https://github.com/D4isDAVID/loadscreen -c "[vendor]"
```

```sh
dxm add "[system]" https://github.com/citizenfx/cfx-server-data -n "[system]"
```

```sh
dxm update --resources loadscreen ox_lib
```

```sh
dxm update --resources
```

```sh
dxm remove loadscreen
```

### Valid Resource URLs

- Direct Download URLs - will try to install as a zip.
- GitHub Repository - will use the attached version (release/tag/branch/commit)
  or the default branch.
  - Regular: `https://github.com/D4isDAVID/loadscreen`
  - `.git` suffix: `https://github.com/D4isDAVID/loadscreen.git`
  - Archive: `https://github.com/D4isDAVID/loadscreen/archive/refs/heads/main.zip`
  - Commit: `https://github.com/D4isDAVID/loadscreen/commit/6247c4d0c6b8c3b31639f7d9e0cb1d1fe3e24068`
  - Tree/blob: `https://github.com/D4isDAVID/loadscreen/tree/v2.2.1`
  - Release: `https://github.com/D4isDAVID/loadscreen/releases/tag/v2.2.1`

## Patching Resources

> [!NOTE]
> Resource patching requires `git` binary to be installed and available in the
> environment PATH.

Some third-party resources require editing their files for configuration, and
sometimes simply tweaking some functionality is useful. dxm provides an easy way
to do so with persistent patches.

Using this feature, you only commit the manifest file and the patch file
specific to your changes, rather than the entire resource.

To persistently patch a resource across installs, first use:

```
dxm patch prepare <resource>
```

This will create a temporary Git repository for that resource, where you can
make your changes.

After making the changes you need, **do not** commit the changes directly.
Instead, use:

```
dxm patch commit <resource>
```

This will create a patch file for the resource, and add it to the manifest
file.

Next time your repository is cloned, the patch containing your changeswill be
automatically applied.

> [!NOTE]
> After updating resources which have patches, you may need to recreate the
> patch in case of conflicts.

To remove a resource patch, use:

```
dxm patch remove <resource>
```

## Managing Monitor

To install a third-party txAdmin replacement, use:

```
dxm monitor install <url>
```

This will replace the `monitor` system resource in the artifacts and add it to
the manifest file.

To update the third-party txAdmin replacement, use:

```sh
dxm update --monitor
```

To remove the third-party txAdmin replacement, use:

```sh
dxm monitor remove
```

### Examples

```sh
dxm monitor install https://github.com/SomeAussieGaymer/fxPanel
```

## Installing and Updating a dxm-Managed Server

dxm creates `.gitignore` files for resources and artifacts to prevent them from
being pushed to your own repository.

To install all resources and artifacts at once after cloning your server
repository, use:

```sh
dxm install
```

To update all resources, artifacts and third-party txAdmin replacement at once,
use:

```sh
dxm update
```

## Starting a dxm-Managed Server

To start FXServer via dxm, use:

```sh
dxm start
```

This will start FXServer inside of the server's data directory, defined in the
manifest file.

You can also specify environment variables and server arguments:

```sh
dxm start --env TXHOST_DATA_PATH ./my/path/to/txData --env TXHOST_GAME_NAME fivem -- +exec server.cfg
```

This will start FXServer with the environment variable `TXHOST_DATA_PATH` set to
`./my/path/to/txData`, `TXHOST_GAME_NAME` set to `fivem`, and the startup
arguments `+exec server.cfg`.

### Profiles

To specify commonly used environment variables and startup arguments, you can
specify profiles in your `dxm.toml`:

```toml
[profiles.<PROFILE NAME>]
server_args = ["+exec", "server.cfg"]
env_vars.TXHOST_DATA_PATH = "./my/path/to/txData"
env_vars.TXHOST_GAME_NAME = "fivem"
```

Then run:

```
dxm start [profile]
```

When not specifying a profile, the `default` profile is used.

### Examples

```toml
[profiles.default]
server_args = ["+exec", "server.cfg"]
env_vars.TXHOST_DATA_PATH = "./txData-fivem"
env_vars.TXHOST_GAME_NAME = "fivem"

[profiles.redm]
env_vars.TXHOST_DATA_PATH = "./txData-redm"
env_vars.TXHOST_GAME_NAME = "redm"

[profiles.no-tx]
server_args = ["+exec", "server.cfg"]
```

[github release]: https://github.com/D4isDAVID/dxm/releases
[`cfx-server-data`]: https://github.com/citizenfx/cfx-server-data
