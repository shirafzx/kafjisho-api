-- Your SQL goes here

-- Create the pos enum type
CREATE TYPE pos AS ENUM ('NOUN', 'PRONOUN', 'VERB', 'ADVERB', 'ADJECTIVE', 'PREPOSITION', 'CONJUNCTION', 'INTERJECTION');

-- Alter existing jp_words table to add pos column
ALTER TABLE jp_words
ADD COLUMN pos pos NOT NULL;

-- Create jp_definitions table
CREATE TABLE jp_definitions (
    id UUID PRIMARY KEY,
    jp_word_id UUID NOT NULL,
    jp_definition VARCHAR,
    jp_example VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    FOREIGN KEY (jp_word_id) REFERENCES jp_words(id)
);

-- Create th_words table
CREATE TABLE th_words (
    id UUID PRIMARY KEY,
    word VARCHAR,
    pos pos NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

-- Create th_definitions table
CREATE TABLE th_definitions (
    id UUID PRIMARY KEY,
    th_word_id UUID NOT NULL,
    th_definition VARCHAR,
    th_example VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    FOREIGN KEY (th_word_id) REFERENCES th_words(id)
);

-- Create jp_th_definition join table
CREATE TABLE jp_th_definitions (
    id UUID PRIMARY KEY,
    jp_def_id UUID NOT NULL,
    th_def_id UUID NOT NULL,
    FOREIGN KEY (jp_def_id) REFERENCES jp_definitions(id),
    FOREIGN KEY (th_def_id) REFERENCES th_definitions(id)
);

-- Remove UNIQUE constraint from kanji column if needed
ALTER TABLE jp_words
DROP CONSTRAINT jp_words_kanji_key;