import argparse
import os
import sys

from pyscan_yb import scan


def main():
    parser = argparse.ArgumentParser(
        prog="pyscan-yb",
        description="A fast Python file scanner",
    )
    parser.add_argument(
        "path",
        nargs="?",
        default=".",
        help="Directory to scan (default: current directory)",
    )
    parser.add_argument(
        "-q",
        "--quiet",
        action="store_true",
        help="Quiet mode: only print file paths, no messages",
    )

    args = parser.parse_args()
    path = os.path.abspath(args.path)

    files = scan(path)

    if not files:
        if not args.quiet:
            print(f"No Python files found in {args.path}")
        sys.exit(1)

    for f in files:
        print(f)

    sys.exit(0)
