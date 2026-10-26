//! `stile`: CSS written in Neapolitan, translated at compile time.
//!
//! `` stile`buttone:passanno { culore: russo; spazio: 8px }` `` becomes
//! `button:hover { color: red; padding: 8px }`. Property names, value
//! keywords, pseudo-classes, `@quanno` (`@media`) and the Neapolitan tag
//! names of [`crate::html`] in selectors are translated; every other word
//! passes through, so plain CSS stays valid. `${ … }` values are kept as
//! they are, and so are strings, comments and `url(…)`.

use crate::gomorra::Position;

/// (Neapolitan property, CSS property)
pub const PROPERTIES: &[(&str, &str)] = &[
    ("culore", "color"),
    ("sfunno", "background"),
    ("sfunno-culore", "background-color"),
    ("sfunno-immagine", "background-image"),
    ("sfunno-grannezza", "background-size"),
    ("scrittura", "font-family"),
    ("scrittura-grannezza", "font-size"),
    ("scrittura-peso", "font-weight"),
    ("scrittura-stile", "font-style"),
    ("allinea-testo", "text-align"),
    ("decorazione-testo", "text-decoration"),
    ("trasforma-testo", "text-transform"),
    ("altezza-riga", "line-height"),
    ("spazio-lettere", "letter-spacing"),
    ("larghezza", "width"),
    ("altezza", "height"),
    ("larghezza-minima", "min-width"),
    ("larghezza-massima", "max-width"),
    ("altezza-minima", "min-height"),
    ("altezza-massima", "max-height"),
    ("margine", "margin"),
    ("margine-ncoppa", "margin-top"),
    ("margine-sotto", "margin-bottom"),
    ("margine-manca", "margin-left"),
    ("margine-destra", "margin-right"),
    ("spazio", "padding"),
    ("spazio-ncoppa", "padding-top"),
    ("spazio-sotto", "padding-bottom"),
    ("spazio-manca", "padding-left"),
    ("spazio-destra", "padding-right"),
    ("bordo", "border"),
    ("bordo-culore", "border-color"),
    ("bordo-larghezza", "border-width"),
    ("bordo-stile", "border-style"),
    ("bordo-tunno", "border-radius"),
    ("bordo-ncoppa", "border-top"),
    ("bordo-sotto", "border-bottom"),
    ("bordo-manca", "border-left"),
    ("bordo-destra", "border-right"),
    ("contorno", "outline"),
    ("mostra", "display"),
    ("posizione", "position"),
    ("ncoppa", "top"),
    ("sotto", "bottom"),
    ("manca", "left"),
    ("destra", "right"),
    ("livello", "z-index"),
    ("trabocca", "overflow"),
    ("visibilità", "visibility"),
    ("visibilita", "visibility"),
    ("opacità", "opacity"),
    ("opacita", "opacity"),
    ("ombra", "box-shadow"),
    ("ombra-testo", "text-shadow"),
    ("cursore", "cursor"),
    ("spazio-mmiez", "gap"),
    ("flessibile", "flex"),
    ("direzione", "flex-direction"),
    ("a-capo", "flex-wrap"),
    ("cresce", "flex-grow"),
    ("allinea-cose", "align-items"),
    ("allinea-me", "align-self"),
    ("giustifica", "justify-content"),
    ("colonne", "grid-template-columns"),
    ("righe", "grid-template-rows"),
    ("area-griglia", "grid-area"),
    ("transizione", "transition"),
    ("trasforma", "transform"),
    ("animazione", "animation"),
    ("contenuto", "content"),
    ("scatola", "box-sizing"),
    ("lista-stile", "list-style"),
    ("filtro", "filter"),
    ("puntatore", "pointer-events"),
    ("selezione", "user-select"),
    ("galleggia", "float"),
    ("adatta-oggetto", "object-fit"),
    ("proporzione", "aspect-ratio"),
];

