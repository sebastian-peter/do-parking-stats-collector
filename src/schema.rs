// @generated automatically by Diesel CLI.

diesel::table! {
    parking_info (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 50]
        parking_type -> Varchar,
        total_capacity -> Int2,
        short_capacity -> Int2,
        other_capacity -> Int2,
        updated_time -> Timestamp,
    }
}

diesel::table! {
    parking_stats (id, source_time) {
        id -> Int4,
        source_time -> Timestamp,
        opened -> Bool,
        updated_time -> Timestamp,
        current_total_capacity -> Int2,
        current_short_capacity -> Int2,
        current_other_capacity -> Int2,
        current_total_occupied -> Int2,
        current_short_occupied -> Int2,
        current_other_occupied -> Int2,
    }
}

diesel::joinable!(parking_stats -> parking_info (id));

diesel::allow_tables_to_appear_in_same_query!(
    parking_info,
    parking_stats,
);
