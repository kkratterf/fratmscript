//! 'A libreria `fratm`: helpers with Neapolitan names.
//!
//! `chiamm { a_caso, aspetta } da "fratm"` does not emit an import: the
//! compiler writes the definition of each imported helper in its place, so
//! the output runs anywhere (Node, the browser, the playground) without a
//! package, and helpers that are not imported cost nothing.

/// Module name that selects the library in `chiamm { … } da "fratm"`.
pub const MODULE: &str = "fratm";

/// (name, JavaScript arrow function, short description in Neapolitan). The
/// code is formatted as it appears in the output, from column 0.
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
        r#"(lista) => {
  const copia = [...lista];
  for (let i = copia.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [copia[i], copia[j]] = [copia[j], copia[i]];
  }
  return copia;
}"#,
        "na copia d'a lista cu 'e elementi mischiati (comme 'e carte)",
    ),
    (
        "quanti",
        r#"(cosa) =>
  cosa == null ? 0 : (cosa.size ?? cosa.length ?? Object.keys(cosa).length)"#,
        "quant'elementi tene na lista, na stringa, nu Set/Map o n'oggetto",
    ),
    (
        "arrotonda",
        r#"(nummero, cifre = 0) =>
  Math.round(nummero * 10 ** cifre) / 10 ** cifre"#,
        "arrotonda nu nummero, cu tante cifre doppo 'a virgola quante ne vuò",
    ),
    (
        "ripeti",
        r#"(volte, fa) => {
  for (let i = 0; i < volte; i++) {
    fa(i);
  }
}"#,
        "chiamma na funzione tante vote (cu 'o nummero d'o giro: 0, 1, 2…)",
    ),
    (
        "aspetta",
        "(ms) => new Promise((risolvi) => setTimeout(risolvi, ms))",
        "na Promise ca se risolve doppo ms millisecondi: aspett aspetta(1000)",
    ),
    // node:sqlite is loaded when the database opens, so importing apri_db
    // never breaks a module in the browser or on an older Node
    (
        "apri_db",
        r#"(file = ":memory:") => {
  const sqlite = globalThis.process?.getBuiltinModule?.("node:sqlite");
  if (!sqlite) {
    throw new Error(
      "apri_db vo' Node 22.13 o cchiù nuovo (node:sqlite): int'o browser nun se pò aprì nu database",
    );
  }
  const db = new sqlite.DatabaseSync(file);
  const prepara = (q, valori) =>
    typeof q === "string"
      ? [db.prepare(q), valori]
      : [db.prepare(q.sql), q.params];
  return {
    ripiglia: (q, ...valori) => {
      const [s, p] = prepara(q, valori);
      return s.all(...p).map((riga) => ({ ...riga }));
    },
    ripiglia_uno: (q, ...valori) => {
      const [s, p] = prepara(q, valori);
      const riga = s.get(...p);
      return riga === undefined ? null : { ...riga };
    },
    esegui: (q, ...valori) => {
      const [s, p] = prepara(q, valori);
      const r = s.run(...p);
      return {
        cambiate: Number(r.changes),
        ultimo_id: Number(r.lastInsertRowid),
      };
    },
    chiude: () => db.close(),
  };
}"#,
        "apre nu database SQLite (node:sqlite, Node 22.13+): db.ripiglia(q), db.ripiglia_uno(q), db.esegui(q), db.chiude()",
    ),
    // The compiler translates each stile`…` into CSS; in a browser the
    // rules are added to the page once
    (
        "stile",
        r#"(() => {
  const fatti = new Set();
  return (css) => {
    if (
      typeof document !== "undefined" &&
      css.includes("{") &&
      !fatti.has(css)
    ) {
      fatti.add(css);
      const s = document.createElement("style");
      s.textContent = css;
      document.head.append(s);
    }
    return css;
  };
})()"#,
        "CSS in napoletano: stile`citofono:passanno { culore: russo }` → mette 'e regole int'a pagina e torna 'o CSS",
    ),
    // The compiler translates each gomorra`…` and passes the SQL here
    (
        "gomorra",
        r#"(sql, text, params) => ({
  sql,
  params,
  text,
  values: [...params],
})"#,
        "na query GomorraSQL: gomorra`ripigliammo nomme mmiez 'a pizze arò id = ${id}` → { sql, params, text, values }",
    ),
];

/// Local names of `name` in `chiamm { name } da "fratm"` (an alias with
/// `comme` included).
pub fn local_names(program: &crate::parser::Program, name: &str) -> Vec<String> {
    program
        .statements
        .iter()
        .filter_map(|statement| match statement {
            crate::parser::Statement::Import {
                specifiers, source, ..
            } if source == MODULE => Some(specifiers),
            _ => None,
        })
        .flatten()
        .filter(|s| s.imported == name)
        .map(|s| s.local.clone())
        .collect()
}

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
