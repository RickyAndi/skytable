<html>
<div align="center">
<img src="assets/logo.jpg" height=64 width=64>
<h1>Skytable</h1><h3>Your next NoSQL database</h3>

![GitHub Workflow Status](<https://img.shields.io/github/workflow/status/skybasedb/skybase/Test%20(push)?style=flat-square>) ![Development](https://img.shields.io/badge/development-regular-32CD32?style=flat-square) ![GitHub release (latest SemVer including pre-releases)](https://img.shields.io/github/v/release/skybasedb/skybase?include_prereleases&sort=semver&style=flat-square)
[![Docs](https://img.shields.io/badge/readthedocs-here-blueviolet?style=flat-square)](https://docs.skytable.io) [![Contribute Now](https://img.shields.io/badge/%F0%9F%8C%9Fcontribute-now-a94064?style=flat-square)](https://ohsayan.github.io/skythanks) [![Discord](https://img.shields.io/badge/talk-on%20discord-7289DA?logo=discord&style=flat-square)](https://discord.gg/QptWFdx)

</div>
</html>

## What is Skytable?

Skytable is a free and open-source NoSQL database that aims to provide flexible data modeling at
scale. For us simplicity, performance and flexibility are our guiding design principles.
We were previously known as TerrabaseDB or Skybase and are nicknamed Sky, SDB or STable by the community.

Features like keyspaces, tables, data types, authn+authz, snapshots and more are ready for you to use while we're working on [several new data models and features](https://github.com/skytable/skytable/issues/203). Skytable's key/value store is performant, secure and ready for you to deploy.

## Getting started 🚀

1. Download a bundle for your platform from [here ⬇️ ](https://github.com/skytable/skytable/releases)
2. Unzip the bundle
3. Make the files executable (run `chmod +x skyd skysh` on \*nix systems)
4. First run `skyd` to start the database server and then run `skysh` to start the interactive shell
5. Run commands like: `SET foo bar` , `GET bar` , `UPDATE cat mitten` or `DEL proprietary` on `skysh`!

You can learn more about installation [here](https://docs.skytable.io/getting-started)

## Features

- **Insanely fast**: Scale to millions of queries per second per node. See [benchmarks here](https://github.com/ohsayan/sky-benches).
- **Multiple keyspaces/tables**: Seamlessly integrates with actions to provide a SQL-like experience
- **Key/value store**: `GET` , `SET` , `UPDATE` and [all that stuff](https://docs.skytable.io/all-actions). With the `str` and `binstr` types.
- **Authn/Authz**: Simple and secure authentication/authorization
- **Volatile tables**: For all the caching you need
- **Snapshots**: Automated (and tunable) snapshots for stress-free backups
- **Secure**: Secure connections are built into Skytable with SSL/TLS
- **Multithreaded**: Designed to exploit all CPU cores
- **Resource friendly**: The database server doesn't need more than 1MB to run
- **Convenient**: Without the setup hassle and system-specific dependencies

**🛣️ There's a lot more coming! View our [roadmap](https://github.com/skytable/skytable/issues/203)**

## Clients 🔌

The project currently maintains an official [Rust driver](https://github.com/skytable/client-rust) and we have plans
to support more languages along the way!
We also maintain a list of [community supported drivers here](https://github.com/skytable/skytable/wiki/Drivers).

If you want to use a different language, for now, you'll just need to implement the simple and performant [Skyhash Protocol](https://docs.skytable.io/protocol/skyhash).

## Community 👐

A project which is powered by the community believes in the power of community! If you get stuck anywhere - here are your options!

<html>
<a href="https://gitter.im/skytable/community"><img src="https://img.shields.io/badge/chat%20on-gitter-ed1965?logo=gitter&style=flat-square"></img>
</a><a href="https://discord.gg/QptWFdx"><img src="https://img.shields.io/badge/talk-on%20discord-7289DA?logo=discord&style=flat-square"></img></a>
</html>

## Platforms 💻

![Linux supported](https://img.shields.io/badge/Linux%2032--bit%2F64--bit-Supported%20✓-%23228B22?style=flat-square&logo=linux) ![macOS supported](https://img.shields.io/badge/macOS%20x86__64%2Farm64-supported%20✓-228B22?style=flat-square&logo=apple) ![Windows supported](https://img.shields.io/badge/Windows%2032--bit%2F64--bit-supported%20✓-228B22?style=flat-square&logo=windows)

## Versioning

This project strictly follows semver, however, since this project is currently in the development phase (0.x.y), the API may change unpredictably

## Contributing

[![Contribute Now](https://img.shields.io/badge/%F0%9F%8C%9Fcontribute-now-a94064?style=for-the-badge)](https://ohsayan.github.io/skythanks)

You are welcome to contribute to Skytable! Beginner friendly issues are marked with the [<img src=https://img.shields.io/badge/L--easy-C71585>](https://github.com/skytable/skytable/labels/L-easy) label. Read the guide [here](./CONTRIBUTING.md).

## Contributors

You can see a full list of contributors [here](https://ohsayan.github.io/skythanks)

## License

This project is licensed under the [AGPL-3.0 License](./LICENSE).
