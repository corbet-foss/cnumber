# Focused validation and release preparation

Crow runs `.ci/ccid.toml` through the pinned native `ccid` command library.
The repository commands also work on a suitably provisioned build host.
Workstations stage committed source and inspect evidence; they do not compile it.
GitHub Actions has an equivalent manual public-source check route; publication
remains a separate operation.

## Manual GitHub Actions route

When hosted execution is available, free, and appropriate for the public inputs,
run `.github/workflows/ci.yml` with a comma-separated `checks` selection. It uses
the same repository commands on Linux with two Cargo build/test threads and a
15-minute job limit. Default selection: `metadata,rust`. Rust selects current
`stable`; the optional `rust-msrv` check reads the minimum from `Cargo.toml` and
runs separately. It does not fix the default compiler to that minimum.

JavaScript, package, and published-consumer selectors provision only their needed
hosted tools. For additional manager coverage, select `js-package` before
`js-pnpm`, `js-yarn`, or `js-bun` in the same run. The resulting artifact contains
package receipts plus `hosted-run.json`, with the source/archive/workflow identity,
locked input hashes, actual tools, and final check outcome. Hosted evidence does
not claim execution by ccid or native Windows/macOS coverage.

Checks requiring operator-supplied artifact manifests (`rust-dependencies` and,
where defined, `typst-preview`) stay on Crow. Private inputs and publishing
credentials never enter this hosted workflow. Public registry consumer checks
fail visibly when their exact dependency versions have not been published.

This manual workflow is not an expanded automatic provider mapping. Keep any
existing `.ci/providers.toml` pilot unchanged until equivalent hosted execution
and fallback identity are proved. Confirm unavailable execution before falling
back to Crow; a failed test is not provider unavailability. Publication remains
separate from these credential-free preparation/check jobs.

## Publishing prepared artifacts

