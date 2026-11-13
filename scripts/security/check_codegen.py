"""Inspect final linked code, not pre-LTO IR. Fail closed on unknown codegen.

This is a regression tripwire, not a proof of all possible compiler behavior.
It requires a zero-valued bulk memset reachable from each production API fixture;
the initial fill takes a runtime nonzero byte and does not satisfy the check.
"""
import argparse
import json
import pathlib
import re


def check(text, roots=("audit_drop_packet", "audit_recycle_packet")):
    functions = {}
    addresses, relocations = {}, {}
    current = None
    for line in text.splitlines():
        match = re.match(r"^([0-9a-f]+) <(.+)>:$", line)
        if match:
            current = match[2]
            addresses[int(match[1], 16)] = current
            functions[current] = []
        elif current:
            functions[current].append(line)
        if "R_X86_64_RELATIVE" in line:
            fields = line.split()
            relocations[int(fields[0], 16)] = int(fields[-1].split("+0x")[-1], 16)
    results = {}
    for root in roots:
        assert root in functions, f"missing final linked symbol {root}"
        visited, hits = set(), []

        def visit(name, depth):
            if name in visited or depth > 5:
                return
            # Some CPU targets lower zero-initialized allocation to malloc plus
            # memset rather than calloc. Initialization before live bytes exist
            # is not erasure and must never satisfy this tripwire.
            if "alloc_zeroed" in name or name in {"calloc", "__libc_calloc"}:
                return
            visited.add(name)
            lines = functions.get(name, [])
            for i, line in enumerate(lines):
                if not re.search(r"\b(?:callq?|bl|b|jmpq?)\s", line):
                    continue
                # Objdump demangles names; preserve nested <> in Rust generics.
                target = re.search(r"\b(?:callq?|bl|b|jmpq?)\s+[^<]*<(.+)>\s*$", line)
                callee = target[1] if target else ""
                got = re.search(r"#\s*([0-9a-f]+)\s+<", line)
                if got and int(got[1], 16) in relocations:
                    callee = addresses.get(relocations[int(got[1], 16)], "")
                indirect = re.search(r"\bcallq?\s+\*(%\w+)\s*$", line)
                if indirect:
                    # LLVM can cache memset's GOT address in a callee-saved
                    # register. Resolve the last assignment, not any old mention.
                    for earlier in reversed(lines[:i]):
                        if re.search(r"," + re.escape(indirect[1]) + r"\s*(?:#.*)?$", earlier):
                            symbol = re.search(r"<(.+)>\s*$", earlier)
                            callee = symbol[1] if symbol else ""
                            break
                previous = "\n".join(lines[max(0, i - 7):i])
                writes = [l for l in lines[max(0, i - 7):i] if re.search(r",%[er]si\s*$|\b(?:mov|movk|orr|add|ldr)\s+[wx]1,", l)]
                zero = writes and re.search(r"\b(?:xor\s+%esi,%esi|mov[lq]?\s+\$(?:0x)?0,%[er]si|mov\s+[wx]1,\s*(?:#(?:0x)?0|[wx]zr))\b", writes[-1])
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
