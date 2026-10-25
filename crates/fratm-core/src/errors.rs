//! Error types and Neapolitan error messages
//!
//! All errors are in Neapolitan dialect! 🤌

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Main compilation error type
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum CompileError {
    #[error("Riga {line}, colonna {column}: {msg}", msg = napoletanize_error(message))]
    LexerError {
        message: String,
        line: usize,
        column: usize,
    },

    #[error("Riga {line}, colonna {column}: {msg}", msg = napoletanize_error(message))]
    ParseError {
        message: String,
        line: usize,
        column: usize,
    },

    #[error("Riga {line}, colonna {column}: {msg}", msg = napoletanize_error(message))]
    SemanticError {
        message: String,
        line: usize,
        column: usize,
    },

    #[error("Errore interno: {message}")]
    CodeGenError { message: String },
}

impl CompileError {
    pub fn line(&self) -> Option<usize> {
        match self {
            CompileError::LexerError { line, .. } => Some(*line),
            CompileError::ParseError { line, .. } => Some(*line),
            CompileError::SemanticError { line, .. } => Some(*line),
            CompileError::CodeGenError { .. } => None,
        }
    }

    pub fn column(&self) -> Option<usize> {
        match self {
            CompileError::LexerError { column, .. } => Some(*column),
            CompileError::ParseError { column, .. } => Some(*column),
            CompileError::SemanticError { column, .. } => Some(*column),
            CompileError::CodeGenError { .. } => None,
        }
    }
}

// ============== Napoletano Error Messages ==============

