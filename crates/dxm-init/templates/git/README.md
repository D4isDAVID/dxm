# dxm Default Server Template with Git

This is a simple template containing only the [`cfx-server-data`] resources.

The [`data/`] directory contains several CFG files for configuration:
- [`server.cfg`] - General server configuration.
- [`resources.cfg`] - Resource configuration.
- [`permissions.cfg`] - Permission configuration.
- [`env.cfg`] - Environment-specific configuration (ignored by Git).
- [`secrets.cfg`] - Credentials configuration (ignored by Git).

To start the server, enter your Cfx.re Server Registration Key in
[`secrets.cfg`], then use:

```sh
dxm start
```

[`cfx-server-data`]: https://github.com/citizenfx/cfx-server-data
[`data/`]: ./data
[`server.cfg`]: ./data/server.cfg
[`resources.cfg`]: ./data/resources.cfg
[`permissions.cfg`]: ./data/permissions.cfg
[`env.cfg`]: ./data/env.cfg
[`secrets.cfg`]: ./data/secrets.cfg
