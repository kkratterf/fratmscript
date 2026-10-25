//! `fratm nuovo`: creates a new FratmScript project.
//!
//! The default template is a React app built with Vite, written in `.fratm`
//! files with JSX and compiled by the `fratmscript` plugin; `--senza-react`
//! writes the same Vite setup with plain DOM code, and `--semplice` just a
//! `main.fratm` to run with `fratm lancia`.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Files of the React template: (path, contents). `{{name}}` and
/// `{{fratmscript}}` (the npm dependency) are replaced when the project is
/// written.
const REACT_FILES: &[(&str, &str)] = &[
    (
        "package.json",
        r##"{
  "name": "{{name}}",
  "private": true,
  "version": "0.0.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "dependencies": {
    "react": "^19.0.0",
    "react-dom": "^19.0.0"
  },
  "devDependencies": {
    "@vitejs/plugin-react": "^4.3.0",
    "fratmscript": "{{fratmscript}}",
    "vite": "^5.4.0"
  }
}
"##,
    ),
    (
        "vite.config.js",
        r##"import fratmscript from 'fratmscript/vite';
import react from '@vitejs/plugin-react';

export default {
  // react() adds Fast Refresh: saving a .fratm component keeps its state
  plugins: [fratmscript(), react({ include: /\.(fratm|[jt]sx?)$/ })],
};
"##,
    ),
    (
        "index.html",
        r##"<!doctype html>
<html lang="it">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{{name}}</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.fratm"></script>
  </body>
</html>
"##,
    ),
    (
        "src/main.fratm",
        r##"// 'O punto 'e partenza: index.html carica stu file,
// e React disegna <App /> dint'o <div id="root">.
chiamm { StrictMode } da "react"
chiamm { createRoot } da "react-dom/client"
chiamm App da "./App.fratm"
chiamm "./stile.css"

createRoot(document.getElementById("root")).render(
    <StrictMode>
        <App />
    </StrictMode>
)
"##,
    ),
    (
        "src/App.fratm",
        r##"chiamm { useState } da "react"
chiamm Pizza da "./components/Pizza.fratm"

// 'O menù: na lista 'e oggetti, comme in JavaScript
chist è MENU = [
    { nomme: "Margherita", prezzo: 5, ingredienti: "pummarola, muzzarella, vasenicola" },
    { nomme: "Marinara", prezzo: 4, ingredienti: "pummarola, aglio, arecheta" },
    { nomme: "Diavola", prezzo: 7, ingredienti: "pummarola, muzzarella, salame piccante" },
]

// Nu componente è na funzione ca torna JSX: HTML dint'o codice
mann for predefinit facc App() {
    // useState: nu valore ca, quanno cagna, fa ridisegnà 'o componente
    tien [ordine, setOrdine] = useState([])
    chist è totale = ordine.reduce((somma, pizza) => somma + pizza.prezzo, 0)

    facc ordina(pizza) {
        setOrdine([...ordine, pizza])
    }

    piglie (
        <main className="pizzeria">
            <header>
                <h1>{{name}} 🍕</h1>
                <p>Scritto tutto in FratmScript: cagna <code>src/App.fratm</code> e salva.</p>
            </header>
            <ul className="menu">
                {MENU.map((pizza) => (
                    <Pizza key={pizza.nomme} pizza={pizza} onOrdina={() => ordina(pizza)} />
                ))}
            </ul>
            <footer>
                {ordine.length === 0
                    ? "Ancora nisciuna pizza. Che te magne?"
                    : `Pizze ordinate: ${ordine.length} · Totale: ${totale} €`}
            </footer>
        </main>
    )
}
"##,
    ),
    (
        "src/components/Pizza.fratm",
        r##"// CSS modules: 'e classi 'e Pizza.module.css valeno sulo pe stu componente
chiamm stile da "./Pizza.module.css"

// 'E props arrivano comme n'oggetto: { pizza, onOrdina }
mann for predefinit facc Pizza({ pizza, onOrdina }) {
    piglie (
        <li className={stile.carta}>
            <div>
                <h2 className={stile.nomme}>{pizza.nomme}</h2>
                <p className={stile.ingredienti}>{pizza.ingredienti}</p>
            </div>
            <button className={stile.bottone} onClick={onOrdina}>
                {pizza.prezzo} € · Ordina
            </button>
        </li>
    )
}
"##,
    ),
    (
        "src/components/Pizza.module.css",
        r##".carta {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 16px 20px;
  background: #fff;
  border: 1px solid #ecdcd2;
  border-radius: 14px;
}

