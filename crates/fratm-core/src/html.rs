//! Neapolitan names for HTML: tags, attributes with their enumerated values,
//! and events. The same tables are htmlemmurt's vocabulary, so a word means
//! the same thing in a `.fratm` component and in an `.emmurt` page.
//!
//! In JSX, `<cascett>`, `<chiacchier>` and `<citofono>` compile to `div`, `p`
//! and `button`, and `classe`, `esempio` and `quanno-tuocche` on them to
//! `className`, `placeholder` and `onClick`, so the DOM, devtools and CSS only
//! ever see standard names. HTML and React names keep working: a name that is
//! not in a table is used as written.

use serde::Serialize;

/// (Neapolitan tag, HTML tag, short description in Neapolitan)
pub const TAGS: &[(&str, &str, &str)] = &[
    // Document
    ("cristian", "html", "'a paggena sana"),
    ("capa", "head", "'a capa d'a paggena, cu titolo e inciuci"),
    ("panz", "body", "'a panza d'a paggena: chello ca se vede"),
    ("titolo", "title", "'o titolo ncopp'a scheda d'o browser"),
    ("inciucc", "meta", "n'informazione ncopp'a paggena"),
    (
        "attacca",
        "link",
        "attacca nu file, tipo nu foglio 'e stile",
    ),
    ("basa", "base", "l'indirizzo 'e base d'a paggena"),
    ("cess", "style", "CSS dint'a paggena"),
    ("fratm", "script", "codice ca se esegue"),
    (
        "senzafratm",
        "noscript",
        "chello ca se vede si 'o codice nun parte",
    ),
    // Sections
    ("cappiell", "header", "'o cappiello d'a paggena"),
    ("pere", "footer", "'o pere d'a paggena"),
    ("tomtom", "nav", "'e collegamenti pe girà"),
    ("core", "main", "'o core d'a paggena"),
    ("piezz", "section", "na sezione"),
    (
        "fattariello",
        "article",
        "n'articolo, nu fattariello ca se legge sulo",
    ),
    ("allato", "aside", "na cosa 'e lato"),
    ("indirizzo", "address", "addò se trova chi ha scritto"),
    ("luccolone", "h1", "'o titolo d'a paggena"),
    ("luccolo", "h2", "nu titolo cchiù piccerillo"),
    ("luccoliello", "h3", "nu titolo 'e terzo livello"),
    ("luccolillo", "h4", "nu titolo 'e quarto livello"),
    ("luccolino", "h5", "nu titolo 'e quinto livello"),
    ("luccolicchio", "h6", "nu titolo 'e sesto livello"),
    // Grouping
    ("cascett", "div", "na cascetta pe raggruppà cose"),
    ("chiacchier", "p", "nu paragrafo"),
    ("stacc", "hr", "na linea ca sparte"),
    ("accussì", "pre", "testo lassato accussì comm'è"),
    ("accussi", "pre", "testo lassato accussì comm'è"),
    ("citazione", "blockquote", "na citazione longa"),
    ("list", "ul", "na lista cu 'e pallini"),
    ("elenco", "ol", "na lista cu 'e nummeri"),
    ("voce", "li", "na voce 'e na lista"),
    ("vocabolario", "dl", "na lista 'e parole e significati"),
    ("parola", "dt", "na parola d'o vocabolario"),
    ("significato", "dd", "'o significato 'e na parola"),
    ("quadro", "figure", "nu quadro: na figura cu 'a didascalia"),
    ("didascalia", "figcaption", "'a didascalia 'e nu quadro"),
    // Text
    ("vattenn", "a", "nu collegamento a n'ata paggena"),
    ("scatulell", "span", "nu piezzo 'e testo dint'a na riga"),
    ("acap", "br", "va a capo"),
    ("fort", "strong", "testo forte"),
    ("corsivo", "em", "testo calcato"),
    ("guapp", "b", "testo grassetto"),
    ("stuorto", "i", "testo stuorto"),
    ("sottolineato", "u", "testo sottolineato"),
    ("cancellato", "s", "testo cancellato"),
    ("piccirill", "small", "testo piccerillo"),
    ("evidenziato", "mark", "testo evidenziato"),
    ("codice", "code", "nu piezzo 'e codice"),
    ("fonte", "cite", "'a fonte 'e na citazione"),
    ("virgulette", "q", "na citazione corta, dint'ê virgulette"),
    ("abbreviazione", "abbr", "n'abbreviazione"),
    ("tiempo", "time", "na data o n'ora"),
    ("sotto", "sub", "testo ca scenne sotto"),
    ("ncoppa", "sup", "testo ca saglie ncoppa"),
    ("tastiera", "kbd", "nu tasto d'a tastiera"),
    ("levato", "del", "testo levato"),
    ("aggiunto", "ins", "testo aggiunto"),
    // Media
    ("cartulin", "img", "na fotografia"),
    ("cornice", "picture", "na cornice cu cchiù fotografie"),
    (
        "funtana",
        "source",
        "'a funtana 'e nu filmato o 'e na fotografia",
    ),
    ("filmato", "video", "nu filmato"),
    ("suono", "audio", "nu suono"),
    ("fenestella", "iframe", "na fenestella cu n'ata paggena"),
    ("tela", "canvas", "na tela pe disegnà"),
    ("disegno", "svg", "nu disegno vettoriale"),
    // Tables
    ("tavulat", "table", "na tavulata"),
    ("cartello", "caption", "'o cartello 'e na tavulata"),
    ("capotavula", "thead", "'a capa 'e na tavulata"),
    ("cuorpotavula", "tbody", "'o cuorpo 'e na tavulata"),
    ("peretavula", "tfoot", "'o pere 'e na tavulata"),
    ("fila", "tr", "na fila 'e tavulata"),
    ("casella", "td", "na casella 'e tavulata"),
    ("capocasella", "th", "na casella 'e capa 'e tavulata"),
    // Forms
    ("modulo", "form", "nu modulo da mannà"),
    ("miett", "input", "na casella pe scrivere"),
    ("citofono", "button", "nu buttone ca se pò premere"),
    ("etichetta", "label", "l'etichetta 'e na casella"),
    ("scegli", "select", "nu menù a tendina"),
    ("opzione", "option", "na voce 'e nu menù a tendina"),
    ("gruppo", "optgroup", "nu gruppo 'e opzioni"),
    ("ragionamento", "textarea", "nu spazio pe scrivere assaje"),
    ("recinto", "fieldset", "nu recinto attuorno a cchiù caselle"),
    ("leggenda", "legend", "'o titolo 'e nu recinto"),
    ("suggerimenti", "datalist", "suggerimenti pe na casella"),
    ("risultato", "output", "'o risultato 'e nu calcolo"),
    ("avanzamento", "progress", "quanto manca a fernì"),
    ("misura", "meter", "na misura dint'a nu intervallo"),
    // Interactive and components
    ("spoiler", "details", "na cosa ca s'arape e se chiude"),
    ("anteprim", "summary", "chello ca se vede 'e nu spoiler"),
    ("dialogo", "dialog", "na fenesta 'e dialogo"),
    ("stampo", "template", "nu stampo ca nun se vede"),
    ("fessura", "slot", "na fessura dint'a nu componente"),
];

