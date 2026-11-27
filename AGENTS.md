# AGENTS.md

Korte instructie voor AI-codeeragents in deze repository.

Openbare studentensite van de opleiding Software Development (Firda Sneek): lessen, opdrachten, projectdocumentatie en voorbeeldcode. Gebouwd met mdBook en sli.dev, in de Firda-huisstijl.

Op `main` staat nog het skelet (README, licentie, CODEOWNERS). De build, het thema, de print-CSS, de voorbeelddecks en CI zitten in de open PR-stack #9 tot en met #13. De commando's hieronder gelden zodra die stack op `main` staat; tot die tijd werk je op die branches of erna.

Beoogde URL: <https://firda-sneek.github.io/software-developer/>. GitHub Pages gaat pas aan na expliciete ok van Sven.

## Commando's

Installeer tools met `mise install` (Python, uv, Java 21, Gradle, mdBook 0.5.4) en daarna `npm ci` voor de decks.

| Task | Wat het doet |
| --- | --- |
| `mise run fonts` | Downloadt en subset de Firda-fonts naar `theme/fonts/` |
| `mise run paginas` | Maakt ontbrekende Markdown-pagina's uit `src/SUMMARY.md` en bouwt de site |
| `mise run compile` | Compileert de voorbeeldprojecten in `voorbeelden/` |
| `mise run build` | Volledige lokale site: fonts, voorbeelden, mdBook, print-QR en decks |
| `mise run qr` | Zet de printfooter met QR in de gebouwde HTML |
| `mise run decks` | Bouwt de sli.dev-decks naar `book/slides/` |

CI (`.github/workflows/bouw.yml`, in PR #13) draait `mise run build` en uploadt `book/` als Pages-artifact. De deploy-job staat bewust op `if: false`. Zet die niet zelf aan en zet Pages niet zelf aan.

Hulpscripts: Python met PEP 723, `uv run --script`. Geen `.sh` of `.ps1`.

## Structuur

- `src/SUMMARY.md`: bron van de structuur, met de hand. Een nieuwe pagina hoort daar eerst. Ontbreekt het bestand, dan maakt mdBook het bij de build (`create-missing = true`).
- `src/`: mdBook-bron (jaren, reeksen, projectopdrachten, projectdocumentatie).
- `theme/`: Firda-thema en gegenereerde fonts.
- `slides/` en scripts voor sli.dev-decks.
- `voorbeelden/`: voorbeeldprojecten (o.a. Kotlin).
- `book/`: build-output, niet committen.

Docentmateriaal hoort in de privé-repo [software-developer.docent](https://github.com/Firda-Sneek/software-developer.docent). Link vanuit deze publieke site niet naar die repo.

## Conventies

- Lesinhoud stem je af met Tess; verzin geen curriculum.
- Documentatie in Typst als je nieuwe docs toevoegt; lespagina's van de site zijn Markdown (mdBook).
- CODEOWNERS: elke wijziging vraagt een review van het team Docenten.
- Repo-namen volgen `prefix.kebab-case`.

## Niet aanraken

- Gegenereerde output: `book/`, `node_modules/`, `.gradle/`, `**/build/`, gegenereerde `*.woff2`.
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