.nomme {
  margin: 0;
  font-size: 1.1rem;
}

.ingredienti {
  margin: 4px 0 0;
  color: #7a6a62;
  font-size: 0.9rem;
}

.bottone {
  flex-shrink: 0;
  padding: 10px 14px;
  border: 0;
  border-radius: 10px;
  background: #c8321e;
  color: #fff;
  font: inherit;
  font-weight: 600;
  cursor: pointer;
}

.bottone:hover {
  background: #a82816;
}
"##,
    ),
    (
        "src/stile.css",
        r##"/* Stile globale: CSS normale, importato da main.fratm */
:root {
  font-family: system-ui, -apple-system, "Segoe UI", sans-serif;
  color: #2b1f1a;
  background: #fbf5ef;
}

body {
  margin: 0;
}

.pizzeria {
  max-width: 640px;
  margin: 0 auto;
  padding: 40px 16px;
}

.pizzeria h1 {
  margin: 0 0 8px;
  font-size: 2.2rem;
}

.menu {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin: 32px 0;
  padding: 0;
  list-style: none;
}

footer {
  font-weight: 600;
}
"##,
    ),
    (".gitignore", "node_modules\ndist\n"),
    (
        "README.md",
        r##"# {{name}}

A React app written in [FratmScript](https://github.com/kkratterf/fratmscript), made with `fratm nuovo`.

```bash
npm install
npm run dev     # development server
npm run build   # production build in dist/
```

- `src/main.fratm` starts the app, `src/App.fratm` is the main component and
  `src/components/Pizza.fratm` a child component with props.
- Components return JSX: HTML tags as usual, FratmScript inside `{ }`. Hooks keep
  their React names (`useState`, `useEffect`).
- Styles are plain CSS (`src/stile.css`, imported with `chiamm "./stile.css"`) and
  CSS modules (`Pizza.module.css`, used as `className={stile.carta}`). Tailwind
  works too: follow its Vite guide and use `className="…"`.
- `npm run dev` has Fast Refresh: save a component and the page updates without
  losing its state (`@vitejs/plugin-react` in `vite.config.js`).
- To use Preact instead of React, write `// @jsxImportSource preact` at the top
  of each file, or pass `fratmscript({ jsxImportSource: 'preact' })` in
  `vite.config.js` (and drop `react()`).
"##,
    ),
];

/// Files of the Vite template without React: (path, contents), with the same
/// placeholders as `REACT_FILES`.
const VITE_FILES: &[(&str, &str)] = &[
    (
        "package.json",
        r##"{
  "name": "{{name}}",
  "private": true,
  "version": "0.0.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "devDependencies": {
    "fratmscript": "{{fratmscript}}",
    "vite": "^5.4.0"
  }
}
"##,
    ),
    (
        "vite.config.js",
        r##"import fratmscript from 'fratmscript/vite';

export default {
  plugins: [fratmscript()],
};
"##,
    ),
    (
        "index.html",
        r##"<!doctype html>
<html lang="it">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{{name}}</title>
  </head>
  <body>
    <h1>{{name}} 🤌</h1>
    <p id="conteggio"></p>
    <button id="sforna">Sforna na pizza</button>
    <script type="module" src="/src/main.fratm"></script>
  </body>
</html>
"##,
    ),
    (
        "src/main.fratm",
        r##"// 'O punto 'e partenza: index.html carica stu file.
chiamm { descrivi } da "./pizze.fratm"

chist è conteggio = document.querySelector("#conteggio")
chist è sforna = document.querySelector("#sforna")
tien pizze = 0

facc aggiorna() {
    conteggio.textContent = descrivi(pizze)
}

sforna.addEventListener("click", () => {
    pizze++
    aggiorna()
})

aggiorna()
"##,
    ),
    (
        "src/pizze.fratm",
        r##"// Nu modulo: 'e funzioni cu "mann for" se ponno chiammà da n'ato file.
mann for facc descrivi(pizze) {
    si (pizze == 0) {
        piglie "Ancora nisciuna pizza. Accendi 'o furno!"
    }
    piglie `Pizze sfurnate: ${pizze}`
}
"##,
    ),
    (".gitignore", "node_modules\ndist\n"),
    (
        "README.md",
        r##"# {{name}}

A [FratmScript](https://github.com/kkratterf/fratmscript) project made with `fratm nuovo`.

```bash
npm install
npm run dev     # development server
npm run build   # production build in dist/
```

The code is in `src/*.fratm`; the `fratmscript` Vite plugin compiles it to JavaScript.
"##,
    ),
];