/// The HTML tag for a JSX tag name: the translation of a Neapolitan name,
/// or the name itself.
pub fn tag(name: &str) -> &str {
    TAGS.iter()
        .find(|(nap, _, _)| *nap == name)
        .map_or(name, |(_, html, _)| html)
}

/// An attribute of HTML tags. `only` lists the HTML tags it applies to
/// (empty: all of them); `values` translates its enumerated values,
/// (Neapolitan, HTML), which htmlemmurt writes into its pages.
#[derive(Debug, Serialize)]
pub struct Attribute {
    pub nap: &'static str,
    pub html: &'static str,
    /// The React prop, for JSX.
    pub react: &'static str,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub only: &'static [&'static str],
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub values: &'static [(&'static str, &'static str)],
}

const fn attr(nap: &'static str, html: &'static str, react: &'static str) -> Attribute {
    Attribute {
        nap,
        html,
        react,
        only: &[],
        values: &[],
    }
}

const BOOLEANS: &[(&str, &str)] = &[("overo", "true"), ("sfòls", "false"), ("sfols", "false")];

const INPUT_TYPES: &[(&str, &str)] = &[
    ("testo", "text"),
    ("fattimiei", "password"),
    ("segreto", "password"),
    ("posta", "email"),
    ("numero", "number"),
    ("spunta", "checkbox"),
    ("pallino", "radio"),
    ("manna", "submit"),
    ("azzera", "reset"),
    ("buttone", "button"),
    ("carta", "file"),
    ("data", "date"),
    ("ora", "time"),
    ("dataeora", "datetime-local"),
    ("mese", "month"),
    ("settimana", "week"),
    ("colore", "color"),
    ("cursore", "range"),
    ("cerca", "search"),
    ("telefono", "tel"),
    ("indirizzo", "url"),
    ("annascuso", "hidden"),
    ("cartulin", "image"),
];

const BUTTON_TYPES: &[(&str, &str)] = &[
    ("manna", "submit"),
    ("azzera", "reset"),
    ("buttone", "button"),
];

/// Attributes, element-specific entries before the general ones.
pub const ATTRIBUTES: &[Attribute] = &[
    // Element-specific
    Attribute {
        only: &["form"],
        ..attr("addo", "action", "action")
    },
    Attribute {
        only: &["input"],
        values: INPUT_TYPES,
        ..attr("tipo", "type", "type")
    },
    Attribute {
        only: &["button"],
        values: BUTTON_TYPES,
        ..attr("tipo", "type", "type")
    },
    Attribute {
        only: &["script"],
        values: &[("modulo", "module")],
        ..attr("tipo", "type", "type")
    },
    // Global
    attr("classe", "class", "className"),
    attr("chi", "id", "id"),
    attr("cess", "style", "style"),
    attr("titolo", "title", "title"),
    attr("lingua", "lang", "lang"),
    attr("annascuso", "hidden", "hidden"),
    attr("turno", "tabindex", "tabIndex"),
    attr("ruolo", "role", "role"),
    Attribute {
        values: BOOLEANS,
        ..attr("scrivibile", "contenteditable", "contentEditable")
    },
    Attribute {
        values: BOOLEANS,
        ..attr("trascinabile", "draggable", "draggable")
    },
    Attribute {
        values: BOOLEANS,
        ..attr("correggi", "spellcheck", "spellCheck")
    },
    // Links and resources
    attr("addo", "href", "href"),
    attr("da", "src", "src"),
    attr("spiegazione", "alt", "alt"),
    attr("larghezza", "width", "width"),
    attr("altezza", "height", "height"),
    Attribute {
        values: &[
            ("nuova", "_blank"),
            ("ccà", "_self"),
            ("cca", "_self"),
            ("pate", "_parent"),
            ("ncoppa", "_top"),
        ],
        ..attr("arape", "target", "target")
    },
    Attribute {
        values: &[
            ("stile", "stylesheet"),
            ("icona", "icon"),
            ("manifesto", "manifest"),
            ("precarica", "preload"),
        ],
        ..attr("rapporto", "rel", "rel")
    },
    attr("scarica", "download", "download"),
    attr("tipo", "type", "type"),
    attr("codifica", "charset", "charSet"),
    attr("nomme", "name", "name"),
    attr("cuntenuto", "content", "content"),
    attr("rrobba", "property", "property"),
    Attribute {
        values: &[("chianochiano", "lazy"), ("subbeto", "eager")],
        ..attr("carica", "loading", "loading")
    },
    attr("aropp", "defer", "defer"),
    attr("ampressa", "async", "async"),
    Attribute {
        values: &[("ncognito", "anonymous"), ("cunosciuto", "use-credentials")],
        ..attr("furastiero", "crossorigin", "crossOrigin")
    },
    attr("verace", "integrity", "integrity"),
    // Forms
    attr("valore", "value", "value"),
    attr("esempio", "placeholder", "placeholder"),
    attr("stutato", "disabled", "disabled"),
    attr("spuntato", "checked", "checked"),
    attr("scigliuto", "selected", "selected"),
    attr("obbligatorio", "required", "required"),
    attr("nunsetocca", "readonly", "readOnly"),
    attr("assaje", "multiple", "multiple"),
    attr("subbeto", "autofocus", "autoFocus"),
    attr("minimo", "min", "min"),
    attr("massimo", "max", "max"),
    attr("passo", "step", "step"),
    attr("lunghezzaminima", "minlength", "minLength"),
    attr("lunghezzamassima", "maxlength", "maxLength"),
    attr("modello", "pattern", "pattern"),
    attr("pe", "for", "htmlFor"),
    attr("suggerimenti", "list", "list"),
    attr("accetta", "accept", "accept"),
    attr("etichetta", "label", "label"),
    attr("righe", "rows", "rows"),
    attr("colonne", "cols", "cols"),
    Attribute {
        values: &[("piglia", "get"), ("manna", "post"), ("dialogo", "dialog")],
        ..attr("metodo", "method", "method")
    },
    Attribute {
        values: &[("sì", "on"), ("si", "on"), ("no", "off")],
        ..attr("ricorda", "autocomplete", "autoComplete")
    },
    // Tables, media, interactive
    attr("allarga", "colspan", "colSpan"),
    attr("allonga", "rowspan", "rowSpan"),
    attr("partesubbeto", "autoplay", "autoPlay"),
    attr("comandi", "controls", "controls"),
    attr("ripete", "loop", "loop"),
    attr("muto", "muted", "muted"),
    attr("copertina", "poster", "poster"),
    attr("quanno", "datetime", "dateTime"),
    attr("araputo", "open", "open"),
];

/// Props that exist only in React, for JSX: (Neapolitan, prop).
pub const JSX_ONLY: &[(&str, &str)] = &[("chiave", "key"), ("valore-iniziale", "defaultValue")];

/// Events: (Neapolitan, HTML attribute, React prop).
pub const EVENTS: &[(&str, &str, &str)] = &[
    ("quanno-tuocche", "onclick", "onClick"),
    ("quanno-tuocche-doje-vote", "ondblclick", "onDoubleClick"),
    ("quanno-scrive", "oninput", "onInput"),
    ("quanno-cagne", "onchange", "onChange"),
    ("quanno-manne", "onsubmit", "onSubmit"),
    ("quanno-carica", "onload", "onLoad"),
    ("quanno-premi", "onkeydown", "onKeyDown"),
    ("quanno-aize", "onkeyup", "onKeyUp"),
    ("quanno-trase", "onmouseenter", "onMouseEnter"),
    ("quanno-esce", "onmouseleave", "onMouseLeave"),
    ("quanno-piglie", "onfocus", "onFocus"),
    ("quanno-perde", "onblur", "onBlur"),
    ("quanno-scorre", "onscroll", "onScroll"),
];

/// Whether an attribute applies to the HTML tag `tag`.
pub fn applies(attribute: &Attribute, tag: &str) -> bool {
    attribute.only.is_empty() || attribute.only.contains(&tag)
}

/// The React prop for an attribute of the HTML tag `tag`: the translation of
/// a Neapolitan name (or of `class` and `for`), or the name itself.
pub fn attribute<'a>(name: &'a str, tag: &str) -> &'a str {
    match name {
        "class" => return "className",
        "for" => return "htmlFor",
        _ => {}
    }
    EVENTS
        .iter()
        .find(|(nap, _, _)| *nap == name)
        .map(|(_, _, react)| *react)
        .or_else(|| {
            ATTRIBUTES
                .iter()
                .find(|a| a.nap == name && applies(a, tag))
                .map(|a| a.react)
        })
        .or_else(|| {
            JSX_ONLY
                .iter()
                .find(|(nap, _)| *nap == name)
                .map(|(_, react)| *react)
        })
        .unwrap_or(name)
}