/// Get Napoletano message for error
pub fn napoletanize_error(message: &str) -> String {
    if let Some(jsx) = jsx_message(message) {
        return jsx;
    }
    let found = found_token(message);
    if let Some(c) = message.strip_prefix("Unknown character '") {
        return format!(
            "Stu carattere '{}' nun 'o canosco: FratmScript nun 'o usa!",
            c.trim_end_matches('\'')
        );
    }
    if message.contains("between the elements") {
        let closing = message
            .strip_prefix("Expected ',' or '")
            .and_then(|rest| rest.split('\'').next())
            .unwrap_or(")");
        return format!(
            "Ccà ce vo' 'na virgola ',' (o '{}' pe chiudere), ma aggio truvato {}!",
            closing,
            found.as_deref().unwrap_or("n'ata cosa")
        );
    }
    // A closing bracket followed by the end of the file is really unclosed;
    // anything else gets named, so the message does not blame the wrong bracket.
    let unclosed = matches!(found.as_deref(), None | Some(END_OF_FILE));
    for closing in ["'}'", "')'", "']'"] {
        if expects(message, closing) && !unclosed {
            return format!(
                "Ccà ce vo' {}, ma aggio truvato {}!",
                closing,
                found.as_deref().unwrap_or("n'ata cosa")
            );
        }
    }
    if expects(message, "'}'") {
        return "Uè, hai aperto 'na parentesi graffa ma nun l'hai chiusa! Mettece '}'!".to_string();
    }
    if expects(message, "')'") {
        return "Manca 'a parentesi chiusa! Ce vo' ')'!".to_string();
    }
    if expects(message, "']'") {
        return "E 'a parentesi quadra? Chiudela cu ']'!".to_string();
    }
    if expects(message, "'='") {
        return "E addò sta l'uguale? Ce vo' '=' pe assegnà 'o valore!".to_string();
    }
    if expects(message, "';'") {
        return "Manca 'o punto e virgola! Ma va bene, nun te preoccupà.".to_string();
    }
    if expects(message, "un nome") || message.contains("Expected an identifier") {
        return "Ccà ce vo' nu nome! Che cosa vuò chiamà sta variabile?".to_string();
    }
    if expects(message, "una stringa") || message.contains("Expected a string") {
        return "Ccà ce vo' 'na stringa! Mettece 'e virgolette!".to_string();
    }
    if expects(message, "'è'") {
        return "Doppo 'chist' ce vo' 'è'! Scrivi 'chist è' pe fà 'na costante.".to_string();
    }
    if expects(message, "'che'") {
        return "Doppo 'mentre' ce vo' 'che'! Scrivi 'mentre che'.".to_string();
    }
    if expects(message, "'ogni'") {
        return "Doppo 'pe' ce vo' 'ogni'! Scrivi 'pe ogni'.".to_string();
    }
    if expects(message, "'vir'") {
        return "Doppo 'mo' ce vo' 'vir'! Scrivi 'mo vir facc' pe 'na funzione asincrona."
            .to_string();
    }
    if expects(message, "'bell'") {
        return "Doppo 'nu' ce vo' 'bell'! Scrivi 'nu bell' pe creà n'oggetto nuovo.".to_string();
    }
    if expects(message, "'famiglie'") {
        return "Doppo 'na' ce vo' 'famiglie'! Scrivi 'na famiglie' pe fà 'na classe.".to_string();
    }
    if expects(message, "'cos'") {
        return "Doppo 'stu' ce vo' 'cos'! Scrivi 'stu cos' pe riferisce a this.".to_string();
    }
    if expects(message, "'comme'") {
        return "Ccà ce vo' 'comme' pe dà nu nomme: 'chiamm * comme fs da \"node:fs\"'."
            .to_string();
    }
    if expects(message, "'for'") {
        return "Doppo 'mann' ce vo' 'for'! Scrivi 'mann for' pe esportà.".to_string();
    }
    if expects(message, "'dì'") {
        return "Doppo 'stamm a' ce vo' 'dì'! Scrivi 'stamm a dì' pe stampà.".to_string();
    }
    if message.contains("after the end of the statement") {
        let found = message.split('\'').nth(1).unwrap_or("");
        return format!(
            "Ccà ce sta '{}' ma l'istruzione era già fernuta! Va' a capo o miette ';'.",
            found
        );
    }
    if message.contains("rest element must be the last") {
        return "L'elemento cu '...' adda essere l'urdemo!".to_string();
    }
    if message.contains("Destructuring needs a value") {
        return "Pe spacchettà ce vo' 'nu valore! Aggiunge '= …'.".to_string();
    }
    if message.contains("rest parameter must be the last") {
        return "'O parametro cu '...' adda essere l'urdemo!".to_string();
    }
    if message.contains("Unterminated regex") {
        return "'A regex nun è chiusa! Ce vo' 'nu '/' 'a fine, ncopp' 'a stessa riga.".to_string();
    }
    if message.starts_with("Invalid regex flags") {
        return format!(
            "Sti flag d''a regex nun esistono: {} (se ponno usà d, g, i, m, s, u, v, y, una vota sola)",
            message.trim_start_matches("Invalid regex flags: ")
        );
    }
    if message.contains("nested too deeply") {
        return "Troppa roba una dint'a n'ata! Spartisce 'o codice in variabili o funzioni cchiù piccirille."
            .to_string();
    }
    if message.contains("Unterminated template") {
        return "'O template nun è chiuso! Ce vo' 'nu backtick '`' 'a fine.".to_string();
    }
    if message.contains("Invalid assignment target") {
        return "Ccà nun se pò assegnà niente! Ce vo' 'na variabile o 'na proprietà, tipo 'x' o 'stu cos.nome'.".to_string();
    }
    if let Some(found) =
        message.strip_prefix("Missing 'mentre che (…)' on the line that closes 'primm fa', found ")
    {
        return format!(
            "Doppo 'primm fa {{ … }}' ce vo' 'mentre che (…)' ncopp' 'a stessa riga, ma aggio truvato {}!",
            found
        );
    }
    if let Some(found) = message.strip_prefix("Missing 'caso' or 'predefinit' in 'vedimm', found ")
    {
        return format!(
            "Dint'a 'vedimm' ogni pezzo accummencia cu 'caso …:' o 'predefinit:', ma aggio truvato {}!",
            found
        );
    }
    if message.contains("only one 'predefinit'") {
        return "Nu 'vedimm' po' tené nu 'predefinit' sulo!".to_string();
    }
    if let Some(found) = message.strip_prefix("Missing 'dint' in 'pe aspett', found ") {
        return format!(
            "'pe aspett' vo' 'nu ciclo cu 'dint', tipo 'pe aspett (chist è x dint lista)', ma aggio truvato {}!",
            found
        );
    }
    if message.contains("'pe aspett' works only with 'dint'") {
        return "'pe aspett' funziona sulo cu 'dint', no cu ''ncopp'!".to_string();
    }
    if let Some(rest) = message.strip_prefix('\'') {
        if let Some((name, line)) = rest.split_once("' is already declared (line ") {
            return format!(
                "'{}' già esiste: l'hai dichiarato a riga {}! Dalle n'atu nomme.",
                name,
                line.trim_end_matches(')')
            );
        }
    }
    if let Some(rest) = message.strip_prefix('\'') {
        if let Some((name, available)) =
            rest.split_once("' is not in the fratm library (available: ")
        {
            return format!(
                "'{}' nun sta int'a libreria \"fratm\". Ce stanno: {}.",
                name,
                available.trim_end_matches(')')
            );
        }
    }
    if let Some(names) = message.strip_prefix("The fratm library is imported by name: ") {
        return format!(
            "'A libreria \"fratm\" se chiamma pe nomme, cu 'e graffe: {}",
            names
        );
    }
    if let Some((jump, label)) = message
        .strip_prefix('\'')
        .and_then(|rest| rest.split_once("': no enclosing label '"))
    {
        let label = label.trim_end_matches('\'');
        return if jump.starts_with("salta") {
            format!("'{jump}': nun ce sta nisciuno ciclo cu l'etichetta '{label}:' attuorno")
        } else {
            format!("'{jump}': nun ce sta nisciuna etichetta '{label}:' attuorno")
        };
    }
    if let Some(rest) = message.strip_prefix('\'') {
        if let Some(name) = rest.strip_suffix("' is not declared in the class") {
            return format!(
                "'{}' nun sta dichiarato dint'a sta famiglie: miettelo int'a classe, tipo {} = 0",
                name, name
            );
        }
        if let Some(name) = rest.strip_suffix("' is declared twice in the class") {
            return format!(
                "'{}' sta dichiarato doje vote dint'a stessa famiglie!",
                name
            );
        }
        if let Some(name) = rest.strip_suffix("' is exported but not declared") {
            return format!(
                "'{}' nun esiste: comme 'o manne for si nun l'hai dichiarato?",
                name
            );
        }
    }
    if let Some(keyword) = message.strip_suffix("' is not allowed here") {
        let hint = match keyword.trim_start_matches('\'') {
            "piglie" => "'piglie' se po' usà sulo dint'a na funzione: facc nomme() { piglie … }",
            "aspett" => "'aspett' se po' usà sulo dint'a na funzione 'mo vir' (async): mo vir facc nomme() { aspett … }",
            "pe aspett" => "'pe aspett' se po' usà sulo dint'a na funzione 'mo vir' (async)",
            "rompe" => "'rompe' se po' usà sulo dint'a nu ciclo (mentre, pe, primm fa) o dint'a nu vedimm",
            "o pate" => "''o pate' se po' usà sulo dint'e metodi 'e na famiglie: 'o pate.metodo()",
            "o pate()" => "''o pate(…)' se po' chiammà sulo dint'o costruttore 'e na famiglie figlia: na famiglie A figlio 'e B { facc costruttore() { 'o pate() } }",
            "leva" => "'leva' leva na proprietà, nun na variabile: leva oggetto.campo",
            "mann for" => "'mann for' se po' usà sulo fora 'e funzioni e blocchi, 'ncoppa 'o file",
            "chiamm" => "'chiamm … da' se po' usà sulo fora 'e funzioni e blocchi, 'ncoppa 'o file",
            _ => "'salta' se po' usà sulo dint'a nu ciclo (mentre, pe, primm fa)",
        };
        return hint.to_string();
    }
    if message.contains("'caccia' works only inside") {
        return "'caccia' se po' usà sulo dint'a nu generatore: facc* nomme() { caccia … }"
            .to_string();
    }
    if message.contains("'damme' getter takes no parameters") {
        return "Nu 'damme' (getter) nun piglia parametri: damme nome() { … }".to_string();
    }
    if message.contains("'miette' setter takes exactly one parameter") {
        return "Nu 'miette' (setter) piglia nu parametro sulo: miette nome(valore) { … }"
            .to_string();
    }
    if message.starts_with("Invalid number") {
        return format!(
            "Stu nummero nun se capisce: {}",
            message.trim_start_matches("Invalid number: ")
        );
    }
    if message.contains("expression") || message.contains("espressione") {
        return match found {
            Some(found) => {
                format!(
                    "Ma che staje scrivenn?! Ccà ce vo' 'na espressione, ma aggio truvato {found}!"
                )
            }
            None => "Ma che staje scrivenn?! Ccà ce vo' 'na espressione!".to_string(),
        };
    }

    // Any other "Expected 'x', but found 'y'" from the parser
    if let Some(rest) = message.strip_prefix("Expected '") {
        if let (Some((expected, _)), Some(found)) = (rest.split_once("', but found '"), found) {
            return format!("Ccà ce vo' '{}', ma aggio truvato {}!", expected, found);
        }
    }

    // Default: return with some Napoletano flair
    format!("Uè, c'è nu problema: {}", message)
}

