# Terminus Rework - logique du jeu et premiere partie

## Objectif de ce document

Ce document sert de base de travail pour le rework. Il rassemble la logique du Terminus original, ce qui existe deja dans le prototype Godot `terminus-rpg.zip`, et une proposition claire pour coder la premiere partie sans partir dans tous les sens.

Sources analysees :

- depot original HTML : `https://github.com/gtolontop/Terminus`
- prototype Godot local : `C:\Users\teamr\Downloads\terminus-rpg.zip`
- extraction propre : `source/terminus-rpg-godot/`
- reserve d'assets : `assets/`

## Idee centrale

Terminus est un jeu d'aventure qui transforme les commandes de terminal en mecaniques de RPG.

Dans le jeu original, le monde fonctionne comme un faux systeme de fichiers :

- les lieux sont des dossiers ;
- les personnages et objets sont des fichiers ;
- `cd` sert a se deplacer ;
- `ls` sert a voir les sorties et objets disponibles ;
- `cat` sert a lire, parler et obtenir des indices ;
- de nouvelles commandes sont apprises en parlant a des PNJ ou en lisant des objets ;
- chaque commande debloque ensuite une action concrete dans le monde.

Le rework Godot transpose cette idee en vue RPG 2D :

- le joueur se deplace physiquement dans une map ;
- les zones de collision remplacent une partie des `cd` ;
- les touches remplacent les commandes tapees ;
- les sorts/commandes apprises sont stockes dans une liste globale ;
- certains objets deviennent interactifs seulement si la commande correspondante est apprise.

## Boucle de gameplay

La boucle principale a garder est simple :

1. Explorer une zone.
2. Trouver un PNJ, panneau, objet ou passage.
3. Interagir avec la bonne commande.
4. Apprendre une nouvelle commande ou ouvrir un nouveau chemin.
5. Revenir dans une zone precedente avec cette commande pour resoudre une enigme.

Cette boucle est importante parce que Terminus n'est pas juste un jeu de dialogue. Chaque commande est a la fois une lecon informatique et une cle de progression.

## Commandes et equivalents RPG

| Commande | Sens original | Equivalent rework |
|---|---|---|
| `cd` | changer de dossier | changer de scene en traversant une sortie |
| `ls` | lister le contenu | afficher/identifier objets et sorties visibles |
| `cat` | lire un fichier / parler | interaction principale avec PNJ, panneaux et objets |
| `pwd` | afficher le lieu courant | afficher le nom de la scene actuelle |
| `mv` | deplacer un fichier/objet | attraper, porter, deposer un objet ou PNJ |
| `rm` | supprimer | enlever un obstacle |
| `mkdir` | creer un dossier | construire/faire apparaitre une zone |
| `touch` | creer un fichier | creer un objet simple |
| `cp` | copier | dupliquer un objet |
| `grep` | chercher dans des fichiers | trouver un indice/mot de passe dans une masse d'infos |
| `sudo` | executer avec droits admin | action finale protegee par mot de passe |

Pour la premiere partie, le prototype Godot implemente surtout `cat`, `pwd` et `mv`.

## Etat global dans le prototype Godot

Le script central est `source/terminus-rpg-godot/scripts/general.gd`.

Il garde :

- la position du joueur (`playerpos`) ;
- la scene courante (`current_scene`) ;
- les limites de camera (`cameralimit`) ;
- les commandes/sorts appris (`sorts`) ;
- les positions persistantes du professeur et des piliers ;
- l'objet actuellement attrape par `mv` ;
- les objets ranges dans la boite.

La liste de depart est :

```text
["cd", "cat", "ls"]
```

Ensuite :

- parler a la Palourde ajoute `pwd` ;
- parler au Professeur ajoute `mv`.

## Scenes existantes dans le prototype Godot

