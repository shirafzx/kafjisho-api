// @generated automatically by Diesel CLI.

diesel::table! {
    japanese_words (id) {
        id -> Uuid,
        #[max_length = 255]
        kanji -> Nullable<Varchar>,
        #[max_length = 255]
        reading -> Nullable<Varchar>,
        #[max_length = 255]
        furigana -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