/// Errors of the JSX lexer. Tag and attribute names are quoted in the
/// English message and kept as they are.
fn jsx_message(message: &str) -> Option<String> {
    let quoted: Vec<&str> = message.split('\'').skip(1).step_by(2).collect();
    let first = quoted.first().copied().unwrap_or("");
    let second = quoted.get(1).copied().unwrap_or("");
    let text = if message.starts_with("The fragment '<>'") {
        "'O frammento '<>' nun è chiuso: ce vo' '</>' 'a fine!".to_string()
    } else if message.starts_with("Expected '>' after '/'") {
        format!(
            "Doppo '/' ce vo' '>': scrivi {} />",
            quoted.last().copied().unwrap_or("")
        )
    } else if message.contains("' is closed by '</") {
        format!(
            "Hai araputo {} ma l'hai chiuso cu {}: 'e tag hann'a essere uguali!",
            first, second
        )
    } else if message.starts_with("Expected '>' after '</") {
        format!("Manca '>' pe chiudere {}>", second)
    } else if message.starts_with("Only '{...props}' can go in braces") {
        format!(
            "Mmiez'all'attribute 'e {} dint'e graffe ce po' stà sulo {{...props}}. Pe n'attribute scrive nomme={{valore}}",
            second
        )
    } else if message.starts_with("The tag '<") {
        format!("'O tag {} nun è maje chiuso: ce vo' '>'!", first)
    } else if message.starts_with("Unexpected '") && message.contains("in the tag '<") {
        format!("Che ce fa '{}' dint'o tag {}?", first, second)
    } else if message.starts_with("Unterminated string in the attribute") {
        format!("'A stringa 'e l'attribute '{}' nun è chiusa!", first)
    } else if message.starts_with("The attribute '") && message.contains("empty braces") {
        format!(
            "L'attribute '{}' tene 'e graffe vacante: miette nu valore dint'e {{ }}",
            first
        )
    } else if message.starts_with("The attribute '") && message.contains("needs a value") {
        format!(
            "L'attribute '{}' vo' nu valore tra virgolette o tra graffe: {}=\"…\" o {}={{…}}",
            first, first, first
        )
    } else if message.contains("' is never closed: add '</") {
        format!("{} nun è maje chiuso: ce vo' {}!", first, second)
    } else if message.starts_with("'{...lista}' is not allowed among the children") {
        "'{...lista}' nun se po' mettere mmiez'e figlie: scrive {lista}".to_string()
    } else if message.starts_with("Unexpected '}' in the text of a JSX element") {
        "Che ce fa '}' dint'o testo? Si 'o vuò scrivere, miette {'}'}".to_string()
    } else if message.starts_with("A '{' in JSX is never closed") {
        "Hai araputo na graffa '{' dint'o JSX ma nun l'hai chiusa: ce vo' '}'!".to_string()
    } else if message.starts_with("Expected a tag name") {
        match quoted.first() {
            Some(found) => format!(
                "Doppo '<' ce vo' 'o nomme 'e nu tag, tipo <div>, ma aggio truvato '{}'!",
                found
            ),
            None => "Doppo '<' ce vo' 'o nomme 'e nu tag, tipo <div>!".to_string(),
        }
    } else if message.starts_with("Expected '}' to close the JSX braces") {
        "Ccà ce vo' '}' pe chiudere 'e graffe d'o JSX!".to_string()
    } else {
        return None;
    };
    Some(text)
}

