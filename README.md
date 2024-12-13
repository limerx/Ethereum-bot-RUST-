# Ethereum Bot in Rust

## Description
This bot is designed to monitor and analyze new token pairs created on Uniswap V2. It connects to the Ethereum network via WebSocket (using Infura) and evaluates pairs for liquidity, taxes, and blacklist status. The results are logged in real-time and stored in a CSV file for further analysis.

---

## Features
- **Real-Time Monitoring**: Detects `PairCreated` events from the Uniswap Factory contract.
- **Liquidity Validation**: Ensures sufficient liquidity (e.g., 0.5 ETH) before processing a pair.
- **Tax Analysis**: Retrieves buy and sell taxes or simulates transfers to identify hidden taxes.
- **Blacklist Protection**: Identifies tokens potentially blacklisted by simulating transactions.
- **Data Export**: Saves token pair details (addresses, symbols, taxes, etc.) to a CSV file.

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
