use self::models::*;
use diesel::prelude::*;
use wealth_watcher::*;

fn main() {
    use self::schema::processed_block::dsl::*;

    let connection = &mut establish_connection();
    let results = processed_block
        .limit(1)
        .select(Block::as_select())
        .load(connection)
        .expect("Error loading posts");

    let result_length = results.len();

    if result_length > 0 {
        println!("Displaying {} block number", result_length);
        for block in results {
            println!("{}", block.block_number);
            println!("-----------\n");
        }
    } else {
        let block = Block {
            id: 1,
            block_number: 19516000,
        };

        let _ = diesel::insert_into(processed_block)
            .values(&block)
            .execute(connection)
            .expect("Error saving new post");
    }
}
