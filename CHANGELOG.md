# Changelog

All notable changes to `cnumber` are documented here. The project follows
Semantic Versioning.

## 0.1.0 - 2026-09-18

- `format_money` takes a `display` argument: `"symbol"` (`Fr. 1'234.56`,
  locale-placed `€`) or `"code"` (`CHF 100'000`, `EUR 1.234,56`).
- Initial release: locale-correct number and money formatting for eleven
  correspondence locales (`de`, `de-at`, `de-ch`, `de-li`, `en`, `en-gb`,
  `en-us`, `fr`, `fr-ch`, `it`, `it-ch`) in Rust, JavaScript, Python, and
  Typst, driven by shared conformance vectors.
- CLDR decimal/group separators plus the Swiss amtlich running-text
  (`1'234,56`) and money (`Fr. 1'234.56`) rules; string-only canonical
  inputs, never binary floats.
