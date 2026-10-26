# AGENTS.md

Rules for AI coding agents (Claude, Codex, Copilot, Cursor, ...) and humans working on FratmScript.
Read this file before changing anything. If a rule here conflicts with an instruction from the maintainer, the maintainer wins.

## What this repo is

FratmScript is a Neapolitan-flavoured language that transpiles to JavaScript.

| Path | What it is |
| --- | --- |
| `crates/fratm-core` | Compiler: `lexer/` → `parser/` (AST in `ast.rs`) → `codegen/`, plus `errors.rs` and `sourcemap.rs` |
| `crates/fratm-cli` | `fratm` binary (`build`, `run`, `lancia`) |
| `crates/fratm-wasm` | WASM bindings used by the playground |
| `packages/playground` | React + Vite + CodeMirror playground, deployed to GitHub Pages from `main` |
| `packages/playground/public/pkg` | WASM build of `fratm-wasm` (generated, git-ignored) |
| `packages/fratmscript` | npm package: WASM compiler, JS API and Vite plugin (`wasm/` is generated, git-ignored) |
| `packages/create-fratm` | npm package behind `npm create fratm`: writes a project with the templates of `fratm nuovo` |
| `packages/vscode-extension` | VS Code extension (grammar, snippets, LSP) |
| `packages/agent-plugin` | Claude Code plugin for AI agents: the `fratmscript` skill and the `fratm mcp` server (marketplace in `.claude-plugin/marketplace.json`) |
| `crates/fratm-cli/src/guida.md` | Language reference for AI agents: `fratm guida`, the MCP server and the site's `llms-full.txt` |
| `examples/*.fratm` | Example programs; CI compiles every one of them |

## Language of the project

- **Code, comments, doc comments, commit messages, PR titles and PR descriptions: English.**
- FratmScript keywords, examples and user-facing compiler and runtime error messages stay in Neapolitan/Italian. That is the product, not a style choice.
- Playground UI copy comes in Italian (default), English and Neapolitan, picked from a menu in the header. Every UI string lives in `packages/playground/src/lib/i18n.tsx` in all three languages; tour lessons (`src/lib/lessons.ts`), guide sections (`src/guida/guida.json`, also read by the agent reference, so keep its keys stable), syntax reference rows (`SyntaxHelp.tsx`), example names (`src/lib/examples.ts`) and keyword hover texts (`src/lib/keywordText.ts`, Italian from the shared keyword table) carry their own translations.
- When you touch a file that has Italian comments, translate the comments you edit; do not rewrite unrelated ones in the same PR.

## Build and test

