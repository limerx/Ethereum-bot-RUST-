use ethers::providers::{ Provider, Ws, Middleware };
use ethers::types::{ H160, Filter, U256, NameOrAddress, TransactionRequest };
use ethers::contract::abigen;
use ethers::abi::Token;
use std::sync::Arc;
use dotenv::dotenv;
use serde::Serialize;
use std::env;
use std::fs::OpenOptions;
use futures_util::StreamExt;
use tokio::time::{ sleep, Duration };

// Génération automatique de l'interface ERC-20
abigen!(
    ERC20,
    r#"[ 
        function symbol() view returns (string)
        function totalSupply() view returns (uint256)
        function buyTaxRate() view returns (uint256)
        function sellTaxRate() view returns (uint256)
    ]"#
);

abigen!(
    UniswapV2Pair,
    r#"[ 
        function getReserves() view returns (uint112, uint112, uint32)
    ]"#
);

/// Structure pour le tableau des informations de paires
#[derive(Debug, Serialize)]
struct PairInfo {
    token1: String,
    token1_symbol: Option<String>,
    token0: String,
    token0_symbol: Option<String>,
    total_supply: Option<U256>,
    pair: String,
    buy_tax: Option<u64>,
    sell_tax: Option<u64>,
    dynamic_tax_detected: Option<u64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Charger l'URL WebSocket depuis le fichier .env
    let ws_url = env::var("INFURA_WS_URL").expect("URL WebSocket Infura manquante");
    let provider = Arc::new(Provider::<Ws>::connect(ws_url).await?);
    println!("✅ Connecté au réseau Ethereum via Infura !");

    // Factory Uniswap V2
    let factory_address: H160 = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f".parse().unwrap();
    let filter = Filter::new().address(factory_address);

    println!("🎧 Écoute des événements PairCreated...");

    // Écoute des logs
    let mut log_stream = provider.subscribe_logs(&filter).await?;
    while let Some(log) = log_stream.next().await {
        if let Some(pair_info) = handle_pair_created(log, provider.clone()).await {
            save_to_csv(pair_info)?;
        }
    }

    Ok(())
}

/// Traiter un événement PairCreated et récupérer les informations des tokens
async fn handle_pair_created(
    log: ethers::types::Log,
    provider: Arc<Provider<Ws>>
) -> Option<PairInfo> {
    if let (Some(token0), Some(token1)) = (log.topics.get(1), log.topics.get(2)) {
        let token0_address = H160::from_slice(&token0.as_bytes()[12..]);
        let token1_address = H160::from_slice(&token1.as_bytes()[12..]);
        let pair_address = H160::from_slice(&log.data[12..32]);

        println!(
            "🔍 Nouvelle paire détectée : Token0 = {:?}, Token1 = {:?}, Pair = {:?}",
            token0_address,
            token1_address,
            pair_address
        );

        // Contrat de la paire
        let pair_contract = UniswapV2Pair::new(pair_address, provider.clone());
        let eth_address: H160 = "0xC02aaa39b223FE8D0A0E5C4F27eAD9083C756Cc2".parse().unwrap(); // Adresse WETH

        // Boucle pour attendre l'injection de liquidité
        let mut retries = 0;
        while retries < 30 {
            match pair_contract.get_reserves().call().await {
                Ok((reserve0, reserve1, _)) => {
                    let reserve0 = U256::from(reserve0);
                    let reserve1 = U256::from(reserve1);

                    if !reserve0.is_zero() || !reserve1.is_zero() {
                        let eth_reserve = if token0_address == eth_address {
                            reserve0
                        } else if token1_address == eth_address {
                            reserve1
                        } else {
                            U256::zero()
                        };

                        if eth_reserve >= U256::from(5e17 as u64) {
                            // 0.5 ETH
                            println!(
                                "✅ Liquidité suffisante détectée : {} ETH.",
                                (eth_reserve.as_u128() as f64) / 1e18
                            );
                            break;
                        } else {
                            println!(
                                "❌ Liquidité insuffisante : {} ETH. Tentative {}/30.",
                                (eth_reserve.as_u128() as f64) / 1e18,
                                retries + 1
                            );
                        }
                    } else {
                        println!("🔄 Aucune liquidité détectée. Tentative {}/30.", retries + 1);
                    }
                }
                Err(err) => {
                    println!(
                        "❌ Erreur lors de la récupération des réserves : {:?}. Tentative {}/30.",
                        err,
                        retries + 1
                    );
                }
            }

            retries += 1;
            sleep(Duration::from_secs(5)).await; // Attente de 5 secondes entre chaque tentative
        }

        if retries == 30 {
            println!("⏰ Temps écoulé. La paire n'a pas reçu suffisamment de liquidité.");
            return None;
        }

        // Vérification de blacklistage
        if simulate_transaction(token1_address, provider.clone()).await {
            println!("❌ La paire est ignorée car l'adresse est blacklistée.");
            return None;
        }

        // Vérification des taxes
        let token1_contract = ERC20::new(token1_address, provider.clone());
        let buy_tax = match token1_contract.method::<(), u64>("buyTaxRate", ()) {
            Ok(call) => call.call().await.ok(),
            Err(_) => None,
        };

        let sell_tax = match token1_contract.method::<(), u64>("sellTaxRate", ()) {
            Ok(call) => call.call().await.ok(),
            Err(_) => None,
        };

        let dynamic_tax_detected = if buy_tax.is_none() || sell_tax.is_none() {
            simulate_transfer(
                token1_address,
                token0_address,
                U256::exp10(18),
                provider.clone()
            ).await
        } else {
            None
        };

        let effective_buy_tax = buy_tax.or(dynamic_tax_detected);
        if effective_buy_tax.map_or(false, |tax| tax > 6) {
            println!("❌ Taxes trop élevées : {} %", effective_buy_tax.unwrap());
            return None;
        }

        println!("🔍 Taxes acceptables : Buy = {:?} %, Sell = {:?} %", buy_tax, sell_tax);

        // Extraction des symboles et enregistrement
        let token1_symbol = token1_contract.symbol().await.ok();
        let total_supply = token1_contract.total_supply().await.ok();
        let token0_contract = ERC20::new(token0_address, provider.clone());
        let token0_symbol = token0_contract.symbol().await.ok();

        println!(
            "🔍 Symboles récupérés : Token0 = {:?}, Token1 = {:?}, TotalSupply = {:?}",
            token0_symbol,
            token1_symbol,
            total_supply
        );

        println!("✅ Nouvelle paire détectée et décodée !");
        Some(PairInfo {
            token1: format!("{:?}", token1_address),
            token1_symbol,
            token0: format!("{:?}", token0_address),
            token0_symbol,
            total_supply,
            pair: format!("{:?}", pair_address),
            buy_tax,
            sell_tax,
            dynamic_tax_detected,
        })
    } else {
        None
    }
}