const END_OF_FILE: &str = "'a fine d'o file";

/// The token after `but found '…'`, quoted, or described when it is a line
/// break or the end of the file.
fn found_token(message: &str) -> Option<String> {
    let (_, rest) = message.split_once("but found '")?;
    let token = rest.strip_suffix('\'').unwrap_or(rest);
    Some(match token {
        "\\n" => "'a fine d'a riga".to_string(),
        "EOF" => END_OF_FILE.to_string(),
        other => format!("'{other}'"),
    })
}

/// The parser reports `Expected 'x', but found …`; older messages used `Aspettavo`.
fn expects(message: &str, what: &str) -> bool {
    message.contains(&format!("Expected {}", what))
        || message.contains(&format!("Aspettavo {}", what))
}

// ============== Error Suggestions ==============

/// Get helpful suggestions for an error
pub fn get_suggestion(error: &CompileError) -> Option<String> {
    match error {
        CompileError::ParseError { message, .. } => {
            // Checked first: the message quotes the offending token, which may be `stamm`, `}`...
            if message.contains("after the end of the statement") {
                Some("💡 Scrive n'istruzione pe riga, o separale cu ';'".to_string())
            } else if message.contains("'primm fa'") {
                Some("💡 Esempio: primm fa { i++ } mentre che (i < 3)".to_string())
            } else if message.contains("between the elements") {
                Some("💡 Separa 'e elemente cu ',': f(1, 2), [a, b], { a: 1, b: 2 }".to_string())
            } else if expects(message, "'}'") {
                Some("💡 Conta 'e parentesi graffe: ogni '{' adda avè 'o suo '}'".to_string())
            } else if expects(message, "')'") {
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
        CompileError::SemanticError { message, .. } if message.contains("already declared") => Some(
            "💡 Pe cagnà 'o valore nun ce vo' n'ata dichiarazione: scrive sulo 'x = …' (cu 'tien')"
                .to_string(),
        ),
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

// ============== Typos of dialect keywords ==============

/// Keywords worth suggesting for a typo, with the phrase to write.
/// Short words (`si`, `pe`, `mo`, ...) are left out: too many names are one
/// letter away from them.
const TYPO_CANDIDATES: &[(&str, &str)] = &[
    ("stamm", "stamm a dì"),
    ("avvis", "avvis a dì"),
    ("scrive", "scrive a dì"),
    ("chist", "chist è"),
    ("tien", "tien"),
    ("facc", "facc"),
    ("piglie", "piglie"),
    ("sinnò", "sinnò"),
    ("mentre", "mentre che"),
    ("overo", "overo"),
    ("sfòls", "sfòls"),
    ("nisciun", "nisciun"),
    ("aspett", "aspett"),
    ("pruvamm", "pruvamm"),
    ("schiatta", "e si schiatta"),
    ("iett", "iett"),
    ("famiglie", "na famiglie"),
    ("chiamm", "chiamm"),
    ("predefinit", "predefinit"),
    ("rompe", "rompe"),
    ("salta", "salta"),
    ("fisso", "fisso"),
    ("figlio", "figlio 'e"),
    ("fermete", "fermete"),
    ("manco", "manco"),
    ("vedimm", "vedimm"),
    ("aropp", "aropp"),
    ("primm", "primm fa"),
    ("caccia", "caccia"),
    ("costruttore", "costruttore"),
];

/// Like [`get_suggestion`], but first checks whether the error comes from a
/// misspelled keyword on the error line (`stama a dì` → `stamm a dì`).
pub fn suggest(error: &CompileError, source: &str) -> Option<String> {
    if let CompileError::ParseError { line, column, .. } = error {
        if let Some((word, phrase)) = javascript_keyword(source, *line, *column) {
            return Some(format!(
                "💡 In FratmScript '{}' se scrive '{}'",
                word, phrase
            ));
        }
        if let Some((typo, phrase)) = keyword_typo(source, *line, *column) {
            return Some(format!(
                "💡 Forse vulive scrivere '{}' invece 'e '{}'?",
                phrase, typo
            ));
        }
    }
    get_suggestion(error)
}

// ============== JavaScript keywords ==============

/// JavaScript keywords that FratmScript spells in dialect, with the phrase to
/// write instead. `typeof` and `instanceof` are left out: they are accepted.
const JS_KEYWORDS: &[(&str, &str)] = &[
    ("const", "chist è"),
    ("let", "tien"),
    ("var", "tien"),
    ("function", "facc"),
    ("return", "piglie"),
    ("if", "si"),
    ("else", "sinnò"),
    ("while", "mentre che"),
    ("for", "pe"),
    ("do", "primm fa"),
    ("switch", "vedimm"),
    ("case", "caso"),
    ("default", "predefinit"),
    ("break", "rompe"),
    ("continue", "salta"),
    ("true", "overo"),
    ("false", "sfòls"),
    ("null", "nisciun"),
    ("this", "stu cos"),
    ("new", "nu bell"),
    ("class", "na famiglie"),
    ("extends", "figlio 'e"),
    ("super", "'o pate"),
    ("static", "fisso"),
    ("constructor", "costruttore"),
    ("try", "pruvamm"),
    ("catch", "e si schiatta"),
    ("finally", "aropp"),
    ("throw", "iett"),
    ("async", "mo vir"),
    ("await", "aspett"),
    ("yield", "caccia"),
    ("import", "chiamm"),
    ("export", "mann for"),
    ("delete", "leva"),
    ("debugger", "fermete"),
];

/// The FratmScript phrase for a JavaScript keyword (`const` → `chist è`).
pub fn fratm_for_javascript(word: &str) -> Option<&'static str> {
    JS_KEYWORDS
        .iter()
        .find(|(js, _)| *js == word)
        .map(|(_, phrase)| *phrase)
}

/// The JavaScript keyword closest before the error position (on the same
/// line), with the FratmScript phrase to write instead.
fn javascript_keyword(source: &str, line: usize, column: usize) -> Option<(String, &'static str)> {
    let tokens = crate::lexer::Lexer::new(source).tokenize();
    tokens
        .iter()
        .filter(|t| t.span.line == line && t.span.column <= column)
        .rev()
        .find_map(|t| match &t.kind {
            crate::lexer::TokenKind::Identifier(name) => {
                fratm_for_javascript(name).map(|phrase| (name.clone(), phrase))
            }
            _ => None,
        })
}

/// The identifier closest before the error position (on the same line) that
/// looks like a misspelled keyword, with the keyword phrase it resembles.
fn keyword_typo(source: &str, line: usize, column: usize) -> Option<(String, &'static str)> {
    let tokens = crate::lexer::Lexer::new(source).tokenize();
    tokens
        .iter()
        .filter(|t| t.span.line == line && t.span.column <= column)
        .rev()
        .find_map(|t| match &t.kind {
            crate::lexer::TokenKind::Identifier(name) => {
                closest_keyword(name).map(|phrase| (name.clone(), phrase))
            }
            _ => None,
        })
}

fn closest_keyword(name: &str) -> Option<&'static str> {
    let name = strip_accents(&name.to_lowercase());
    if name.chars().count() < 3 {
        return None;
    }
    TYPO_CANDIDATES
        .iter()
        .filter_map(|(keyword, phrase)| {
            let keyword = strip_accents(keyword);
            let max = if keyword.chars().count() <= 5 { 1 } else { 2 };
            let distance = edit_distance(&name, &keyword);
            (distance > 0 && distance <= max).then_some((distance, *phrase))
        })
        .min_by_key(|(distance, _)| *distance)
        .map(|(_, phrase)| phrase)
}

fn strip_accents(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'à' | 'á' => 'a',
            'è' | 'é' => 'e',
            'ì' | 'í' => 'i',
            'ò' | 'ó' => 'o',
            'ù' | 'ú' => 'u',
            c => c,
        })
        .collect()
}

