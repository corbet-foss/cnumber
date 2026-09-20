//! Deterministic locale-correct number formatting for correspondence.
//!
//! Thousands grouping and decimal separators per locale live as data in
//! `tables/numbers.json` (see `tables/README.md` for the schema, the CLDR
//! source pin, and the resolution rule); `tests/vectors/*.json` is the
//! executable contract every language port runs. The Typst module in
//! `typst/` derives from the same table.
//!
//! Inputs are canonical decimal strings (`-?digits(.digits)?` with a dot,
//! e.g. `"1234567.89"`), never binary floats: the same input always yields
//! the same output with no rounding, no I/O, and no operating-system locale.
//! Anything else (empty strings, comma decimals, pre-grouped input such as
//! `"1'234"`, whitespace, signs other than a single leading `-`) returns
//! `None`. Grouping separators are never auto-detected.
//!
//! The `amtlich` style implements the Bundeskanzlei *Schreibweisungen*
//! (chapter 5) on top of CLDR data: only `de-ch`/`de-li` change, using a
//! comma decimal in running text, while money amounts keep the dot.

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

#[derive(serde::Deserialize)]
struct LocaleEntry {
    decimal: String,
    group: String,
    #[serde(default)]
    amtlich_text_decimal: Option<String>,
}

#[derive(serde::Deserialize)]
struct NumbersFile {
    locales: HashMap<String, LocaleEntry>,
    supported: HashSet<String>,
    fallback: String,
}

static NUMBERS: LazyLock<NumbersFile> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../tables/numbers.json"))
        .expect("tables/numbers.json is valid")
});

fn base_language(code: &str) -> &str {
    code.split('-').next().unwrap_or(code)
}

/// Resolve a lowercase table key: case-insensitive exact code, base language,
/// then English fallback. All stored and returned locale IDs are lowercase.
fn resolve_key(locale: &str) -> &'static str {
    let lower = locale.to_ascii_lowercase();
    NUMBERS
        .locales
        .get_key_value(lower.as_str())
        .or_else(|| NUMBERS.locales.get_key_value(base_language(&lower)))
        .map_or(NUMBERS.fallback.as_str(), |(key, _)| key.as_str())
}

/// Decimal and grouping separators for a locale. Unknown locales fall back
/// through the base language to English; this always succeeds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct Symbols {
    /// Single-character decimal separator (`","` for `de`, `"."` for `en`).
    pub decimal: &'static str,
    /// Single-character thousands-grouping separator (`"."` for `de`,
    /// `"'"` for `de-ch`, narrow no-break space for `fr`).
    pub group: &'static str,
}

/// Separators for a locale: `de` → decimal `,`, group `.`;
/// `de-ch` → decimal `.`, group `'`. Unknown locales fall back to English.
#[must_use]
pub fn symbols(locale: &str) -> Symbols {
    let entry = &NUMBERS.locales[resolve_key(locale)];
    Symbols {
        decimal: entry.decimal.as_str(),
        group: entry.group.as_str(),
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Parsed<'a> {
    negative: bool,
    int: &'a str,
    frac: Option<&'a str>,
}

/// Split a canonical decimal string (`-?digits(.digits)?`). Anything else —
/// empty input, comma decimals, pre-grouped digits, whitespace, a second
/// sign or dot — is rejected so callers notice malformed data loudly.
fn parse_canonical(canonical: &str) -> Option<Parsed<'_>> {
    let (negative, rest) = match canonical.strip_prefix('-') {
        Some(tail) => (true, tail),
        None => (false, canonical),
    };
    if rest.is_empty() {
        return None;
    }
    let (int, frac) = match rest.split_once('.') {
        Some((head, tail)) => (head, Some(tail)),
        None => (rest, None),
    };
    if int.is_empty() || !int.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    if let Some(digits) = frac
        && (digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()))
    {
        return None;
    }
    Some(Parsed {
        negative,
        int,
        frac,
    })
}

/// Group an integer digit string in threes from the right once it reaches
/// `threshold` digits. Correspondence running text uses 5 (so `1234` stays
/// ungrouped and years/postal codes pass through when callers disable
/// grouping); money amounts use 4 (standard thousands grouping).
fn grouped(int: &str, sep: &str, threshold: usize) -> String {
    if int.len() < threshold {
        return int.to_owned();
    }
    let bytes = int.as_bytes();
    let first = bytes.len() % 3;
    let mut out = String::with_capacity(int.len() + int.len() / 3);
    let mut head = 0;
    if first > 0 {
        out.push_str(&int[..first]);
        head = first;
    }
    while head < bytes.len() {
        if !out.is_empty() {
            out.push_str(sep);
        }
        out.push_str(&int[head..head + 3]);
        head += 3;
    }
    out
}

