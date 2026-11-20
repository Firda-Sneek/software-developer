#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///
"""Bouwt elk sli.dev-deck in slides/ naar book/slides/…, met --base en zonder notes.

--out is een absoluut pad: Slidev lost --out relatief t.o.v. het Markdown-bestand op.
Aanroepen vanuit de wortel van de repo: uv run --script scripts/bouw-decks.py
"""
from __future__ import annotations

import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SLIDES = ROOT / "slides"
BOOK_SLIDES = ROOT / "book" / "slides"
SITE_BASE = "/software-developer/slides"


def fout(tekst: str) -> None:
    sys.exit(f"bouw-decks.py: FOUT: {tekst}")


def slidev_bin() -> str:
    for naam in ("slidev", "slidev.cmd"):
        kandidaat = ROOT / "node_modules" / ".bin" / naam
        if kandidaat.is_file():
            return str(kandidaat)
    gevonden = shutil.which("slidev")
    if gevonden:
        return gevonden
    fout("slidev ontbreekt. Draai eerst: npm ci (of npm install)")


def main() -> None:
    if not SLIDES.is_dir():
        fout(f"{SLIDES} ontbreekt")
    decks = sorted(p for p in SLIDES.rglob("*.md") if p.is_file())
    if not decks:
        fout(f"geen decks in {SLIDES}")

    slidev = slidev_bin()
    for bron in decks:
        rel = bron.relative_to(SLIDES)
        slug = rel.with_suffix("")  # voorbeeld/dummy
        doel = BOOK_SLIDES / slug
        base = f"{SITE_BASE}/{slug.as_posix()}/"
        if not base.startswith("/") or not base.endswith("/"):
            fout(f"--base moet beginnen en eindigen met /: {base}")
        doel.parent.mkdir(parents=True, exist_ok=True)
        cmd = [
            slidev,
            "build",
            str(bron),
            "--base",
            base,
            "--out",
            str(doel.resolve()),
            "--without-notes",
        ]
        print(" ".join(cmd))
        r = subprocess.run(cmd, cwd=ROOT)
        if r.returncode != 0:
            fout(f"slidev build mislukt voor {bron.relative_to(ROOT)}")
        index = doel / "index.html"
        if not index.is_file():
            fout(f"verwacht {index.relative_to(ROOT)} na de build")
        print(f"deck: {doel.relative_to(ROOT)}")
    print(f"Klaar: {len(decks)} deck(s).")


if __name__ == "__main__":
    main()
