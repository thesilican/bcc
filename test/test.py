#!/usr/bin/env python
from difflib import unified_diff
from pathlib import Path
from subprocess import run
from sys import stdout

ROOT = Path(__file__).joinpath("../..").resolve()
IS_TTY = stdout.isatty()
RESET = "\033[0;0m" if IS_TTY else ""
RED = "\033[0;31m" if IS_TTY else ""
GREEN = "\033[0;32m" if IS_TTY else ""
CYAN = "\033[0;36m" if IS_TTY else ""


def print_diff(output, expected):
    out_lines = [x + "\n" for x in output.splitlines()]
    expected_lines = [x + "\n" for x in expected.splitlines()]
    diff = unified_diff(out_lines, expected_lines, fromfile="output", tofile="expected")
    for line in diff:
        if line.startswith("+"):
            print(f"{GREEN}{line}{RESET}", end="")
        elif line.startswith("-"):
            print(f"{RED}{line}{RESET}", end="")
        elif line.startswith("@"):
            print(f"{CYAN}{line}{RESET}", end="")
        else:
            print(line, end="")


def run_bcc(args):
    bin_path = ROOT.joinpath("bin/bcc").resolve()
    output = run([bin_path, *args], capture_output=True, encoding="utf8")
    return_code = output.returncode
    return return_code, output.stdout


def run_tests():
    root_test_dir = ROOT.joinpath("test/fixtures").resolve()
    for test_dir in root_test_dir.iterdir():
        test_type = test_dir.name
        for infile in test_dir.iterdir():
            if infile.name.endswith(".in"):
                filename = infile.stem
                outfile = infile.joinpath(f"../{filename}.out").resolve()
            else:
                continue
            with open(outfile) as f:
                expect = f.read()
            code, output = run_bcc(["test", test_type, str(infile)])
            if code != 0:
                print(f"[{RED}FAIL{RESET}] {test_type}/{filename}")
                print(f"Exit code: {code}")
                print("Output:")
                print(output)
            elif output == expect:
                print(f"[{GREEN}PASS{RESET}] {test_type}/{filename}")
            else:
                print(f"[{RED}FAIL{RESET}] {test_type}/{filename}")
                print_diff(output, expect)


run_tests()
