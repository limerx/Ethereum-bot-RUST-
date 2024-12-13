# Bot Ethereum en Rust

## Description
Ce bot est conçu pour surveiller et analyser les nouvelles paires de tokens créées sur Uniswap V2. Il se connecte au réseau Ethereum via WebSocket (en utilisant Infura) et évalue les paires pour leur liquidité, leurs taxes et leur statut de blacklist. Les résultats sont enregistrés en temps réel et stockés dans un fichier CSV pour une analyse ultérieure.

---

## Fonctionnalités
- **Surveillance en temps réel** : Détecte les événements `PairCreated` du contrat Factory d'Uniswap.
- **Validation de liquidité** : S'assure qu'un seuil minimal de liquidité (par exemple, 0,5 ETH) est atteint avant de traiter une paire.
- **Analyse des taxes** : Récupère les taxes d'achat et de vente ou simule des transferts pour détecter des taxes cachées.
- **Protection contre le blacklistage** : Identifie les tokens potentiellement blacklistés en simulant des transactions.
- **Export des données** : Sauvegarde les détails des paires de tokens (adresses, symboles, taxes, etc.) dans un fichier CSV.

---

## Prérequis
- Rust installé ([Installer Rust](https://rustup.rs/))
- Compte Infura et URL WebSocket
- Accès à un nœud Ethereum (via Infura ou un autre fournisseur)
- Fichier `.env` avec la configuration requise

---

## Installation
1. **Cloner le repository** :
   ```bash
   git clone https://github.com/limerx/Ethereum-bot-RUST-.git
   cd Ethereum-bot-RUST-
   ```

2. **Installer les dépendances** :
   ```bash
   cargo build
   ```

3. **Configurer les variables d'environnement** :
   - Créez un fichier `.env` à la racine du projet et ajoutez votre URL WebSocket Infura :
     ```env
     INFURA_WS_URL=wss://mainnet.infura.io/ws/v3/VOTRE_PROJECT_ID
     ```

---

## Utilisation
1. **Exécuter le bot** :
   ```bash
   cargo run
   ```

2. **Surveillance** :
   - Le bot écoute les événements `PairCreated` et enregistre les détails dans la console.
   - Les paires détectées sont analysées pour leur liquidité, leurs taxes et leur statut de blacklist.

3. **Résultats** :
   - Les résultats sont sauvegardés dans un fichier CSV nommé `pairs_detected.csv` dans le répertoire du projet.

---

## Configuration
### Variables d'environnement
- `INFURA_WS_URL` : URL WebSocket pour la connexion au nœud Ethereum (exemple : Infura).

### Personnalisation des paramètres
- Modifiez les constantes ou la logique dans `main.rs` pour des seuils spécifiques (par exemple, liquidité minimale, taux de taxes).

---

## Format du fichier CSV
Le bot génère un fichier CSV avec les colonnes suivantes :
- `token1` : Adresse du premier token dans la paire.
- `token1_symbol` : Symbole du premier token.
- `token0` : Adresse du second token dans la paire.
- `token0_symbol` : Symbole du second token.
- `total_supply` : Offre totale du premier token.
- `pair` : Adresse du contrat de la paire.
- `buy_tax` : Taux de taxe à l'achat (si disponible).
- `sell_tax` : Taux de taxe à la vente (si disponible).
- `dynamic_tax_detected` : Taux de taxe détecté dynamiquement (le cas échéant).

---

## Résolution des problèmes
### Erreurs courantes
- **Variable d'environnement manquante** : Vérifiez que le fichier `.env` est correctement configuré avec `INFURA_WS_URL`.
- **Problèmes de connexion** : Assurez-vous que votre URL WebSocket est valide et que le nœud Ethereum est accessible.
- **Erreurs d'écriture CSV** : Vérifiez que le bot a la permission d'écrire dans le répertoire.

### Débogage
- Utilisez les logs de la console pour des informations détaillées sur les paires détectées et les erreurs.
- Modifiez `main.rs` pour ajouter des logs supplémentaires si nécessaire.

---

## Licence
Ce projet est open-source et disponible sous la licence MIT.

---

## Contributions
Les contributions sont les bienvenues ! N'hésitez pas à forker le repository, effectuer des modifications et soumettre une pull request.

---

# Ethereum Bot in Rust

## Description
This bot is designed to monitor and analyze new token pairs created on Uniswap V2. It connects to the Ethereum network via WebSocket (using Infura) and evaluates pairs for liquidity, taxes, and blacklist status. The results are logged in real-time and stored in a CSV file for further analysis.

---

## Features
- **Real-Time Monitoring**: Detects `PairCreated` events from the Uniswap Factory contract.
- **Liquidity Validation**: Ensures sufficient liquidity (e.g., 0.5 ETH) before processing a pair.
- **Tax Analysis**: Retrieves buy and sell taxes or simulates transfers to identify hidden taxes.
- **Blacklist Protection**: Identifies tokens potentially blacklisted by simulating transactions.
- **Data Export**: Saves token pair details (addresses, symbols, taxes, etc.) into a CSV file.

---

## Prerequisites
- Rust installed ([Install Rust](https://rustup.rs/))
- Infura account and WebSocket URL
- Access to an Ethereum node (via Infura or another provider)
- `.env` file with the required configuration

---

## Installation
1. **Clone the Repository**:
   ```bash
   git clone https://github.com/limerx/Ethereum-bot-RUST-.git
   cd Ethereum-bot-RUST-
   ```

2. **Install Dependencies**:
   ```bash
   cargo build
   ```

3. **Set Up Environment Variables**:
   - Create a `.env` file in the root of the project and add your Infura WebSocket URL:
     ```env
     INFURA_WS_URL=wss://mainnet.infura.io/ws/v3/YOUR_PROJECT_ID
     ```

---

## Usage
1. **Run the Bot**:
   ```bash
   cargo run
   ```

2. **Monitoring**:
   - The bot listens for `PairCreated` events and logs the details in the console.
   - Detected pairs are analyzed for liquidity, taxes, and blacklist status.

3. **Results**:
   - Results are saved in a CSV file named `pairs_detected.csv` in the project directory.

---

## Configuration
### Environment Variables
- `INFURA_WS_URL`: WebSocket URL for Ethereum node connection (e.g., Infura).

### Customizing Parameters
- Modify constants or logic in `main.rs` for specific thresholds (e.g., minimum liquidity, tax rates).

---

## CSV File Format
The bot generates a CSV file with the following columns:
- `token1`: Address of the first token in the pair.
- `token1_symbol`: Symbol of the first token.
- `token0`: Address of the second token in the pair.
- `token0_symbol`: Symbol of the second token.
- `total_supply`: Total supply of the first token.
- `pair`: Address of the pair contract.
- `buy_tax`: Buy tax rate (if available).
- `sell_tax`: Sell tax rate (if available).
- `dynamic_tax_detected`: Dynamically detected tax rate (if applicable).

---

## Troubleshooting
### Common Errors
- **Missing Environment Variable**: Ensure the `.env` file is correctly configured with `INFURA_WS_URL`.
- **Connection Issues**: Verify your WebSocket URL and ensure the Ethereum node is accessible.
- **CSV Write Errors**: Ensure the bot has permission to write to the directory.

### Debugging
- Use console logs for detailed information about detected pairs and errors.
- Modify `main.rs` to add additional logging if needed.

---

## License
This project is open-source and available under the MIT License.

---

## Contributions
Contributions are welcome! Feel free to fork the repository, make changes, and submit a pull request.
