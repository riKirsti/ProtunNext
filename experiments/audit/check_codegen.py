"""Inspect final linked code, not pre-LTO IR. Fail closed on unknown codegen.

This is a regression tripwire, not a proof of all possible compiler behavior.
It requires a zero-valued bulk memset reachable from each production API fixture;
the initial fill takes a runtime nonzero byte and does not satisfy the check.
"""
import argparse
import json
import pathlib
import re


def check(text):
    functions = {}
    current = None
    for line in text.splitlines():
        match = re.match(r"^[0-9a-f]+ <(.+)>:$", line)
        if match:
            current = match[1]
            functions[current] = []
        elif current:
            functions[current].append(line)
    results = {}
    for root in ["audit_drop_packet", "audit_recycle_packet"]:
        assert root in functions, f"missing final linked symbol {root}"
        visited, hits = set(), []

        def visit(name, depth):
            if name in visited or depth > 5:
                return
            visited.add(name)
            lines = functions.get(name, [])
            for i, line in enumerate(lines):
                # Objdump demangles names; preserve nested <> in Rust generics.
                target = re.search(r"\b(?:callq?|bl|b|jmpq?)\s+[^<]*<(.+)>\s*$", line)
                if not target:
                    continue
                callee = target[1]
                previous = "\n".join(lines[max(0, i - 7):i])
                zero = re.search(r"\b(?:xor\s+%esi,%esi|mov[lq]?\s+\$0x?0?,%[er]si|mov\s+[wx]1,\s*(?:#0x?0?|[wx]zr))\b", previous)
                if "memset" in callee and zero:
                    hits.append(dict(function=name, call=line.strip(), arguments=previous))
                elif callee in functions:
                    visit(callee, depth + 1)
        visit(root, 0)
        assert hits, f"no recognized zero memset reachable from {root}; inspect disassembly"
        results[root] = hits
    return results


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("disassembly")
    args = parser.parse_args()
    print(json.dumps(check(pathlib.Path(args.disassembly).read_text()), indent=2))
