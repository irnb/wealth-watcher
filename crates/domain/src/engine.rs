use crate::traits::{Telegram, Database, BlockchainDataProvider};


pub struct Engine {
    telegram: Box<dyn Telegram>,
    database: Box<dyn Database>,
    blockchain_data_provider: Box<dyn BlockchainDataProvider>,
}

impl Engine {
    pub fn new(telegram: Box<dyn Telegram>, database: Box<dyn Database>, blockchain_data_provider: Box<dyn BlockchainDataProvider>) -> Self {
        Engine {
            telegram,
            database,
            blockchain_data_provider,
        }
    }

    pub fn run(&self) {
        let blocknumber = self.blockchain_data_provider.get_blocknumber();
        let stored_blocknumber = self.database.get_blocknumber();

        if blocknumber <= stored_blocknumber {
            return;
        }
        self.database.update_blocknumber(blocknumber);
        let new_created_uniswap_v3_pairs = self.blockchain_data_provider.get_new_created_uniswap_v3_pairs();
        for pair in new_created_uniswap_v3_pairs {
            self.telegram.send_message(&pair);
        }
    }
}