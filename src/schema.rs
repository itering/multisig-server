table! {
    wallets (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        address -> Char,
        parties -> Text,
        threshold -> Unsigned<Integer>,
        balance -> Integer,
    }
}
