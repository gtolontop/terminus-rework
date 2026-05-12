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
- C ou Espace : `cat`
- P : `pwd`
- M : `mv` prendre
- V : `mv` poser

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
