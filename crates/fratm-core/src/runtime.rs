//! Neapolitan versions of JavaScript runtime errors.
//!
//! Compile errors are already in dialect; this module does the same for the
//! errors thrown while the generated JavaScript runs. CLI (`fratm lancia`),
//! playground and npm package share it through `fratm-core` and the WASM
//! build. Messages from V8 (Node, Chrome), SpiderMonkey (Firefox) and
//! JavaScriptCore (Safari) are recognised; anything else is left alone.

/// Translates a runtime error into Neapolitan, keeping the original in
/// brackets: `'x' nun esiste … (ReferenceError: x is not defined)`.
///
/// Errors the program throws itself (`iett nu bell Error("…")`) and
/// messages that are not recognised come back as `name: message`.
pub fn napoletanize_runtime_error(name: &str, message: &str) -> String {
    let original = if message.is_empty() {
        name.to_string()
    } else {
        format!("{}: {}", name, message)
    };
    match translate(message) {
        Some(text) => format!("{} ({})", text, original),
        None => original,
    }
}

fn translate(message: &str) -> Option<String> {
    let m = message.trim();

    // ReferenceError
    if let Some(name) = m.strip_suffix(" is not defined") {
        return Some(not_declared(&unquote(name)));
    }
    if let Some(name) = m.strip_prefix("Can't find variable: ") {
        return Some(not_declared(name));
    }
    if let Some(rest) = m.strip_prefix("Cannot access ") {
        if let Some(name) = rest.strip_suffix(" before initialization") {
            return Some(format!(
                "'{}' l'hê usato primma 'e l'avè dichiarato",
                unquote(name)
            ));
        }
    }
    if let Some(rest) = m.strip_prefix("can't access lexical declaration ") {
        let name = rest.split(' ').next().unwrap_or(rest);
        return Some(format!(
            "'{}' l'hê usato primma 'e l'avè dichiarato",
            unquote(name)
        ));
    }

    // TypeError
    if m == "Assignment to constant variable." || m.starts_with("invalid assignment to const") {
        return Some(
            "Nun se pò cagnà na costante ('chist è'): si 'o vuò cagnà, dichiaralo cu 'tien'"
                .to_string(),
        );
    }
    if let Some(name) = m.strip_suffix(" is not a function") {
        return Some(format!(
            "'{}' nun è na funzione, nun 'o puoi chiammà",
            unquote(name)
        ));
    }
    if let Some(name) = m.strip_suffix(" is not a constructor") {
        return Some(format!(
            "'{}' nun se pò creà cu 'nu bell': nun è na famiglie",
            unquote(name)
        ));
    }
    if let Some(name) = m.strip_suffix(" is not iterable") {
        return Some(format!(
            "'{}' nun se pò scorrere cu 'pe ogni': ce vo' na lista",
            unquote(name)
        ));
    }
    for (prefix, verb) in [
        ("Cannot read properties of ", "leggere"),
        ("Cannot set properties of ", "mettere"),
    ] {
        if let Some(rest) = m.strip_prefix(prefix) {
            let (value, prop) = match rest.split_once(" (") {
                Some((value, prop)) => (value, prop),
                None => (rest, ""),
            };
            let prop = prop
                .trim_end_matches(')')
                .trim_start_matches("reading ")
                .trim_start_matches("setting ");
            return Some(empty_value_message(verb, value, &unquote(prop)));
        }
    }
    // Firefox: can't access property "x", o is undefined
    if let Some(rest) = m.strip_prefix("can't access property ") {
        if let Some((prop, tail)) = rest.split_once(", ") {
            let value = tail.rsplit(' ').next().unwrap_or("undefined");
            return Some(empty_value_message("leggere", value, &unquote(prop)));
        }
    }
    // Safari: undefined is not an object (evaluating 'o.x')
    if let Some(rest) = m.strip_suffix(')') {
        if let Some((value, expr)) = rest.split_once(" is not an object (evaluating ") {
            let prop = unquote(expr);
            let prop = prop.rsplit('.').next().unwrap_or(&prop).to_string();
            return Some(empty_value_message("leggere", value, &prop));
        }
    }

    // RangeError
    if m.starts_with("Maximum call stack size exceeded") || m == "too much recursion" {
        return Some(
            "Troppa ricorsione! Na funzione se chiamma da sola e nun se ferma maje".to_string(),
        );
    }
    if m == "Invalid array length" {
        return Some("Na lista nun pò tenè sta lunghezza".to_string());
    }

    None
}

