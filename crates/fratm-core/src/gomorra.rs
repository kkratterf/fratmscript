//! GomorraSQL: SQL written in Neapolitan, translated at compile time.
//!
//! `` gomorra`ripigliammo nomme mmiez 'a pizze arò prezzo > ${minimo}` ``
//! becomes `SELECT nomme FROM pizze WHERE prezzo > ?`, with every `${ … }`
//! turned into a bound parameter: values never end up inside the SQL text.
//!
//! The keywords come from GomorraSQL by Donato Rimenti
//! (<https://github.com/aurasphere/gomorra-sql>, MIT), a Java translator
//! over JDBC. This is a port of its dialect with a few additions for real
//! apps (`mettimmo in fila pe`, `nun cchiù 'e`, `facimm 'a tavula`, …).
//! Queries may span lines and use parentheses; words that are not
//! keywords (column names, `COUNT`, `PRIMARY KEY`, …) pass through as SQL.

/// The SQL of a query in the two placeholder styles drivers expect.
#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    /// `?` placeholders: SQLite, MySQL
    pub sql: String,
    /// `$1`, `$2`… placeholders: PostgreSQL (`pg`)
    pub text: String,
}

/// Where an error sits: the template chunk (`quasis[chunk]`), and the line
/// and character inside that chunk, both from 0.
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub chunk: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GomorraError {
    /// User-facing message, in Neapolitan
    pub message: String,
    pub position: Position,
}

/// Local names of `gomorra` in `chiamm { gomorra } da "fratm"` (an alias
/// with `comme` included): the tags whose templates get translated.
pub fn tag_names(program: &crate::parser::Program) -> Vec<String> {
    program
        .statements
        .iter()
        .filter_map(|statement| match statement {
            crate::parser::Statement::Import {
                specifiers, source, ..
            } if source == crate::stdlib::MODULE => Some(specifiers),
            _ => None,
        })
        .flatten()
        .filter(|s| s.imported == "gomorra")
        .map(|s| s.local.clone())
        .collect()
}

/// Line and column (from 1) in the `.fratm` file of an error position.
/// The first chunk starts right after the backtick; later chunks are
/// placed at the `${ … }` before them, whose end the AST does not keep.
pub fn source_position(
    template: crate::lexer::Span,
    expressions: &[crate::parser::Expression],
    position: &Position,
) -> (usize, usize) {
    let (line, column) = match position.chunk {
        0 => (template.line, template.column + 1),
        n => {
            let span = expressions[n - 1].span();
            (span.line, span.column)
        }
    };
    if position.line == 0 {
        (line, column + position.column)
    } else {
        (line + position.line, position.column + 1)
    }
}

