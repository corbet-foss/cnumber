"""Compile already prepared preview consumer examples with the pinned binding."""
from pathlib import Path
import sys

import typst

root, packages, *examples = sys.argv[1:]
if not examples:
    raise SystemExit("At least one preview consumer example is required")
for example in examples:
    pdf = typst.compile(example, root=root, package_path=packages)
    if not pdf.startswith(b"%PDF"):
        raise ValueError("Preview consumer did not produce a PDF")
    print(f"Typst preview import passed: {Path(example).name}", flush=True)