/// Levenshtein distance, with a swap of two neighbouring letters counted as one edit.
fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut d = vec![vec![0usize; b.len() + 1]; a.len() + 1];
    for (i, row) in d.iter_mut().enumerate() {
        row[0] = i;
    }
    for (j, cell) in d[0].iter_mut().enumerate() {
        *cell = j;
    }
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            d[i][j] = (d[i - 1][j] + 1)
                .min(d[i][j - 1] + 1)
                .min(d[i - 1][j - 1] + cost);
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                d[i][j] = d[i][j].min(d[i - 2][j - 2] + 1);
            }
        }
    }
    d[a.len()][b.len()]
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

    fn hint(source: &str) -> Option<String> {
        let error = crate::compile(source, Default::default()).unwrap_err();
        suggest(&error, source)
    }

    #[test]
    fn test_jsx_errors_are_neapolitan() {
        let message = |source: &str| {
            crate::compile(source, Default::default())
                .unwrap_err()
                .to_string()
        };
        assert_eq!(
            message("x = <div>ciao"),
            "Riga 1, colonna 5: <div> nun è maje chiuso: ce vo' </div>!"
        );
        assert_eq!(
            message("x = <b>ciao</i>"),
            "Riga 1, colonna 5: Hai araputo <b> ma l'hai chiuso cu </i>: 'e tag hann'a essere uguali!"
        );
        assert_eq!(
            message("x = <br/"),
            "Riga 1, colonna 5: Doppo '/' ce vo' '>': scrivi <br />"
        );
        assert_eq!(
            message("x = <a href=/>"),
            "Riga 1, colonna 5: L'attribute 'href' vo' nu valore tra virgolette o tra graffe: href=\"…\" o href={…}"
        );
        assert_eq!(
            message("x = <p>{a b}</p>"),
            "Riga 1, colonna 11: Ccà ce vo' '}' pe chiudere 'e graffe d'o JSX!"
        );
    }

    #[test]
    fn test_javascript_keywords_point_to_the_dialect() {
        assert_eq!(
            hint("const x = 1").as_deref(),
            Some("💡 In FratmScript 'const' se scrive 'chist è'")
        );
        assert_eq!(
            hint("facc f() { return 1 }").as_deref(),
            Some("💡 In FratmScript 'return' se scrive 'piglie'")
        );
        assert_eq!(
            hint("tien d = new Date()").as_deref(),
            Some("💡 In FratmScript 'new' se scrive 'nu bell'")
        );
        assert_eq!(
            hint("if (overo) { }").as_deref(),
            Some("💡 In FratmScript 'if' se scrive 'si'")
        );
        assert_eq!(
            hint("function saluta() { }").as_deref(),
            Some("💡 In FratmScript 'function' se scrive 'facc'")
        );
    }

    #[test]
    fn test_reserved_names_that_compile_get_no_javascript_hint() {
        // `for` is a valid FratmScript name; the error here is the missing value
        assert!(!hint("tien for =").unwrap_or_default().contains("'for'"));
    }

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

    fn message(source: &str) -> String {
        crate::compile(source, Default::default())
            .unwrap_err()
            .to_string()
    }

    #[test]
    fn test_missing_comma_is_not_reported_as_an_unclosed_bracket() {
        for (source, closing) in [
            ("tien o = { a: 1 b: 2 }", "'}'"),
            ("tien a = [1 2]", "']'"),
            ("f(1 2)", "')'"),
            ("facc f(a b) { }", "')'"),
        ] {
            let msg = message(source);
            assert!(
                msg.contains(&format!("'na virgola ',' (o {closing} pe chiudere)")),
                "{source}: {msg}"
            );
            assert!(!msg.contains("nun l'hai chiusa"), "{source}: {msg}");
        }
        assert!(hint("f(1 2)")
            .unwrap()
            .contains("Separa 'e elemente cu ','"));
    }

    #[test]
    fn test_closing_bracket_errors_name_the_token_found() {
        assert!(message("si (x { }").contains("Ccà ce vo' ')', ma aggio truvato '{'!"));
        // At the end of the file the bracket really is unclosed
        assert!(message("si (x) {\n  y = 1\n").contains("nun l'hai chiusa! Mettece '}'"));
        assert!(message("tien x = [1, 2").contains("Chiudela cu ']'"));
    }

    #[test]
    fn test_expression_errors_name_the_token_found() {
        assert!(message("chist è y = 3 +* 4").contains("'na espressione, ma aggio truvato '*'!"));
        assert!(message("tien x = 1 +").contains("ma aggio truvato 'a fine d'o file!"));
        // A stray ')' no longer gets the "count your parentheses" hint
        assert_eq!(hint("tien x = (1 +\n)"), None);
    }

    #[test]
    fn test_unknown_characters_are_named() {
        assert!(message("tien x = 1 @ 2").contains("Stu carattere '@' nun 'o canosco"));
    }

    #[test]
    fn test_line_breaks_are_described_not_escaped() {
        let msg = message("facc f\n() { }");
        assert!(msg.contains("'a fine d'a riga"), "{msg}");
        assert!(!msg.contains("\\n"), "{msg}");
    }

    fn suggestion_for(source: &str) -> Option<String> {
        let err = crate::compile(source, Default::default()).unwrap_err();
        suggest(&err, source)
    }

    #[test]
    fn suggests_misspelled_keywords() {
        let cases = [
            ("stama a dì(1)\n", "'stamm a dì' invece 'e 'stama'"),
            ("stam a di(1)\n", "'stamm a dì' invece 'e 'stam'"),
            ("chsit è x = 1\n", "'chist è' invece 'e 'chsit'"),
            (
                "facc f() {\n    piglia 1\n}\n",
                "'piglie' invece 'e 'piglia'",
            ),
            (
                "tien x = 1\nmentra che (x) { }\n",
                "'mentre che' invece 'e 'mentra'",
            ),
            ("fac f() { }\n", "'facc' invece 'e 'fac'"),
        ];
        for (source, expected) in cases {
            let suggestion = suggestion_for(source).unwrap_or_default();
            assert!(
                suggestion.contains(expected),
                "{:?}: got {:?}",
                source,
                suggestion
            );
        }
    }

    #[test]
    fn no_typo_hint_for_unrelated_names() {
        // `pizza x` is a juxtaposition error, but `pizza` is not close to any keyword
        let suggestion = suggestion_for("tien pizza = 1\npizza 2\n").unwrap_or_default();
        assert!(!suggestion.contains("Forse"), "{:?}", suggestion);
        // Other errors keep their existing suggestion
        let suggestion = suggestion_for("chist è x = (1 + 2\n").unwrap_or_default();
        assert!(suggestion.contains("tonne"), "{:?}", suggestion);
    }

    #[test]
    fn edit_distance_counts_swaps_once() {
        assert_eq!(edit_distance("chsit", "chist"), 1);
        assert_eq!(edit_distance("stama", "stamm"), 1);
        assert_eq!(edit_distance("tien", "tien"), 0);
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