/// Simuler une transaction pour vérifier si une adresse est blacklistée
async fn simulate_transaction(token_address: H160, provider: Arc<Provider<Ws>>) -> bool {
    let test_address: H160 = "0xdDC65f4E22fe2ad7a9DfF7d5790B94Ea0470c595".parse().unwrap();
    let data = ethers::abi::encode(&[Token::Address(test_address)]);
    let transaction = TransactionRequest {
        to: Some(NameOrAddress::Address(token_address)),
        data: Some(data.into()),
        ..Default::default()
    };

    match provider.call(&transaction.into(), None).await {
        Ok(_) => false,
        Err(_) => true,
    }
}

/// Enregistrer les informations dans un fichier CSV
fn save_to_csv(pair_info: PairInfo) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "pairs_detected.csv";
    let file_exists = std::path::Path::new(file_path).exists();

    let file = OpenOptions::new().create(true).append(true).open(file_path)?;

    let mut writer = csv::Writer::from_writer(&file);

    if !file_exists {
        writer.write_record(
            &[
                "token1",
                "token1_symbol",
                "token0",
                "token0_symbol",
                "total_supply",
                "pair",
                "buy_tax",
                "sell_tax",
                "dynamic_tax_detected",
            ]
        )?;
    }

    writer.serialize(pair_info)?;
    writer.flush()?;

    println!("📂 Paire enregistrée dans le fichier : {}", file_path);
    Ok(())
}
// Ajoutez cette fonction à la fin de votre fichier
async fn simulate_transfer(
    token_address: H160,
    recipient: H160,
    amount: U256,
    provider: Arc<Provider<Ws>>
) -> Option<u64> {
    let data = ethers::abi::encode(&[Token::Address(recipient), Token::Uint(amount)]);
    let transaction = TransactionRequest {
        to: Some(NameOrAddress::Address(token_address)),
        data: Some(data.into()),
        ..Default::default()
    };

    match provider.call(&transaction.into(), None).await {
        Ok(result) => {
            if result.len() >= 32 {
                let received = U256::from_big_endian(&result[0..32]);
                let tax = ((amount - received) * U256::from(100)) / amount;
                println!("📊 Taxes dynamiques détectées : {} %", tax.as_u64());
                Some(tax.as_u64())
            } else {
                println!("⚠️ Résultat inattendu lors de la simulation : {:?}", result);
                None
            }
        }
        Err(err) => {
            println!(
                "❌ Erreur lors de la simulation de transfert pour le token {:?}: {:?}",
                token_address,
                err
            );
            None
        }
    }
}
