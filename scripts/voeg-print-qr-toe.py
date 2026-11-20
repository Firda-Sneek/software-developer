#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["segno"]
# ///
"""Zet in elke mdBook-HTML-pagina een printfooter met canonieke URL en QR-code.

Draait ná `mdbook build`. Idempotent. Geen externe QR-API, geen CDN.
Aanroepen vanuit de wortel van de repo: uv run --script scripts/voeg-print-qr-toe.py
"""
from __future__ import annotations

import sys
from pathlib import Path

try:
    import segno
except ImportError:
    sys.exit(
        "voeg-print-qr-toe.py: segno ontbreekt. "
        "Start dit script met: uv run --script scripts/voeg-print-qr-toe.py"
    )

ROOT = Path(__file__).resolve().parent.parent
BOOK = ROOT / "book"
SITE = "https://firda-sneek.github.io/software-developer/"
MARKERING = 'id="firda-print-footer"'


def fout(tekst: str) -> None:
    sys.exit(f"voeg-print-qr-toe.py: FOUT: {tekst}")


def canoniek(html_pad: Path) -> str:
    rel = html_pad.relative_to(BOOK).as_posix()
    if rel == "index.html":
        return SITE
    return SITE + rel


def footer(url: str, qr_svg: str) -> str:
    return (
        f'<footer id="firda-print-footer" hidden>\n'
        f'  <div class="firda-print-inner">\n'
        f'    <div class="firda-print-qr" aria-hidden="true">{qr_svg}</div>\n'
        f'    <p>Nieuwste versie: <span class="firda-print-url">{url}</span>'
        f' — gedrukt op <span id="firda-print-datum">…</span>. '
        f"Controleer online of deze pagina is bijgewerkt.</p>\n"
        f"  </div>\n"
        f"</footer>\n"
    )


def verwerk(html_pad: Path) -> bool:
    tekst = html_pad.read_text(encoding="utf-8")
    if MARKERING in tekst:
        # Bestaande footer vervangen, zodat een herbouw schone URL's houdt.
        start = tekst.find("<footer id=\"firda-print-footer\"")
        einde = tekst.find("</footer>", start)
        if start == -1 or einde == -1:
            fout(f"bestaande printfooter onvolledig in {html_pad}")
        tekst = tekst[:start] + tekst[einde + len("</footer>") :].lstrip("\n")

    if "</body>" not in tekst:
        fout(f"geen </body> in {html_pad}")

    url = canoniek(html_pad)
    qr = segno.make(url, error="m")
    # compacte SVG, zwarte modules, geen ruime marge (printfooter is klein)
    qr_svg = qr.svg_inline(scale=4, border=1)
    nieuw = tekst.replace("</body>", footer(url, qr_svg) + "</body>", 1)
    html_pad.write_text(nieuw, encoding="utf-8")
    return True


def kopieer_fonts() -> None:
    """Zet de WOFF2-bestanden naast de thema-CSS in de book-output."""
    bron = ROOT / "theme" / "fonts"
    doel = BOOK / "theme" / "fonts"
    bestanden = sorted(bron.glob("*.woff2"))
    if not bestanden:
        fout("geen WOFF2 in theme/fonts/. Draai eerst: mise run fonts")
    doel.mkdir(parents=True, exist_ok=True)
    for pad in bestanden:
        (doel / pad.name).write_bytes(pad.read_bytes())
        print(f"font: theme/fonts/{pad.name}")


def main() -> None:
    if not BOOK.is_dir():
        fout(f"{BOOK} ontbreekt. Draai eerst: mdbook build")
    kopieer_fonts()

    # Alleen mdBook-paginas; niet de Slidev-decks in book/slides/.
    paginas = [
        p
        for p in BOOK.rglob("*.html")
        if "slides" not in p.relative_to(BOOK).parts
        and p.name not in {"toc.html", "print.html"}
    ]
    if not paginas:
        fout("geen HTML-paginas in book/")

    aantal = 0
    for pad in sorted(paginas):
        verwerk(pad)
        aantal += 1
        print(f"footer: {pad.relative_to(BOOK)}")
    print(f"Klaar: printfooter in {aantal} pagina's.")


if __name__ == "__main__":
    main()