Run these before you push. They mirror CI (`.github/workflows/ci.yml`).

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build -p fratm-cli --locked && for f in examples/*.fratm; do ./target/debug/fratm build "$f"; done
```

Playground (needs `wasm-pack`, Node 20, pnpm 9):

```bash
just wasm                                  # rebuilds packages/playground/public/pkg
cd packages/playground && pnpm install --frozen-lockfile && pnpm build
```

npm package (needs `wasm-pack`, Node 20): `cd packages/fratmscript && npm run build && npm test`. Then `cd packages/create-fratm && npm install --no-save ../fratmscript && npm test`.

VS Code extension (needs the npm package's WASM build first): `cd packages/vscode-extension && pnpm install && pnpm compile && pnpm test`.

`just` lists every shortcut (`just test`, `just lancia <file>`, `just examples-build`, ...).

Compiler robustness: `crates/fratm-core/tests/robustness.rs` mutates every example and golden case and checks that `compile` and `format` never panic (20 000 mutations in `cargo test`). After touching the lexer, parser or codegen, run it longer with `just fuzz` (500 000 mutations, release build; pick another `seed` to explore). `just bench` times each compiler stage; compare before and after a change that could slow it down.

### The playground WASM (`public/pkg`)

- `public/pkg` is generated and git-ignored. CI and the Pages deploy build it from the current sources with `wasm-pack`.
- Locally, run `just wasm` (or `just playground-dev`, which runs it first) after changing `crates/fratm-core` or `crates/fratm-wasm`. Without it the playground shows "Compiler not available".
- Never commit files in `public/pkg`.

## Rules for changes

- **CI must be green** before a PR is ready. Never skip, `#[ignore]`, delete or weaken a test to get green. Never silence clippy with a blanket `allow`; fix the lint or justify a narrow `#[allow]` with a comment.
- **Every compiler fix ships with a regression test** that fails before the fix and passes after it. Put it next to the code it covers (`#[cfg(test)] mod tests` in the same module) and assert on the generated JavaScript, not only on "it compiles".
- **Golden tests** (`crates/fratm-core/tests/golden.rs`) snapshot the JavaScript generated for every `examples/*.fratm` and every `crates/fratm-core/tests/golden/cases/*.fratm`; when a `.out` file sits next to the snapshot, the JS is also run with Node and stdout must match. New language constructs add a case there with a `.out` file. After an intended output change, regenerate with `FRATM_UPDATE_GOLDEN=1 cargo test -p fratm-core --test golden` and review the diff.
- **Every new language construct** needs: lexer/parser/codegen support, tests, an update to the keyword table in `README.md` and to the agent reference `crates/fratm-cli/src/guida.md` (printed by `fratm guida`; its `fratm` code blocks are compile-tested, and its "not supported yet" list must stay true), the VS Code grammar/snippets if relevant, the playground syntax help, and an example in `examples/` when it is user-visible.
- Do not change existing keywords or their meaning without an issue that says so. Backwards compatibility of `.fratm` files matters.
- Keep PRs small and focused: one issue, one PR. Do not mix refactors with behaviour changes.
- No new dependencies without a reason written in the PR description.
- Match the style of the surrounding code; run `cargo fmt` on Rust files you touch.

## Docs, roadmap and planning

- **Do not commit docs, roadmaps, plans, audits, reports or design notes to this repo.** No `ROADMAP.md`, `PLAN.md`, `docs/`, `notes/`, audit screenshots, etc. The roadmap lives outside the repo.
- Allowed documentation: `README.md` files (user-facing usage only), code comments, and this file plus `CLAUDE.md`.
- **Backlog = GitHub issues only.** No GitHub Projects, no milestones. Every issue carries labels:
  - area: `compiler`, `playground`, `ci`, `ecosystem`, `bug`
  - priority: `P0`, `P1`, `P2`
  - version: `v0.2`, `v0.3`, `v0.4`, ...
- Before opening an issue, search for duplicates. Reference the issue from the PR (`Closes #N`).

## Naming conventions

### Commits

[Conventional Commits](https://www.conventionalcommits.org/): `type(scope): summary`

- Summary in English, imperative mood, lower case, no trailing period, ≤ 72 characters.
- Types: `feat`, `fix`, `perf`, `refactor`, `test`, `docs` (README/comments only), `ci`, `build`, `chore`, `revert`.
- Scopes: `lexer`, `parser`, `codegen`, `core` (more than one compiler stage), `cli`, `wasm`, `playground`, `vscode`, `examples`, `ci`, `release`.
- Breaking changes: `feat(parser)!: ...` plus a `BREAKING CHANGE:` footer.
- Body explains *why*, wrapped at 72 columns.

Examples: `fix(lexer): keep 'e/'o pate strings intact`, `feat(parser): add for...of loops`, `ci: run clippy with -D warnings`.

### Pull requests

- **Title = a Conventional Commit line** (same format as above). Prefer squash merge, so the title becomes the commit on `main`.
- Description: what changed, why, how it was tested, and `Closes #N`. Screenshots for playground UI changes.
- Add the matching **area label** (`compiler`, `playground`, `bug`, `ci`, `ecosystem`). Release notes are grouped by these labels (`.github/release.yml`); use `skip-changelog` for changes users don't care about.
- Open as draft while CI is red or work is incomplete.

### Branches

`type/short-kebab-description`, e.g. `fix/lexer-quoted-e`, `feat/for-of`, `ci/release-workflow`.
Agent platforms that assign their own branch names (e.g. `claude/...`) are fine; the PR title carries the convention.

## Releases

- SemVer, tags `vX.Y.Z` (older `@0.1.0`-style tags are kept as they are).
- Minor release = a version milestone, cut when all `P0` issues of that version label are closed and `main` is green. Patch release when a user-facing fix lands.
- First a version-bump PR (`chore(release): vX.Y.Z`) that updates `[workspace.package] version` in `Cargo.toml` and `version` in `packages/playground/package.json`, `packages/fratmscript/package.json`, `packages/vscode-extension/package.json` and `packages/agent-plugin/.claude-plugin/plugin.json` (the release workflow checks all of them).
- The release is created by the manual `release.yml` workflow (`workflow_dispatch`), which tags, builds the binaries and WASM package, and generates notes.
- **No `CHANGELOG.md`.** The changelog lives only in GitHub Release notes.

## Merging

- Agents may push branches, open PRs and merge their own PRs once CI is green and the PR closes its issue(s). Agents never push directly to `main`, force-push `main`, or bypass a failing check. The maintainer is told about releases, not about every merge.
- Never rewrite history on someone else's branch.
