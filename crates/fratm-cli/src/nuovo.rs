//! `fratm nuovo`: creates a new FratmScript project.
//!
//! The templates live in `fratm_core::template`, shared with the npm
//! package that `npm create fratm` uses; this module writes them to disk.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub use fratm_core::template::*;

/// Writes a new project in `dir` and returns the files it created.
/// Fails if `dir` exists and is not empty. `tailwind` adds Tailwind CSS to
/// the React templates.
pub fn create(
    dir: &Path,
    template: Template,
    version: &str,
    tailwind: bool,
) -> io::Result<Vec<PathBuf>> {
    if dir.exists() && fs::read_dir(dir)?.next().is_some() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("'a cartella {} esiste già e nun è vacante", dir.display()),
        ));
    }
    let folder = dir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let mut written = Vec::new();
    for (path, contents) in files(template, &folder, version, tailwind) {
        let target = dir.join(path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&target, contents)?;
        written.push(target);
    }
    Ok(written)
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
        assert_eq!(package_name("Pizzeria Da Gennaro"), "pizzeria-da-gennaro");
        assert_eq!(package_name("caffè"), "caff");
        assert_eq!(package_name("ok_name.js"), "ok_name.js");
        assert_eq!(package_name("---"), "progetto-fratm");
    }

    #[test]
    fn tailwind_is_added_to_the_react_templates() {
        for template in [Template::React, Template::Full] {
            let dir = temp_dir(&format!("Tailwind {template:?}"));
            create(&dir, template, "1.2.3", true).unwrap();
            let package = fs::read_to_string(dir.join("package.json")).unwrap();
            let json: serde_json::Value = serde_json::from_str(&package).unwrap();
            assert_eq!(json["devDependencies"]["tailwindcss"], TAILWIND_VERSION);
            assert_eq!(
                json["devDependencies"]["@tailwindcss/vite"],
                TAILWIND_VERSION
            );
            let config = fs::read_to_string(dir.join("vite.config.js")).unwrap();
            assert!(
                config.contains("import tailwindcss from '@tailwindcss/vite';"),
                "{config}"
            );
            assert!(config.contains("tailwindcss(),"), "{config}");
            let css = fs::read_to_string(dir.join("src/stile.css")).unwrap();
            assert!(css.starts_with("@import \"tailwindcss\";"), "{css}");
            let app = fs::read_to_string(dir.join("src/App.fratm")).unwrap();
            assert!(app.contains("className=\"mt-8 rounded-xl"), "{app}");
            compile(&app, CompileOptions::default()).unwrap();
        }
        // Without the flag nothing changes
        let dir = temp_dir("Pizzeria liscia");
        create(&dir, Template::React, "1.2.3", false).unwrap();
        let package = fs::read_to_string(dir.join("package.json")).unwrap();
        assert!(!package.contains("tailwind"), "{package}");
    }

    #[test]
    fn vite_template_compiles_and_fills_placeholders() {
        let dir = temp_dir("Vite App");
        let files = create(&dir, Template::Vite, "1.2.3", false).unwrap();
        assert_eq!(files.len(), VITE_FILES.len());

        let package = fs::read_to_string(dir.join("package.json")).unwrap();
        let json: serde_json::Value = serde_json::from_str(&package).unwrap();
        assert_eq!(json["name"], "vite-app");
        assert_eq!(json["devDependencies"]["fratmscript"], "^1.2.3");
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
        let files = create(&dir, Template::React, "1.2.3", false).unwrap();
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
    fn full_template_adds_an_api_and_a_database() {
        let dir = temp_dir("Trattoria");
        let files = create(&dir, Template::Full, "1.2.3", false).unwrap();
        let mut paths: Vec<String> = files
            .iter()
            .map(|f| {
                f.strip_prefix(&dir)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/")
            })
            .collect();
        paths.sort();
        let mut expected: Vec<&str> = REACT_FILES.iter().map(|(p, _)| *p).collect();
        expected.extend(["server/api.fratm", "server/avvia.fratm"]);
        expected.sort();
        assert_eq!(paths, expected);
        for path in &paths {
            let contents = fs::read_to_string(dir.join(path)).unwrap();
            // `style={{ … }}` in JSX is fine, the placeholders are not
            for placeholder in ["{{name}}", "{{fratmscript}}"] {
                assert!(
                    !contents.contains(placeholder),
                    "{path} has {placeholder} left"
                );
            }
        }

        let package = fs::read_to_string(dir.join("package.json")).unwrap();
        let json: serde_json::Value = serde_json::from_str(&package).unwrap();
        assert_eq!(json["name"], "trattoria");
        assert_eq!(
            json["scripts"]["start"],
            "node --import fratmscript/node server/avvia.fratm"
        );
        // npm start runs the server with fratmscript/node, so it is a runtime dependency
        assert_eq!(
            json["dependencies"]["fratmscript"],
            fratmscript_dependency("1.2.3")
        );
        assert!(json["dependencies"]["hono"].is_string());
        assert!(json["dependencies"]["@hono/node-server"].is_string());
        let config = fs::read_to_string(dir.join("vite.config.js")).unwrap();
        assert!(
            config.contains("fratmscript({ api: 'server/api.fratm' })"),
            "{config}"
        );

        let compiled = |path: &str| {
            compile(
                &fs::read_to_string(dir.join(path)).unwrap(),
                CompileOptions::default(),
            )
            .unwrap_or_else(|e| panic!("{path}: {e}"))
            .code
        };
        let app = compiled("src/App.fratm");
        assert!(app.contains("fetch(\"/api/pizze\")"), "{app}");
        assert!(app.contains("\"trattoria 🍕\""), "{app}");
        let api = compiled("server/api.fratm");
        assert!(api.contains("export default api;"), "{api}");
        assert!(
            api.contains("SELECT id, nomme, prezzo, ingredienti FROM pizze ORDER BY prezzo"),
            "{api}"
        );
        assert!(
            api.contains("INSERT INTO ordini (pizza) VALUES (?)\", \"INSERT INTO ordini (pizza) VALUES ($1)\", [pizza]"),
            "{api}"
        );
        let server = compiled("server/avvia.fratm");
        assert!(
            server.contains("import api from \"./api.fratm\";"),
            "{server}"
        );
        assert!(
            server.contains("serve({ fetch: app.fetch, port: porta }"),
            "{server}"
        );
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn simple_template_is_one_file() {
        let dir = temp_dir("semplice");
        create(&dir, Template::Simple, "1.2.3", false).unwrap();
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
        let error = create(&dir, Template::Simple, "1.2.3", false).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);
        assert_eq!(
            fs::read_to_string(dir.join("main.fratm")).unwrap(),
            "stamm a dì(1)\n"
        );
        fs::remove_dir_all(&dir).unwrap();

        // An empty folder is fine
        fs::create_dir_all(&dir).unwrap();
        create(&dir, Template::Simple, "1.2.3", false).unwrap();
        fs::remove_dir_all(&dir).unwrap();
    }
}
