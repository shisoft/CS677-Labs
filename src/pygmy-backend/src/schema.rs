table! {
    item (id) {
        id -> Integer,
        name -> Text,
        stock -> Integer,
        price -> Float,
        topic -> Integer,
    }
}

table! {
    order (id) {
        id -> Integer,
        item -> Integer,
        amount -> Integer,
        total -> Float,
    }
}

table! {
    topic (id) {
        id -> Integer,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    item,
    order,
    topic,
);
