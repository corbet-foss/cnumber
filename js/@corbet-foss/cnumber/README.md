# cnumber

**Reproducible number formatting for correspondence.**

Format a decimal number without a binary float, operating-system locale,
or network dependency. The same inputs produce the same text in Rust,
JavaScript, Python, and Typst.

```js
import { formatNumber } from '@corbet-foss/cnumber';

formatNumber('de-ch', '1234567.89', true, 'amtlich');
// 1'234'567,89
```

## Install

| Environment | Command |
| --- | --- |
| Rust / Cargo | `cargo add cnumber` |
| Python / pip | `python -m pip install cnumber` |
| Python / uv | `uv add cnumber` |
| Node.js / npm | `npm install @corbet-foss/cnumber` |
| pnpm | `pnpm add @corbet-foss/cnumber` |
| Yarn | `yarn add @corbet-foss/cnumber` |
| Bun | `bun add @corbet-foss/cnumber` |
| Deno | `deno add npm:@corbet-foss/cnumber` |

The 0.1.0 release line is published to registries; the
commands above resolve once it is. Python requires 3.10+, Node.js 20+.

## Canonical-input contract

Inputs are canonical decimal strings, never binary floats (so `0.1 + 0.2`
style artefacts cannot leak into correspondence):

- Grammar: `-?digits(.digits)?` with a dot decimal separator, e.g.
  `"1234567.89"`, `"42"`, `"-0.5"`.
- Rejected → `None`/`null`/`none`: `""`, `"12,34"`, `"abc"`,
  `"1'234"` (pre-grouped), `"1,234.56"`, `" 12"`, `"+12"`,
  `"--12"`, `".5"`, `"5."`, `"1.2.3"`.
- Years (`"2026"`) and postal codes must pass `grouping=false`;
  the library does **not** auto-detect them. Four-digit inputs stay
  ungrouped even with `grouping=true`.

## CLDR vs amtlich

Base separators come from Unicode CLDR (`cldr-json`, `main` as of
2026-09-17, `numbers.json` `symbols-numberSystem-latn`). Swiss amtlich
rules come from the Bundeskanzlei *Schreibweisungen*, chapter 5, which
has no CLDR equivalent:

| Context | `de-ch` / `de-li` | Example (`1234.56`) |
| --- | --- | --- |
| CLDR data (`style="cldr"`) | dot decimal, `'` groups | `1'234.56` (money-style dot) |
| Amtlich running text (`style="amtlich"`) | comma decimal, `'` groups | `1'234,56` |
| Money (`format_money`, CHF) | dot decimal always | `Fr. 1'234.56` (`display="symbol"`) or `CHF 1'234.56` (`display="code"`) |

`format_number` groups integer parts of five or more digits; `format_money`
groups from four digits (standard thousands). `style="amtlich"` is
identical to `"cldr"` outside `de-ch`/`de-li`. Unknown styles return
`None`/`null`/`none`.

Money covers exactly two currencies: `CHF` (`Fr. ` prefix with
Swiss grouping and dot decimal, in every locale; `display="code"` gives
`CHF ` prefix instead) and `EUR` (locale separators; `€` prefixed for
`en*`, suffixed with a no-break space for `de*`/`it*`, suffixed with a
narrow no-break space for `fr*`; `display="code"` gives `EUR ` prefix).
Any other currency code — including lowercase `"chf"` — returns `None`,
as does any `display` other than `"symbol"`/`"code"`.

## Rust

```rust
use cnumber::{format_money, format_number};

assert_eq!(
    format_number("de-ch", "1234567.89", true, "amtlich").as_deref(),
    Some("1'234'567,89")
);
assert_eq!(
    format_money("de-ch", "1234.56", "CHF", "symbol").as_deref(),
    Some("Fr. 1'234.56")
);
assert_eq!(
    format_money("de-ch", "100000", "CHF", "code").as_deref(),
    Some("CHF 100'000")
);
```

## Python

```python
from cnumber import format_number

assert format_number("de-ch", "1234567.89", True, "amtlich") == "1'234'567,89"
```

## API

| JavaScript / Python or Rust | Example |
| --- | --- |
| `symbols` | `de` → `{decimal: ",", group: "."}` |
| `formatNumber` / `format_number` | `de` `"1234567.89"` → `1.234.567,89` |
| `formatMoney` / `format_money` | `de-ch` `"1234.56"` CHF → `Fr. 1'234.56` (`symbol`) or `CHF 1'234.56` (`code`) |
| `isSupported` / `is_supported` | Locale support check |
| `availableLocales` / `available_locales` | Sorted lowercase locale IDs |

Eleven number-table entries: `de`, `de-at`, `de-ch`, `de-li`, `en`,
`en-gb`, `en-us`, `fr`, `fr-ch`, `it`, `it-ch`. Matching is
case-insensitive; other locales fall back through the base language to
English. Non-canonical inputs return `null`/`None`. Other family
libraries have different locale coverage.

## Correspondence family

| Library | Responsibility |
| --- | --- |
| [cletter](https://github.com/corbet-foss/cletter) | Compose the correspondence helpers |
| [cgreet](https://github.com/corbet-foss/cgreet) | German salutations and titles |
| [cfarewell](https://github.com/corbet-foss/cfarewell) | Locale-specific closings |
| [cdate](https://github.com/corbet-foss/cdate) | Calendar-date formatting |
| [cnumber](https://github.com/corbet-foss/cnumber) | Number and money formatting |
| [cink](https://github.com/corbet-foss/cink) | Handwritten signature images |

## Development

Behavior is defined by [the locale tables](https://github.com/corbet-foss/cnumber/tree/main/tables)
and [shared conformance vectors](https://github.com/corbet-foss/cnumber/tree/main/tests/vectors).
Rust, JavaScript, and Python run the same vectors. Selected CI checks exercise
installed JavaScript tarballs, Python wheels and command-line entrypoints, and
Typst packages. Release validation records the actual runtime and platform;
Linux results do not establish native Windows or macOS coverage.
All ports forbid unsafe code in their own source.

See [the release guide](https://github.com/corbet-foss/cnumber/blob/main/docs/releasing.md)
for generation, verification, and publication commands.

## License

Copyright 2026 Julian Y. Richard Corbet. The 0.1.0 release line is licensed
under [LGPL-3.0-only](https://github.com/corbet-foss/cnumber/blob/main/LICENSES/LGPL-3.0-only.txt)
[WITH LGPL-3.0-linking-exception](https://github.com/corbet-foss/cnumber/blob/main/LICENSES/LGPL-3.0-only%20WITH%20LGPL-3.0-linking-exception.txt),
with the incorporated [GPL version 3](https://github.com/corbet-foss/cnumber/blob/main/LICENSES/GPL-3.0-only.txt).
Combined works may link statically or dynamically without relinking duties;
library modifications stay LGPL. Applications can use a different license
subject to the LGPL's conditions.
Previously released and already prepared distributions retain their original
grants. The installation examples above refer to those available releases;
0.1.0 is published to registries.

See the [licensing notes](https://github.com/corbet-foss/cnumber/blob/main/LICENSE.md) for distribution conditions and retained notices.
Contributions are subject to the [Contributor License Agreement](CLA.md).
