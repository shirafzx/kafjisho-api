-- Your SQL goes here
CREATE TABLE jp_words (
    id UUID PRIMARY KEY,
    kanji VARCHAR(255) UNIQUE,
    reading VARCHAR(255),
    furigana VARCHAR(255),
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);
