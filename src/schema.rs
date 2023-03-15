table! {
    tasks (id) {
        id -> Uuid,
        title -> Varchar,
        description -> Nullable<Text>,
        done -> Bool,
    }
}