The checked-in `.ci/publish.py` adapter uses the exact shared publisher resource
from the release workflow's verified ccid archive. It imports existing source
archives, preparation receipts and tested packages into one reviewed bundle;
producer identities remain distinct from later CI/publisher commits. See the
[shared import and publication contract](https://github.com/corbet-infra/ccid/blob/main/adapters/registry-publish.md).

The manual Crow `release` workflow reconciles existing bytes by default. Supply
`RELEASE_BUNDLE` (an existing worker file), `RELEASE_BUNDLE_SHA256`, and optionally
`RELEASE_CHANNELS`; select exactly one registry with `RELEASE_OPERATION=publish`
for an intended upload. `publish` with `all` is refused; status can inspect all.
Bundle inspection has no credentials. Registry status loads only the GitHub
release token; publication loads that token and the selected registry
token in its own guarded step. Missing tokens for other registries do
not block inspection, status or a selected upload. Python publication requires
`PYPI_TOKEN`; its absence is an explicit missing prerequisite, not a skipped
success. No build occurs with these credentials.

The manual GHA `release.yml` workflow consumes the same exact bundle from a public
release asset. It can reconcile all public channels and publish Cargo through
configured short-lived OIDC. Token-backed npm, JSR and PyPI publication stays on
Crow. The existing release/tag is verified before uploads. Persistent local and
immutable GitHub release journals prevent retries after an ambiguous upload,
including across providers; no registry restriction is automatically changed.

These adapters do not expand automatic provider mappings, create release tags,
or establish hosted/native execution proof. Existing check recipes and the
release publisher have separate explicit tool pins. Select `release-config` to
check adapter identities, Python/shell syntax and manual credential boundaries
without compiling products or accessing registries. Actionlint is required: Crow
uses the existing tool on PATH or in the Nix store; hosted setup provisions it
only for this selector. Evidence records its actual path and version. The
repository checker installs nothing and fails if the required tool is absent.

Select checks from changed inputs and missing evidence. A changed README does not
require another Rust build. Table/API changes need affected language vectors;
package metadata, exports and dependency changes need installed-package checks.
Use a broader selection when the impact is unknown. Existing results are reusable
only for matching source, dependency closure, tools, configuration and environment.

| Selector | Coverage |
| --- | --- |
| `metadata` | Version alignment and byte-for-byte generated assets |
| `release-config` | Manual release adapter contract and Python/shell syntax; required provisioned Actionlint |
| `fmt` | Rust formatting only |
| `rust` | Locked Rust tests including doctests, formatting, Clippy with denied warnings |
| `rust-msrv` | Minimum Rust tests; Crow requires provisioned 1.94.0, while hosted setup reads `Cargo.toml` |
| `javascript` | Strict TypeScript and shared source vectors |
| `rust-package` | Build and verify the registry-resolved Cargo package |
| `js-package` | Pack once; installed npm/Node/Bun imports and declarations, browser module, JSR dry-run |
| `jsr-package` | Validate exact JSR sources and export a deterministic publication archive without rebuilding npm/Rust |
| `js-pnpm`, `js-yarn`, `js-bun` | Selected additional manager consuming the same verified npm tarball |
| `python-package` | Wheel/sdist, installed-wheel vectors, metadata and JSON CLI consumers |
| `typst-package` | Deterministic archive and actual installed Typst import |
| `typst-preview` | Exact prepared Universe archive and README examples importing `@preview/cnumber:0.1.0` |
| `rust-dependencies` | Pre-publication source tests using checksum-verified sibling crates in private scratch |
| `published-npm`, `published-jsr`, `published-python` | Consumers of the exact version on the selected live registry |

For example, use the existing dispatcher:

```sh
crow-ci plan --repo . --workflow ccid --var CHECKS=metadata,rust
crow-ci run --repo . --workflow ccid --var CHECKS=metadata,rust
```

Before an initial release, choose all affected source and distribution checks:

```sh
crow-ci run --repo . --workflow ccid \
  --var CHECKS=metadata,rust,javascript,rust-package,js-package,python-package,typst-package \
  --var ARTIFACT_ROOT=/absolute/persistent/release-artifacts
```

`ARTIFACT_ROOT` is a worker path supplied by the operator. Each package selector
exports immutable files under `<root>/<package>/<commit>/`, plus its own JSON
receipt, `SOURCE_COMMIT`, and an aggregate `SHA256SUMS`. Additional manager checks
require `js-package` evidence for that exact commit. They never rebuild the tarball.
Use them when manager/export/packaging behavior changed; do not repeat all managers
for every source edit. Publication uploads the verified files separately using
registry credentials outside these verification jobs.

For a JSR metadata correction, select `jsr-package` with `ARTIFACT_ROOT`. It copies
the root README and both license texts, validates only the publication inputs with
`deno publish --dry-run --allow-dirty`, and exports `cnumber-0.1.0-jsr.tar.gz` with a
`jsr-package.json` receipt. Upload that exact archive after checking its receipt
and hash. JSR requires one SPDX identifier, so its metadata declares plain
LGPL-3.0-only; the linking-exception text ships in-bundle and the js README
records the mapping (same treatment as the other LGPL-line packages).

For unpublished sibling dependencies, supply `DEPENDENCY_MANIFEST` and
`DEPENDENCY_MANIFEST_SHA256`. The manifest is an exact JSON document:

```json
{
  "schema": 1,
  "artifacts": [
    {
      "kind": "wheel",
      "name": "cnumber",
      "path": "/absolute/path/to/verified/cnumber.whl",
      "sha256": "the actual 64-character lowercase SHA-256"
    }
  ]
}
```

Kinds are `wheel`, `crate`, or `npm`; each consuming check selects its own kind.
Python installs the specified sibling wheels alongside its own wheel. The
`rust-dependencies` selector copies source and verified sibling crates to owned
scratch, uses temporary Cargo patches and a scratch-only lock, and runs the tests.
This is integration evidence, not registry package verification. `rust-package`
always uses normal registry resolution and requires published dependencies.
Neither the source tree nor the released manifest acquires local path patches.

For a prepared Universe submission, pass one `DEPENDENCY_MANIFEST` artifact of
kind `typst-preview` and the package name, with its exact tarball path and SHA-256.
Select `typst-preview` to unpack that candidate into an isolated preview package
directory and compile its README examples with the pinned Typst binding. The
candidate archive and `typst-preview.json` receipt are exported unchanged. This
check verifies the submission artifact; upstream review controls publication.

The shared runner bounds concurrency to two build/test threads by default and
selected checks to 900 seconds; `CI_JOBS`, `CI_TEST_THREADS`, and `CI_TIMEOUT` can
supply a concrete different budget. It inherits dedicated package caches and
namespaces Cargo targets by canonical repository identity. It verifies source and
tool digests, preserves unchanged source freshness, supervises timeout/cancellation,
and never clears caches or installs system tools. Commands that generate source or
install dependencies run in owned scratch copies, preserving the verified input
tree and its reusable Rust timestamps. The dispatcher checks host
headroom, attaches to identical active work, and refuses a repeated completed
request unless an operator explicitly requests a diagnosed rerun.

Inspect existing runs before dispatch. After a failed Python check, select Python
again after correcting its inputs; a passed unrelated Rust check remains evidence.
Keep exact check receipts and durations when assessing whether wider validation is
needed. Linux results do not establish native Windows/macOS behavior or another
Rust/Node/Python version. Unavailable checks remain explicitly unverified.
