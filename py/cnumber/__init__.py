"""Deterministic locale-correct number formatting for formal correspondence.

Pure-Python port of the cnumber Rust crate: standard library only, no I/O.
Behavior is defined by ``tables/*.json`` at the repository root;
``tests/vectors/*.json`` is the shared conformance suite
(``py/scripts/conformance.py``). Inputs are canonical decimal strings,
never binary floats.
"""

from ._tables import TABLES

_RAW = TABLES["numbers"]

_LOCALES = _RAW["locales"]
_SUPPORTED = _RAW["supported"]
_FALLBACK = _RAW["fallback"]

_CANONICAL_BY_LOWER = {key.lower(): key for key in _LOCALES}


def _base_language(locale: str) -> str:
    return locale.split("-", 1)[0]


def _resolve_key(locale: str) -> str:
    if locale in _LOCALES:
        return locale
    lowered = locale.lower()
    if lowered in _CANONICAL_BY_LOWER:
        return _CANONICAL_BY_LOWER[lowered]
    base = _CANONICAL_BY_LOWER.get(_base_language(lowered))
    if base is not None:
        return base
    return _FALLBACK


def symbols(locale: str) -> dict:
    """Separators for a locale: ``de`` gives decimal ``,`` and group ``.``."""
    entry = _LOCALES[_resolve_key(locale)]
    return {"decimal": entry["decimal"], "group": entry["group"]}


def _parse_canonical(canonical: str):
    """Split a canonical decimal string (``-?digits(.digits)?``); else None."""
    if not isinstance(canonical, str):
        return None
    negative = canonical.startswith("-")
    rest = canonical[1:] if negative else canonical
    if not rest:
        return None
    head, dot, tail = rest.partition(".")
    if dot and "." in tail:
        return None
    frac = tail if dot else None
    if not head or not head.isascii() or not head.isdigit():
        return None
    if frac is not None and (not frac or not frac.isascii() or not frac.isdigit()):
        return None
    return (negative, head, frac)


def _grouped(int: str, sep: str, threshold: int) -> str:
    """Group an integer digit string in threes once it reaches threshold."""
    if len(int) < threshold:
        return int
    first = len(int) % 3
    out = int[:first] if first else ""
    for head in range(first, len(int), 3):
        if out:
            out += sep
        out += int[head : head + 3]
    return out


def _render(negative: bool, int: str, frac, decimal: str) -> str:
    text = ("-" if negative else "") + int
    if frac is not None:
        text += decimal + frac
    return text


def format_number(locale: str, canonical: str, grouping: bool = True, style: str = "cldr") -> str | None:
    """Number for a locale from a canonical decimal string.

    ``style`` is ``"cldr"`` or ``"amtlich"``; amtlich only changes
    ``de-ch``/``de-li`` to the running-text comma decimal. With
    ``grouping``, integer parts of five or more digits group in threes;
    pass ``False`` for years and postal codes.
    """
    amtlich = {"cldr": False, "amtlich": True}.get(style)
    if amtlich is None:
        return None
    parsed = _parse_canonical(canonical)
    if parsed is None:
        return None
    negative, int, frac = parsed
    entry = _LOCALES[_resolve_key(locale)]
    decimal = entry.get("amtlich_text_decimal", entry["decimal"]) if amtlich else entry["decimal"]
    if grouping:
        int = _grouped(int, entry["group"], 5)
    return _render(negative, int, frac, decimal)


def format_money(locale: str, canonical: str, currency: str = "CHF", display: str = "symbol") -> str | None:
    """Money amount for a locale from a canonical decimal string.

    ``display`` selects ``"symbol"`` (``Fr. 1'234.56``, locale-placed
    ``€``) or ``"code"`` (``CHF 100'000``, ``EUR 1.234,56`` with a
    regular space). ``CHF`` always renders Swiss-style (dot decimal);
    ``EUR`` uses locale separators with per-locale ``€`` placement.
    Other currencies, unknown displays, and non-canonical input return
    ``None``.
    """
    parsed = _parse_canonical(canonical)
    if parsed is None:
        return None
    code = {"symbol": False, "code": True}.get(display)
    if code is None:
        return None
    negative, int, frac = parsed
    if currency == "CHF":
        amount = _render(negative, _grouped(int, "'", 4), frac, ".")
        return ("CHF " if code else "Fr. ") + amount
    if currency == "EUR":
        key = _resolve_key(locale)
        entry = _LOCALES[key]
        amount = _render(negative, _grouped(int, entry["group"], 4), frac, entry["decimal"])
        if code:
            return "EUR " + amount
        if key.startswith("en"):
            return "€" + amount
        if key.startswith("fr"):
            return amount + "\u202f€"
        return amount + "\u00a0€"
    return None


def available_locales() -> list:
    """BCP 47 locale codes with a number-format entry, sorted."""
    return sorted(_LOCALES)


def is_supported(locale: str) -> bool:
    """Whether a locale code is supported directly or via its base language."""
    return locale in _SUPPORTED or _base_language(locale.lower()) in _SUPPORTED


__all__ = ["symbols", "format_number", "format_money", "available_locales", "is_supported"]