/// (Neapolitan value, CSS value)
pub const VALUES: &[(&str, &str)] = &[
    ("russo", "red"),
    ("janco", "white"),
    ("niro", "black"),
    ("nero", "black"),
    ("verde", "green"),
    ("blu", "blue"),
    ("azzurro", "skyblue"),
    ("celeste", "lightblue"),
    ("giallo", "yellow"),
    ("arancione", "orange"),
    ("rosa", "pink"),
    ("viola", "purple"),
    ("grigio", "gray"),
    ("marrone", "brown"),
    ("oro", "gold"),
    ("argiento", "silver"),
    ("trasparente", "transparent"),
    ("culore-corrente", "currentcolor"),
    ("nisciuno", "none"),
    ("niente", "none"),
    ("blocco", "block"),
    ("in-linea", "inline"),
    ("blocco-in-linea", "inline-block"),
    ("flessibile", "flex"),
    ("griglia", "grid"),
    ("miezo", "center"),
    ("mmiezo", "center"),
    ("manca", "left"),
    ("destra", "right"),
    ("ncoppa", "top"),
    ("sotto", "bottom"),
    ("inizio", "start"),
    ("fine", "end"),
    ("spartuto", "space-between"),
    ("attuorno", "space-around"),
    ("stiracchiato", "stretch"),
    ("riga", "row"),
    ("colonna", "column"),
    ("fisso", "fixed"),
    ("fissa", "fixed"),
    ("assoluto", "absolute"),
    ("assoluta", "absolute"),
    ("relativo", "relative"),
    ("relativa", "relative"),
    ("appiccecato", "sticky"),
    ("appiccecata", "sticky"),
    ("statico", "static"),
    ("grassetto", "bold"),
    ("corsivo", "italic"),
    ("normale", "normal"),
    ("sottolineato", "underline"),
    ("sbarrato", "line-through"),
    ("maiuscolo", "uppercase"),
    ("minuscolo", "lowercase"),
    ("nascosto", "hidden"),
    ("visibile", "visible"),
    ("scorre", "scroll"),
    ("automatico", "auto"),
    ("manella", "pointer"),
    ("aspetta", "wait"),
    ("ereditato", "inherit"),
    ("iniziale", "initial"),
    ("a-capo", "wrap"),
    ("nun-va-a-capo", "nowrap"),
    ("copre", "cover"),
    ("contiene", "contain"),
    ("ripete", "repeat"),
    ("nun-ripete", "no-repeat"),
    ("continuo", "solid"),
    ("tratteggiato", "dashed"),
    ("puntinato", "dotted"),
    ("bordo-incluso", "border-box"),
    ("facile", "ease"),
    ("lineare", "linear"),
    ("infinito", "infinite"),
    ("alternato", "alternate"),
];

/// (Neapolitan pseudo-class or pseudo-element, CSS name), after `:` or `::`
pub const PSEUDO: &[(&str, &str)] = &[
    ("passanno", "hover"),
    ("premuto", "active"),
    ("fuoco", "focus"),
    ("fuoco-visibile", "focus-visible"),
    ("disattivato", "disabled"),
    ("spuntato", "checked"),
    ("primmo-figlio", "first-child"),
    ("urdemo-figlio", "last-child"),
    ("vacante", "empty"),
    ("radice", "root"),
    ("primma", "before"),
    ("doppo", "after"),
    ("segnaposto", "placeholder"),
];

/// (Neapolitan word, CSS word) inside an `@quanno` (`@media`) prelude
const MEDIA: &[(&str, &str)] = &[
    ("schermo", "screen"),
    ("stampa", "print"),
    ("tutto", "all"),
    ("e", "and"),
];

#[derive(Debug, Clone, PartialEq)]
pub struct StileError {
    /// User-facing message, in Neapolitan
    pub message: String,
    pub position: Position,
}

/// Stands for a `${ … }` between two chunks while the text is translated
const HOLE: char = '\u{0}';

/// Translates the text chunks of a `` stile`…` `` template, with a value
/// between each pair of chunks; the result has as many chunks.
pub fn translate(chunks: &[String]) -> Result<Vec<String>, StileError> {
    if let Some(chunk) = chunks.iter().position(|c| c.contains(HOLE)) {
        let column = chunks[chunk]
            .split('\n')
            .next_back()
            .unwrap_or("")
            .chars()
            .count();
        return Err(StileError {
            message: "nu carattere NUL nun pò stà dint'o CSS".to_string(),
            position: Position {
                chunk,
                line: chunks[chunk].matches('\n').count(),
                column,
            },
        });
    }
    let text: Vec<char> = chunks.join(&HOLE.to_string()).chars().collect();
    let out = Translator {
        text: &text,
        out: String::new(),
    }
    .run()?;
    // The translation copies each HOLE once and never writes a new one
    Ok(out.split(HOLE).map(str::to_string).collect())
}

