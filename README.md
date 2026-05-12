# terminus-rework

Rework de Terminus base sur deux sources :

- `gtolontop/Terminus`, le scrap HTML complet du jeu original avec solution et images.
- `terminus-rpg.zip`, le prototype Godot fourni dans `Downloads`.

## Structure

- `docs/terminus-game-logic.md` : document principal qui decrit la logique du jeu, les mecaniques, les etapes et la premiere partie a viser.
- `assets/original-terminus/` : assets recuperes depuis le Terminus HTML original.
- `assets/terminus-rpg/` : sprites du prototype Godot de rework.
- `source/terminus-rpg-godot/` : extraction propre du prototype Godot, sans cache `.godot`.

## Intention

La premiere passe sert a comprendre le jeu et poser une base propre. Rien n'est encore fige cote gameplay : on garde les sources, on documente les mecaniques, puis on pourra recreer une premiere partie jouable dans une architecture plus nette.
