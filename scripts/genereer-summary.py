#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["pyyaml"]
# ///
"""Schrijft src/SUMMARY.md uit catalogus.yaml.

Controleert of elk genoemd bestand bestaat. Ontbreekt er een, dan exit 1.
Aanroepen vanuit de wortels van de repo: uv run --script scripts/genereer-summary.py
"""
from __future__ import annotations

import sys
from pathlib import Path

try:
    import yaml
except ImportError:
    sys.exit(
        "genereer-summary.py: PyYAML ontbreekt. "
        "Start dit script met: uv run --script scripts/genereer-summary.py"
    )

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "src"
CATALOGUS = ROOT / "catalogus.yaml"
SUMMARY = SRC / "SUMMARY.md"


def fout(tekst: str) -> None:
    sys.exit(f"genereer-summary.py: FOUT: {tekst}")


def item_regels(item: object, diepte: int, fouten: list[str]) -> list[str]:
    if not isinstance(item, dict):
        fouten.append(f"ongeldig item (geen mapping): {item!r}")
        return []

    kinderen = item.get("kinderen") or []
    if kinderen and not isinstance(kinderen, list):
        fouten.append(f"'kinderen' moet een lijst zijn bij {item!r}")
        kinderen = []

    kop = item.get("kop")
    titel = item.get("titel")
    bestand = item.get("bestand")
    regels: list[str] = []

    if kop:
        if bestand:
            fouten.append(f"een kop mag geen bestand hebben: {kop}")
        regels.append(f"# {kop}")
        regels.append("")
        for kind in kinderen:
            regels.extend(item_regels(kind, 0, fouten))
        if regels and regels[-1] != "":
            regels.append("")
        return regels

    if not titel or not bestand:
        fouten.append(f"item mist 'titel' of 'bestand': {item!r}")
        return []

    pad = SRC / bestand
    if not pad.is_file():
        fouten.append(f"bestand ontbreekt: src/{bestand}")

    indent = "  " * diepte
    regels.append(f"{indent}- [{titel}]({bestand})")
    for kind in kinderen:
        regels.extend(item_regels(kind, diepte + 1, fouten))
    return regels


def main() -> None:
    if not CATALOGUS.is_file():
        fout(f"{CATALOGUS.name} ontbreekt")

    data = yaml.safe_load(CATALOGUS.read_text(encoding="utf-8"))
    if not isinstance(data, dict) or "paginas" not in data:
        fout("catalogus.yaml moet een mapping met 'paginas' zijn")
    paginas = data["paginas"]
    if not isinstance(paginas, list) or not paginas:
        fout("'paginas' moet een niet-lege lijst zijn")

    fouten: list[str] = []
    regels = ["# Summary", ""]
    for item in paginas:
        chunk = item_regels(item, 0, fouten)
        if chunk and chunk[0].startswith("# ") and regels and regels[-1] != "":
            regels.append("")
        regels.extend(chunk)

    if fouten:
        for regel in fouten:
            print(f"genereer-summary.py: FOUT: {regel}", file=sys.stderr)
        sys.exit(1)

    while regels and regels[-1] == "":
        regels.pop()
    tekst = "\n".join(regels) + "\n"
    SUMMARY.parent.mkdir(parents=True, exist_ok=True)
    SUMMARY.write_text(tekst, encoding="utf-8")
    print(f"geschreven: {SUMMARY.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