| Scene | Role actuel |
|---|---|
| `menu.tscn` | ecran de depart du prototype |
| `intro.tscn` | sequence d'intro avec etapes au clavier |
| `depart.tscn` | zone de depart, Palourde, sorties vers Prairie et Bois |
| `palourdekiparle.tscn` | dialogue apres interaction avec Palourde |
| `prairie.tscn` | zone laterale reliee au depart |
| `boisdeslutins.tscn` | zone avec panneau et acces a l'academie |
| `lirepanneau.tscn` | lecture du panneau |
| `academiedesbots.tscn` | hub de l'academie |
| `cours.tscn` | scene de cours avec professeur |
| `professeurparle.tscn` | dialogue du professeur |
| `salledentrainement.tscn` | salle d'entrainement avec boite et piliers |
| `player.tscn` | joueur, camera, animations, UI de commandes |
| `pilier1/2/3.tscn` | objets manipulables via `mv` |
| `boite.tscn` | zone qui peut stocker des objets portes |

## Flow de la premiere partie

La premiere partie peut etre definie comme tout ce qui va du menu jusqu'a l'apprentissage de `mv` et la resolution de la salle d'entrainement.

### 1. Menu

Le joueur commence sur `menu.tscn`.

Action attendue :

- cliquer/jouer lance `intro.tscn`.

But pour le rework :

- avoir un menu simple, lisible, avec lancement de partie ;
- eviter de bloquer le joueur avec trop de texte.

### 2. Intro

Dans `intro.gd`, une variable `etape` controle l'avancement.

Chaque pression sur `enter` passe a l'etape suivante :

1. personnage qui parle ;
2. demande de nom ;
3. demande d'adresse ;
4. tutoriel ;
5. transition vers `depart.tscn`.

But pour le rework :

- garder le principe d'intro progressive ;
- clarifier le texte ;
- stocker plus tard le nom/adresse si on veut les reutiliser.

### 3. Depart

Le joueur arrive en scene `depart`.

Elements :

- joueur ;
- Palourde ;
- sortie vers `prairie` ;
- sortie vers `boisdeslutins`.

Interaction cle :

- si le joueur est dans la zone de la Palourde et appuie sur `cat`, il apprend `pwd` puis passe a `palourdekiparle.tscn`.

Effet de gameplay :

- `pwd` devient disponible ;
- le joueur comprend que les commandes sont des sorts.

### 4. Commande `pwd`

Le script `pwdtext.gd` affiche ou masque le nom de la scene courante.

Condition :

- touche `pwd` pressee ;
- `pwd` present dans `general.sorts`.

Effet :

- affiche `general.current_scene.to_upper()`.

But pour le rework :

- garder `pwd` comme feedback de localisation ;
- plus tard, afficher un nom de lieu propre plutot que le nom technique de scene.

### 5. Bois des lutins

Depuis `depart`, une zone de collision appelle `general.departtobois()`.

Cette fonction :

- change `current_scene` vers `boisdeslutins` ;
- place le joueur a la bonne position ;
- met a jour les limites camera ;
- recharge les objets persistants.

Dans le bois :

- un panneau peut etre lu ;
- une sortie retourne au depart ;
- une sortie mene a l'academie.

But pour le rework :

- utiliser cette zone comme transition vers la vraie premiere enigme ;
- garder le panneau comme tuto supplementaire ou indice.

### 6. Academie des bots

`academiedesbots.tscn` sert de petit hub.

Sorties :

- retour vers Bois des lutins ;
- acces au Cours ;
- acces a la Salle d'entrainement.

But :

- amener le joueur au professeur ;
- montrer ensuite une salle d'application pour `mv`.

### 7. Cours et apprentissage de `mv`

Le professeur est instancie si `general.profdansscene == current_scene`.

Interaction :

- le joueur entre dans sa zone ;
- `general.inboxprof` passe a `true` ;
- si le joueur appuie sur `cat` et qu'aucun objet n'est porte, `mv` est ajoute a `general.sorts` ;
- le jeu lance `professeurparle.tscn`.

Effet :

- `mv` devient utilisable ;
- le professeur devient manipulable si le joueur appuie sur la touche de capture.

But pour le rework :

- faire du professeur le moment ou le joueur comprend que les commandes peuvent changer le monde ;
- mieux expliquer `mv` avant la salle d'entrainement.

