table! {
    survey_fields (id) {
        id -> Int8,
        form_id -> Int8,
        key -> Varchar,
        title -> Varchar,
        required -> Bool,
        profile -> Json,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    survey_forms (id) {
        id -> Int8,
        user_id -> Int8,
        title -> Varchar,
        body -> Text,
        media_type -> Varchar,
        nbf -> Date,
        exp -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    survey_responses (id) {
        id -> Int8,
        form_id -> Int8,
        ip -> Inet,
        body -> Json,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    survey_fields,
    survey_forms,
    survey_responses,
);
