// The `prelude` module provides a convenient way to import a number
// of common dependencies at once. This can be useful if you are working
// with multiple parts of the library and want to avoid having
// to import each dependency individually.
use ethers::prelude::*;

const RPC_URL: &str = "https://eth.llamarpc.com";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider: Provider<Http> = Provider::<Http>::try_from(RPC_URL)?;
    let block_number: U64 = provider.get_block_number().await?;
    println!("{block_number}");

    let block_data = provider.get_block(block_number).await?;
    let block_tx = block_data.unwrap().transactions;

    for tx in block_tx {
        println!("{:?}", tx);
    }


    Ok(())
}
