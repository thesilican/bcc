#!/usr/bin/env python
from difflib import unified_diff
from pathlib import Path
from subprocess import run

ROOT = Path(__file__).joinpath("../..").resolve()


def print_diff(output, expected):
    out_lines = [x + "\n" for x in output.splitlines()]
    expected_lines = [x + "\n" for x in expected.splitlines()]
    diff = unified_diff(out_lines, expected_lines, fromfile="output", tofile="expected")
    diff_str = "".join(diff)
    print(diff_str)


def run_bcc(args):
    bin_path = ROOT.joinpath("bin/bcc").resolve()
    output = run([bin_path, *args], capture_output=True, encoding="utf8")
    return_code = output.returncode
    assert return_code == 0, f"bcc exited with code {return_code}"
    return output.stdout


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
            output = run_bcc(["test", test_type, str(infile)])
            RESET = "\033[0;0m"
            RED = "\033[1;31m"
            GREEN = "\033[0;32m"
            if output == expect:
                print(f"[{GREEN}PASS{RESET}] {test_type}/{filename}")
            else:
                print(f"[{RED}FAIL{RESET}] {test_type}/{filename}")
                print_diff(output, expect)


run_tests()