/// The HTML value for the Neapolitan value `value` of the attribute `name`
/// (Neapolitan, HTML or React name) on the HTML tag `tag`, as htmlemmurt
/// translates it: `tipo="fattimiei"` on `<miett>` is `password`. `None` when
/// the value is not one of the attribute's Neapolitan values.
pub fn value(name: &str, tag: &str, value: &str) -> Option<&'static str> {
    ATTRIBUTES
        .iter()
        .find(|a| (a.nap == name || a.html == name || a.react == name) && applies(a, tag))?
        .values
        .iter()
        .find(|(nap, _)| *nap == value)
        .map(|(_, html)| *html)
}

/// The HTML attribute for the name `name` in a CSS attribute selector on the
/// HTML tag `tag` (empty when the selector names no tag): `tipo` is `type`,
/// `addo` is `href`, or `action` on `form`. Neapolitan names (those of v0.11
/// and v0.12 too), events and React props (`className`, `htmlFor`) are
/// translated; `None` for any other name, which stays as written.
pub fn html_attribute(name: &str, tag: &str) -> Option<&'static str> {
    let from_react = |react: &str| {
        EVENTS
            .iter()
            .find(|(_, _, r)| *r == react)
            .map(|(_, html, _)| *html)
            .or_else(|| ATTRIBUTES.iter().find(|a| a.react == react).map(|a| a.html))
    };
    EVENTS
        .iter()
        .find(|(nap, _, _)| *nap == name)
        .map(|(_, html, _)| *html)
        .or_else(|| {
            ATTRIBUTES
                .iter()
                .find(|a| a.nap == name && applies(a, tag))
                .map(|a| a.html)
        })
        .or_else(|| {
            DEPRECATED_ATTRIBUTES
                .iter()
                .find(|(old, _, _)| *old == name)
                .and_then(|(_, react, _)| from_react(react))
        })
        .or_else(|| from_react(name))
        .filter(|html| *html != name)
}

