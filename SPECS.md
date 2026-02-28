# Système de Caisse (POS) — Spécifications

## Présentation

Une application de caisse (Point de Vente) légère pour un stand de convention, construite avec Tauri v2 (backend Rust) et Svelte 5. Le système gère les ventes de snacks et de boissons. Il ne gère **aucun** traitement de paiement.

Les données sont stockées localement dans SQLite (géré par Tauri). Aucune base de données distante ni connexion internet n'est requise.

---

## Fonctionnalités

### 1. Gestion des produits

Un écran pour gérer le catalogue de produits disponibles à la vente.

- Chaque produit possède : **nom**, **prix** (en centimes, affiché en euros), **catégorie** (`snack` ou `drink`), un **emoji** optionnel, et un booléen **disponible**.
- Les produits peuvent être ajoutés, modifiés et activés/désactivés.
- Les produits indisponibles n'apparaissent pas sur l'écran de vente.
- Les produits ne peuvent pas être supprimés (pour préserver l'intégrité de l'historique des transactions), seulement marqués comme indisponibles.

### 2. Écran de vente (Nouvelle commande)

L'écran principal utilisé pendant l'événement pour créer et soumettre des commandes.

- Affiche tous les produits **disponibles** sous forme de grille de boutons, chacun montrant le nom du produit et son prix.
- Appuyer sur un produit ajoute une unité à la commande en cours.
- La commande en cours est affichée à côté de la grille de produits, montrant :
    - Chaque produit sélectionné avec son nom, prix unitaire, quantité et sous-total.
    - Le **montant total** de la commande.
- Pour chaque article de la commande en cours, l'utilisateur peut :
    - **Augmenter** la quantité (+1).
    - **Diminuer** la quantité (-1, supprimant la ligne quand elle atteint 0).
- Un bouton **Encaisser** ouvre une étape d'encaissement où l'utilisateur sélectionne un mode de paiement (**espèces** ou **carte**).
    - Pour les paiements en espèces, l'utilisateur peut saisir le montant reçu et voir la **monnaie** à rendre.
    - La confirmation de l'encaissement enregistre la commande comme une transaction terminée (incluant le mode de paiement).
    - Après l'encaissement, la commande en cours est effacée et une nouvelle peut être commencée.
    - Le mode de paiement est enregistré uniquement à des fins de reporting — aucun traitement de paiement réel n'est effectué.
- Un bouton **Effacer** réinitialise la commande en cours sans rien enregistrer.

### 3. Tableau de bord (Résumé des ventes)

Un écran en lecture seule affichant un résumé de toutes les transactions terminées.

- **Tableau récapitulatif par produit** montrant, pour chaque produit vendu au moins une fois :
    - Nom du produit.
    - Quantité totale vendue.
    - Chiffre d'affaires total (quantité x prix unitaire au moment de la vente).
- **Total général** de tous les revenus de ventes.
- **Ventilation par mode de paiement** (espèces vs. carte).
- **Nombre total de transactions** terminées.

### 4. Journal des transactions

Un écran (ou section dans le tableau de bord) listant toutes les transactions terminées.

- Chaque entrée affiche : horodatage, nombre d'articles, montant total et mode de paiement.
- Permet d'**exporter** le journal complet des transactions (par ex. en CSV ou JSON).

### 5. Inventaire basique (Optionnel)

Suivi optionnel du stock pour les produits.

- Définir un **stock initial** par produit.
- Le stock diminue automatiquement à chaque vente.
- Afficher un **avertissement de stock bas** lorsque la quantité restante d'un produit descend en dessous d'un seuil configurable.

---

## Modèle de données

### Product

| Champ     | Type    | Description                                              |
|-----------|---------|----------------------------------------------------------|
| id        | string  | Identifiant unique (UUID ou similaire)                   |
| name      | string  | Nom affiché                                              |
| price     | number  | Prix en centimes (entier)                                |
| category  | string  | `"snack"`, `"soft_drink"`, `"alcohol"`, `"sweets"`       |
| available | boolean | Si le produit est en vente                               |

### Order

| Champ | Type     | Description                                    |
|-------|----------|------------------------------------------------|
| id    | string   | Identifiant unique (UUID ou similaire)         |
| items | relation | Liste de OrderItem liés à cette commande       |
| total | number   | Valeur totale de cette commande                |

### OrderItem

| Champ       | Type   | Description                                                                         |
|-------------|--------|-------------------------------------------------------------------------------------|
| id          | string | Identifiant unique (UUID ou similaire)                                              |
| productId   | string | Référence à un Product                                                              |
| productName | string | Nom du produit au moment de la vente                                                |
| unitPrice   | number | Prix en centimes au moment de la vente                                              |
| quantity    | number | Nombre d'unités                                                                     |
| total       | number | Prix total de cet article (prix unitaire multiplié par la quantité, en centimes)    |

---

## Navigation

L'application a quatre routes principales :

| Route        | Écran              |
|--------------|--------------------|
| `/`          | Écran de vente     |
| `/products`  | Gestion produits   |
| `/orders`    | Commandes          |
| `/dashboard` | Tableau de bord    |

Une barre de navigation persistante permet de basculer entre les écrans.

---

## Contraintes

- Tous les prix sont stockés et calculés comme des **entiers en centimes** pour éviter les erreurs de virgule flottante. Affichés en euros (ex. `150` -> `1,50 EUR`).
- Pas d'authentification, pas de comptes utilisateur.
- Pas de traitement de paiement. Le mode de paiement (espèces/carte) est enregistré uniquement pour le reporting.
- Aucune connexion internet requise pour les fonctionnalités principales.
- L'application est empaquetée comme un binaire de bureau via Tauri. Toutes les données restent locales.
- L'interface doit être utilisable sur tablette (grandes zones tactiles, polices lisibles).
- Les données doivent être stockées dans une base SQLite sauvegardée à côté du binaire.

---

## Produits par défaut

- Boissons sans alcool :
  - Thé, café : 1 EUR
  - Soda, jus de fruit : 2 EUR
- Boissons alcoolisées :
  - Bière (pichet) : 12 EUR
  - Bière (25cl) : 3 EUR
  - Cidre (doux/brut) : 3 EUR
- Consigne verre : 1 EUR
- Sucreries :
  - Bonbon/M&Ms/Twix... : 1 EUR
  - Part de gâteau : 1 EUR
  - Crêpe nature : 2 EUR
  - Crêpe au sucre : 2,50 EUR
  - Crêpe à la confiture : 3,50 EUR
  - Crêpe au caramel : 3,50 EUR
  - Crêpe au Nutella : 3,50 EUR
- Cake salé : 1 EUR
- Sandwich : 4 EUR
- Panini : 4 EUR

---

## Tableau de bord

Cet élément affiche un tableau avec la liste de tous les produits, ainsi que le nombre d'éléments vendus pour chaque produit, un rappel du prix unitaire, et le prix total par produit (correspondant au nombre total de produits vendus multiplié par leur prix).

La liste des éléments affichés sur ce tableau de bord se base uniquement sur les "OrderItem", et pas sur "Product", ce qui permet d'être certains qu'une modification de "Product" ne change pas les résultats finaux.