const SIMPLE_FILES: &[(&str, &str)] = &[(
    "main.fratm",
    r##"// Lancialo cu: fratm lancia main.fratm
chist è nomme = "{{name}}"

facc saluta(chi) {
    piglie `Uè, ${chi}!`
}

stamm a dì(saluta(nomme))
"##,
)];

/// Which template `fratm nuovo` writes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Template {
    React,
    Vite,
    Simple,
}

/// The `fratmscript` npm dependency for `version`. The package is not on the
/// npm registry yet, so projects install it from the GitHub release, which
/// carries the `npm pack` tarball (#167).
pub fn fratmscript_dependency(version: &str) -> String {
    format!(
        "https://github.com/kkratterf/fratmscript/releases/download/v{version}/fratmscript-{version}.tgz"
    )
}

/// Writes a new project in `dir` and returns the files it created.
/// Fails if `dir` exists and is not empty.
pub fn create(dir: &Path, template: Template, version: &str) -> io::Result<Vec<PathBuf>> {
    if dir.exists() && fs::read_dir(dir)?.next().is_some() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("'a cartella {} esiste già e nun è vacante", dir.display()),
        ));
    }
    let name = package_name(dir);
    let files = match template {
        Template::React => REACT_FILES,
        Template::Vite => VITE_FILES,
        Template::Simple => SIMPLE_FILES,
    };
    let dependency = fratmscript_dependency(version);
    let mut written = Vec::new();
    for (path, contents) in files {
        let target = dir.join(path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        let contents = contents
            .replace("{{name}}", &name)
            .replace("{{fratmscript}}", &dependency);
        fs::write(&target, contents)?;
        written.push(target);
    }
    Ok(written)
}