/// JSX tag names from v0.11 and v0.12, replaced by the shared vocabulary:
/// (old name, HTML tag, new name). They keep compiling to the same tag, with
/// a warning. They are not part of htmlemmurt's vocabulary.
pub const DEPRECATED_TAGS: &[(&str, &str, &str)] = &[
    ("scatola", "div", "cascett"),
    ("piezzo", "span", "scatulell"),
    ("scritta", "p", "chiacchier"),
    ("titolo2", "h2", "luccolo"),
    ("titolo3", "h3", "luccoliello"),
    ("titolo4", "h4", "luccolillo"),
    ("titolo5", "h5", "luccolino"),
    ("titolo6", "h6", "luccolicchio"),
    ("buttone", "button", "citofono"),
    ("quaderno", "textarea", "ragionamento"),
    ("lista", "ul", "list"),
    ("collegamento", "a", "vattenn"),
    ("immagine", "img", "cartulin"),
    ("pede", "footer", "pere"),
    ("sezione", "section", "piezz"),
    ("articolo", "article", "fattariello"),
    ("navigazione", "nav", "tomtom"),
    ("acapo", "br", "acap"),
    ("linea", "hr", "stacc"),
    ("forte", "strong", "fort"),
    ("tabella", "table", "tavulat"),
    ("riga", "tr", "fila"),
    ("cella", "td", "casella"),
    ("capocella", "th", "capocasella"),
];