fn render(negative: bool, int: &str, frac: Option<&str>, decimal: &str) -> String {
    let mut out = String::new();
    if negative {
        out.push('-');
    }
    out.push_str(int);
    if let Some(digits) = frac {
        out.push_str(decimal);
        out.push_str(digits);
    }
    out
}

/// Number for a locale from a canonical decimal string.
///
/// `style` is `"cldr"` or `"amtlich"`; amtlich only changes `de-ch`/`de-li`
/// to the running-text comma decimal and is identical to CLDR elsewhere.
/// With `grouping`, integer parts of five or more digits group in threes;
/// four-digit inputs (and years such as `"2026"`) stay ungrouped. Pass
/// `grouping = false` for years and postal codes, which must never group.
/// Unknown locales fall back to English. Returns `None` for non-canonical
/// input or an unknown style.
#[must_use]
pub fn format_number(locale: &str, canonical: &str, grouping: bool, style: &str) -> Option<String> {
    let amtlich = match style {
        "cldr" => false,
        "amtlich" => true,
        _ => return None,
    };
    let parsed = parse_canonical(canonical)?;
    let entry = &NUMBERS.locales[resolve_key(locale)];
    let decimal = if amtlich {
        entry
            .amtlich_text_decimal
            .as_deref()
            .unwrap_or(entry.decimal.as_str())
    } else {
        entry.decimal.as_str()
    };
    let int = if grouping {
        grouped(parsed.int, entry.group.as_str(), 5)
    } else {
        parsed.int.to_owned()
    };
    Some(render(parsed.negative, int.as_str(), parsed.frac, decimal))
}

/// Money amount for a locale from a canonical decimal string.
///
/// Only two currencies are supported; anything else returns `None`:
/// `CHF` renders Swiss-style (dot decimal even in amtlich contexts),
/// while `EUR` uses the locale separators. `display` selects `"symbol"`
/// (`Fr. 1'234.56`, locale-placed `€`) or `"code"` (`CHF 100'000`,
/// `EUR 1.234,56` with a regular space); anything else returns `None`.
/// Thousands group from four digits. Returns `None` for non-canonical
/// input, an unsupported currency or an unknown display.
#[must_use]
pub fn format_money(
    locale: &str,
    canonical: &str,
    currency: &str,
    display: &str,
) -> Option<String> {
    let parsed = parse_canonical(canonical)?;
    let code = match display {
        "symbol" => false,
        "code" => true,
        _ => return None,
    };
    match currency {
        "CHF" => {
            let int = grouped(parsed.int, "'", 4);
            let amount = render(parsed.negative, int.as_str(), parsed.frac, ".");
            Some(if code {
                format!("CHF {amount}")
            } else {
                format!("Fr. {amount}")
            })
        }
        "EUR" => {
            let key = resolve_key(locale);
            let entry = &NUMBERS.locales[key];
            let int = grouped(parsed.int, entry.group.as_str(), 4);
            let amount = render(
                parsed.negative,
                int.as_str(),
                parsed.frac,
                entry.decimal.as_str(),
            );
            if code {
                Some(format!("EUR {amount}"))
            } else if key.starts_with("en") {
                Some(format!("€{amount}"))
            } else if key.starts_with("fr") {
                Some(format!("{amount}\u{202f}€"))
            } else {
                Some(format!("{amount}\u{a0}€"))
            }
        }
        _ => None,
    }
}

/// Whether a locale code is supported: present in the supported set
/// directly or through its (lowercased) base language.
#[must_use]
pub fn is_supported(locale: &str) -> bool {
    NUMBERS.supported.contains(locale)
        || NUMBERS
            .supported
            .contains(base_language(&locale.to_ascii_lowercase()))
}