/// What a run of text between `{`, `}` and `;` turns out to be.
#[derive(PartialEq)]
enum Segment {
    /// Before `{`: selectors or an at-rule
    Prelude,
    /// Before `{` inside a `@keyframes`: `da`, `a` or a percentage
    Keyframe,
    /// Before `;`, `}` or the end: `property: value`
    Declaration,
}

struct Translator<'a> {
    text: &'a [char],
    out: String,
}

impl Translator<'_> {
    fn run(mut self) -> Result<String, StileError> {
        // Each open block: where its `{` is, and whether it is a @keyframes,
        // whose preludes are `da`/`a`/percentages instead of selectors
        let mut open: Vec<(usize, bool)> = Vec::new();
        let mut start = 0;
        let mut i = 0;
        while i < self.text.len() {
            match self.text[i] {
                '/' if self.text.get(i + 1) == Some(&'*') => {
                    i = self.skip_comment(i)?;
                    continue;
                }
                '"' | '\'' => {
                    i = self.skip_string(i)?;
                    continue;
                }
                '\\' => {
                    i += 2;
                    continue;
                }
                '(' => {
                    i = self.skip_parens(i);
                    continue;
                }
                '{' => {
                    let inside_keyframes = open.last().is_some_and(|(_, k)| *k);
                    let kind = if inside_keyframes {
                        Segment::Keyframe
                    } else {
                        Segment::Prelude
                    };
                    let prelude: String = self.text[start..i].iter().collect();
                    self.segment(start, i, kind, !open.is_empty())?;
                    self.out.push('{');
                    open.push((i, prelude.trim_start().starts_with("@keyframes")));
                    start = i + 1;
                }
                '}' => {
                    self.segment(start, i, Segment::Declaration, true)?;
                    if open.pop().is_none() {
                        return Err(self.error("chesta '}' nun chiude niente", i));
                    }
                    self.out.push('}');
                    start = i + 1;
                }
                ';' => {
                    self.segment(start, i, Segment::Declaration, !open.is_empty())?;
                    self.out.push(';');
                    start = i + 1;
                }
                _ => {}
            }
            i += 1;
        }
        self.segment(start, self.text.len(), Segment::Declaration, false)?;
        if let Some(&(at, _)) = open.last() {
            return Err(self.error("manca na '}' pe chiudere chesta '{'", at));
        }
        Ok(self.out)
    }

    fn error(&self, message: &str, at: usize) -> StileError {
        let mut position = Position {
            chunk: 0,
            line: 0,
            column: 0,
        };
        for &c in &self.text[..at] {
            match c {
                HOLE => {
                    position = Position {
                        chunk: position.chunk + 1,
                        line: 0,
                        column: 0,
                    }
                }
                '\n' => {
                    position.line += 1;
                    position.column = 0;
                }
                _ => position.column += 1,
            }
        }
        StileError {
            message: message.to_string(),
            position,
        }
    }

    /// Index after the `*/` of a comment starting at `at`.
    fn skip_comment(&self, at: usize) -> Result<usize, StileError> {
        (at + 2..self.text.len().saturating_sub(1))
            .find(|&j| self.text[j] == '*' && self.text[j + 1] == '/')
            .map(|j| j + 2)
            .ok_or_else(|| self.error("stu commento /* … nun se chiude maje: manca '*/'", at))
    }

    /// Index after the closing quote of a string starting at `at`.
    fn skip_string(&self, at: usize) -> Result<usize, StileError> {
        let quote = self.text[at];
        let mut j = at + 1;
        while j < self.text.len() {
            match self.text[j] {
                '\\' => j += 1,
                c if c == quote => return Ok(j + 1),
                '\n' => break,
                _ => {}
            }
            j += 1;
        }
        Err(self.error(&format!("sta stringa nun se chiude: manca {quote}"), at))
    }

    /// Index after the `)` matching the `(` at `at` (or the end): `{`, `;`
    /// and `}` inside parentheses are not CSS structure.
    fn skip_parens(&self, at: usize) -> usize {
        let mut depth = 0;
        let mut j = at;
        while j < self.text.len() {
            match self.text[j] {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        return j + 1;
                    }
                }
                '\\' => j += 1,
                _ => {}
            }
            j += 1;
        }
        self.text.len()
    }

    /// Writes `text[start..end]` translated as `kind`.
    fn segment(
        &mut self,
        start: usize,
        end: usize,
        kind: Segment,
        in_block: bool,
    ) -> Result<(), StileError> {
        let part = &self.text[start..end];
        let trimmed: String = part.iter().collect::<String>().trim().to_string();
        let first = part.iter().position(|c| !c.is_whitespace());
        if kind == Segment::Keyframe {
            self.words(part, |w, _, _| match w {
                "da" => Some("from"),
                "a" => Some("to"),
                _ => None,
            });
            return Ok(());
        }
        if kind == Segment::Prelude {
            let media = trimmed.starts_with("@quanno")
                && !is_word_char(part_char(part, first.unwrap_or(0) + "@quanno".len()));
            if media {
                let at = first.unwrap_or(0);
                self.copy(&part[..at]);
                self.out.push_str("@media");
                let rest = &part[at + "@quanno".len()..];
                self.words(rest, |w, _, _| {
                    lookup(MEDIA, w)
                        .or_else(|| lookup(PROPERTIES, w))
                        .or_else(|| lookup(VALUES, w))
                });
            } else if trimmed.starts_with('@') {
                self.copy(part);
            } else {
                self.selector(part);
            }
            return Ok(());
        }
        // Empty, an at-rule like `@import …;`, or a whole `${regole}`
        if trimmed.is_empty()
            || trimmed.starts_with('@')
            || (trimmed.contains(HOLE) && !trimmed.contains(':'))
        {
            self.copy(part);
            return Ok(());
        }
        let Some(colon) = top_level_colon(part) else {
            let at = start + first.unwrap_or(0);
            let message = if in_block {
                format!("'{trimmed}' nun è na proprietà: se scrive proprietà: valore, comme culore: russo")
            } else {
                format!("'{trimmed}' nun è na proprietà: se scrive proprietà: valore (o selettore {{ … }})")
            };
            return Err(self.error(&message, at));
        };
        let (property, value) = part.split_at(colon);
        self.words(property, |w, _, _| lookup(PROPERTIES, w));
        self.out.push(':');
        self.value(&value[1..]);
        Ok(())
    }

    fn copy(&mut self, part: &[char]) {
        self.out.extend(part.iter());
    }

    /// A value: keywords translated, `url(…)`, strings and comments kept.
    fn value(&mut self, part: &[char]) {
        let mut i = 0;
        while i < part.len() {
            let c = part[i];
            if c == '!'
                && starts_with(&part[i + 1..], "importante")
                && !is_word_char(part_char(part, i + 11))
            {
                self.out.push_str("!important");
                i += 11;
                continue;
            }
            if c == '"' || c == '\'' || (c == '/' && part.get(i + 1) == Some(&'*')) {
                let end = opaque_end(part, i);
                self.copy(&part[i..end]);
                i = end;
                continue;
            }
            if is_word_char(c) {
                let end = word_end(part, i);
                let word: String = part[i..end].iter().collect();
                let prev = if i == 0 { ' ' } else { part[i - 1] };
                if word.eq_ignore_ascii_case("url") && part.get(end) == Some(&'(') {
                    let close = part[end..]
                        .iter()
                        .position(|&c| c == ')')
                        .map_or(part.len(), |p| end + p + 1);
                    self.copy(&part[i..close]);
                    i = close;
                    continue;
                }
                let next = part.get(end).copied().unwrap_or(' ');
                let translated = (!word.starts_with(|c: char| c.is_ascii_digit())
                    && !matches!(prev, '#' | '.' | HOLE | '@' | '$')
                    && next != HOLE)
                    .then(|| lookup(VALUES, &word))
                    .flatten();
                self.out.push_str(translated.unwrap_or(&word));
                i = end;
                continue;
            }
            self.out.push(c);
            i += 1;
        }
    }

    /// Selectors: Neapolitan tags and pseudo-classes, but not class names,
    /// ids, attribute selectors or strings.
    fn selector(&mut self, part: &[char]) {
        let mut i = 0;
        let mut brackets = 0;
        while i < part.len() {
            let c = part[i];
            match c {
                '[' => brackets += 1,
                ']' => brackets -= 1,
                '"' | '\'' => {
                    let end = opaque_end(part, i);
                    self.copy(&part[i..end]);
                    i = end;
                    continue;
                }
                '/' if part.get(i + 1) == Some(&'*') => {
                    let end = opaque_end(part, i);
                    self.copy(&part[i..end]);
                    i = end;
                    continue;
                }
                '(' if brackets == 0 => {
                    // `:not(scatola)`, `:is(…)`: a selector list inside
                    self.out.push('(');
                    let close = matching(part, i);
                    self.selector(&part[i + 1..close.min(part.len())]);
                    if close < part.len() {
                        self.out.push(')');
                    }
                    i = close + 1;
                    continue;
                }
                _ => {}
            }
            if is_word_char(c) && brackets == 0 {
                let end = word_end(part, i);
                let word: String = part[i..end].iter().collect();
                let prev = if i == 0 { ' ' } else { part[i - 1] };
                let next = part.get(end).copied().unwrap_or(' ');
                let translated = if next == HOLE || matches!(prev, HOLE | '.' | '#' | '-') {
                    None
                } else if prev == ':' {
                    lookup(PSEUDO, &word)
                } else {
                    Some(crate::html::tag(&word)).filter(|tag| *tag != word)
                };
                self.out.push_str(translated.unwrap_or(&word));
                i = end;
                continue;
            }
            self.out.push(c);
            i += 1;
        }
    }

    /// Translates each word of `part` with `f`, copying everything else.
    fn words(&mut self, part: &[char], f: impl Fn(&str, char, char) -> Option<&'static str>) {
        let mut i = 0;
        while i < part.len() {
            if is_word_char(part[i]) {
                let end = word_end(part, i);
                let word: String = part[i..end].iter().collect();
                let prev = if i == 0 { ' ' } else { part[i - 1] };
                let next = part.get(end).copied().unwrap_or(' ');
                let translated = (prev != HOLE && next != HOLE && prev != '-')
                    .then(|| f(&word, prev, next))
                    .flatten();
                self.out.push_str(translated.unwrap_or(&word));
                i = end;
            } else {
                self.out.push(part[i]);
                i += 1;
            }
        }
    }
}

