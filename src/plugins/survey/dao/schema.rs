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
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    survey_logs (id) {
        id -> Int8,
        form_id -> Int8,
        user_id -> Nullable<Int8>,
        message -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    survey_responses (id) {
        id -> Int8,
        form_id -> Int8,
        ip -> Varchar,
        body -> Json,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    survey_fields,
    survey_forms,
    survey_logs,
    survey_responses,
);
