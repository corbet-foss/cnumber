"""JSON command-line bridge for callers in any programming language."""
import argparse
import json
import sys
import cnumber as api


def main():
    parser = argparse.ArgumentParser(description="Call cnumber functions with JSON arguments.")
    parser.add_argument("function", choices=api.__all__)
    parser.add_argument("arguments", nargs="?", default="[]", help="JSON array of positional arguments or object of keyword arguments; '-' reads UTF-8 stdin")
    args = parser.parse_args()
    try:
        values = json.loads(sys.stdin.buffer.read().decode("utf-8") if args.arguments == "-" else args.arguments, parse_constant=_reject_constant)
        if not isinstance(values, (list, dict)):
            raise ValueError("arguments must be a JSON array or object")
        fn = getattr(api, args.function)
        result = fn(*values) if isinstance(values, list) else fn(**values)
        print(json.dumps(result, ensure_ascii=True, allow_nan=False))
    except (ValueError, TypeError, KeyError, IndexError, AttributeError, OverflowError) as error:
        parser.error(str(error))


def _reject_constant(value):
    raise ValueError(f"{value} is not a JSON number")


if __name__ == "__main__":
    main()
