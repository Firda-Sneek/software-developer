# AGENTS.md

Korte instructie voor AI-codeeragents in deze repository.

Openbare studentensite van de opleiding Software Development (Firda Sneek): lessen, opdrachten, projectdocumentatie en voorbeeldcode. Gebouwd met mdBook en sli.dev, in de Firda-huisstijl.

Op `main` staat nog het skelet (README, licentie, CODEOWNERS). De build, het thema, de print-CSS, de voorbeelddecks en CI zitten in de open PR-stack #10 tot en met #13 (#9 is gemerged). De commando's hieronder gelden zodra die stack op `main` staat; tot die tijd werk je op die branches of erna.

Beoogde URL: <https://firda-sneek.github.io/software-developer/>. GitHub Pages gaat pas aan na expliciete ok van Sven.

## Commando's

Installeer tools met `mise install` (Java 21, Gradle, mdBook 0.5.4, Rust en rust-script) en daarna `npm ci` voor de decks. Zet daarna eenmalig de pre-commit hook aan met `mise run setup`.

| Task | Wat het doet |
| --- | --- |
| `mise run paginas` | Maakt ontbrekende Markdown-pagina's uit `src/SUMMARY.md` en bouwt de site |
| `mise run compile` | Compileert de voorbeeldprojecten in `voorbeelden/` |
| `mise run build` | Volledige lokale site: voorbeelden, include-check, mdBook, print-QR en decks |
| `mise run qr` | Zet de printfooter met QR in de gebouwde HTML (`scripts/print-qr.rs`) |
| `mise run decks` | Bouwt de sli.dev-decks naar `book/slides/` (roept `slidev build` direct aan; één regel per deck in `mise.toml`) |
| `mise run check-includes` | Controleert alle `#region`-verwijzingen in `src/` en `slides/` (exit 1 bij een fout) |
| `mise run test-scripts` | Draait de unittests van de rust-scripts (`regions.rs`, `print-qr.rs`) |
| `mise run setup` | Zet de pre-commit hook aan (`git config core.hooksPath .githooks`) |

CI (`.github/workflows/bouw.yml`, in PR #13) draait `mise run build` en uploadt `book/` als Pages-artifact. De deploy-job staat bewust op `if: false`. Zet die niet zelf aan en zet Pages niet zelf aan.

Hulpscripts: alleen waar mdBook of mise het niet zelf kan, en dan als `rust-script`-bestand (`.rs`, afhankelijkheden in het `cargo`-blok in de doc-comment). Geen Python, geen `.sh` of `.ps1`. Blijf zo dicht mogelijk bij gewone mdBook: eigen tooling is een uitzondering. De mdBook-preprocessor en include-check voor regions (`scripts/regions.rs`) en de printfooter (`scripts/print-qr.rs`) zijn de enige uitzonderingen.

De include-check draait ook in `mise run build`, in CI en bij elke `git commit` (hook in `.githooks/pre-commit`). Omzeil hem niet met `--no-verify`.

## Structuur

- `src/SUMMARY.md`: bron van de structuur, met de hand. Een nieuwe pagina hoort daar eerst. Ontbreekt het bestand, dan maakt mdBook het bij de build (`create-missing = true`).
- `src/`: mdBook-bron (jaren, reeksen, projectopdrachten, projectdocumentatie).
- `theme/`: Firda-thema en de fonts (WOFF2 staan in git; mdBook neemt `theme/fonts/` zelf mee, met `hash-files = false`).
- `slides/`: sli.dev-decks (gebouwd met de mise-taak `decks`).
- `scripts/`: rust-scripts: `regions.rs` (mdBook-preprocessor en include-check voor regions) en `print-qr.rs` (printfooter).
- `voorbeelden/`: voorbeeldprojecten (o.a. Kotlin). Alle code die in pagina's of dia's komt staat hier, nooit inline in Markdown.
- `docs/docent/werkwijze.md`: de werkwijze voor Sven. Enige bron; niet op de site.
- `.agents/skills/software-developer/SKILL.md`: stappenplannen voor terugkerend werk. Lees die voordat je aan lessen, voorbeeldcode of de build werkt.
- `.githooks/`: de pre-commit hook.
- `book/`: build-output, niet committen.

Docentmateriaal hoort in de privé-repo [software-developer.docent](https://github.com/Firda-Sneek/software-developer.docent). Link vanuit deze publieke site niet naar die repo.

## Code delen tussen handout en deck

Voorbeeldcode staat in een bronbestand onder `voorbeelden/` en markeert wat getoond mag worden met `// #region naam` … `// #endregion naam` (Python-voorbeelden in de les: `# region naam` … `# endregion naam`). Regionamen zijn Engels en uniek per bestand, het einde draagt altijd de naam.

- mdBook: `{{#include pad:naam}}` (pad relatief aan de pagina).
- Slidev: `<<< pad#naam taal` (pad relatief aan het deck, niet `@/`).

Include nooit een heel bestand uit `voorbeelden/`. Details, voorbeelden en foutmeldingen: `docs/docent/werkwijze.md`.

## Conventies

- Lesinhoud stem je af met Tess; verzin geen curriculum.
- Documentatie in Typst als je nieuwe docs toevoegt; lespagina's van de site zijn Markdown (mdBook). Uitzondering: `AGENTS.md`, de skill en `docs/docent/werkwijze.md` zijn Markdown.
- Code in het Engels (regionamen, identifiers). Standaardtaal voor nieuwe code en scripts is Rust, JavaScript alleen als dat logischer is. Lesvoorbeelden volgen de taal van de les.
- CODEOWNERS: elke wijziging vraagt een review van het team Docenten.
- Repo-namen volgen `prefix.kebab-case`.

## Niet aanraken

- De check in `scripts/regions.rs` uitzetten of omzeilen om een kapotte verwijzing te verbergen. Los de verwijzing op.
- Gegenereerde output: `book/`, `node_modules/`, `.gradle/`, `**/build/`.
- De inactieve Pages-deploy-job.
- Echte leerlinggegevens: nooit in de repo, testdata of screenshots. Alleen fictieve gegevens.
- Nooit naar Eduarte schrijven.
- Geen wachtwoorden, tokens of codes in de chat of in de repo.
- Geen tags of releases maken; dat doet Sven.
- Niets openbaar maken of Pages aanzetten zonder expliciete ok van Sven.

## Pull requests

- Branch vanaf `main`, tenzij je bewust op de fase-0-stack voortbouwt.
- Commits: `Refs #n` (nooit `Closes` of `Fixes`), en altijd `Signed-off-by` (`git commit -s`).
- GitHub-werk van Grok Bot loopt via het account **Imbotz**, zodat Sven als controleur kan goedkeuren.
- Review en merge door Sven of een docent (verplichte review door het team Docenten). Nooit zelf mergen.
