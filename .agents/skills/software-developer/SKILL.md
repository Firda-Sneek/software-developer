---
name: software-developer
description: Werken in de repo Firda-Sneek/software-developer (studentensite met mdBook en Slidev). Gebruik dit bij een nieuwe reeks, les of handout, bij voorbeeldcode die in een pagina én een deck moet komen (regions), bij bouwen en controleren van de site, en bij het openen van een PR in deze repo.
---

# Skill: software-developer

Stappenplan voor terugkerend werk in deze repo. De uitleg en de reden achter de regels staan in `docs/docent/werkwijze.md`; lees dat eerst als iets onduidelijk is. Houd je aan de vaste regels in `AGENTS.md`.

## Vooraf

- Lesinhoud verzin je niet. Stem af met Tess of Sven.
- Geen leerlinggegevens, nooit. Alleen fictieve data.
- Code, regionamen en identifiers in het Engels. Standaardtaal voor nieuwe code en scripts is Rust (JS alleen als dat logischer is). Losse hulptools zijn `rust-script`-bestanden.
- Draai commando's via `mise run …` vanuit de repowortel. Eerst `mise install` en `npm ci`.
- Hook aan? Controleer met `git config core.hooksPath`. Staat die niet op `.githooks`, draai `mise run setup`.

## Workflow A: een reeks of pagina toevoegen

1. Maak `src/reeksen/<reeks>/` met `index.md` en de pagina's. Geen YAML-frontmatter in de Markdown.
2. Voeg de pagina's toe aan `catalogus.yaml`.
3. `mise run summary` (nooit `src/SUMMARY.md` met de hand).
4. `mise run build` en kijk of de pagina in `book/` staat.

## Workflow B: voorbeeldcode in een pagina én een deck

Code staat nooit inline in Markdown. Altijd een bronbestand met een region.

1. Zet of vind de code in `voorbeelden/<reeks>/<project>/…` (compileerbaar project).
2. Omring het stukje met markers, naam in het Engels, eind met naam:
   - `// #region greeting` … `// #endregion greeting` (Kotlin, Rust, Java, JS, TS)
   - `# region greeting` … `# endregion greeting` (Python)
3. Pagina (pad relatief aan de pagina), binnen een codeblok met de juiste taal:
   `{{#include ../../../voorbeelden/<reeks>/<project>/src/main/kotlin/…/Bestand.kt:greeting}}`
4. Deck (pad relatief aan het deck, niet `@/`): `<<< ../../voorbeelden/<reeks>/<project>/…/Bestand.kt#greeting kotlin`
5. `mise run compile` (moet slagen), `mise run check-includes`, `mise run build`.
6. Controleer in `book/<pagina>.html` en `book/slides/…` dat alleen het fragment staat: geen `#region`-regels, geen leeg blok.

Niet doen: een heel bestand uit `voorbeelden/` includen, `ANCHOR:` gebruiken, kaal `#endregion` schrijven, dezelfde regionaam twee keer in één bestand.

## Workflow C: een region hernoemen, verplaatsen of weghalen

1. Zoek alle verwijzingen: `rg "naam" src slides voorbeelden`.
2. Pas bronbestand, pagina's en dia's samen aan.
3. `mise run check-includes`. Een fout is een gemiste verwijzing; een waarschuwing "nergens gebruikt" is een overgebleven region.

## Workflow D: controleren voor een commit

1. `mise run check-includes` (draait ook in de pre-commit hook, in `mise run build` en in CI).
2. `mise run test-scripts` als je `scripts/regions.rs` hebt aangepast.
3. `mise run build`.

Geen `--no-verify`. Los de fout op.

## Workflow E: commit en PR

1. Branch vanaf `main` (of op de fase-0-stack als die nog niet op `main` staat; noem dat in de PR).
2. Commit in het Nederlands, met `Refs #n` (nooit `Closes` of `Fixes`) en `Signed-off-by` (`git commit -s`). Geen issue? Maak er een.
3. Open een PR, vraag review van `SvenImholz`. GitHub-werk van Grok Bot loopt via Imbotz.
4. Nooit zelf mergen. Geen tags of releases. Pages en de deploy-job blijven uit.

## Als de check faalt

| Melding | Doen |
| --- | --- |
| `region 'x' bestaat niet` | Typfout of ontbrekende marker. Pas aan, nooit de check uitzetten. |
| `bestand bestaat niet` | Relatief pad fout; tel de `../`. |
| `niet afgesloten` / `noem de regionaam` | Zet `#endregion naam` onder het stukje. |
| `zonder region gebruikt` | Voeg `:naam` (mdBook) of `#naam` (Slidev) toe. |
| `rust-script: command not found` | `mise install`, en draai via `mise run …`. |

## Niet aanraken

`book/`, `node_modules/`, `.gradle/`, `**/build/`, `src/SUMMARY.md` (alleen via `mise run summary`), de inactieve Pages-deploy-job. Uitwerkingen en lesverloop horen in de privé-repo `software-developer.docent`; link daar niet naar vanaf de site.
