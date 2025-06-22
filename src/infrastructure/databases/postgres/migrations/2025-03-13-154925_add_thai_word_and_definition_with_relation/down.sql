-- This file should undo anything in `up.sql`

-- Drop the jp_th_definition join table
DROP TABLE IF EXISTS jp_th_definition;

-- Drop the th_definitions table
DROP TABLE IF EXISTS th_definitions;

-- Drop the th_words table
DROP TABLE IF EXISTS th_words;

-- Drop the jp_definitions table
DROP TABLE IF EXISTS jp_definitions;

-- Remove the pos column from jp_words
ALTER TABLE jp_words
DROP COLUMN IF EXISTS pos;

-- Re-add the UNIQUE constraint to kanji column if it was removed
ALTER TABLE jp_words
ADD CONSTRAINT jp_words_kanji_key UNIQUE (kanji);

-- Drop the pos enum type
DROP TYPE IF EXISTS pos;