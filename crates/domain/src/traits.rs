pub trait Telegram {
    fn new() -> Self where Self: Sized;
    fn send_message(&self, message: &str);
}

pub trait Database {
    fn new() -> Self where Self: Sized ;
    fn update_blocknumber(&self, blocknumber: u64);
    fn get_blocknumber(&self) -> u64;   
}

pub trait BlockchainDataProvider {
    fn new() -> Self where Self: Sized;
    fn get_blocknumber(&self) -> u64;
    fn get_block(&self, blocknumber: u64) -> String;
    fn get_new_created_uniswap_v3_pairs(&self) -> Vec<String>;
}