table! {
    todos (id) {
        id -> Integer,
        title -> Text,
        status -> Text,
        created -> Unsigned<BigInt>,
        ended -> Unsigned<BigInt>,
        schedule -> Unsigned<BigInt>,
        is_schedule -> Bool,
    }
}