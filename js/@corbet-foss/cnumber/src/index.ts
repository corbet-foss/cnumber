/**
 * Deterministic locale-correct number formatting for formal correspondence.
 *
 * Pure TypeScript port of the cnumber Rust crate: zero dependencies, zero
 * Node APIs, synchronous, no I/O. Behavior is defined by `tables/*.json`
 * at the repository root; `tests/vectors/*.json` is the shared conformance
 * suite. Inputs are canonical decimal strings, never binary floats.
 */
import { NUMBERS_TABLE } from './generated/tables.ts';

export interface LocaleEntry {
    decimal: string;
    group: string;
    amtlich_text_decimal?: string;
}

export interface NumbersTable {
    locales: Record<string, LocaleEntry>;
    supported: string[];
    fallback: string;
}

export interface Symbols {
    decimal: string;
    group: string;
}

const table = NUMBERS_TABLE as NumbersTable;

function baseLanguage(locale: string): string {
    return locale.split('-')[0] ?? locale;
}

function resolveKey(locale: string): string {
    const lower = locale.toLowerCase();
    if (Object.hasOwn(table.locales, lower)) return lower;
    const base = baseLanguage(lower);
    return Object.hasOwn(table.locales, base) ? base : table.fallback;
}

/**
 * Separators for a locale: `de` gives decimal `,` and group `.`,
 * `de-ch` gives decimal `.` and group `'`. Unknown locales fall back
 * through the base language to English.
 */
export function symbols(locale: string): Symbols {
    const entry = table.locales[resolveKey(locale)];
    return { decimal: entry.decimal, group: entry.group };
}

interface Parsed {
    negative: boolean;
    int: string;
    frac: string | null;
}

function isDigits(text: string): boolean {
    return text.length > 0 && /^[0-9]+$/.test(text);
}

/** Split a canonical decimal string (`-?digits(.digits)?`); else `null`. */
function parseCanonical(canonical: string): Parsed | null {
    if (typeof canonical !== 'string') return null;
    const negative = canonical.startsWith('-');
    const rest = negative ? canonical.slice(1) : canonical;
    if (rest.length === 0) return null;
    const dot = rest.indexOf('.');
    const int = dot === -1 ? rest : rest.slice(0, dot);
    const frac = dot === -1 ? null : rest.slice(dot + 1);
    if (!isDigits(int)) return null;
    if (frac !== null && !isDigits(frac)) return null;
    if (frac !== null && rest.indexOf('.', dot + 1) !== -1) return null;
    return { negative, int, frac };
}

/**
 * Group an integer digit string in threes from the right once it reaches
 * `threshold` digits: 5 for running text, 4 for money amounts.
 */
function grouped(int: string, sep: string, threshold: number): string {
    if (int.length < threshold) return int;
    const first = int.length % 3;
    let out = first > 0 ? int.slice(0, first) : '';
    for (let head = first; head < int.length; head += 3) {
        if (out !== '') out += sep;
        out += int.slice(head, head + 3);
    }
    return out;
}

function render(negative: boolean, int: string, frac: string | null, decimal: string): string {
    return `${negative ? '-' : ''}${int}${frac === null ? '' : `${decimal}${frac}`}`;
}

/**
 * Number for a locale from a canonical decimal string. `style` is `"cldr"`
 * or `"amtlich"`; amtlich only changes `de-ch`/`de-li` to the running-text
 * comma decimal. With `grouping`, integer parts of five or more digits
 * group in threes; pass `false` for years and postal codes. Returns `null`
 * for non-canonical input or an unknown style.
 */
export function formatNumber(
    locale: string,
    canonical: string,
    grouping = true,
    style = 'cldr',
): string | null {
    const amtlich = style === 'cldr' ? false : style === 'amtlich' ? true : null;
    if (amtlich === null) return null;
    const parsed = parseCanonical(canonical);
    if (parsed === null) return null;
    const entry = table.locales[resolveKey(locale)];
    const decimal = amtlich ? (entry.amtlich_text_decimal ?? entry.decimal) : entry.decimal;
    const int = grouping ? grouped(parsed.int, entry.group, 5) : parsed.int;
    return render(parsed.negative, int, parsed.frac, decimal);
}

/**
 * Money amount for a locale from a canonical decimal string. `display`
 * selects `"symbol"` (`Fr. 1'234.56`, locale-placed `€`) or `"code"`
 * (`CHF 100'000`, `EUR 1.234,56` with a regular space). `CHF` always
 * renders Swiss-style (dot decimal); `EUR` uses locale separators with
 * per-locale `€` placement. Anything else returns `null`, as does
 * non-canonical input, an unsupported currency, or an unknown display.
 */
export function formatMoney(
    locale: string,
    canonical: string,
    currency = 'CHF',
    display = 'symbol',
): string | null {
    const parsed = parseCanonical(canonical);
    if (parsed === null) return null;
    const code = display === 'symbol' ? false : display === 'code' ? true : null;
    if (code === null) return null;
    if (currency === 'CHF') {
        const amount = render(parsed.negative, grouped(parsed.int, "'", 4), parsed.frac, '.');
        return code ? `CHF ${amount}` : `Fr. ${amount}`;
    }
    if (currency === 'EUR') {
        const key = resolveKey(locale);
        const entry = table.locales[key];
        const amount = render(parsed.negative, grouped(parsed.int, entry.group, 4), parsed.frac, entry.decimal);
        if (code) return `EUR ${amount}`;
        if (key.startsWith('en')) return `€${amount}`;
        if (key.startsWith('fr')) return `${amount}\u202f€`;
        return `${amount}\u00a0€`;
    }
    return null;
}

/** BCP 47 locale codes with a number-format entry, sorted. */
export function availableLocales(): string[] {
    return Object.keys(table.locales).sort();
}

/** Whether a locale code is supported: present in the supported set
 * directly or through its (lowercased) base language. */
export function isSupported(locale: string): boolean {
    if (table.supported.includes(locale)) return true;
    return table.supported.includes(baseLanguage(locale.toLowerCase()));
}
