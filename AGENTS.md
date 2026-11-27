# AGENTS.md

Korte instructie voor AI-codeeragents in deze repository.

Openbare studentensite van de opleiding Software Development (Firda Sneek): lessen, opdrachten, projectdocumentatie en voorbeeldcode. Gebouwd met mdBook en sli.dev, in de Firda-huisstijl.

Op `main` staat nog het skelet (README, licentie, CODEOWNERS). De build, het thema, de print-CSS, de voorbeelddecks en CI zitten in de open PR-stack #10 tot en met #13 (#9 is gemerged). De commando's hieronder gelden zodra die stack op `main` staat; tot die tijd werk je op die branches of erna.

Beoogde URL: <https://firda-sneek.github.io/software-developer/>. GitHub Pages gaat pas aan na expliciete ok van Sven.

## Commando's

Installeer tools met `mise install` (Java 21, Gradle, mdBook 0.5.4, Rust en rust-script) en daarna `npm ci` voor de decks.

| Task | Wat het doet |
| --- | --- |
| `mise run paginas` | Maakt ontbrekende Markdown-pagina's uit `src/SUMMARY.md` en bouwt de site |
| `mise run compile` | Compileert de voorbeeldprojecten in `voorbeelden/` |
| `mise run build` | Volledige lokale site: voorbeelden, mdBook, print-QR en decks |
| `mise run qr` | Zet de printfooter met QR in de gebouwde HTML (`scripts/print-qr.rs`) |
| `mise run decks` | Bouwt de sli.dev-decks naar `book/slides/` (roept `slidev build` direct aan; één regel per deck in `mise.toml`) |
| `mise run test-scripts` | Draait de unittests van de rust-scripts |

CI (`.github/workflows/bouw.yml`, in PR #13) draait `mise run build` en uploadt `book/` als Pages-artifact. De deploy-job staat bewust op `if: false`. Zet die niet zelf aan en zet Pages niet zelf aan.

Hulpscripts: alleen waar mdBook of mise het niet zelf kan, en dan als `rust-script`-bestand (`.rs`, afhankelijkheden in het `cargo`-blok in de doc-comment). Geen Python, geen `.sh` of `.ps1`. Blijf zo dicht mogelijk bij gewone mdBook: eigen tooling is een uitzondering.

## Structuur

- `src/SUMMARY.md`: bron van de structuur, met de hand. Een nieuwe pagina hoort daar eerst. Ontbreekt het bestand, dan maakt mdBook het bij de build (`create-missing = true`).
- `src/`: mdBook-bron (jaren, reeksen, projectopdrachten, projectdocumentatie).
- `theme/`: Firda-thema en de fonts (WOFF2 staan in git; mdBook neemt `theme/fonts/` zelf mee, met `hash-files = false`).
- `slides/`: sli.dev-decks (gebouwd met de mise-taak `decks`).
- `scripts/`: rust-scripts (nu `print-qr.rs`).
- `voorbeelden/`: voorbeeldprojecten (o.a. Kotlin).
- `book/`: build-output, niet committen.

Docentmateriaal hoort in de privé-repo [software-developer.docent](https://github.com/Firda-Sneek/software-developer.docent). Link vanuit deze publieke site niet naar die repo.

## Conventies

- Lesinhoud stem je af met Tess; verzin geen curriculum.
- Documentatie in Typst als je nieuwe docs toevoegt; lespagina's van de site zijn Markdown (mdBook).
- CODEOWNERS: elke wijziging vraagt een review van het team Docenten.
- Repo-namen volgen `prefix.kebab-case`.

## Niet aanraken

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