fn lookup(table: &[(&str, &'static str)], word: &str) -> Option<&'static str> {
    table
        .iter()
        .find(|(nap, _)| *nap == word)
        .map(|(_, css)| *css)
}

/// Letters (accented ones too), digits, `-` and `_`: one CSS identifier.
fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '-' || c == '_'
}

fn part_char(part: &[char], i: usize) -> char {
    part.get(i).copied().unwrap_or(' ')
}

fn starts_with(part: &[char], word: &str) -> bool {
    word.chars()
        .enumerate()
        .all(|(i, c)| part.get(i) == Some(&c))
}

fn word_end(part: &[char], start: usize) -> usize {
    (start..part.len())
        .find(|&j| !is_word_char(part[j]))
        .unwrap_or(part.len())
}

/// End of the string or comment starting at `start` (strings and comments
/// were checked to be closed before).
fn opaque_end(part: &[char], start: usize) -> usize {
    if part[start] == '/' {
        return (start + 2..part.len().saturating_sub(1))
            .find(|&j| part[j] == '*' && part[j + 1] == '/')
            .map_or(part.len(), |j| j + 2);
    }
    let quote = part[start];
    let mut j = start + 1;
    while j < part.len() {
        match part[j] {
            '\\' => j += 1,
            c if c == quote => return j + 1,
            _ => {}
        }
        j += 1;
    }
    part.len()
}

/// Index of the `)` matching the `(` at `open`, or `part.len()`.
fn matching(part: &[char], open: usize) -> usize {
    let mut depth = 0;
    for (j, &c) in part.iter().enumerate().skip(open) {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return j;
                }
            }
            _ => {}
        }
    }
    part.len()
}