/// JSX tag names that meant another element in v0.11 and v0.12:
/// (name, HTML tag before, word for that tag now). They compile to their
/// new tag, with a warning.
pub const CHANGED_TAGS: &[(&str, &str, &str)] = &[
    ("titolo", "h1", "luccolone"),
    ("casella", "input", "miett"),
    ("capa", "header", "cappiell"),
];

/// JSX attribute names from v0.11 and v0.12, replaced by the shared
/// vocabulary: (old name, React prop, new name). They keep compiling to the
/// same prop, with a warning.
pub const DEPRECATED_ATTRIBUTES: &[(&str, &str, &str)] = &[
    ("segnaposto", "placeholder", "esempio"),
    ("fonte", "src", "da"),
    ("indirizzo", "href", "addo"),
    ("testo-alternativo", "alt", "spiegazione"),
    ("disattivato", "disabled", "stutato"),
    ("stile", "style", "cess"),
    ("controllato", "checked", "spuntato"),
    ("scelto", "selected", "scigliuto"),
    ("sulo-lettura", "readOnly", "nunsetocca"),
    ("multiplo", "multiple", "assaje"),
    ("nascosto", "hidden", "annascuso"),
    ("auto-fuoco", "autoFocus", "subbeto"),
    ("azione", "action", "addo"),
    ("destinazione", "target", "arape"),
    ("quanno-se-clicca", "onClick", "quanno-tuocche"),
    (
        "quanno-se-doppio-clicca",
        "onDoubleClick",
        "quanno-tuocche-doje-vote",
    ),
    ("quanno-se-scrive", "onChange", "quanno-cagne"),
    ("quanno-se-cagna", "onChange", "quanno-cagne"),
    ("quanno-se-manna", "onSubmit", "quanno-manne"),
    ("quanno-se-preme", "onKeyDown", "quanno-premi"),
    ("quanno-se-lassa", "onKeyUp", "quanno-aize"),
    ("quanno-se-passa", "onMouseEnter", "quanno-trase"),
    ("quanno-se-leva", "onMouseLeave", "quanno-esce"),
    ("quanno-se-trase", "onFocus", "quanno-piglie"),
    ("quanno-se-jesce", "onBlur", "quanno-perde"),
    ("quanno-se-carica", "onLoad", "quanno-carica"),
    ("quanno-se-scorre", "onScroll", "quanno-scorre"),
];