/// (words, SQL, kind). Longer phrases are tried first, so `nun è nisciun`
/// wins over `nun è` and `nun`.
const PHRASES: &[(&[&str], &str, Kind)] = &[
    (
        &["tutto", "chillo", "ch'era", "'o", "nuostro"],
        "*",
        Kind::Value,
    ),
    (&["mettimmo", "in", "fila", "pe"], "ORDER BY", Kind::Clause),
    (&["facimm", "na'", "strage"], "DELETE", Kind::Statement),
    (
        &["facimm", "'a", "tavula"],
        "CREATE TABLE IF NOT EXISTS",
        Kind::Statement,
    ),
    (&["facimm", "'e", "mucchie", "pe"], "GROUP BY", Kind::Clause),
    (
        &["abbrucimm", "'a", "tavula"],
        "DROP TABLE IF EXISTS",
        Kind::Statement,
    ),
    (&["iamme", "bello", "ia'"], "COMMIT", Kind::Statement),
    (&["nun", "è", "nisciun"], "IS NOT NULL", Kind::Value),
    (&["nun", "cchiù", "'e"], "LIMIT", Kind::Clause),
    (&["nun", "assumiglia", "a"], "NOT LIKE", Kind::Operator),
    (&["pesc", "e", "pesc"], "JOIN", Kind::Clause),
    (&["mmiez", "'a"], "FROM", Kind::Clause),
    (&["nzipp", "'ngoppa"], "INSERT INTO", Kind::Statement),
    (&["ua", "uagliò"], "BEGIN", Kind::Statement),
    (&["è", "nisciun"], "IS NULL", Kind::Value),
    (&["nun", "è"], "IS NOT", Kind::Operator),
    (&["nun", "dint'a"], "NOT IN", Kind::Operator),
    (&["assumiglia", "a"], "LIKE", Kind::Operator),
    (&["a", "salì"], "ASC", Kind::Value),
    (&["a", "scennere"], "DESC", Kind::Value),
    (&["ripigliammo"], "SELECT", Kind::Statement),
    (&["rifacimm"], "UPDATE", Kind::Statement),
    (&["sfaccimm"], "ROLLBACK", Kind::Statement),
    (&["accunza"], "SET", Kind::Clause),
    (&["accussì"], "=", Kind::Operator),
    (&["arò"], "WHERE", Kind::Clause),
    (&["chist"], "VALUES", Kind::Clause),
    (&["zumpanno"], "OFFSET", Kind::Clause),
    (&["dint'a"], "IN", Kind::Operator),
    (&["comme"], "AS", Kind::Operator),
    (&["e"], "AND", Kind::Operator),
    (&["o"], "OR", Kind::Operator),
    (&["è"], "IS", Kind::Operator),
    (&["nun"], "NOT", Kind::Operator),
    (&["nisciun"], "NULL", Kind::Value),
    (&["overo"], "TRUE", Kind::Value),
    (&["sfòls"], "FALSE", Kind::Value),
];

/// First words that mean nothing on their own: a typo in the rest of the
/// phrase gets a hint instead of reaching the database as a column name.
const PHRASE_STARTS: &[&str] = &[
    "tutto",
    "mettimmo",
    "facimm",
    "abbrucimm",
    "iamme",
    "pesc",
    "mmiez",
    "nzipp",
    "ua",
    "assumiglia",
];

/// Statement words, as written in the error that lists them.
const STATEMENTS: &str = "ripigliammo, nzipp 'ngoppa, rifacimm, facimm na' strage, facimm 'a tavula, abbrucimm 'a tavula, ua uagliò, iamme bello ia' o sfaccimm";

#[derive(Debug, Clone, Copy, PartialEq)]
enum Kind {
    /// Starts a query: `ripigliammo`, `nzipp 'ngoppa`…
    Statement,
    /// Starts a part of it: `mmiez 'a`, `arò`, `chist`…
    Clause,
    /// Needs something after it: `e`, `accussì`, `dint'a`…
    Operator,
    /// Complete on its own: `nisciun`, `a scennere`, `*`…
    Value,
}

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Word(String),
    /// Text of a `"…"` string, without the quotes
    Str(String),
    Num(String),
    /// The n-th `${ … }`
    Param(usize),
    Punct(&'static str),
}

#[derive(Debug, Clone)]
struct Token {
    tok: Tok,
    /// The word or symbol as written, for messages
    written: String,
    position: Position,
}

