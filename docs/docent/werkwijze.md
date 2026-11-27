# Werkwijze in deze repo

Dit document is voor Sven (en voor agents die hier meewerken). Het is de enige bron voor hoe we in deze repo werken: `AGENTS.md` en de skill (`.agents/skills/software-developer/SKILL.md`) verwijzen hiernaar en herhalen het niet.

Het staat bewust **niet** op de studentensite: het ligt buiten `src/` en staat niet in `src/SUMMARY.md`. Uitwerkingen, lesverloop en andere docentnotities horen in de privé-repo `software-developer.docent`, niet hier.

## 1. De repo in vogelvlucht

| Map of bestand | Wat |
| --- | --- |
| `src/` | De lespagina's (Markdown, mdBook). `src/SUMMARY.md` is de inhoudsopgave en wordt met de hand onderhouden, zoals mdBook het bedoelt. |
| `slides/` | De Slidev-decks (Markdown). Elk deck heeft een regel in de mise-taak `decks`. |
| `voorbeelden/` | De voorbeeldcode: echte, compileerbare projecten. Hier woont alle code die in pagina's en dia's komt. |
| `scripts/` | Rust-scripts: `regions.rs` (regions) en `print-qr.rs` (printfooter). Zie paragraaf 6. |
| `theme/` | Firda-thema en de fonts (WOFF2 staan in git; mdBook neemt ze zelf mee). |
| `docs/docent/` | Dit document. Niet op de site. |
| `.agents/skills/` | De repo-skill voor agents. |
| `.githooks/` | De pre-commit hook. |

## 2. Hoe we werken

- Werk op een branch en open een PR. De review door het team Docenten is verplicht (CODEOWNERS).
- Commits: Nederlandse tekst, `Refs #n` (nooit `Closes` of `Fixes`) en `Signed-off-by` (`git commit -s`).
- Agents mergen nooit zelf. Tags en releases doe jij.
- GitHub Pages en de deploy-job blijven uit tot jij ze aanzet.
- Lesinhoud stem je af met Tess. Verzin geen curriculum.
- Geen leerlinggegevens in de repo, ook niet in voorbeelden of screenshots. Alleen fictieve gegevens.

## 3. Code in losse bestanden, met regions

De regel: **voorbeeldcode staat nooit inline in Markdown.** Elk codevoorbeeld staat in een bronbestand onder `voorbeelden/`. Pagina's en dia's laten alleen het stukje zien dat je met een *region* aanwijst. Zo komt dezelfde code in de handout en in het deck, kan de compiler hem controleren en loopt er niets uit elkaar.

### Regions markeren

Zet in het bronbestand een commentaarregel vóór en na het stukje:

```kotlin
// #region greeting
fun greeting(name: String): String = "Hello, $name"
// #endregion greeting
```

```python
# region greeting
def greeting(name):
    return f"Hello, {name}"
# endregion greeting
```

Afspraken:

- **Eén conventie:** `#region naam` en `#endregion naam`. Bij `//`-talen (Kotlin, Rust, Java, JavaScript, TypeScript) is dat `// #region naam`; bij Python `# region naam`. Andere commentaarstijlen die Slidev kent (`/* #region naam */`, `<!-- #region naam -->`) werken ook.
- **Naam in het Engels**, zoals alle code: `greeting`, `loop-over-list`, `main-function`. Alleen letters, cijfers, `-` en `_`, beginnend met een letter. Geen spaties of punten.
- **Naam bij het einde.** Schrijf altijd `#endregion naam`, nooit kaal `#endregion`. Zo kloppen nesten en beide tools.
- **Uniek per bestand.** Regions mogen genest zijn, maar dezelfde naam komt maar één keer voor in een bestand.
- **Nooit een heel bestand includen** uit `voorbeelden/`. Dan zie je de markerregels. Wijs altijd een region aan.
- Geen `ANCHOR:`-markers meer. De check geeft er een fout op.

### Regions tonen

In een mdBook-pagina (pad relatief aan de pagina):

````markdown
```kotlin
{{#include ../../../voorbeelden/voorbeeld/hello/src/main/kotlin/voorbeeld/Hello.kt:greeting}}
```
````