/// The HTML tag for a JSX tag name, like [`tag`], plus the warning for a
/// name from v0.11 and v0.12 that was replaced or changed meaning.
pub fn jsx_tag(name: &str) -> (&str, Option<String>) {
    if let Some((_, html, new)) = DEPRECATED_TAGS.iter().find(|(old, _, _)| *old == name) {
        return (
            html,
            Some(format!("<{name}> è vecchio, mo se dice <{new}>")),
        );
    }
    let html = tag(name);
    let warning =
        CHANGED_TAGS
            .iter()
            .find(|(word, _, _)| *word == name)
            .map(|(word, before, now)| {
                format!("<{word}> mo è <{html}>: si vulive <{before}>, scrive <{now}>")
            });
    (html, warning)
}

/// The React prop for a JSX attribute of the HTML tag `tag`, like
/// [`attribute`], plus the warning for a name from v0.11 and v0.12 that was
/// replaced.
pub fn jsx_attribute<'a>(name: &'a str, tag: &str) -> (&'a str, Option<String>) {
    match DEPRECATED_ATTRIBUTES
        .iter()
        .find(|(old, _, _)| *old == name)
    {
        Some((_, react, new)) => (
            react,
            Some(format!("'{name}' è vecchio, mo se dice '{new}'")),
        ),
        None => (attribute(name, tag), None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerated_values_become_html() {
        assert_eq!(value("tipo", "input", "fattimiei"), Some("password"));
        assert_eq!(value("type", "input", "spunta"), Some("checkbox"));
        assert_eq!(value("tipo", "button", "manna"), Some("submit"));
        // HTML values, unknown words and attributes without values pass through
        assert_eq!(value("tipo", "input", "password"), None);
        assert_eq!(value("tipo", "input", "pizza"), None);
        assert_eq!(value("classe", "div", "testo"), None);
    }

    #[test]
    fn old_jsx_words_keep_compiling_with_a_warning() {
        let (html, warning) = jsx_tag("scatola");
        assert_eq!(html, "div");
        assert_eq!(
            warning.unwrap(),
            "<scatola> è vecchio, mo se dice <cascett>"
        );
        assert_eq!(jsx_tag("cella").0, "td");
        let (prop, warning) = jsx_attribute("quanno-se-clicca", "button");
        assert_eq!(prop, "onClick");
        assert_eq!(
            warning.unwrap(),
            "'quanno-se-clicca' è vecchio, mo se dice 'quanno-tuocche'"
        );
        assert_eq!(jsx_attribute("stile", "div").0, "style");
        assert_eq!(jsx_attribute("indirizzo", "a").0, "href");
        // Current words have no warning
        assert_eq!(jsx_tag("cascett"), ("div", None));
        assert_eq!(jsx_attribute("addo", "a"), ("href", None));
    }

    #[test]
    fn words_that_changed_meaning_warn() {
        let (html, warning) = jsx_tag("casella");
        assert_eq!(html, "td");
        assert_eq!(
            warning.unwrap(),
            "<casella> mo è <td>: si vulive <input>, scrive <miett>"
        );
        assert_eq!(jsx_tag("titolo").0, "title");
        assert_eq!(jsx_tag("capa").0, "head");
    }

    #[test]
    fn deprecated_words_are_not_current_words() {
        for (old, html, new) in DEPRECATED_TAGS {
            assert!(
                TAGS.iter().all(|(nap, _, _)| nap != old),
                "<{old}> is current"
            );
            assert_eq!(tag(new), *html, "<{new}> is not <{html}>");
        }
        for (word, _, now) in CHANGED_TAGS {
            assert_ne!(tag(word), *word, "<{word}> is not a tag");
            assert!(TAGS.iter().any(|(nap, _, _)| nap == now), "<{now}>");
        }
        for (old, react, new) in DEPRECATED_ATTRIBUTES {
            assert_eq!(attribute(old, "a"), *old, "'{old}' is current");
            let current = ["a", "form", "input", "img", "button", "div"]
                .iter()
                .any(|t| attribute(new, t) == *react);
            assert!(current, "'{new}' is not {react}");
        }
    }

    #[test]
    fn neapolitan_tags_become_html() {
        assert_eq!(tag("cascett"), "div");
        assert_eq!(tag("citofono"), "button");
        assert_eq!(tag("luccolo"), "h2");
        assert_eq!(tag("titolo"), "title");
        // HTML names and custom elements pass through
        assert_eq!(tag("div"), "div");
        assert_eq!(tag("mio-widget"), "mio-widget");
    }

    #[test]
    fn neapolitan_attributes_become_react_props() {
        assert_eq!(attribute("classe", "div"), "className");
        assert_eq!(attribute("class", "div"), "className");
        assert_eq!(attribute("pe", "label"), "htmlFor");
        assert_eq!(attribute("esempio", "input"), "placeholder");
        assert_eq!(attribute("chiave", "li"), "key");
        assert_eq!(attribute("valore-iniziale", "input"), "defaultValue");
        // The same word on different tags
        assert_eq!(attribute("addo", "form"), "action");
        assert_eq!(attribute("addo", "a"), "href");
        // Events
        assert_eq!(attribute("quanno-tuocche", "button"), "onClick");
        assert_eq!(attribute("quanno-scrive", "input"), "onInput");
        assert_eq!(attribute("quanno-cagne", "select"), "onChange");
        // React names, data and aria attributes pass through
        assert_eq!(attribute("onClick", "button"), "onClick");
        assert_eq!(attribute("data-id", "div"), "data-id");
        assert_eq!(attribute("aria-label", "div"), "aria-label");
    }

    #[test]
    fn element_specific_attributes_win() {
        let input = ATTRIBUTES
            .iter()
            .find(|a| a.nap == "tipo" && applies(a, "input"))
            .unwrap();
        assert_eq!(input.values, INPUT_TYPES);
        let ol = ATTRIBUTES
            .iter()
            .find(|a| a.nap == "tipo" && applies(a, "ol"))
            .unwrap();
        assert!(ol.values.is_empty());
    }

    #[test]
    fn neapolitan_names_are_unique_and_not_html_names() {
        let html: Vec<&str> = TAGS.iter().map(|(_, h, _)| *h).collect();
        for (i, (nap, _, _)) in TAGS.iter().enumerate() {
            assert!(!html.contains(nap), "<{nap}> is an HTML tag");
            assert!(!nap.contains('-'), "<{nap}> would read as a custom element");
            assert!(
                TAGS[i + 1..].iter().all(|(other, _, _)| other != nap),
                "<{nap}> twice"
            );
        }
        let props: Vec<&str> = ATTRIBUTES
            .iter()
            .map(|a| a.react)
            .chain(JSX_ONLY.iter().map(|(_, r)| *r))
            .chain(EVENTS.iter().map(|(_, _, r)| *r))
            .collect();
        for (i, a) in ATTRIBUTES.iter().enumerate() {
            assert!(!props.contains(&a.nap), "{} is a React prop", a.nap);
            assert!(
                ATTRIBUTES[i + 1..]
                    .iter()
                    .all(|b| b.nap != a.nap || b.only != a.only),
                "{} twice for the same tags",
                a.nap
            );
            for (nap, _) in a.values {
                assert!(
                    !a.values.iter().any(|(_, h)| h == nap),
                    "value {nap} of {} is also an HTML value",
                    a.nap
                );
            }
        }
        for (i, (nap, _, _)) in EVENTS.iter().enumerate() {
            assert!(
                EVENTS[i + 1..].iter().all(|(other, _, _)| other != nap),
                "{nap} twice"
            );
        }
    }
}