/// One piece of the translated query.
#[derive(Debug, Clone)]
enum Item {
    Keyword {
        sql: &'static str,
        kind: Kind,
        written: String,
    },
    Name {
        name: String,
        /// A table name: `pizze (…)` keeps its space
        table: bool,
    },
    Str(String),
    Num(String),
    Param(usize),
    Punct(&'static str),
}

/// Translates the text chunks of a `` gomorra`…` `` template, with a
/// parameter between each pair of chunks.
pub fn translate(chunks: &[String]) -> Result<Query, GomorraError> {
    let tokens = tokenize(chunks)?;
    let items = parse(&tokens)?;
    Ok(Query {
        sql: render(&items, |_| "?".to_string()),
        text: render(&items, |n| format!("${}", n + 1)),
    })
}

fn error(message: impl Into<String>, position: &Position) -> GomorraError {
    GomorraError {
        message: message.into(),
        position: position.clone(),
    }
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '\''
}

fn tokenize(chunks: &[String]) -> Result<Vec<Token>, GomorraError> {
    let mut tokens = Vec::new();
    for (chunk, text) in chunks.iter().enumerate() {
        // Typographic apostrophes, as phones and word processors write them
        let chars: Vec<char> = text
            .chars()
            .map(|c| if c == '’' { '\'' } else { c })
            .collect();
        let (mut line, mut line_start) = (0, 0);
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            let position = Position {
                chunk,
                line,
                column: i - line_start,
            };
            if c == '\n' {
                line += 1;
                line_start = i + 1;
                i += 1;
                continue;
            }
            if c.is_whitespace() {
                i += 1;
                continue;
            }
            let start = i;
            let tok = if c == '-' && chars.get(i + 1) == Some(&'-') {
                // `-- comment` up to the end of the line
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
                continue;
            } else if c == '"' {
                i += 1;
                let mut value = String::new();
                loop {
                    match chars.get(i) {
                        None => {
                            let message = if chunk + 1 < chunks.len() {
                                "Nun mettere ${…} dint'e virgolette: 'o valore ce 'o mette 'o database. Scrive arò nomme = ${nomme}"
                            } else {
                                "'A stringa nun è chiusa: ce vo' n'ata virgoletta \""
                            };
                            return Err(error(message, &position));
                        }
                        Some('"') => {
                            i += 1;
                            break;
                        }
                        // The template text is raw: `\"` and `\\` stand for one character
                        Some('\\') if i + 1 < chars.len() => {
                            value.push(match chars[i + 1] {
                                'n' => '\n',
                                't' => '\t',
                                other => other,
                            });
                            i += 2;
                        }
                        Some('\n') => {
                            return Err(error(
                                "'A stringa nun è chiusa ncopp' 'a stessa riga: ce vo' n'ata virgoletta \"",
                                &position,
                            ))
                        }
                        Some(&other) => {
                            value.push(other);
                            i += 1;
                        }
                    }
                }
                Tok::Str(value)
            } else if c.is_ascii_digit() {
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                if chars.get(i) == Some(&'.')
                    && chars.get(i + 1).is_some_and(|d| d.is_ascii_digit())
                {
                    i += 1;
                    while i < chars.len() && chars[i].is_ascii_digit() {
                        i += 1;
                    }
                }
                Tok::Num(chars[start..i].iter().collect())
            } else if is_word_char(c) {
                while i < chars.len() && is_word_char(chars[i]) {
                    i += 1;
                }
                Tok::Word(chars[start..i].iter().collect())
            } else {
                let two: String = chars[i..chars.len().min(i + 2)].iter().collect();
                let symbol = match two.as_str() {
                    "!=" => Some("!="),
                    "<>" => Some("<>"),
                    "<=" => Some("<="),
                    ">=" => Some(">="),
                    "||" => Some("||"),
                    _ => None,
                };
                match symbol {
                    Some(symbol) => {
                        i += 2;
                        Tok::Punct(symbol)
                    }
                    None => {
                        i += 1;
                        Tok::Punct(match c {
                            ',' => ",",
                            '(' => "(",
                            ')' => ")",
                            '.' => ".",
                            '*' => "*",
                            '=' => "=",
                            '<' => "<",
                            '>' => ">",
                            '+' => "+",
                            '-' => "-",
                            '/' => "/",
                            '%' => "%",
                            ';' => {
                                return Err(error(
                                    "Na query sola pe vota: leva 'o ';' e scrive n'atu gomorra`…`",
                                    &position,
                                ))
                            }
                            other => {
                                return Err(error(
                                    format!(
                                        "Stu carattere '{other}' nun se usa int'a na query gomorra"
                                    ),
                                    &position,
                                ))
                            }
                        })
                    }
                }
            };
            tokens.push(Token {
                written: chars[start..i].iter().collect(),
                tok,
                position,
            });
        }
        if chunk + 1 < chunks.len() {
            let position = Position {
                chunk,
                line,
                column: chars.len() - line_start,
            };
            tokens.push(Token {
                tok: Tok::Param(chunk),
                written: "${…}".to_string(),
                position,
            });
        }
    }
    Ok(tokens)
}