Wil je deze syntax uitleggen in een pagina onder `src/`, zet dan een `\` voor `{{#include`, anders voert mdBook hem uit.

In een Slidev-deck (pad relatief aan het deck, taal erachter):

```markdown
<<< ../../voorbeelden/voorbeeld/hello/src/main/kotlin/voorbeeld/Hello.kt#greeting kotlin
```

Gebruik in decks **geen** `@/`: dat wijst naar de map van het deck, niet naar de repowortel.

### Hoe het werkt

- **Slidev** snapt `#region` zelf en haalt de markerregels weg.
- **mdBook** kent alleen `ANCHOR`. Daarom draait vóór mdBook een kleine preprocessor, `scripts/regions.rs` (rust-script), die `{{#include pad:naam}}` vult uit dezelfde `#region`-markers. Includes met regelnummers (`pad:2:10`) laat hij met rust; die doet mdBook zelf.
- Beide tools falen **stil** bij een fout: mdBook toont een leeg blok, Slidev toont het hele bestand met markerregels. Daarom is er een check (volgende paragraaf).

## 4. De check en de pre-commit hook

`mise run check-includes` leest alle verwijzingen in `src/` en `slides/` en controleert:

- bestaat het bestand?
- bestaat de region in dat bestand?
- zijn alle markers goed gevormd (afgesloten, uniek, geldige naam, geen `ANCHOR`)?
- wijst niets naar een bestand uit `voorbeelden/` zonder region?

Bij een fout stopt de check met exitcode 1. Daarnaast geeft hij waarschuwingen (geen fout) voor regions die nergens gebruikt worden en voor code die inline in Markdown staat.

De check draait op drie plekken:

1. in `mise run build` (hij is daar een afhankelijkheid van),
2. in CI (`.github/workflows/bouw.yml`, via `mise run build`),
3. bij elke `git commit`, via de hook in `.githooks/pre-commit`.

**Hook aanzetten** (eenmalig per clone):

```
mise install
mise run setup
```

`mise run setup` doet `git config core.hooksPath .githooks`. De hook is een minimaal bestand dat `mise run check-includes` aanroept; er staan geen `.sh`- of `.ps1`-scripts in de repo. Hij controleert de werkmap, niet alleen wat je gestaged hebt. Een commit voorbij de hook forceren (`--no-verify`) doen we niet; CI vangt het alsnog.

De unittests van het script draai je met `mise run test-scripts`.

## 5. Een nieuwe reeks of les toevoegen

1. Stem de inhoud af met Tess.
2. Maak de map `src/reeksen/<reeks>/` met een `index.md` en de pagina's. Zet ze met de hand in `src/SUMMARY.md`. Ontbreekt een bestand, dan maakt mdBook het bij de build (`create-missing = true`; `mise run paginas`).
3. Maak (of breid uit) het voorbeeldproject onder `voorbeelden/<reeks>/<project>/` en zet `#region`-markers om de stukjes die je laat zien. `mise run compile` moet blijven slagen.
4. Verwijs in de pagina met `{{#include …:naam}}` en in het deck met `<<< …#naam taal`. Zelfde region, geen kopie.
5. Deck nodig? Zet het in `slides/<reeks>/<naam>.md`, voeg een regel toe in de taak `decks` in `mise.toml` (kopieer de bestaande en pas naam en mappen aan) en link ernaar vanaf de reekspagina.
6. Draai `mise run build`. Kijk in `book/` en `book/slides/` of het fragment klopt, zonder markerregels en zonder lege blokken.
7. Commit met `Refs #n` en `Signed-off-by`, open een PR en vraag review.

## 6. Zo dicht mogelijk bij gewone mdBook

We gebruiken wat mdBook en mise zelf kunnen en schrijven alleen tooling waar dat echt niet anders kan:

- **Inhoudsopgave:** `src/SUMMARY.md` met de hand, `create-missing = true` in `book.toml`. Geen generator.
- **Fonts:** de WOFF2-bestanden staan in `theme/fonts/`; mdBook kopieert ze naar `book/fonts/`. `hash-files = false` in `book.toml` houdt de bestandsnamen vast, zodat `firda.css` ernaar kan verwijzen. Herkomst en SHA-256's staan in `theme/fonts/README.md`.
- **Decks:** geen script, maar de mise-taak `decks` die `slidev build` aanroept, één regel per deck.
- **Eigen tooling** is er alleen voor wat mdBook niet kan: `scripts/regions.rs` (preprocessor en include-check) en `scripts/print-qr.rs` (printfooter met QR).

Welke taal:

- **Standaard is Rust.** Alleen als JavaScript de logische keuze is (bijvoorbeeld in de browser of bij Slidev) gebruiken we JS. De lesvoorbeelden volgen uiteraard de taal van de les.
- **Losse hulptools** schrijven we als `rust-script`-bestand (`.rs`, met de afhankelijkheden in een `cargo`-blok in de doc-comment), tot `cargo script` stabiel is. `rust-script` en Rust komen via mise (`mise install`).
- **Geen Python-tooling meer** in deze repo (ook geen `uv`). Geen `.sh` of `.ps1`.
- Code, regionamen en identifiers zijn Engels. Meldingen en documentatie zijn Nederlands.

## 7. Als er iets misgaat

| Melding | Oorzaak en oplossing |
| --- | --- |
| `region 'x' bestaat niet (aanwezig: …)` | Typfout in de naam of de marker ontbreekt. Pas de verwijzing of het bronbestand aan. |
| `kan … niet lezen` / `bestand bestaat niet` | Het relatieve pad klopt niet. Tel de `../` vanaf de pagina of het deck. |
| `… wordt niet afgesloten` / `noem de regionaam bij het einde` | Zet `#endregion naam` onder het stukje. |
| `… wordt zonder region gebruikt` | Voeg `:naam` (mdBook) of `#naam` (Slidev) toe. |
| `rust-script: command not found` of `The command rust-script … wasn't found` (mdBook) | Draai `mise install` en start via `mise run …`. Voor `mdbook serve` of `mdbook build` los: `mise exec -- mdbook serve`, of activeer mise in je shell. |
