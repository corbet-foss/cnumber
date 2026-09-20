"""Run canonical vectors against either the source port or an installed wheel."""
import importlib
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
if "--installed" not in sys.argv:
    sys.path.insert(0, str(ROOT / "py"))
api = importlib.import_module("cnumber")

ARGUMENTS = {
    "symbols": ["locale"],
    "format_number": ["locale", "canonical", "grouping", "style"],
    "format_money": ["locale", "canonical", "currency", "display"],
    "is_supported": ["locale"],
    "available_locales": [],
}

DEFAULTS = {
    "format_number": {"grouping": True, "style": "cldr"},
    "format_money": {"currency": "CHF", "display": "symbol"},
}

files = sorted((ROOT / "tests/vectors").glob("*.json"))
assert files, "No conformance vectors"
count = 0
for file in files:
    for vector in json.loads(file.read_text(encoding="utf-8")):
        name = vector["fn"]
        merged = dict(DEFAULTS.get(name, {}))
        merged.update({key: value for key, value in vector.items() if key in ARGUMENTS[name]})
        fn = getattr(api, name)
        actual = fn(*(merged.get(key) for key in ARGUMENTS[name]))
        if actual != vector["expected"]:
            raise AssertionError(f"{file.name} :: {vector['name']}: {actual!r} != {vector['expected']!r}")
        count += 1
print(f"Python cnumber: {count} vectors passed across {len(files)} files")
