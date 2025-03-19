# dxm

> [!NOTE]
> This project is still in development.

A manager for FXServer artifacts & resources.

- [Key Features](#key-features)
- [Installation](#installation)
- [Usage](#usage)

## Key Features

### Artifacts Management

dxm can automatically install and update new artifacts for you.
Supports JG Scripts' [Artifacts DB]!

### Planned

- Resource Management
- Recipes

## Installation

### Install

dxm includes a command that sets up a new `.dxm` directory in your user's home
directory, and adds it to the environment `PATH`.

1. Install the appropriate archive for your Operating System from the latest
  [GitHub release].
2. Unpack the binary from the downloaded archive.
3. Open a terminal in the directory of the installed binary.
4. Run `./dxm.exe self setup` on Windows or `./dxm self setup` on Linux.

After completing these steps, you will be able to use `dxm` from anywhere next
time you start up the terminal.

### Update

To update dxm, run `dxm self update`.
This will automatically install the latest [GitHub release].

### Uninstall

To uninstall dxm, simply run `dxm self uninstall`.
This will remove the dxm files, and remove them from the environment `PATH`.

## Usage

### The Manifest

dxm works using a `dxm.toml` file in the root of your server.

This file will contain all the data dxm needs to manage it.

### Creating a Server

You can use `dxm new [name]` or `dxm init` to create a new server with some
basic files, a git repository, and the latest artifacts.

You can then use `dxm run` to start the server.

### Managing Artifacts

You can use `dxm install [version]` to install a new artifacts version.
You may use either version numbers, or aliases such as `recommended`, `latest`,
and `latest-jg`. If you have a `dxm.toml` file with a valid version specified,
you may emit the version from the command completely.

Next time you want to update, you can use `dxm update` to download the latest
artifact applicable to the update channel in `dxm.toml`.

[github release]: https://github.com/D4isDAVID/dxm/releases
[artifacts db]: https://artifacts.jgscripts.com
