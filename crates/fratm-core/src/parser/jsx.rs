//! Turns the JSX tokens of the lexer into AST nodes: parses every `{ … }`
//! and cleans up text the way Babel and TypeScript do.

use super::{parse_interpolation, Expression, JsxAttribute, JsxChild, JsxElement, ParseError};
use crate::lexer::{JsxToken, JsxTokenAttribute, JsxTokenChild, JsxTokenValue, Span, Token};

pub(super) fn element(
    token: &JsxToken,
    span: Span,
    depth: usize,
) -> Result<JsxElement, ParseError> {
    let attributes = token
        .attributes
        .iter()
        .map(|attribute| self::attribute(attribute, span, depth))
        .collect::<Result<Vec<_>, _>>()?;
    let mut children = Vec::new();
    for child in token.children.iter().flatten() {
        match child {
            JsxTokenChild::Text(raw) => {
                let text = jsx_text(raw);
                if !text.is_empty() {
                    children.push(JsxChild::Text(text));
                }
            }
            // `{}` and `{/* comment */}` render nothing
            JsxTokenChild::Expression(tokens, _) if tokens.is_empty() => {}
            JsxTokenChild::Expression(tokens, brace) => {
                children.push(JsxChild::Expression(expression(tokens, *brace, depth)?))
            }
            JsxTokenChild::Element(nested) => {
                children.push(JsxChild::Element(element(nested, span, depth)?))
            }
        }
    }
    Ok(JsxElement {
        name: token.name.clone(),
        attributes,
        children,
    })
}

fn attribute(
    attribute: &JsxTokenAttribute,
    span: Span,
    depth: usize,
) -> Result<JsxAttribute, ParseError> {
    Ok(match attribute {
        JsxTokenAttribute::Spread(tokens) => JsxAttribute::Spread(expression(tokens, span, depth)?),
        JsxTokenAttribute::Named {
            name,
            value,
            span: name_span,
        } => JsxAttribute::Named {
            name: name.clone(),
            value: match value {
                None => None,
                Some(JsxTokenValue::String(raw)) => Some(Expression::String {
                    value: decode_entities(raw),
                    span: *name_span,
                }),
                Some(JsxTokenValue::Expression(tokens)) => {
                    Some(expression(tokens, *name_span, depth)?)
                }
                Some(JsxTokenValue::Element(nested)) => Some(Expression::Jsx {
                    element: Box::new(element(nested, *name_span, depth)?),
                    span: *name_span,
                }),
            },
            span: *name_span,
        },
    })
}

fn expression(tokens: &[Token], span: Span, depth: usize) -> Result<Expression, ParseError> {
    parse_interpolation(tokens.to_vec(), span, depth).map_err(|mut error| {
        error.message = error
            .message
            .replace("inside '${}'", "inside '{}'")
            .replace(
                "Expected '}', but",
                "Expected '}' to close the JSX braces, but",
            );
        error
    })
}

/// JSX text as React receives it: lines are trimmed, lines with only
/// whitespace are dropped, the others are joined with one space, and HTML
/// entities are decoded. Same rules as Babel's `cleanJSXElementLiteralChild`.
pub fn jsx_text(raw: &str) -> String {
    let lines: Vec<&str> = raw.split('\n').map(|l| l.trim_end_matches('\r')).collect();
    let last_non_empty = lines
        .iter()
        .rposition(|l| l.chars().any(|c| c != ' ' && c != '\t'));
    let mut out = String::new();
    for (i, line) in lines.iter().enumerate() {
        let mut line = line.replace('\t', " ");
        if i > 0 {
            line = line.trim_start_matches(' ').to_string();
        }
        if i + 1 < lines.len() {
            line = line.trim_end_matches(' ').to_string();
        }
        if !line.is_empty() {
            out.push_str(&line);
            if Some(i) != last_non_empty {
                out.push(' ');
            }
        }
    }
    decode_entities(&out)
}

/// `&amp;`, `&nbsp;`, `&#123;`, `&#x7B;` and the other common entities;
/// anything else is left as written.
pub fn decode_entities(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        rest = &rest[amp..];
        let decoded = rest[1..]
            .find(';')
            .filter(|end| *end <= 10)
            .and_then(|end| entity(&rest[1..1 + end]).map(|c| (c, end + 2)));
        match decoded {
            Some((c, len)) => {
                out.push(c);
                rest = &rest[len..];
            }
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

fn entity(name: &str) -> Option<char> {
    if let Some(number) = name.strip_prefix('#') {
        let code = match number.strip_prefix(['x', 'X']) {
            Some(hex) => u32::from_str_radix(hex, 16).ok()?,
            None => number.parse().ok()?,
        };
        return char::from_u32(code);
    }
    Some(match name {
        "amp" => '&',
        "lt" => '<',
        "gt" => '>',
        "quot" => '"',
        "apos" => '\'',
        "nbsp" => '\u{a0}',
        "copy" => '©',
        "reg" => '®',
        "trade" => '™',
        "hellip" => '…',
        "mdash" => '—',
        "ndash" => '–',
        "laquo" => '«',
        "raquo" => '»',
        "lsquo" => '‘',
        "rsquo" => '’',
        "ldquo" => '“',
        "rdquo" => '”',
        "euro" => '€',
        "times" => '×',
        "divide" => '÷',
        "middot" => '·',
        "bull" => '•',
        "deg" => '°',
        "rarr" => '→',
        "larr" => '←',
        "hearts" => '♥',
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_is_trimmed_like_babel() {
        assert_eq!(jsx_text("ciao"), "ciao");
        assert_eq!(jsx_text("  a  b  "), "  a  b  ");
        assert_eq!(jsx_text("\n    Uè,\n    Gennaro!\n  "), "Uè, Gennaro!");
        assert_eq!(jsx_text("\n   \n  "), "");
        assert_eq!(jsx_text("Pizze: "), "Pizze: ");
        assert_eq!(jsx_text("\n  fine"), "fine");
        assert_eq!(jsx_text("a\t\n\tb"), "a b");
    }

    #[test]
    fn entities_are_decoded() {
        assert_eq!(decode_entities("a &amp; b &lt;3"), "a & b <3");
        assert_eq!(decode_entities("&#123;&#x7D;&nbsp;"), "{}\u{a0}");
        assert_eq!(decode_entities("&mdash; &euro; &copy;"), "— € ©");
        // Unknown or unterminated entities stay as written
        assert_eq!(decode_entities("&pizza; & &amp"), "&pizza; & &amp");
    }
}
