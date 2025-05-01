#!/usr/bin/env python3
"""
Generate a Mermaid flowchart of Rust functions grouped by module,
with approximate call-graph edges.

Limitations (practical but not perfect):
  • Only free functions (no impl methods).
  • Edges found by a regex search – misses macro-generated / fully-qualified calls,
    may show a few false positives if different modules reuse the same fn name.
  • Heavy macro or unusual formatting?   -> switch to tree-sitter/syn.

Usage:
    python rust_funcs_to_mermaid.py <project_root> > out.mmd
"""

import re
import sys
from pathlib import Path
from collections import defaultdict

# ---------- regexes ---------------------------------------------------------
SIG_RE = re.compile(
    r"""(?xm)
    ^\s*                               # leading spaces
    (pub\s+)?(async\s+)?fn\s+          # fn header
    (?P<name>[A-Za-z0-9_]+)\s*         # function name
    \((?P<params>[^\)]*)\)             # parameter list
    (\s*->\s*(?P<ret>[^{\s]+))?        # optional return type (until { or whitespace)
    \s*\{                              # opening brace of body
    """,
)
# Any identifier followed by '(' that is *not* preceded by 'fn', 'struct', etc.
CALL_RE = re.compile(r"\b([A-Za-z_][A-Za-z0-9_]*)\s*\(")

# ---------------------------------------------------------------------------


def module_path(rs_file: Path, root: Path) -> str:
    """Convert a .rs file path to its Rust module path."""
    rel = rs_file.relative_to(root)
    parts = list(rel.parts)
    if parts[-1] == "mod.rs":
        parts = parts[:-1]
    else:
        parts[-1] = parts[-1][:-3]
    return "::".join(parts)


def extract_functions(text: str):
    """
    Yield (span_start, span_end, header_match) for every fn definition.
    span_* are body-inclusive (from 'fn' to matching '}').
    """
    for m in SIG_RE.finditer(text):
        start = m.start()
        # naive brace matching – fine for ordinary code without unbalanced braces in strings.
        depth = 0
        i = m.end() - 1  # position of the '{'
        for j, ch in enumerate(text[i:], i):
            if ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    end = j + 1
                    yield start, end, m
                    break


def main(root: Path):
    modules = defaultdict(list)  # mod → [(id,label)]
    bodies = {}  # node_id → body str
    name_index = defaultdict(list)  # simple fn name → [node_id]
    node_counter = 0

    # Pass 1: gather definitions
    for rs_file in root.rglob("*.rs"):
        mod = module_path(rs_file, root)
        text = rs_file.read_text(encoding="utf-8", errors="ignore")
        for start, end, m in extract_functions(text):
            node_id = f"n{node_counter}"
            node_counter += 1
            name = m.group("name")
            params = m.group("params").strip()
            ret = (m.group("ret") or "").strip()
            label = f"fn {name}({params}) {('-> ' + ret) if ret else ''}".strip()
            modules[mod].append((node_id, label))
            bodies[node_id] = text[m.end() : end]  # body w/o signature
            name_index[name].append(node_id)

    # Pass 2: find call edges
    edges = set()
    for caller_id, body in bodies.items():
        for call in CALL_RE.finditer(body):
            callee_simple = call.group(1)
            # connect to every function with that simple name
            for callee_id in name_index.get(callee_simple, []):
                if caller_id != callee_id:
                    edges.add((caller_id, callee_id))

    # ---------- emit Mermaid -------------------------------------------------
    print("flowchart LR")
    for mod, funcs in sorted(modules.items()):
        print(f"  subgraph {mod}")
        for nid, lbl in funcs:
            safe = lbl.replace('"', r"\"")
            print(f'    {nid}["{safe}"]')
        print("  end")
    for a, b in sorted(edges):
        print(f"  {a} --> {b}")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.exit("Usage: rust_funcs_to_mermaid.py <project_root>")
    main(Path(sys.argv[1]).resolve())
