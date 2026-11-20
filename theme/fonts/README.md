# Fonts

Zelf gehost als WOFF2, gesubset op Latijn. Alle OpenType-features zijn behouden.

| Bestand | Bron | Licentie |
|---|---|---|
| `WorkSans-{Light,Regular,Italic,Medium,SemiBold,Bold,BoldItalic}.woff2` | Work Sans v2.010 (https://github.com/weiweihuanghuang/Work-Sans) | SIL OFL 1.1, `OFL-WorkSans.txt` |
| `LibertinusSerif-Regular.woff2` | Libertinus v7.051 (https://github.com/alerque/libertinus) | SIL OFL 1.1, `OFL-LibertinusSerif.txt` (Reserved Font Names: "Linux Libertine", "Biolinum", "STIX Fonts"; die namen gebruiken we niet) |

**Larken** (koppen in de huisstijl) is commercieel en staat hier niet. Libertinus Serif is dezelfde fallback als in het Typst-template.

De `.woff2`-bestanden staan niet in git. Na het klonen: `mise run fonts` (of `uv run --script scripts/fonts.py`). Het script downloadt vaste versies, controleert SHA-256 en maakt de subsets. Idempotent; `--force` maakt alles opnieuw.