fn empty_value_message(verb: &str, value: &str, prop: &str) -> String {
    let dialect = match value {
        "null" => "nisciun (null)",
        _ => "boh (undefined)",
    };
    if prop.is_empty() {
        format!("Staje cercanno 'e {} na proprietà 'e {}", verb, dialect)
    } else {
        format!(
            "Staje cercanno 'e {} '{}' 'e {}: 'o valore nun ce sta",
            verb, prop, dialect
        )
    }
}

/// Message for a name that was never declared. A JS keyword used as a name
/// is escaped to `$true`, `$this`…: then the dialect keyword is named.
fn not_declared(name: &str) -> String {
    match name
        .strip_prefix('$')
        .and_then(crate::errors::fratm_for_javascript)
    {
        Some(phrase) => format!(
            "'{}' nun esiste: in FratmScript se scrive '{}'",
            &name[1..],
            phrase
        ),
        None => format!(
            "'{}' nun esiste: nun l'hê maje dichiarato cu 'chist è' o 'tien'",
            name
        ),
    }
}

fn unquote(s: &str) -> String {
    s.trim()
        .trim_matches(|c| c == '\'' || c == '"' || c == '`')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nap(name: &str, message: &str) -> String {
        napoletanize_runtime_error(name, message)
    }

    #[test]
    fn translates_reference_errors() {
        assert_eq!(
            nap("ReferenceError", "nonEsiste is not defined"),
            "'nonEsiste' nun esiste: nun l'hê maje dichiarato cu 'chist è' o 'tien' \
             (ReferenceError: nonEsiste is not defined)"
        );
        assert!(nap("ReferenceError", "Can't find variable: x").starts_with("'x' nun esiste"));
        assert_eq!(
            nap("ReferenceError", "$true is not defined"),
            "'true' nun esiste: in FratmScript se scrive 'overo' \
             (ReferenceError: $true is not defined)"
        );
        assert!(nap("ReferenceError", "Can't find variable: $this")
            .starts_with("'this' nun esiste: in FratmScript se scrive 'stu cos'"));
        assert!(
            nap("ReferenceError", "Cannot access 'x' before initialization")
                .starts_with("'x' l'hê usato primma")
        );
    }

    #[test]
    fn translates_type_errors() {
        assert!(nap("TypeError", "Assignment to constant variable.")
            .starts_with("Nun se pò cagnà na costante"));
        assert!(nap("TypeError", "lista.mappa is not a function")
            .starts_with("'lista.mappa' nun è na funzione"));
        assert!(
            nap("TypeError", "Pizza is not a constructor").starts_with("'Pizza' nun se pò creà")
        );
        assert!(nap("TypeError", "x is not iterable").starts_with("'x' nun se pò scorrere"));
        assert_eq!(
            nap(
                "TypeError",
                "Cannot read properties of undefined (reading 'lungh')"
            ),
            "Staje cercanno 'e leggere 'lungh' 'e boh (undefined): 'o valore nun ce sta \
             (TypeError: Cannot read properties of undefined (reading 'lungh'))"
        );
        assert!(
            nap("TypeError", "Cannot set properties of null (setting 'x')")
                .starts_with("Staje cercanno 'e mettere 'x' 'e nisciun (null)")
        );
    }

    #[test]
    fn translates_other_engines() {
        assert!(
            nap("TypeError", "can't access property \"x\", o is undefined")
                .starts_with("Staje cercanno 'e leggere 'x' 'e boh")
        );
        assert!(
            nap("TypeError", "undefined is not an object (evaluating 'o.x')")
                .starts_with("Staje cercanno 'e leggere 'x' 'e boh")
        );
        assert!(nap("InternalError", "too much recursion").starts_with("Troppa ricorsione"));
    }

    #[test]
    fn leaves_program_errors_alone() {
        assert_eq!(nap("Error", "Pizza bruciata"), "Error: Pizza bruciata");
        assert_eq!(nap("Error", ""), "Error");
    }
}