/// `aro'` and `arò` are the same word, like `Arò` and `arò`.
fn normalize(word: &str) -> String {
    let lower = word.to_lowercase();
    let mut chars: Vec<char> = lower.chars().collect();
    if chars.len() >= 2 && chars[chars.len() - 1] == '\'' {
        let accented = match chars[chars.len() - 2] {
            'a' => Some('à'),
            'e' => Some('è'),
            'i' => Some('ì'),
            'o' => Some('ò'),
            'u' => Some('ù'),
            _ => None,
        };
        if let Some(accented) = accented {
            chars.pop();
            chars.pop();
            chars.push(accented);
        }
    }
    chars.into_iter().collect()
}

/// The phrase starting at `tokens[i]`, with how many tokens it takes.
fn phrase_at(tokens: &[Token]) -> Option<(usize, &'static str, Kind)> {
    PHRASES.iter().find_map(|(words, sql, kind)| {
        let matches = words.len() <= tokens.len()
            && words
                .iter()
                .zip(tokens)
                .all(|(word, token)| match &token.tok {
                    Tok::Word(w) => normalize(w) == normalize(word),
                    _ => false,
                });
        matches.then_some((words.len(), *sql, *kind))
    })
}

fn parse(tokens: &[Token]) -> Result<Vec<Item>, GomorraError> {
    let Some(first) = tokens.first() else {
        return Err(error(
            format!("'A query è vacante: accummencia cu {STATEMENTS}"),
            &Position {
                chunk: 0,
                line: 0,
                column: 0,
            },
        ));
    };

    // Words → keywords, names, values
    let mut items: Vec<(Item, &Token)> = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        if let Some((len, sql, kind)) = phrase_at(&tokens[i..]) {
            let written = tokens[i..i + len]
                .iter()
                .map(|t| t.written.as_str())
                .collect::<Vec<_>>()
                .join(" ");
            items.push((Item::Keyword { sql, kind, written }, token));
            i += len;
            continue;
        }
        let item = match &token.tok {
            Tok::Word(w) => {
                let lower = normalize(w);
                if PHRASE_STARTS.contains(&lower.as_str()) {
                    let phrases: Vec<String> = PHRASES
                        .iter()
                        .filter(|(words, _, _)| normalize(words[0]) == lower)
                        .map(|(words, _, _)| words.join(" "))
                        .collect();
                    return Err(error(
                        format!(
                            "'{w}' sulo nun vo' dicere niente: forse vulive scrivere {}?",
                            phrases.join(" o ")
                        ),
                        &token.position,
                    ));
                }
                if w.contains('\'') {
                    return Err(error(
                        format!("Nun capisco '{w}': 'e stringhe int'a query vanno tra virgolette \"…\", o meglio ancora cu ${{…}}"),
                        &token.position,
                    ));
                }
                Item::Name {
                    name: w.clone(),
                    table: false,
                }
            }
            Tok::Str(s) => Item::Str(s.clone()),
            Tok::Num(n) => Item::Num(n.clone()),
            Tok::Param(n) => Item::Param(*n),
            Tok::Punct(p) => Item::Punct(p),
        };
        items.push((item, token));
        i += 1;
    }

    // The query must start with a statement word
    let statement = match &items[0].0 {
        Item::Keyword {
            sql,
            kind: Kind::Statement,
            ..
        } => *sql,
        _ => {
            return Err(error(
                format!(
                    "Na query gomorra accummencia cu {STATEMENTS}, no cu '{}'",
                    first.written
                ),
                &first.position,
            ))
        }
    };

    let mut depth = 0usize;
    for (index, (item, token)) in items.iter().enumerate() {
        let next = items.get(index + 1);
        match item {
            Item::Punct("(") => depth += 1,
            Item::Punct(")") => {
                if depth == 0 {
                    return Err(error(
                        "Che ce fa sta ')'? Nun ce sta nisciuna '(' aperta",
                        &token.position,
                    ));
                }
                depth -= 1;
            }
            // `dint'a (ripigliammo …)`: a subquery is the only statement inside another
            Item::Keyword { sql, written, .. }
                if index > 0
                    && is_statement(sql)
                    && !(*sql == "SELECT" && matches!(items[index - 1].0, Item::Punct("("))) =>
            {
                return Err(error(
                    format!(
                        "'{written}' po' stà sulo 'o principio d'a query: na query sola pe vota"
                    ),
                    &token.position,
                ));
            }
            Item::Keyword {
                sql, kind, written, ..
            } => {
                // `facimm na' strage` is followed by `mmiez 'a`, checked below
                let needs_more = matches!(kind, Kind::Clause | Kind::Operator)
                    || (*kind == Kind::Statement
                        && !matches!(*sql, "BEGIN" | "COMMIT" | "ROLLBACK" | "DELETE"));
                let complete = matches!(
                    next,
                    Some((
                        Item::Name { .. } | Item::Str(_) | Item::Num(_) | Item::Param(_),
                        _
                    )) | Some((Item::Punct("(" | "*" | "-"), _))
                        | Some((
                            Item::Keyword {
                                kind: Kind::Value | Kind::Operator,
                                ..
                            },
                            _
                        ))
                );
                if needs_more && !complete {
                    let found = next
                        .map(|(_, t)| format!(", ma aggio truvato '{}'", t.written))
                        .unwrap_or_default();
                    return Err(error(
                        format!("Doppo '{written}' ce vo' quaccosa{found}"),
                        &token.position,
                    ));
                }
                if matches!(*sql, "BEGIN" | "COMMIT" | "ROLLBACK") {
                    if let Some((_, t)) = next {
                        return Err(error(
                            format!(
                                "'{written}' se scrive sulo, senza niente doppo: che ce fa '{}'?",
                                t.written
                            ),
                            &t.position,
                        ));
                    }
                }
                // Names of tables and columns are SQL, not values
                let wants_name = matches!(
                    *sql,
                    "FROM"
                        | "JOIN"
                        | "UPDATE"
                        | "INSERT INTO"
                        | "CREATE TABLE IF NOT EXISTS"
                        | "DROP TABLE IF EXISTS"
                        | "ORDER BY"
                        | "GROUP BY"
                        | "AS"
                );
                if wants_name {
                    if let Some((Item::Param(_) | Item::Str(_), t)) = next {
                        return Err(error(
                            format!("Doppo '{written}' ce vo' nu nomme scritto dint'a query, no {}: 'e nomme 'e tavule e colonne nun ponno essere parametri", t.written),
                            &t.position,
                        ));
                    }
                }
            }
            _ => {}
        }
    }
    if let Some((Item::Punct(p), last)) = items.last() {
        if !matches!(*p, ")" | "*") {
            return Err(error(
                format!("'A query nun po' fernì cu '{p}': ce manca quaccosa doppo"),
                &last.position,
            ));
        }
    }
    if depth > 0 {
        let (_, last) = items.last().expect("the query is not empty");
        return Err(error(
            "Manca 'a parentesi chiusa: ce vo' ')'",
            &last.position,
        ));
    }
    if statement == "DELETE"
        && !matches!(items.get(1), Some((Item::Keyword { sql: "FROM", .. }, _)))
    {
        return Err(error(
            "Doppo 'facimm na' strage' ce vo' 'mmiez 'a' e 'a tavula: facimm na' strage mmiez 'a pizze arò …",
            &items[0].1.position,
        ));
    }
    if statement == "UPDATE"
        && !items
            .iter()
            .any(|(item, _)| matches!(item, Item::Keyword { sql: "SET", .. }))
    {
        return Err(error(
            "'rifacimm' vo' sapé che cagnà: rifacimm pizze accunza prezzo accussì ${prezzo} arò …",
            &items[0].1.position,
        ));
    }

    let mut items: Vec<Item> = items.into_iter().map(|(item, _)| item).collect();
    // `pizze (…)` after INSERT INTO and CREATE TABLE is not a function call
    for index in 1..items.len() {
        let follows_table_word = matches!(
            items[index - 1],
            Item::Keyword {
                sql: "INSERT INTO" | "CREATE TABLE IF NOT EXISTS" | "FROM" | "JOIN" | "UPDATE",
                ..
            }
        );
        if let (true, Item::Name { table, .. }) = (follows_table_word, &mut items[index]) {
            *table = true;
        }
    }
    if statement == "INSERT INTO" {
        items = insert_parentheses(items);
    }
    Ok(items)
}