/// The `:` between a property and its value: not inside parentheses,
/// strings or comments.
fn top_level_colon(part: &[char]) -> Option<usize> {
    let mut depth = 0;
    let mut i = 0;
    while i < part.len() {
        match part[i] {
            '(' => depth += 1,
            ')' => depth -= 1,
            '"' | '\'' => {
                i = opaque_end(part, i);
                continue;
            }
            '/' if part.get(i + 1) == Some(&'*') => {
                i = opaque_end(part, i);
                continue;
            }
            ':' if depth == 0 => return Some(i),
            _ => {}
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn css(text: &str) -> String {
        translate(&[text.to_string()]).unwrap().remove(0)
    }

    fn err(text: &str) -> StileError {
        translate(&[text.to_string()]).unwrap_err()
    }

    #[test]
    fn properties_values_and_selectors() {
        assert_eq!(
            css(".pizza { culore: russo; sfunno-culore: janco; spazio: 8px 4px }"),
            ".pizza { color: red; background-color: white; padding: 8px 4px }"
        );
        assert_eq!(
            css("buttone:passanno, scatola > scritta::primma { mostra: flessibile; giustifica: spartuto }"),
            "button:hover, div > p::before { display: flex; justify-content: space-between }"
        );
        // Declarations without a selector, for one element
        assert_eq!(
            css("culore: niro; margine: 0 automatico"),
            "color: black; margin: 0 auto"
        );
        assert_eq!(
            css("posizione: fissa !importante"),
            "position: fixed !important"
        );
    }

    #[test]
    fn plain_css_and_names_stay_as_they_are() {
        let plain = "@import url(\"a.css\");\n.russo#janco > div:hover { color: #fff; font-family: 'russo', sans-serif }";
        assert_eq!(css(plain), plain);
        // Custom properties keep their name, values are translated
        assert_eq!(
            css(":radice { --accento: russo }"),
            ":root { --accento: red }"
        );
        // Class names, ids, units and numbers are never translated
        assert_eq!(css(".sotto { ncoppa: 10px }"), ".sotto { top: 10px }");
        assert_eq!(
            css("scatola { sfunno: url(russo.png) /* russo */ russo }"),
            "div { background: url(russo.png) /* russo */ red }"
        );
        // Attribute selectors and :not() lists
        assert_eq!(
            css("casella[type=\"buttone\"]:not(.a, buttone) { culore: verde }"),
            "input[type=\"buttone\"]:not(.a, button) { color: green }"
        );
    }

    #[test]
    fn media_queries_and_nesting() {
        assert_eq!(
            css("@quanno schermo e (larghezza-massima: 600px) { core { spazio: 0 } }"),
            "@media screen and (max-width: 600px) { main { padding: 0 } }"
        );
        assert_eq!(
            css(".a { culore: russo; &:passanno { culore: blu } }"),
            ".a { color: red; &:hover { color: blue } }"
        );
        // Other at-rules pass through; inside @keyframes `da`/`a` are from/to
        assert_eq!(
            css("@keyframes gira { from { trasforma: rotate(0) } }"),
            "@keyframes gira { from { transform: rotate(0) } }"
        );
        assert_eq!(
            css("@keyframes trase { da { opacità: 0 } 50% { opacità: 0.5 } a { opacità: 1 } }"),
            "@keyframes trase { from { opacity: 0 } 50% { opacity: 0.5 } to { opacity: 1 } }"
        );
        // Outside @keyframes, `a` is still the link tag
        assert_eq!(css("a { culore: blu }"), "a { color: blue }");
    }

    #[test]
    fn values_from_code_are_kept() {
        let chunks: Vec<String> = [".pizza { culore: ", "; larghezza: ", "px }"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(
            translate(&chunks).unwrap(),
            [".pizza { color: ", "; width: ", "px }"]
        );
        // A word glued to a value is not a keyword
        let chunks: Vec<String> = ["sfunno: russo", "", ""]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(translate(&chunks).unwrap(), ["background: russo", "", ""]);
        // A whole block of rules from code
        let chunks: Vec<String> = ["", "\nscatola { culore: russo }"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(translate(&chunks).unwrap(), ["", "\ndiv { color: red }"]);
    }

    #[test]
    fn errors_name_the_problem_and_its_place() {
        let e = err(".a { culore: russo }\n}");
        assert_eq!(e.message, "chesta '}' nun chiude niente");
        assert_eq!((e.position.line, e.position.column), (1, 0));
        let e = err(".a {\n  culore: russo\n");
        assert!(e.message.starts_with("manca na '}'"), "{}", e.message);
        assert_eq!((e.position.line, e.position.column), (0, 3));
        let e = err(".a {\n  culore russo }");
        assert_eq!(
            e.message,
            "'culore russo' nun è na proprietà: se scrive proprietà: valore, comme culore: russo"
        );
        assert_eq!((e.position.line, e.position.column), (1, 2));
        assert!(err(".a { /* uè }").message.contains("'*/'"));
        assert!(err(".a { contenuto: \"uè }")
            .message
            .contains("nun se chiude"));
        let chunks: Vec<String> = ["culore: ", ";\n  sbagliato"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let e = translate(&chunks).unwrap_err();
        assert_eq!(
            (e.position.chunk, e.position.line, e.position.column),
            (1, 1, 2)
        );
    }

    #[test]
    fn tables_have_no_duplicates() {
        for table in [PROPERTIES, VALUES, PSEUDO] {
            for (i, (nap, _)) in table.iter().enumerate() {
                assert!(table[i + 1..].iter().all(|(o, _)| o != nap), "{nap} twice");
            }
        }
    }
}
