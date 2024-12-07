
# Password Generator

Ce projet est une application en ligne de commande (CLI) écrite en **Rust** qui génère des mots de passe sécurisés en fonction des préférences de l'utilisateur. L'utilisateur peut personnaliser la longueur du mot de passe, ainsi que les types de caractères à inclure, tels que les lettres majuscules, les chiffres et les caractères spéciaux.

## Fonctionnalités

- Génération de mots de passe de longueur personnalisée.
- Options pour inclure ou exclure des lettres majuscules, des chiffres et des caractères spéciaux.
- Interface utilisateur simple via la ligne de commande avec `clap` pour la gestion des arguments.

## Installation

### Prérequis

Avant de commencer, assurez-vous que **Rust** et **Cargo** sont installés sur votre machine. Si ce n'est pas le cas, vous pouvez les télécharger et installer en suivant les instructions sur le site officiel de [Rust](https://www.rust-lang.org/tools/install).

### Cloner le dépôt

```bash
git clone https://github.com/VotreNomUtilisateur/password_generator.git
cd password_generator
```

### Compilation et Exécution

Pour compiler le projet, exécutez la commande suivante :

```bash
cargo build
```

Pour exécuter le générateur de mots de passe avec des options personnalisées, vous pouvez utiliser la commande suivante :

```bash
cargo run -- --length 16 --include-uppercase --include-numbers --include-special-chars
```

Cela générera un mot de passe de 16 caractères avec des lettres majuscules, des chiffres et des caractères spéciaux.

### Options disponibles

Le générateur accepte plusieurs arguments via la ligne de commande :

- `--length <LENGTH>` : Spécifie la longueur du mot de passe. Par défaut, la longueur est de 12 caractères.
- `--include-uppercase` : Inclut les lettres majuscules dans le mot de passe. Par défaut, cette option est activée.
- `--include-numbers` : Inclut les chiffres dans le mot de passe. Par défaut, cette option est activée.
- `--include-special-chars` : Inclut les caractères spéciaux dans le mot de passe. Par défaut, cette option est activée.
- `--help` : Affiche l'aide et les options disponibles.

### Exemples

1. **Générer un mot de passe avec 16 caractères, y compris les majuscules, les chiffres et les caractères spéciaux** :

   ```bash
   cargo run -- --length 16 --include-uppercase --include-numbers --include-special-chars
   ```

2. **Générer un mot de passe de 16 caractères sans majuscules, mais avec des chiffres et des caractères spéciaux** :

   ```bash
   cargo run -- --length 16 --include-numbers --include-special-chars
   ```

3. **Générer un mot de passe de 8 caractères uniquement avec des chiffres et des caractères spéciaux, sans majuscules** :

   ```bash
   cargo run -- --length 8 --include-numbers --include-special-chars
   ```

## Contribuer

Les contributions sont les bienvenues ! Si vous souhaitez contribuer à ce projet, veuillez suivre ces étapes :

1. Forker le dépôt.
2. Créer une branche de fonctionnalité (`git checkout -b feature/ma-fonctionnalite`).
3. Commiter vos modifications (`git commit -am 'Ajouter une nouvelle fonctionnalité'`).
4. Pousser votre branche (`git push origin feature/ma-fonctionnalite`).
5. Ouvrir une Pull Request.

## License

Ce projet est sous la licence **MIT**. Voir le fichier [LICENSE](LICENSE) pour plus de détails.

## Auteurs

- **Hassan El Bahraoui** - Développeur principal

---

Merci d'utiliser ce générateur de mots de passe ! 🚀
