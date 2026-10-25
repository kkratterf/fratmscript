//! 'A libreria `fratm`: helpers with Neapolitan names.
//!
//! `chiamm { a_caso, aspetta } da "fratm"` does not emit an import: the
//! compiler writes the definition of each imported helper in its place, so
//! the output runs anywhere (Node, the browser, the playground) without a
//! package, and helpers that are not imported cost nothing.

/// Module name that selects the library in `chiamm { … } da "fratm"`.
pub const MODULE: &str = "fratm";

/// (name, JavaScript arrow function, short description in Neapolitan)
pub const HELPERS: &[(&str, &str, &str)] = &[
    (
        "a_caso",
        "(min, max) => Math.floor(Math.random() * (max - min + 1)) + min",
        "nu nummero intero a caso tra min e max (tutt'e duje compresi)",
    ),
    (
        "uno_a_caso",
        "(lista) => lista[Math.floor(Math.random() * lista.length)]",
        "n'elemento a caso 'e na lista",
    ),
    (
        "mischia",
        "(lista) => { const copia = [...lista]; for (let i = copia.length - 1; i > 0; i--) { const j = Math.floor(Math.random() * (i + 1)); [copia[i], copia[j]] = [copia[j], copia[i]]; } return copia; }",
        "na copia d'a lista cu 'e elementi mischiati (comme 'e carte)",
    ),
    (
        "quanti",
        "(cosa) => cosa == null ? 0 : cosa.size ?? cosa.length ?? Object.keys(cosa).length",
        "quant'elementi tene na lista, na stringa, nu Set/Map o n'oggetto",
    ),
    (
        "arrotonda",
        "(nummero, cifre = 0) => Math.round(nummero * 10 ** cifre) / 10 ** cifre",
        "arrotonda nu nummero, cu tante cifre doppo 'a virgola quante ne vuò",
    ),
    (
        "ripeti",
        "(volte, fa) => { for (let i = 0; i < volte; i++) fa(i); }",
        "chiamma na funzione tante vote (cu 'o nummero d'o giro: 0, 1, 2…)",
    ),
    (
        "aspetta",
        "(ms) => new Promise((risolvi) => setTimeout(risolvi, ms))",
        "na Promise ca se risolve doppo ms millisecondi: aspett aspetta(1000)",
    ),
    // The compiler translates each gomorra`…` and passes the SQL here
    (
        "gomorra",
        "(sql, text, params) => ({ sql, params, text, values: [...params] })",
        "na query GomorraSQL: gomorra`ripigliammo nomme mmiez 'a pizze arò id = ${id}` → { sql, params, text, values }",
    ),
];

/// JavaScript source of a helper, if the library has it.
pub fn helper(name: &str) -> Option<&'static str> {
    HELPERS
        .iter()
        .find(|(n, _, _)| *n == name)
        .map(|(_, js, _)| *js)
}

/// Names of every helper, comma separated (for error messages).
pub fn names() -> String {
    HELPERS
        .iter()
        .map(|(n, _, _)| *n)
        .collect::<Vec<_>>()
        .join(", ")
}
