// @generated automatically by Diesel CLI.

diesel::table! {
    processed_block (id) {
        id -> Int4,
        block_number -> Int8,
    }
}
