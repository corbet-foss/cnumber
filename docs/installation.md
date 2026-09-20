# Installation and distribution

The main branch ships 0.1.0 under LGPL-3.0-only WITH LGPL-3.0-linking-exception, published to registries.

This guide describes version 0.1.0. Check the linked registry or release
for availability; a source manifest alone does not establish publication.

## JavaScript and Rust

Use Cargo for Rust and npm, pnpm, Yarn, or Bun for JavaScript. These JavaScript
package managers share the npm registry; each consumes the same package.
Deno can use `npm:@corbet-foss/cnumber`. The browser export bundles runtime
dependencies and needs no import map. The declared Rust minimum is 1.94. Release checks use the worker's current stable compiler; a separate minimum-version check is required to
verify that lower bound.

## Python and the command line

The pure Python package requires Python 3.10+ and will be published on
[PyPI](https://pypi.org/project/cnumber/) once 0.1.0 is released. Install it
with pip or uv in your Python environment:

```sh
python -m pip install cnumber==0.1.0
```

```sh
uv pip install cnumber==0.1.0
```

For a uv project, `uv add cnumber==0.1.0` adds the package to your dependencies.

After installation, functions can be called from Python or through either CLI
entrypoint:

```sh
cnumber --help
python -m cnumber --help
```

The CLI takes a function name and a JSON array of positional arguments or an
object of keyword arguments. Use `-` to read arguments from stdin. It writes
JSON to stdout; errors use stderr and a nonzero exit status.

For an isolated CLI environment, use `pipx install cnumber==0.1.0` or
`uv tool install cnumber==0.1.0`.

The wheel contains no native extensions and is platform independent. Release
evidence records the Python version and operating system actually exercised.
Verified wheels and source distributions are attached to the
[GitHub release](https://github.com/corbet-labs/cnumber/releases/tag/v0.1.0)
once published.

## JSR

The JSR package will be
[`@corbet-foss/cnumber`](https://jsr.io/@corbet-foss/cnumber):

```sh
deno add jsr:@corbet-foss/cnumber@0.1.0
```

## Typst

Download `cnumber-0.1.0-typst.tar.gz` from the matching GitHub release and
extract its contents into `typst/packages/local/cnumber/0.1.0` under your
[Typst data directory](https://github.com/typst/packages#local-packages):

| System | Data directory |
| --- | --- |
| Linux | `$XDG_DATA_HOME`, or `~/.local/share` |
| macOS | `~/Library/Application Support` |
| Windows | `%APPDATA%` |

```typst
#import "@local/cnumber:0.1.0": *
```

The archive includes its manifest, tables, source, and licenses. CI compiles
an example against a fresh installation of the actual archive with Typst 0.15.
For the Typst web app, upload the extracted files and import the entrypoint
listed in `typst.toml` by its relative path.

## System package managers

These are language libraries, with a portable Python CLI. Homebrew, APT, RPM,
WinGet, Chocolatey, and Scoop are not additional registries for importing a
Rust crate or JavaScript module. Use their Python or Node runtime and the
language package manager above. No native system-package listing is claimed.
