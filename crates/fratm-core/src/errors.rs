//! Error types and Napoletano error messages
//!
//! Tutti l'errori so' in napoletano! 🤌

use thiserror::Error;
use serde::{Serialize, Deserialize};

/// Main compilation error type
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum CompileError {
    #[error("Riga {line}, colonna {column}: {message}")]
    LexerError {
        message: String,
        line: usize,
        column: usize,
    },
    
    #[error("Riga {line}, colonna {column}: {message}")]
    ParseError {
        message: String,
        line: usize,
        column: usize,
    },
    
    #[error("Errore interno: {message}")]
    CodeGenError {
        message: String,
    },
}

impl CompileError {
    pub fn line(&self) -> Option<usize> {
        match self {
            CompileError::LexerError { line, .. } => Some(*line),
            CompileError::ParseError { line, .. } => Some(*line),
            CompileError::CodeGenError { .. } => None,
        }
    }
    
    pub fn column(&self) -> Option<usize> {
        match self {
            CompileError::LexerError { column, .. } => Some(*column),
            CompileError::ParseError { column, .. } => Some(*column),
            CompileError::CodeGenError { .. } => None,
        }
    }
}

// ============== Napoletano Error Messages ==============

/// Get Napoletano message for error
pub fn napoletanize_error(message: &str) -> String {
    // Pattern matching for common errors
    if message.contains("Aspettavo '}'") {
        return "Uè, hai aperto 'na parentesi graffa ma nun l'hai chiusa! Mettece '}'!".to_string();
    }
    if message.contains("Aspettavo ')'") {
        return "Manca 'a parentesi chiusa! Ce vo' ')'!".to_string();
    }
    if message.contains("Aspettavo ']'") {
        return "E 'a parentesi quadra? Chiudela cu ']'!".to_string();
    }
    if message.contains("Aspettavo '='") {
        return "E addò sta l'uguale? Ce vo' '=' pe assegnà 'o valore!".to_string();
    }
    if message.contains("Aspettavo ';'") {
        return "Manca 'o punto e virgola! Ma va bene, nun te preoccupà.".to_string();
    }
    if message.contains("Aspettavo un nome") {
        return "Ccà ce vo' nu nome! Che cosa vuò chiamà sta variabile?".to_string();
    }
    if message.contains("Aspettavo una stringa") {
        return "Ccà ce vo' 'na stringa! Mettece 'e virgolette!".to_string();
    }
    if message.contains("Aspettavo 'è'") {
        return "Doppo 'chist' ce vo' 'è'! Scrivi 'chist è' pe fà 'na costante.".to_string();
    }
    if message.contains("Aspettavo 'che'") {
        return "Doppo 'mentre' ce vo' 'che'! Scrivi 'mentre che'.".to_string();
    }
    if message.contains("Aspettavo 'ogni'") {
        return "Doppo 'pe' ce vo' 'ogni'! Scrivi 'pe ogni'.".to_string();
    }
    if message.contains("Aspettavo 'vir'") {
        return "Doppo 'mo' ce vo' 'vir'! Scrivi 'mo vir facc' pe 'na funzione asincrona.".to_string();
    }
    if message.contains("Aspettavo 'bell'") {
        return "Doppo 'nu' ce vo' 'bell'! Scrivi 'nu bell' pe creà n'oggetto nuovo.".to_string();
    }
    if message.contains("Aspettavo 'famiglie'") {
        return "Doppo 'na' ce vo' 'famiglie'! Scrivi 'na famiglie' pe fà 'na classe.".to_string();
    }
    if message.contains("Aspettavo 'cos'") {
        return "Doppo 'stu' ce vo' 'cos'! Scrivi 'stu cos' pe riferisce a this.".to_string();
    }
    if message.contains("Aspettavo 'for'") {
        return "Doppo 'mann' ce vo' 'for'! Scrivi 'mann for' pe esportà.".to_string();
    }
    if message.contains("Aspettavo 'dì'") {
        return "Doppo 'stamm a' ce vo' 'dì'! Scrivi 'stamm a dì' pe stampà.".to_string();
    }
    if message.contains("expression") || message.contains("espressione") {
        return "Ma che staje scrivenn?! Ccà ce vo' 'na espressione!".to_string();
    }
    
    // Default: return with some Napoletano flair
    format!("Uè, c'è nu problema: {}", message)
}

// ============== Error Suggestions ==============

/// Get helpful suggestions for an error
pub fn get_suggestion(error: &CompileError) -> Option<String> {
    match error {
        CompileError::ParseError { message, .. } => {
            if message.contains("'}'") {
                Some("💡 Conta 'e parentesi graffe: ogni '{' adda avè 'o suo '}'".to_string())
            } else if message.contains("')'") {
                Some("💡 Conta 'e parentesi tonne: ogni '(' adda avè 'o suo ')'".to_string())
            } else if message.contains("chist") {
                Some("💡 Esempio: chist è nome = \"Gennaro\"".to_string())
            } else if message.contains("facc") {
                Some("💡 Esempio: facc saluta(nome) { piglie \"Ciao \" + nome }".to_string())
            } else if message.contains("stamm") {
                Some("💡 Esempio: stamm a dì(\"Uè!\")".to_string())
            } else {
                None
            }
        }
        CompileError::LexerError { message, .. } => {
            if message.contains("string") {
                Some("💡 'E stringhe s'aprono e se chiudono cu \" o '".to_string())
            } else {
                None
            }
        }
        _ => None,
    }
}

// ============== Random Napoletano Phrases ==============

/// Get a random encouragement phrase
pub fn random_encouragement() -> &'static str {
    const PHRASES: &[&str] = &[
        "Nun te preoccupà, capita a tutt'!",
        "Vire buono 'o codice e riprova!",
        "Cu 'a calma se fa tutto!",
        "Nisciuno nasce imparato!",
        "Piano piano se va luntano!",
        "'A pazienza è 'a virtù d''e forte!",
    ];
    PHRASES[0] // In real implementation, use random
}

/// Get a success message
pub fn success_message() -> &'static str {
    const PHRASES: &[&str] = &[
        "Tutto appost! 🤌",
        "Uè, funziona! Bravo!",
        "Perfetto! Comme 'na pizza margherita!",
        "Eh, vedi che ce l'hai fatta!",
        "Bellillo! 'O codice è pronto!",
    ];
    PHRASES[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_formatting() {
        let error = CompileError::ParseError {
            message: "Aspettavo '}'".to_string(),
            line: 5,
            column: 10,
        };
        let msg = format!("{}", error);
        assert!(msg.contains("parentesi graffa"));
        assert!(msg.contains("Riga 5"));
    }

    #[test]
    fn test_suggestion() {
        let error = CompileError::ParseError {
            message: "Aspettavo '}'".to_string(),
            line: 1,
            column: 1,
        };
        let suggestion = get_suggestion(&error);
        assert!(suggestion.is_some());
        assert!(suggestion.unwrap().contains("parentesi"));
    }
}