fn is_statement(sql: &str) -> bool {
    PHRASES
        .iter()
        .any(|(_, s, kind)| *s == sql && *kind == Kind::Statement)
}

/// GomorraSQL writes `nzipp 'ngoppa user name chist "Pinco"`: the column
/// list and the values get the parentheses SQL needs, unless they are
/// already there (`chist (1, "a"), (2, "b")` for several rows).
fn insert_parentheses(items: Vec<Item>) -> Vec<Item> {
    let values = items
        .iter()
        .position(|item| matches!(item, Item::Keyword { sql: "VALUES", .. }));
    let Some(values) = values else {
        return items;
    };
    let mut out = Vec::with_capacity(items.len() + 4);
    // INSERT INTO table [columns] VALUES …
    out.extend(items[..2.min(values)].iter().cloned());
    let columns = &items[2.min(values)..values];
    let wrap_columns = !columns.is_empty() && !matches!(columns[0], Item::Punct("("));
    if wrap_columns {
        out.push(Item::Punct("("));
    }
    out.extend(columns.iter().cloned());
    if wrap_columns {
        out.push(Item::Punct(")"));
    }
    out.push(items[values].clone());
    let rest = &items[values + 1..];
    // `RETURNING …` (SQLite, PostgreSQL) stays after the values
    let end = rest
        .iter()
        .position(|item| matches!(item, Item::Name { name, .. } if name.eq_ignore_ascii_case("returning")))
        .unwrap_or(rest.len());
    let wrap_values = !matches!(rest.first(), Some(Item::Punct("(")));
    if wrap_values {
        out.push(Item::Punct("("));
    }
    out.extend(rest[..end].iter().cloned());
    if wrap_values {
        out.push(Item::Punct(")"));
    }
    out.extend(rest[end..].iter().cloned());
    out
}