/// BCP 47 locale codes with a number-format entry, sorted.
#[must_use]
pub fn available_locales() -> Vec<&'static str> {
    let mut codes: Vec<&'static str> = NUMBERS.locales.keys().map(String::as_str).collect();
    codes.sort_unstable();
    codes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tables_schema() {
        assert!(!NUMBERS.locales.is_empty(), "table must not be empty");
        assert!(
            NUMBERS.locales.contains_key(NUMBERS.fallback.as_str()),
            "fallback must be a known locale"
        );
        for key in NUMBERS.locales.keys() {
            let base = base_language(key);
            assert!(!base.is_empty(), "locale key must not be empty");
            assert_eq!(
                key,
                &key.to_ascii_lowercase(),
                "locale ID must be lowercase: {key:?}"
            );
        }
        for code in &NUMBERS.supported {
            assert!(
                NUMBERS.locales.contains_key(code.as_str())
                    || NUMBERS.locales.contains_key(base_language(code)),
                "supported code resolves nowhere: {code:?}"
            );
        }
        for (key, entry) in &NUMBERS.locales {
            for separator in [&entry.decimal, &entry.group] {
                assert_eq!(
                    separator.chars().count(),
                    1,
                    "separator must be one character: {key:?} {separator:?}"
                );
            }
            if let Some(marker) = &entry.amtlich_text_decimal {
                assert_eq!(
                    marker.chars().count(),
                    1,
                    "amtlich marker must be one character: {key:?}"
                );
                assert_ne!(
                    marker, &entry.decimal,
                    "amtlich marker must differ from CLDR decimal: {key:?}"
                );
            }
        }
        assert!(
            NUMBERS.locales["de-ch"].amtlich_text_decimal.is_some(),
            "de-ch needs the amtlich running-text marker"
        );
        assert!(
            NUMBERS.locales["de-li"].amtlich_text_decimal.is_some(),
            "de-li needs the amtlich running-text marker"
        );
    }

    #[test]
    fn canonical_inputs_reject_formatted_text() {
        for bad in [
            "", "-", ".", ".5", "5.", "1.2.3", "12,34", "1'234", "1,234.56", " 12", "12 ", "+12",
            "--12", "abc", "NaN", "1e3",
        ] {
            assert_eq!(parse_canonical(bad), None, "must reject {bad:?}");
            assert_eq!(format_number("de", bad, true, "cldr"), None);
            assert_eq!(format_money("de-ch", bad, "CHF", "symbol"), None);
            assert_eq!(format_money("de-ch", bad, "CHF", "code"), None);
        }
        for good in ["0", "-0", "42", "-1234.5", "1234567.89", "007"] {
            assert!(parse_canonical(good).is_some(), "must accept {good:?}");
        }
    }

    #[test]
    fn grouping_thresholds() {
        assert_eq!(grouped("1234", ".", 5), "1234");
        assert_eq!(grouped("12345", ".", 5), "12.345");
        assert_eq!(grouped("1234567", "'", 5), "1'234'567");
        assert_eq!(grouped("1234", "'", 4), "1'234");
        assert_eq!(grouped("123", "'", 4), "123");
    }

    fn run_vector(file: &std::path::Path, vector: &serde_json::Value) {
        let name = vector["name"].as_str().unwrap_or("<unnamed>");
        let context = format!("{} :: {name}", file.display());
        if vector["fn"] == "available_locales" {
            assert_eq!(
                serde_json::to_value(available_locales()).unwrap(),
                vector["expected"],
                "{context}"
            );
            return;
        }
        let locale = vector["locale"].as_str().expect("vector needs locale");
        if vector["fn"].as_str().unwrap_or("") == "is_supported" {
            let actual = serde_json::Value::Bool(is_supported(locale));
            let expected = vector
                .get("expected")
                .cloned()
                .unwrap_or(serde_json::Value::Null);
            assert_eq!(actual, expected, "{context}");
            return;
        }
        if vector["fn"].as_str().unwrap_or("") == "symbols" {
            let actual = serde_json::to_value(symbols(locale)).unwrap();
            let expected = vector
                .get("expected")
                .cloned()
                .unwrap_or(serde_json::Value::Null);
            assert_eq!(actual, expected, "{context}");
            return;
        }
        let canonical = vector["canonical"]
            .as_str()
            .expect("vector needs canonical");
        let actual: serde_json::Value = match vector["fn"].as_str().unwrap_or("") {
            "format_number" => {
                let grouping = vector["grouping"].as_bool().unwrap_or(true);
                let style = vector["style"].as_str().unwrap_or("cldr");
                format_number(locale, canonical, grouping, style).into()
            }
            "format_money" => {
                let currency = vector["currency"].as_str().unwrap_or("CHF");
                let display = vector["display"].as_str().unwrap_or("symbol");
                format_money(locale, canonical, currency, display).into()
            }
            other => panic!("{context}: unknown fn {other:?}"),
        };
        let expected = vector
            .get("expected")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        assert_eq!(actual, expected, "{context}");
    }

    #[test]
    fn conformance_vectors() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/vectors");
        let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&dir)
            .expect("tests/vectors exists")
            .map(|entry| entry.expect("readable entry").path())
            .collect();
        files.sort();
        assert!(!files.is_empty(), "no vector files in tests/vectors");
        let mut count = 0;
        for file in &files {
            let raw = std::fs::read_to_string(file).expect("vector file is readable");
            let vectors: Vec<serde_json::Value> =
                serde_json::from_str(&raw).expect("vector file is valid JSON");
            for vector in &vectors {
                run_vector(file, vector);
                count += 1;
            }
        }
        assert!(count > 0, "no vectors ran");
    }
}
