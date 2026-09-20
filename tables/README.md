# tables — canonical number-format data

One concern only: which decimal separator and thousands-grouping
separator each correspondence locale uses. `tests/vectors/` is the
executable form of this contract.

## Upstream sources

- CLDR decimal/group symbols: `unicode-org/cldr-json` `main` as of
  2026-09-17, file `numbers.json`, key `symbols-numberSystem-latn` per
  locale. Spot-verified that day: `de-CH` decimal `.` group `'` (U+0027),
  `de` decimal `,` group `.`, `en` decimal `.` group `,`.
- Swiss amtlich rules: Bundeskanzlei *Schreibweisungen*, chapter 5 —
  running text uses a comma decimal (`1'234,56`), money amounts use a
  dot decimal (`Fr. 1'234.56`), thousands are grouped with `'` only
  from five-digit numbers upward, and years/postal codes are never
  grouped.

CLDR knows nothing about the amtlich running-text/money split, so the
table carries one explicit marker on top of CLDR data:

- `amtlich_text_decimal: ","` on `de-ch`/`de-li`: the decimal
  separator used when `format_number` runs with `style="amtlich"`.
  Every other locale formats identically in both styles.
- Swiss locales (`de-ch`, `de-li`, `fr-ch`, `it-ch`) use the ASCII
  apostrophe U+0027 as the group separator (CLDR reference value).
- `fr` uses the CLDR narrow no-break space U+202F as group separator,
  stored as the `\u202f` escape.

## Schema (`numbers.json`)

| Key | Meaning |
|-----|---------|
| `locales` | Lowercase BCP 47 code → `{decimal, group, amtlich_text_decimal?}`. Separators are single-character strings. |
| `supported` | Every BCP 47 code the resolver accepts. |
| `fallback` | Locale used when neither the exact code nor its base language is present. Always `en`. |

## Resolution (all languages)

1. Exact code (`de-ch`) wins.
2. Otherwise the base language (`de-ch` → `de`).
3. Otherwise `fallback` (`en`).
4. Non-canonical number inputs yield no output
   (`None`/`null`/`none`), never a best effort. Canonical means
   `-?digits(.digits)?` with a dot decimal separator.

Locale IDs are lowercase in tables, returned locale lists, examples and paths.
Lookups accept mixed-case input and resolve the lowercase exact code, then base
language, then fallback. `de-li`, `fr-ch` and `it-ch` have explicit entries.
