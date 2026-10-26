//! Neapolitan names for HTML tags.
//!
//! `<scatola>`, `<scritta>` and `<buttone>` in JSX compile to `div`, `p` and
//! `button`, so the DOM, devtools and CSS only ever see standard tags. HTML
//! names keep working: a name that is not in the table is used as written.

/// (Neapolitan tag, HTML tag, short description in Neapolitan)
pub const TAGS: &[(&str, &str, &str)] = &[
    ("scatola", "div", "na scatola pe raggruppà cose"),
    ("piezzo", "span", "nu piezzo 'e testo dint'a na riga"),
    ("scritta", "p", "nu paragrafo"),
    ("titolo", "h1", "'o titolo d'a pagina"),
    ("titolo2", "h2", "nu titolo cchiù piccerillo"),
    ("titolo3", "h3", "nu titolo 'e terzo livello"),
    ("titolo4", "h4", "nu titolo 'e quarto livello"),
    ("titolo5", "h5", "nu titolo 'e quinto livello"),
    ("titolo6", "h6", "nu titolo 'e sesto livello"),
    ("buttone", "button", "nu buttone ca se pò premere"),
    ("casella", "input", "na casella pe scrivere"),
    ("quaderno", "textarea", "nu quaderno pe scrivere assaje"),
    ("scegli", "select", "nu menù a tendina"),
    ("opzione", "option", "na voce 'e nu menù a tendina"),
    ("etichetta", "label", "l'etichetta 'e na casella"),
    ("modulo", "form", "nu modulo da mannà"),
    ("lista", "ul", "na lista cu 'e pallini"),
    ("elenco", "ol", "na lista cu 'e nummeri"),
    ("voce", "li", "na voce 'e na lista"),
    ("collegamento", "a", "nu collegamento a n'ata pagina"),
    ("immagine", "img", "na fotografia"),
    ("capa", "header", "'a capa d'a pagina"),
    ("pede", "footer", "'o pede d'a pagina"),
    ("core", "main", "'o core d'a pagina"),
    ("sezione", "section", "na sezione"),
    ("articolo", "article", "n'articolo"),
    ("navigazione", "nav", "'e collegamenti pe girà"),
    ("acapo", "br", "va a capo"),
    ("linea", "hr", "na linea ca sparte"),
    ("forte", "strong", "testo forte"),
    ("corsivo", "em", "testo storto"),
    ("codice", "code", "nu piezzo 'e codice"),
    ("tabella", "table", "na tabella"),
    ("riga", "tr", "na riga 'e tabella"),
    ("cella", "td", "na cella 'e tabella"),
    ("capocella", "th", "na cella 'e capa 'e tabella"),
    ("tela", "canvas", "na tela pe disegnà"),
];

/// The HTML tag for a JSX tag name: the translation of a Neapolitan name,
/// or the name itself.
pub fn tag(name: &str) -> &str {
    TAGS.iter()
        .find(|(nap, _, _)| *nap == name)
        .map_or(name, |(_, html, _)| html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neapolitan_tags_become_html() {
        assert_eq!(tag("scatola"), "div");
        assert_eq!(tag("buttone"), "button");
        assert_eq!(tag("titolo2"), "h2");
        // HTML names and custom elements pass through
        assert_eq!(tag("div"), "div");
        assert_eq!(tag("mio-widget"), "mio-widget");
    }

    #[test]
    fn every_translation_is_unique_and_not_an_html_name() {
        let html: Vec<&str> = TAGS.iter().map(|(_, h, _)| *h).collect();
        for (i, (nap, _, _)) in TAGS.iter().enumerate() {
            assert!(!html.contains(nap), "{nap} is an HTML tag");
            assert!(
                TAGS[i + 1..].iter().all(|(other, _, _)| other != nap),
                "{nap} twice"
            );
        }
    }
}
