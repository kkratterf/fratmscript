//! Neapolitan names for HTML tags and their attributes.
//!
//! `<scatola>`, `<scritta>` and `<buttone>` in JSX compile to `div`, `p` and
//! `button`, and `classe`, `segnaposto` and `quanno-se-clicca` on them to
//! `className`, `placeholder` and `onClick`, so the DOM, devtools and CSS only
//! ever see standard names. HTML and React names keep working: a name that is
//! not in a table is used as written.

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

/// (Neapolitan attribute, React prop) for HTML tags. Components get their
/// props as written, since their names are the component's own.
pub const ATTRIBUTES: &[(&str, &str)] = &[
    ("chiave", "key"),
    ("classe", "className"),
    ("class", "className"),
    ("pe", "htmlFor"),
    ("for", "htmlFor"),
    ("tipo", "type"),
    ("valore", "value"),
    ("valore-iniziale", "defaultValue"),
    ("segnaposto", "placeholder"),
    ("fonte", "src"),
    ("indirizzo", "href"),
    ("testo-alternativo", "alt"),
    ("disattivato", "disabled"),
    ("nomme", "name"),
    ("titolo", "title"),
    ("stile", "style"),
    ("controllato", "checked"),
    ("scelto", "selected"),
    ("obbligatorio", "required"),
    ("sulo-lettura", "readOnly"),
    ("multiplo", "multiple"),
    ("nascosto", "hidden"),
    ("auto-fuoco", "autoFocus"),
    ("larghezza", "width"),
    ("altezza", "height"),
    ("righe", "rows"),
    ("colonne", "cols"),
    ("minimo", "min"),
    ("massimo", "max"),
    ("passo", "step"),
    ("azione", "action"),
    ("metodo", "method"),
    ("destinazione", "target"),
    ("lingua", "lang"),
    ("ruolo", "role"),
    ("quanno-se-clicca", "onClick"),
    ("quanno-se-doppio-clicca", "onDoubleClick"),
    ("quanno-se-scrive", "onChange"),
    ("quanno-se-cagna", "onChange"),
    ("quanno-se-manna", "onSubmit"),
    ("quanno-se-preme", "onKeyDown"),
    ("quanno-se-lassa", "onKeyUp"),
    ("quanno-se-passa", "onMouseEnter"),
    ("quanno-se-leva", "onMouseLeave"),
    ("quanno-se-trase", "onFocus"),
    ("quanno-se-jesce", "onBlur"),
    ("quanno-se-carica", "onLoad"),
    ("quanno-se-scorre", "onScroll"),
];

/// The React prop for an attribute of an HTML tag: the translation of a
/// Neapolitan name (or of `class` and `for`), or the name itself.
pub fn attribute(name: &str) -> &str {
    ATTRIBUTES
        .iter()
        .find(|(nap, _)| *nap == name)
        .map_or(name, |(_, react)| react)
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
    fn neapolitan_attributes_become_react_props() {
        assert_eq!(attribute("classe"), "className");
        assert_eq!(attribute("class"), "className");
        assert_eq!(attribute("pe"), "htmlFor");
        assert_eq!(attribute("segnaposto"), "placeholder");
        assert_eq!(attribute("quanno-se-clicca"), "onClick");
        assert_eq!(attribute("quanno-se-scrive"), "onChange");
        // React names, data and aria attributes pass through
        assert_eq!(attribute("onClick"), "onClick");
        assert_eq!(attribute("data-id"), "data-id");
        assert_eq!(attribute("aria-label"), "aria-label");
    }

    #[test]
    fn every_attribute_is_listed_once_and_not_a_react_prop() {
        let react: Vec<&str> = ATTRIBUTES.iter().map(|(_, r)| *r).collect();
        for (i, (nap, _)) in ATTRIBUTES.iter().enumerate() {
            assert!(!react.contains(nap), "{nap} is a React prop");
            assert!(
                ATTRIBUTES[i + 1..].iter().all(|(other, _)| other != nap),
                "{nap} twice"
            );
        }
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
