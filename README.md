# Software Developer

Studentensite van de opleiding Software Development, Firda Sneek.

De site komt op <https://firda-sneek.github.io/software-developer/>.

Dit is nog een skelet: er staat nog geen lesinhoud in.

## Lokaal bouwen

Je hebt [mise](https://mise.jdx.dev) nodig. Daarna:

    mise install
    npm ci
    mise run build

De gebouwde site staat in `book/`.

GitHub Pages is nog niet aangezet. De workflow bouwt de site en de voorbeelden wel, maar het deploy-job staat bewust uit (`if: false`) tot Pages in de repo-instellingen aan gaat (bron: GitHub Actions).
