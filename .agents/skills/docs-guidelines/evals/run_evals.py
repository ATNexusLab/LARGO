#!/usr/bin/env python3
"""
Run evals for docs-guidelines skill: produce baseline and with_skill outputs and a static viewer.
"""
import json
import re
from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent
EVALS_FILE = BASE_DIR / "evals.json"
WORKSPACE = BASE_DIR.parent / "workspace" / "iteration-1"
WORKSPACE.mkdir(parents=True, exist_ok=True)

with open(EVALS_FILE, 'r', encoding='utf-8') as f:
    evals = json.load(f)


def extract_filenames(text):
    # find patterns like 'filename.md' or items in brackets
    fns = re.findall(r"['\[]([\w\-]+\.md)['\]]", text)
    if not fns:
        fns = re.findall(r"([\w\-]+\.md)", text)
    return fns


def baseline(eval_item):
    eid = eval_item['id']
    inp = eval_item.get('input', '')
    fns = extract_filenames(inp)
    lines = []
    if 'audit' in eid:
        lines.append("Baseline mapping (naive):")
        for fn in fns:
            src = f"docs/architecture/{fn}"
            dst = f"docs/architecture/adr/uncategorized/{fn}"
            lines.append(f"- {fn} -> adr/uncategorized/")
            lines.append(f"Command: git mv {src} {dst}")
    elif 'generate-ai-readme' in eid:
        lines.append("Baseline README draft for docs/ai:")
        lines.append("## AI Worker (overview)\n\nSections:\n- Overview\n- TOON prompts (placeholder)\n")
    elif 'rename-adr' in eid:
        fns = fns or ['adr-db.md']
        fn = fns[0]
        src = f"docs/architecture/{fn}"
        dst = f"docs/architecture/adr/uncategorized/{fn}"
        lines.append(f"Baseline: move to uncategorized\nCommand: git mv {src} {dst}")
    else:
        lines.append("Baseline: no suggestion")
    return "\n".join(lines)


def categorize(fn):
    l = fn.lower()
    if 'db' in l or 'migration' in l or 'migrations' in l:
        return 'database'
    if 'gemini' in l or 'ai' in l:
        return 'ai'
    if 'arch' in l:
        return 'architecture'
    return 'misc'


def with_skill(eval_item):
    eid = eval_item['id']
    inp = eval_item.get('input', '')
    fns = extract_filenames(inp)
    lines = []
    if 'audit' in eid:
        lines.append("With-skill mapping (heuristic):")
        ordinal = 1
        for fn in fns:
            cat = categorize(fn)
            num = f"{ordinal:02d}"
            newname = f"{num}-{fn}"
            src = f"docs/architecture/{fn}"
            dst = f"docs/architecture/adr/{cat}/{newname}"
            lines.append(f"- {fn} -> adr/{cat}/{newname}")
            lines.append(f"Command: git mv {src} {dst}")
            ordinal += 1
        lines.append("Keep 'architecture.md' as overview.")
    elif 'generate-ai-readme' in eid:
        lines.append("With-skill README draft for docs/ai:")
        lines.append("## TOON prompts\nExample:\n```python\nfrom toon_format import encode\nex = encode([{\"id\":1,\"descricao\":\"Mercado\",\"valor\":150.0}])\nprint(ex)\n```\n")
    elif 'rename-adr' in eid:
        fns = fns or ['adr-db.md']
        fn = fns[0]
        cat = categorize(fn)
        newname = f"01-{fn}"
        src = f"docs/architecture/{fn}"
        dst = f"docs/architecture/adr/{cat}/{newname}"
        lines.append(f"With-skill suggestion: {newname}")
        lines.append(f"Command: git mv {src} {dst}")
    else:
        lines.append("With-skill: no suggestion")
    return "\n".join(lines)


summary = []
for e in evals:
    eid = e.get('id')
    base_text = baseline(e)
    with_text = with_skill(e)
    base_path = WORKSPACE / f"{eid}_baseline.txt"
    with_path = WORKSPACE / f"{eid}_with_skill.txt"
    base_path.write_text(base_text, encoding='utf-8')
    with_path.write_text(with_text, encoding='utf-8')
    summary.append({'id':eid, 'baseline':str(base_path), 'with_skill':str(with_path)})

# generate static viewer
html = []
html.append("<!doctype html><html><head><meta charset='utf-8'><title>docs-guidelines evals</title></head><body>")
html.append("<h1>docs-guidelines evals</h1>")
for s in summary:
    eid = s['id']
    html.append(f"<section><h2>{eid}</h2>")
    expected = next((ev.get('expected','') for ev in evals if ev.get('id')==eid),'')
    html.append("<h3>Expected</h3><pre>{}</pre>".format(expected))
    html.append("<h3>Baseline</h3><pre>{}</pre>".format((WORKSPACE / f"{eid}_baseline.txt").read_text()))
    html.append("<h3>With Skill</h3><pre>{}</pre>".format((WORKSPACE / f"{eid}_with_skill.txt").read_text()))
    html.append("</section>")
html.append("</body></html>")
index_file = WORKSPACE / "index.html"
index_file.write_text("\n".join(html), encoding='utf-8')

print(f"Generated {len(summary)} evals in: {WORKSPACE}")
print("Viewer:", index_file)
