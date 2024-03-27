// use ethers::{
//     abi::AbiEncode,
//     core::types::{Address, Filter, H160, H256, U256},
//     providers::{Http, Middleware, Provider},
// };
// use eyre::Result;
// use std::sync::Arc;

// use reqwest;

// const HTTP_URL: &str = "https://rpc.flashbots.net";
// const V3FACTORY_ADDRESS: &str = "0x1F98431c8aD98523631AE4a59f267346ea31F984";
// const DAI_ADDRESS: &str = "0x6B175474E89094C44Da98b954EedeAC495271d0F";
// const USDC_ADDRESS: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
// const USDT_ADDRESS: &str = "0xdAC17F958D2ee523a2206206994597C13D831ec7";

// /// This example demonstrates filtering and parsing event logs by fetching all Uniswap V3 pools
// /// where both tokens are in the set [USDC, USDT, DAI].
// ///
// /// V3 factory reference: https://github.com/Uniswap/v3-core/blob/main/contracts/interfaces/IUniswapV3Factory.sol
// #[tokio::main]
// async fn main() -> Result<()> {
//     let provider = Provider::<Http>::try_from(HTTP_URL)?;
//     let client = Arc::new(provider);
//     let token_topics = [
//         H256::from(USDC_ADDRESS.parse::<H160>()?),
//         H256::from(USDT_ADDRESS.parse::<H160>()?),
//         H256::from(DAI_ADDRESS.parse::<H160>()?),
//     ];
//     let filter = Filter::new()
//         .address(V3FACTORY_ADDRESS.parse::<Address>()?)
//         .event("PoolCreated(address,address,uint24,int24,address)")
//         // .topic1(token_topics.to_vec())
//         // .topic2(token_topics.to_vec())
//         .from_block(19516000);
//     let logs = client.get_logs(&filter).await?;
//     println!("{} pools found!", logs.iter().len());
//     for log in logs.iter() {
//         let token0 = Address::from(log.topics[1]).encode_hex();
//         let token0_address = token0
//             .chars()
//             .rev()
//             .take(40)
//             .collect::<String>()
//             .chars()
//             .rev()
//             .collect::<String>();
//         let token1 = Address::from(log.topics[2]).encode_hex();
//         let token1_address = token1
//             .chars()
//             .rev()
//             .take(40)
//             .collect::<String>()
//             .chars()
//             .rev()
//             .collect::<String>();
//         let token0_etherscan = format!("https://etherscan.io/address/0x{}", token0_address);
//         let token1_etherscan = format!("https://etherscan.io/address/0x{}", token1_address);
//         let fee_tier = U256::from_big_endian(&log.topics[3].as_bytes()[29..32]);
//         let tick_spacing = U256::from_big_endian(&log.data[29..32]);
//         let pool = Address::from(&log.data[44..64].try_into()?);
//         println!(
//             "pool = {pool}, token0 = {token0}, token1 = {token1}, fee = {fee_tier}, spacing = {tick_spacing}"
//         );
//         println!("token0: {token0_etherscan}, token1: {token1_etherscan}");

//         let http_client = reqwest::Client::new();
//         let telegram_api = "https://api.telegram.org";
//         let telegram_bot = "bot7019980938:AAGkRrfIkzd4qSMPjLPatexkogjIFDYsqS0";
//         let telegram_chat = "-1002066122929";
//         let message = format!(
//             "New Pool Created %0APool Address: {}%0Atoken0: {}%0Atoken1: {}",
//             pool, token0_etherscan, token1_etherscan
//         );
//         // let response = http_client.post("https://api.telegram.org/bot7019980938:AAGkRrfIkzd4qSMPjLPatexkogjIFDYsqS0/sendMessage?chat_id=-1002066122929&text=%3Cour%20text%3E");
//         let response = http_client.post(
//             format!(
//                 "{telegram_api}/{telegram_bot}/sendMessage?chat_id={telegram_chat}&text={message}"
//             )
//             .as_str(),
//         );
//         let response = response.send().await?;
//         println!("{:?}", response);
//     }
//     Ok(())
// }

// use env_logger::Builder;
// use log::LevelFilter;
// use rand::Rng;
// use tokio::time::{sleep, Duration};
// use tokio::task::{self, JoinSet};

// async fn sleepy() {
//     log::info!("Sleeping for 2 seconds");
//     sleep(Duration::from_secs(2)).await;
//     log::info!("Awake!");
// }

// async fn do_something_fun(runner: u8) {
//     let mut i = 0;
//     loop {
//         log::info!("runner {}: {}",runner, i );
//         i += 1;
//         sleep(Duration::from_secs(1)).await;

//         if runner == 1 && i == 15 {
//             break;
//         }

//     }
// }

// async fn run() {

//     let mut set = JoinSet::new();

//     set.spawn(async {
//         do_something_fun(1).await;
//     });

//     set.spawn(async {
//         do_something_fun(2).await;
//     });

//     let response = set.join_next().await;

//     log::info!("Response: {:?}", response);

//     sleepy().await;
// }

// #[tokio::main]
// async fn main() {
//     println!("Hello, world!");
//     Builder::new().filter(None, LevelFilter::Info).init();

//     run().await;

//     sleep(Duration::from_secs(10)).await;
// }

enum Message {
    Tick,
}

async fn sender(tx: tokio::sync::mpsc::Sender<Message>) {
    let mut iteration = 0;
    loop {
        tx.send(Message::Tick).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        iteration += 1;

        if iteration == 5 {
            break;
        }
    }
}

async fn receiver(mut rx: tokio::sync::mpsc::Receiver<Message>) {
    let mut none_counter = 0;
    loop {
        let message = rx.recv().await;
        match message {
            Some(Message::Tick) => {
                println!("Tick");
            }
            None => {
                println!("Unknown message");
                none_counter += 1;
                println!("None Counter: {}", none_counter);
            }
        }

        if none_counter == 10 {
            break;
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = tokio::sync::mpsc::channel::<Message>(100);

    tokio::spawn(sender(tx));
    receiver(rx).await;
}
