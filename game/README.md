# Terminus Rework - Rust Prototype

Prototype jouable de la premiere partie, ecrit en Rust avec `macroquad`.

## Lancer

Depuis la racine du repo :

```bash
cargo run --manifest-path game/Cargo.toml
```

Ou depuis `game/` :

```bash
cargo run
```

## Controles

- Fleches ou WASD : deplacement
- Entree/Espace : valider menu, intro, dialogues
- Les autres actions sont affichees en bas dans `Possible ici`.
- Le jeu n'affiche `pwd`, `mv`, les sorties et les interactions que quand elles sont vraiment disponibles.

## Tranche implementee

- Menu
- Intro
- Depart
- Palourde qui apprend `pwd`
- Affichage du lieu courant avec `pwd`
- Bois des lutins et panneau
- Academie
- Cours avec Professeur qui apprend `mv`
- Salle d'entrainement
- Piliers deplacables avec `mv`
- Boite qui valide la premiere partie quand les trois piliers sont ranges
