# Fonts

Zelf gehost als WOFF2, gesubset op Latijn. Alle OpenType-features zijn behouden. De bestanden staan in git, zodat mdBook ze zelf meeneemt (`theme/fonts/` wordt naar `book/fonts/` gekopieerd) en de build geen download of extra tooling nodig heeft.

| Bestand | Bron | Licentie |
|---|---|---|
| `WorkSans-{Light,Regular,Italic,Medium,SemiBold,Bold,BoldItalic}.woff2` | Work Sans v2.010 (https://github.com/weiweihuanghuang/Work-Sans) | SIL OFL 1.1, `OFL-WorkSans.txt` |
| `LibertinusSerif-Regular.woff2` | Libertinus v7.051 (https://github.com/alerque/libertinus) | SIL OFL 1.1, `OFL-LibertinusSerif.txt` (Reserved Font Names: "Linux Libertine", "Biolinum", "STIX Fonts"; die namen gebruiken we niet) |

**Larken** (koppen in de huisstijl) is commercieel en staat hier niet. Libertinus Serif is dezelfde fallback als in het Typst-template.

`firda.css` verwijst naar `../../fonts/…` (vanaf `book/theme/css/`). Daarom staat `hash-files = false` in `book.toml`: met hashes in de bestandsnamen zou die verwijzing niet meer kloppen.

## Herkomst (om de bestanden opnieuw te maken)

- Work Sans: tag v2.010, commit `6634e7b2395e9dba4ac8611c4529968a9719d606`, map `fonts/static/TTF`.
- Libertinus: release-zip v7.051, bestand `Libertinus-7.051/static/OTF/LibertinusSerif-Regular.otf` (de OTF bevat de ligaturen).
- SHA-256 van de bron-TTF's van Work Sans:
  - Light `16d4241f82605d7a104e49ace6b11c07c1ed4e5b70bbdc6cd88311a3152665eb`
  - Regular `22e7f1607ebc29d03be61d893ec47dde307847eaf60fbec260e286695001982a`
  - Italic `8fcbc2499374a09594ff28ed488852191f3451ec2ef430d421ec327b3d6c15e8`
  - Medium `3c5ae1ef9260a0c1cdf1f59841f28620c40826f6ca3bd4c71516997c29fec7dc`
  - SemiBold `399b8be04df8e5c531cc1cf490d9d4d781299440e086d61097db5175e2c71321`
  - Bold `e5c36b1191be27046270da352f231fc342ad4333cfb2967f393e00d18c274cdc`
  - BoldItalic `8f7b9e47221fab81026d48d1d746f3e8cbad4b045524c09645d052efd181ca19`
- SHA-256 van de Libertinus-zip: `4d9be29b5cb380c35af8ba967abcc752ad1e07be1f738a9789c33e0dd7478c92`.
- Subset met fonttools (`pyftsubset`), `--layout-features=*`, `--flavor=woff2` en deze Unicode-bereiken: `U+0000-00FF,U+0100-017F,U+0131,U+0152-0153,U+02BB-02BC,U+02C6,U+02DA,U+02DC,U+2000-206F,U+20AC,U+2122,U+2190-2193,U+2212,U+2215,U+FEFF,U+FFFD`.
