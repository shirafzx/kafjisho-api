// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "pos"))]
    pub struct Pos;
}

diesel::table! {
    jp_definitions (id) {
        id -> Uuid,
        jp_word_id -> Uuid,
        jp_definition -> Nullable<Varchar>,
        jp_example -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    jp_th_definition (id) {
        id -> Uuid,
        jp_def_id -> Uuid,
        th_def_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pos;

    jp_words (id) {
        id -> Uuid,
        #[max_length = 255]
        kanji -> Nullable<Varchar>,
        #[max_length = 255]
        reading -> Nullable<Varchar>,
        #[max_length = 255]
        furigana -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        pos -> Nullable<Pos>,
    }
}

diesel::table! {
    th_definitions (id) {
        id -> Uuid,
        th_word_id -> Uuid,
        th_definition -> Nullable<Varchar>,
        th_example -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Pos;

    th_words (id) {
        id -> Uuid,
        word -> Nullable<Varchar>,
        pos -> Nullable<Pos>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(jp_definitions -> jp_words (jp_word_id));
diesel::joinable!(jp_th_definition -> jp_definitions (jp_def_id));
diesel::joinable!(jp_th_definition -> th_definitions (th_def_id));
diesel::joinable!(th_definitions -> th_words (th_word_id));

diesel::allow_tables_to_appear_in_same_query!(
    jp_definitions,
    jp_th_definition,
    jp_words,
    th_definitions,
    th_words,
);
