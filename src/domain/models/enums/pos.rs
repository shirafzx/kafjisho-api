use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Pos {
    NOUN,
    PRONOUN,
    VERB,
    ADVERB,
    ADJECTIVE,
    PREPOSITION,
    CONJUNCTION,
    INTERJECTION,
}
