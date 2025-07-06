diesel::table! {
    trade (id) {
        id -> Text,
        pair -> Text,
        amount -> Double,
        price -> Double,
        order_type -> Text,
        trade_type -> Text,
        timestamp -> Integer,
    }
}
