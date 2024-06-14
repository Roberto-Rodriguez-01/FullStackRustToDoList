diesel::table! {
    tasks (id) {
        id -> Integer,
        description -> Text,
        done -> Bool,
    }
}