### 8. Salle d'entrainement

La salle contient :

- trois piliers ;
- une boite ;
- une sortie vers l'academie.

Les piliers ont chacun un script (`pilier.gd`, `pilier_2.gd`, `pilier_3.gd`).

Logique :

- si le joueur est proche d'un pilier, un flag global `inboxpilierX` passe a `true` ;
- si `mv` est appris et que le joueur capture, `general.fmvcatch("pilierX")` stocke l'objet attrape ;
- le sprite porte par le joueur devient visible ;
- a la relache, `general.fmvrelease(...)` repositionne le pilier dans la scene courante ;
- si le joueur est dans la boite, l'objet est ajoute a `boiteliste` au lieu d'etre repose dans la scene.

But de l'enigme :

- utiliser `mv` pour retirer/ranger les piliers ;
- valider que le joueur comprend l'action "prendre/deposer".

## Points techniques importants

### Persistence entre scenes

Le prototype ne garde pas les scenes chargees en memoire. Il reconstruit certains objets a chaque chargement via `general.charger_scene()`.

Pour chaque objet persistant, il stocke :

- sa position ;
- la scene ou il doit apparaitre ;
- s'il est dans la boite ou non.

C'est simple et suffisant pour une premiere partie, mais a long terme il faudra probablement un vrai systeme de `GameState`.

### Inputs actuels

Les inputs sont definis dans `project.godot` :

- `enter` : valider/dialogue ;
- `cat` : espace ou C ;
- `pwd` : P ;
- `mvcatch` : M ;
- `mvrelease` : V ;
- fleches : deplacement.

Pour le rework, il faudra rendre ces controles plus lisibles en UI.

### Bugs / points a surveiller

- `general.charger_scene()` verifie `"pilier2" not in boiteliste` pour afficher `pilier3`, ce qui ressemble a une erreur de condition.
- `fmvcatch("boite")` prend le dernier objet de `boiteliste`, donc la boite fonctionne comme une pile.
- `peut_attraper` est remis a `true` seulement dans certaines conditions (`inboxprof` ou `inboxpilier1`), donc les piliers 2/3 peuvent avoir un comportement incomplet.
- les noms techniques de scenes sont affiches directement par `pwd`.
- les dialogues sont separes en scenes entieres, ce qui marche mais peut devenir lourd quand le jeu grossit.

## Architecture conseillee pour la vraie premiere partie

Quand on commencera a coder proprement, viser cette separation :

- `game/state/` : etat global, sorts appris, progression ;
- `game/player/` : mouvement, interaction, objet porte ;
- `game/interactions/` : composants `CatInteractable`, `MvMovable`, `SceneExit` ;
- `game/dialogue/` : textes et UI dialogue ;
- `game/levels/part1/` : depart, bois, academie, cours, salle entrainement ;
- `assets/` : sprites nettoyes et classes par usage.

Le prototype actuel peut etre garde comme reference, mais le code final devrait eviter de mettre toute la logique dans un seul singleton.

## Definition de "premiere partie terminee"

On pourra considerer la premiere partie terminee quand :

1. le joueur lance la partie depuis le menu ;
2. l'intro passe proprement ;
3. le joueur arrive au depart ;
4. il parle a la Palourde et apprend `pwd` ;
5. `pwd` affiche le lieu courant ;
6. il rejoint le Bois des lutins ;
7. il entre dans l'Academie ;
8. il parle au Professeur et apprend `mv` ;
9. il va en Salle d'entrainement ;
10. il utilise `mv` sur les piliers ;
11. la salle reconnait que l'enigme est resolue ;
12. le jeu donne un feedback clair de fin de premiere partie.

## Prochaine etape

Coder cette premiere partie en partant du prototype, mais en nettoyant progressivement :

- garder les assets utiles ;
- transformer les interactions en composants reutilisables ;
- centraliser la progression ;
- remplacer les noms bruts par des labels propres ;
- ajouter un feedback UI quand une commande est apprise.