fn render(items: &[Item], placeholder: impl Fn(usize) -> String) -> String {
    let mut out = String::new();
    let mut previous: Option<&Item> = None;
    for item in items {
        let piece = match item {
            Item::Keyword { sql, .. } => sql.to_string(),
            Item::Name { name, .. } => name.clone(),
            Item::Str(s) => format!("'{}'", s.replace('\'', "''")),
            Item::Num(n) => n.clone(),
            Item::Param(n) => placeholder(*n),
            Item::Punct(p) => p.to_string(),
        };
        let glued = match (previous, item) {
            (None, _) => true,
            (_, Item::Punct("," | ")" | ".")) => true,
            (Some(Item::Punct("(" | ".")), _) => true,
            // `COUNT(*)`, but `INSERT INTO pizze (nomme)`
            (Some(Item::Name { table: false, .. }), Item::Punct("(")) => true,
            _ => false,
        };
        if !glued {
            out.push(' ');
        }
        out.push_str(&piece);
        previous = Some(item);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sql(chunks: &[&str]) -> String {
        let chunks: Vec<String> = chunks.iter().map(|c| c.to_string()).collect();
        translate(&chunks).unwrap().sql
    }

    fn fails(chunks: &[&str]) -> GomorraError {
        let chunks: Vec<String> = chunks.iter().map(|c| c.to_string()).collect();
        translate(&chunks).unwrap_err()
    }

    #[test]
    fn test_original_select_examples() {
        assert_eq!(
            sql(&["ripigliammo tutto chillo ch'era 'o nuostro mmiez 'a user"]),
            "SELECT * FROM user"
        );
        assert_eq!(
            sql(&["ripigliammo email mmiez 'a user arò id = 6 o name è nisciun"]),
            "SELECT email FROM user WHERE id = 6 OR name IS NULL"
        );
        assert_eq!(
            sql(&["ripigliammo email mmiez 'a user pesc e pesc city arò user.id = 6 e user.birth_city = city.id"]),
            "SELECT email FROM user JOIN city WHERE user.id = 6 AND user.birth_city = city.id"
        );
    }

    #[test]
    fn test_original_write_examples() {
        assert_eq!(
            sql(&["facimm na' strage mmiez 'a user arò name nun è nisciun o deleted è true"]),
            "DELETE FROM user WHERE name IS NOT NULL OR deleted IS true"
        );
        assert_eq!(
            sql(&["rifacimm user accunza name accussì \"Pinco\", surname accussì \"Pallo\" arò name è nisciun"]),
            "UPDATE user SET name = 'Pinco', surname = 'Pallo' WHERE name IS NULL"
        );
        assert_eq!(
            sql(&["nzipp 'ngoppa user chist 1, \"Pinco\", \"Pallo\""]),
            "INSERT INTO user VALUES (1, 'Pinco', 'Pallo')"
        );
        assert_eq!(
            sql(&["nzipp 'ngoppa user name chist \"Pinco\""]),
            "INSERT INTO user (name) VALUES ('Pinco')"
        );
        assert_eq!(sql(&["ua uagliò"]), "BEGIN");
        assert_eq!(sql(&["iamme bello ia'"]), "COMMIT");
        assert_eq!(sql(&["sfaccimm"]), "ROLLBACK");
    }

    #[test]
    fn test_parameters_in_both_styles() {
        let chunks: Vec<String> = [
            "ripigliammo nomme mmiez 'a pizze arò prezzo > ",
            " e nomme assumiglia a ",
            "",
        ]
        .iter()
        .map(|c| c.to_string())
        .collect();
        let query = translate(&chunks).unwrap();
        assert_eq!(
            query.sql,
            "SELECT nomme FROM pizze WHERE prezzo > ? AND nomme LIKE ?"
        );
        assert_eq!(
            query.text,
            "SELECT nomme FROM pizze WHERE prezzo > $1 AND nomme LIKE $2"
        );
    }

    #[test]
    fn test_extensions() {
        assert_eq!(
            sql(&["ripigliammo nomme, prezzo mmiez 'a pizze\n  mettimmo in fila pe prezzo a scennere, nomme a salì\n  nun cchiù 'e 10 zumpanno 20"]),
            "SELECT nomme, prezzo FROM pizze ORDER BY prezzo DESC, nomme ASC LIMIT 10 OFFSET 20"
        );
        assert_eq!(
            sql(&["ripigliammo COUNT(*) comme quante mmiez 'a pizze arò (prezzo < 5 o prezzo > 10) e id nun dint'a (1, 2)"]),
            "SELECT COUNT(*) AS quante FROM pizze WHERE (prezzo < 5 OR prezzo > 10) AND id NOT IN (1, 2)"
        );
        assert_eq!(
            sql(&["facimm 'a tavula pizze (id INTEGER PRIMARY KEY, nomme TEXT NOT NULL, prezzo REAL)"]),
            "CREATE TABLE IF NOT EXISTS pizze (id INTEGER PRIMARY KEY, nomme TEXT NOT NULL, prezzo REAL)"
        );
        assert_eq!(
            sql(&["abbrucimm 'a tavula pizze"]),
            "DROP TABLE IF EXISTS pizze"
        );
        assert_eq!(
            sql(&["ripigliammo forno, COUNT(*) mmiez 'a pizze facimm 'e mucchie pe forno"]),
            "SELECT forno, COUNT(*) FROM pizze GROUP BY forno"
        );
        assert_eq!(
            sql(&["nzipp 'ngoppa pizze (nomme, prezzo) chist (\"a\", 1), (\"b\", 2) RETURNING id"]),
            "INSERT INTO pizze (nomme, prezzo) VALUES ('a', 1), ('b', 2) RETURNING id"
        );
        assert_eq!(
            sql(&["ripigliammo nomme mmiez 'a pizze arò buona è overo e fritta è sfòls"]),
            "SELECT nomme FROM pizze WHERE buona IS TRUE AND fritta IS FALSE"
        );
    }

    #[test]
    fn test_spelling_variants() {
        assert_eq!(
            sql(&["Ripigliammo nomme mmiez ’a pizze aro' id = 1"]),
            "SELECT nomme FROM pizze WHERE id = 1"
        );
    }

    #[test]
    fn test_quotes_in_strings_are_escaped() {
        assert_eq!(
            sql(&["ripigliammo id mmiez 'a pizze arò nomme = \"d'a casa\""]),
            "SELECT id FROM pizze WHERE nomme = 'd''a casa'"
        );
    }

    #[test]
    fn test_errors() {
        assert!(fails(&["  "]).message.contains("vacante"));
        let e = fails(&["SELECT * FROM pizze"]);
        assert!(
            e.message.contains("accummencia cu ripigliammo"),
            "{}",
            e.message
        );
        let e = fails(&["ripigliammo nomme mmiez a pizze"]);
        assert!(e.message.contains("mmiez 'a"), "{}", e.message);
        let e = fails(&["ripigliammo nomme\nmmiez 'a pizze arò"]);
        assert!(e.message.contains("Doppo 'arò'"), "{}", e.message);
        assert_eq!((e.position.line, e.position.column), (1, 15));
        let e = fails(&["ripigliammo nomme mmiez 'a ", ""]);
        assert!(e.message.contains("nomme 'e tavule"), "{}", e.message);
        let e = fails(&["ripigliammo nomme mmiez 'a pizze arò nomme = \"", "\""]);
        assert!(e.message.contains("virgolette"), "{}", e.message);
        assert!(fails(&["ripigliammo (nomme mmiez 'a pizze"])
            .message
            .contains("')'"));
        assert!(fails(&["ripigliammo nomme mmiez 'a pizze arò ", " ="])
            .message
            .contains("fernì cu '='"));
        assert!(fails(&["sfaccimm; ripigliammo"]).message.contains("';'"));
        assert!(
            fails(&["ripigliammo nomme mmiez 'a pizze arò nomme = 'Pippo'"])
                .message
                .contains("virgolette")
        );
        assert!(fails(&["facimm na' strage pizze"])
            .message
            .contains("mmiez 'a"));
        assert!(fails(&["rifacimm pizze arò id = 1"])
            .message
            .contains("accunza"));
        assert!(fails(&["iamme bello ia' pizze"]).message.contains("sulo"));
        assert!(fails(&["ripigliammo nomme mmiez 'a pizze ripigliammo"])
            .message
            .contains("principio"));
        assert_eq!(
            sql(&["ripigliammo nomme mmiez 'a pizze arò id dint'a (ripigliammo pizza mmiez 'a ordini)"]),
            "SELECT nomme FROM pizze WHERE id IN (SELECT pizza FROM ordini)"
        );
        assert!(fails(&["ripigliammo nomme mmiez 'a pizze arò a @ b"])
            .message
            .contains("'@'"));
    }
}
