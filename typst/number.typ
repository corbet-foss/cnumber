#import "generated/numbers.typ": numbers-table

/// Lowercase locale IDs with case-insensitive input; exact code, base language,
/// then English fallback. Explicit overrides always win.
#let base-language(locale) = locale.split("-").at(0)

#let resolve-key(locale) = {
  let lowered = lower(locale)
  let base = base-language(lowered)
  if lowered in numbers-table.locales {
    lowered
  } else if base in numbers-table.locales {
    base
  } else {
    numbers-table.fallback
  }
}

/// Separators for a locale: `de` gives decimal `,` and group `.`.
#let symbols(locale) = {
  let entry = numbers-table.locales.at(resolve-key(locale))
  (decimal: entry.decimal, group: entry.group)
}

#let is-digit(char) = char in ("0", "1", "2", "3", "4", "5", "6", "7", "8", "9")

#let all-digits(text) = {
  if text.len() == 0 {
    false
  } else {
    let ok = true
    for char in text.codepoints() {
      if not is-digit(char) { ok = false }
    }
    ok
  }
}

/// Split a canonical decimal string (`-?digits(.digits)?`) into
/// `(negative, int, frac)`; anything else is `none`.
#let parse-canonical(canonical) = {
  if type(canonical) != str { return none }
  let negative = canonical.starts-with("-")
  let rest = if negative { canonical.slice(1) } else { canonical }
  if rest.len() == 0 { return none }
  let parts = rest.split(".")
  if parts.len() > 2 { return none }
  let int = parts.at(0)
  let frac = if parts.len() == 2 { parts.at(1) } else { none }
  if not all-digits(int) { return none }
  if frac != none and not all-digits(frac) { return none }
  (negative: negative, int: int, frac: frac)
}

/// Group an integer digit string in threes from the right once it reaches
/// `threshold` digits: 5 for running text, 4 for money amounts.
#let grouped(int, sep, threshold) = {
  if int.len() < threshold {
    int
  } else {
    let first = calc.rem(int.len(), 3)
    let out = if first > 0 { int.slice(0, first) } else { "" }
    let head = first
    while head < int.len() {
      if out != "" { out += sep }
      out += int.slice(head, head + 3)
      head += 3
    }
    out
  }
}

#let render(negative, int, frac, decimal) = {
  (if negative { "-" } else { "" }) + int + if frac == none { "" } else { decimal + frac }
}

/// Number for a locale from a canonical decimal string. `style` is `"cldr"`
/// or `"amtlich"`; amtlich only changes `de-ch`/`de-li` to the running-text
/// comma decimal. With `grouping`, integer parts of five or more digits
/// group in threes; pass `false` for years and postal codes.
#let format-number(locale, canonical, grouping: true, style: "cldr") = {
  if style != "cldr" and style != "amtlich" {
    none
  } else {
    let parsed = parse-canonical(canonical)
    if parsed == none {
      none
    } else {
      let entry = numbers-table.locales.at(resolve-key(locale))
      let decimal = if style == "amtlich" { entry.at("amtlich_text_decimal", default: entry.decimal) } else { entry.decimal }
      let int = if grouping { grouped(parsed.int, entry.group, 5) } else { parsed.int }
      render(parsed.negative, int, parsed.frac, decimal)
    }
  }
}

/// Money amount for a locale from a canonical decimal string. `display`
/// selects `"symbol"` (`Fr. 1'234.56`, locale-placed `€`) or `"code"`
/// (`CHF 100'000`, `EUR 1.234,56` with a regular space). `CHF` always
/// renders Swiss-style (dot decimal); `EUR` uses locale separators with
/// per-locale `€` placement. Other currencies, unknown displays, and
/// non-canonical input give `none`.
#let format-money(locale, canonical, currency, display) = {
  let parsed = parse-canonical(canonical)
  if parsed == none {
    none
  } else if display != "symbol" and display != "code" {
    none
  } else if currency == "CHF" {
    let amount = render(parsed.negative, grouped(parsed.int, "'", 4), parsed.frac, ".")
    if display == "code" { "CHF " + amount } else { "Fr. " + amount }
  } else if currency == "EUR" {
    let key = resolve-key(locale)
    let entry = numbers-table.locales.at(key)
    let amount = render(parsed.negative, grouped(parsed.int, entry.group, 4), parsed.frac, entry.decimal)
    if display == "code" {
      "EUR " + amount
    } else if key.starts-with("en") {
      "€" + amount
    } else if key.starts-with("fr") {
      amount + "\u{202f}€"
    } else {
      amount + "\u{a0}€"
    }
  } else {
    none
  }
}

/// Whether a locale code is supported: present in the supported set
/// directly or through its (lowercased) base language.
#let is-supported(locale) = {
  locale in numbers-table.supported or base-language(lower(locale)) in numbers-table.supported
}

/// Available lowercase locale IDs, sorted.
#let available-locales() = numbers-table.locales.keys().sorted()