/// A valid npm package name from the folder name: lower case, with anything
/// other than letters, digits, `-`, `_` and `.` turned into `-`.
fn package_name(dir: &Path) -> String {
    let base = dir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let mut name: String = base
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.') {
                c
            } else {
                '-'
            }
        })
        .collect();
    name = name
        .trim_matches(|c| matches!(c, '-' | '.' | '_'))
        .to_string();
    if name.is_empty() {
        "progetto-fratm".to_string()
    } else {
        name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fratm_core::{compile, CompileOptions};

    fn temp_dir(name: &str) -> PathBuf {
        // The project folder keeps `name`, which becomes the package name
        let parent = std::env::temp_dir().join(format!("fratm-nuovo-{}", std::process::id()));
        let dir = parent.join(name);
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn package_names_are_valid_for_npm() {
        assert_eq!(
            package_name(Path::new("/tmp/Pizzeria Da Gennaro")),
            "pizzeria-da-gennaro"
        );
        assert_eq!(package_name(Path::new("caffè")), "caff");
        assert_eq!(package_name(Path::new("ok_name.js")), "ok_name.js");
        assert_eq!(package_name(Path::new("---")), "progetto-fratm");
    }

    #[test]
    fn vite_template_compiles_and_fills_placeholders() {
        let dir = temp_dir("Vite App");
        let files = create(&dir, Template::Vite, "1.2.3").unwrap();
        assert_eq!(files.len(), VITE_FILES.len());

        let package = fs::read_to_string(dir.join("package.json")).unwrap();
        let json: serde_json::Value = serde_json::from_str(&package).unwrap();
        assert_eq!(json["name"], "vite-app");
        assert_eq!(
            json["devDependencies"]["fratmscript"],
            "https://github.com/kkratterf/fratmscript/releases/download/v1.2.3/fratmscript-1.2.3.tgz"
        );
        assert!(!package.contains("{{"));

        let main = compile(
            &fs::read_to_string(dir.join("src/main.fratm")).unwrap(),
            CompileOptions::default(),
        )
        .unwrap();
        assert!(
            main.code
                .starts_with("import { descrivi } from \"./pizze.fratm\";"),
            "{}",
            main.code
        );
        assert!(main.code.contains("pizze++;"), "{}", main.code);
        let pizze = compile(
            &fs::read_to_string(dir.join("src/pizze.fratm")).unwrap(),
            CompileOptions::default(),
        )
        .unwrap();
        assert!(
            pizze.code.contains("export function descrivi(pizze)"),
            "{}",
            pizze.code
        );
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn react_template_compiles_every_file_with_jsx() {
        let dir = temp_dir("Pizzeria React");
        let files = create(&dir, Template::React, "1.2.3").unwrap();
        assert_eq!(files.len(), REACT_FILES.len());
        for (path, _) in REACT_FILES {
            let contents = fs::read_to_string(dir.join(path)).unwrap();
            assert!(!contents.contains("{{"), "{path} has a placeholder left");
        }

        let package = fs::read_to_string(dir.join("package.json")).unwrap();
        let json: serde_json::Value = serde_json::from_str(&package).unwrap();
        assert_eq!(json["name"], "pizzeria-react");
        assert!(json["dependencies"]["react"].is_string());
        assert!(json["devDependencies"]["@vitejs/plugin-react"].is_string());
        let config = fs::read_to_string(dir.join("vite.config.js")).unwrap();
        assert!(
            config.contains("react({ include: /\\.(fratm|[jt]sx?)$/ })"),
            "{config}"
        );
        assert_eq!(
            json["devDependencies"]["fratmscript"],
            fratmscript_dependency("1.2.3")
        );

        let compiled = |path: &str| {
            compile(
                &fs::read_to_string(dir.join(path)).unwrap(),
                CompileOptions::default(),
            )
            .unwrap_or_else(|e| panic!("{path}: {e}"))
            .code
        };
        let main = compiled("src/main.fratm");
        assert!(main.starts_with("import { jsx as _jsx"), "{main}");
        assert!(main.contains("import App from \"./App.fratm\";"), "{main}");
        assert!(main.contains("import \"./stile.css\";"), "{main}");
        assert!(
            main.contains("_jsx(StrictMode, { children: _jsx(App, {}) })"),
            "{main}"
        );
        let app = compiled("src/App.fratm");
        assert!(app.contains("export default function App()"), "{app}");
        assert!(app.contains("\"pizzeria-react 🍕\""), "{app}");
        assert!(
            app.contains(
                "_jsx(Pizza, { pizza: pizza, onOrdina: () => ordina(pizza) }, pizza.nomme)"
            ),
            "{app}"
        );
        let pizza = compiled("src/components/Pizza.fratm");
        assert!(
            pizza.contains("import stile from \"./Pizza.module.css\";"),
            "{pizza}"
        );
        assert!(pizza.contains("className: stile.carta"), "{pizza}");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn simple_template_is_one_file() {
        let dir = temp_dir("semplice");
        create(&dir, Template::Simple, "1.2.3").unwrap();
        let entries: Vec<_> = fs::read_dir(&dir).unwrap().collect();
        assert_eq!(entries.len(), 1);
        let main = compile(
            &fs::read_to_string(dir.join("main.fratm")).unwrap(),
            CompileOptions::default(),
        )
        .unwrap();
        assert!(
            main.code.contains("const nomme = \"semplice\";"),
            "{}",
            main.code
        );
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn refuses_a_folder_that_is_not_empty() {
        let dir = temp_dir("piena");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("main.fratm"), "stamm a dì(1)\n").unwrap();
        let error = create(&dir, Template::Simple, "1.2.3").unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);
        assert_eq!(
            fs::read_to_string(dir.join("main.fratm")).unwrap(),
            "stamm a dì(1)\n"
        );
        fs::remove_dir_all(&dir).unwrap();

        // An empty folder is fine
        fs::create_dir_all(&dir).unwrap();
        create(&dir, Template::Simple, "1.2.3").unwrap();
        fs::remove_dir_all(&dir).unwrap();
    }
}
