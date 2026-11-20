#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["fonttools", "brotli", "certifi"]
# ///
"""Downloadt Work Sans v2.010 en Libertinus Serif v7.051 van de officiële
GitHub-bronnen, controleert SHA-256 en maakt Latin-subsets als WOFF2 in theme/fonts/.

Idempotent: bestaande woff2-bestanden worden overgeslagen (--force of FORCE=1
maakt alles opnieuw). Werkt op Windows, macOS en Linux; geen sh nodig.
Vereist: Python 3 met fonttools en brotli (python -m pip install fonttools brotli).
Gebruik: uv run --script scripts/fonts.py [--force]
"""
import hashlib
import io
import os
import shutil
import ssl
import subprocess
import sys
import tempfile
import urllib.request
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FONTS = ROOT / "theme" / "fonts"

# Work Sans v2.010: tag v2.010 = commit 6634e7b2395e9dba4ac8611c4529968a9719d606
WS_URL = "https://raw.githubusercontent.com/weiweihuanghuang/Work-Sans/6634e7b2395e9dba4ac8611c4529968a9719d606/fonts/static/TTF"
WS = {  # naam: sha256 (TTF's van Work Sans v2.010)
    "Light": "16d4241f82605d7a104e49ace6b11c07c1ed4e5b70bbdc6cd88311a3152665eb",
    "Regular": "22e7f1607ebc29d03be61d893ec47dde307847eaf60fbec260e286695001982a",
    "Italic": "8fcbc2499374a09594ff28ed488852191f3451ec2ef430d421ec327b3d6c15e8",
    "Medium": "3c5ae1ef9260a0c1cdf1f59841f28620c40826f6ca3bd4c71516997c29fec7dc",
    "SemiBold": "399b8be04df8e5c531cc1cf490d9d4d781299440e086d61097db5175e2c71321",
    "Bold": "e5c36b1191be27046270da352f231fc342ad4333cfb2967f393e00d18c274cdc",
    "BoldItalic": "8f7b9e47221fab81026d48d1d746f3e8cbad4b045524c09645d052efd181ca19",
}
# Libertinus v7.051: release-zip
LIB_URL = "https://github.com/alerque/libertinus/releases/download/v7.051/Libertinus-7.051.zip"
LIB_SHA = "4d9be29b5cb380c35af8ba967abcc752ad1e07be1f738a9789c33e0dd7478c92"
LIB_OTF = "Libertinus-7.051/static/OTF/LibertinusSerif-Regular.otf"  # OTF: bevat GSUB (ligaturen)

# Latin, Latin-1, Latin Extended-A, leestekens, euro
UNICODES = "U+0000-00FF,U+0100-017F,U+0131,U+0152-0153,U+02BB-02BC,U+02C6,U+02DA,U+02DC,U+2000-206F,U+20AC,U+2122,U+2190-2193,U+2212,U+2215,U+FEFF,U+FFFD"


def fout(tekst):
    sys.exit(f"fonts.py: FOUT: {tekst}")


def download(url):
    # Python van python.org op macOS heeft soms geen CA-certificaten; dan certifi of curl.
    # Veilig: elke download wordt daarna op SHA-256 gecontroleerd.
    try:
        with urllib.request.urlopen(url, timeout=60) as r:
            return r.read()
    except Exception as e:
        fout_tekst = str(e)
        if "CERTIFICATE_VERIFY_FAILED" not in fout_tekst:
            fout(f"download mislukt: {url} ({e})")
    try:
        import certifi
        ctx = ssl.create_default_context(cafile=certifi.where())
        with urllib.request.urlopen(url, timeout=60, context=ctx) as r:
            return r.read()
    except ImportError:
        pass
    except Exception as e:
        fout(f"download mislukt: {url} ({e})")
    curl = shutil.which("curl")
    if curl:
        r = subprocess.run([curl, "-fsSL", url], capture_output=True)
        if r.returncode == 0:
            return r.stdout
    fout(f"download mislukt: {url} ({fout_tekst}). Op macOS: draai 'Install Certificates.command' "
         f"van je Python, of {Path(sys.executable).name} -m pip install certifi")


def controleer(data, naam, verwacht):
    if hashlib.sha256(data).hexdigest() != verwacht:
        fout(f"SHA-256 klopt niet voor {naam} (verwacht {verwacht}).")


def subset(bron, doel):
    from fontTools import subset as fts
    tmp = doel.with_name(doel.name + ".tmp")
    fts.main([str(bron), f"--unicodes={UNICODES}", "--layout-features=*",
              "--flavor=woff2", f"--output-file={tmp}"])
    os.replace(tmp, doel)
    print(f"gemaakt: theme/fonts/{doel.name}")


def main():
    force = "--force" in sys.argv[1:] or os.environ.get("FORCE") == "1"
    try:
        import fontTools.subset  # noqa: F401
    except ImportError:
        fout(f"fonttools ontbreekt. Installeer met: {Path(sys.executable).name} -m pip install fonttools brotli")
    try:
        import brotli  # noqa: F401
    except ImportError:
        fout(f"brotli ontbreekt (nodig voor WOFF2). Installeer met: {Path(sys.executable).name} -m pip install brotli")

    FONTS.mkdir(exist_ok=True)
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        for naam, sha in WS.items():
            doel = FONTS / f"WorkSans-{naam}.woff2"
            if doel.is_file() and not force:
                print(f"bestaat al: theme/fonts/{doel.name}")
                continue
            data = download(f"{WS_URL}/WorkSans-{naam}.ttf")
            controleer(data, f"WorkSans-{naam}.ttf", sha)
            bron = tmp / f"WorkSans-{naam}.ttf"
            bron.write_bytes(data)
            subset(bron, doel)

        doel = FONTS / "LibertinusSerif-Regular.woff2"
        if doel.is_file() and not force:
            print(f"bestaat al: theme/fonts/{doel.name}")
        else:
            data = download(LIB_URL)
            controleer(data, "libertinus.zip", LIB_SHA)
            bron = tmp / "LibertinusSerif-Regular.otf"
            bron.write_bytes(zipfile.ZipFile(io.BytesIO(data)).read(LIB_OTF))
            subset(bron, doel)
    print("Klaar: fonts staan in theme/fonts/.")


if __name__ == "__main__":
    main()
