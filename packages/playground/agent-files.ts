// Files for AI agents, served next to the playground:
// - llms.txt: index in the llmstxt.org format
// - llms-full.txt: the agent reference (`fratm guida`) plus the guide's
//   English sections and examples, with their output
// - keywords.json: the keyword table shared with the VS Code extension
import fs from 'node:fs'
import path from 'node:path'
import type { Plugin } from 'vite'
import { KEYWORDS } from '../vscode-extension/src/server/keywords'

export const SITE = 'https://kkratterf.github.io/fratmscript/'
export const REPO = 'https://github.com/kkratterf/fratmscript'
const root = path.resolve(__dirname, '../..')
export const guidaDir = path.join(__dirname, 'src/guida')

export interface GuideSection {
  id: string
  title: Record<string, string>
  body: Record<string, string[]>
  snippets: string[]
}

export function version(): string {
  return JSON.parse(fs.readFileSync(path.join(__dirname, 'package.json'), 'utf8')).version
}

/** The guide's sections in English, each example with the output it prints. */
function guideMarkdown(): string {
  const { sections } = JSON.parse(
    fs.readFileSync(path.join(guidaDir, 'guida.json'), 'utf8'),
  ) as { sections: GuideSection[] }
  const parts = [
    '# Guide with runnable examples',
    `The interactive guide of the playground (${SITE}#guida), in English. Each example runs as shown.`,
  ]
  for (const section of sections) {
    parts.push(`## ${section.title.en}`, `${SITE}#guida=${section.id}`, ...section.body.en)
    for (const snippet of section.snippets) {
      const code = fs.readFileSync(path.join(guidaDir, snippet), 'utf8').trimEnd()
      if (snippet.endsWith('.sbagliato.fratm')) {
        parts.push('This example does not compile, on purpose:', '```text\n' + code + '\n```')
        continue
      }
      parts.push('```fratm\n' + code + '\n```')
      const out = path.join(guidaDir, snippet.replace(/\.fratm$/, '.out'))
      if (fs.existsSync(out)) {
        parts.push('Output:', '```text\n' + fs.readFileSync(out, 'utf8').trimEnd() + '\n```')
      }
    }
  }
  return parts.join('\n\n') + '\n'
}

function guida(): string {
  return fs.readFileSync(path.join(root, 'crates/fratm-cli/src/guida.md'), 'utf8')
}

function llmsFull(): string {
  return `${guida().trimEnd()}\n\n${guideMarkdown()}`
}

interface Phrase {
  fratmscript: string
  javascript: string
  aliases?: string[]
  note?: string
}

/** Code spans of a table cell, and the text in brackets after them. */
function cell(text: string): { codes: string[]; extra: string } {
  // The first " (" outside a code span starts the bracketed text
  let at = -1
  for (let i = text.indexOf(' ('); i >= 0; i = text.indexOf(' (', i + 1)) {
    if ((text.slice(0, i).match(/`/g) ?? []).length % 2 === 0) {
      at = i
      break
    }
  }
  const main = at < 0 ? text : text.slice(0, at)
  const extra = at < 0 ? '' : text.slice(at + 2, -1)
  return { codes: [...main.matchAll(/`([^`]+)`/g)].map((m) => m[1]), extra }
}

/** The keyword table of the agent reference, one entry per FratmScript phrase. */
function keywordTable(): Phrase[] {
  const section = guida().split('\n## Keywords\n')[1]?.split('\n## ')[0] ?? ''
  const phrases: Phrase[] = []
  for (const line of section.split('\n').filter((l) => l.startsWith('| `'))) {
    const [left, right] = line
      .slice(1, -1)
      .split(/(?<!\\)\|/)
      .map((c) => cell(c.trim().replace(/\\\|/g, '|')))
    const aliases = [...left.extra.matchAll(/`([^`]+)`/g)].map((m) => m[1])
    if (left.codes.length !== right.codes.length) {
      throw new Error(`guida.md keyword row does not pair up: ${line}`)
    }
    left.codes.forEach((fratmscript, i) => {
      const phrase: Phrase = { fratmscript, javascript: right.codes[i] }
      if (aliases.length && left.codes.length === 1) phrase.aliases = aliases
      if (right.extra) phrase.note = right.extra.replace(/`/g, '')
      phrases.push(phrase)
    })
  }
  if (phrases.length < 30) throw new Error('keyword table not found in guida.md')
  return phrases
}

function llms(): string {
  return `# FratmScript

> FratmScript is JavaScript with Neapolitan keywords: \`chist è\` for const, \`facc\` for function, \`piglie\` for return, \`stamm a dì\` for console.log. A Rust compiler turns \`.fratm\` files into plain modern JavaScript (ES modules) for Node.js and browsers. Version ${version()}.

Only the keywords change: operators, literals and every JavaScript built-in stay the same. JavaScript keywords such as \`const\`, \`function\`, \`return\`, \`if\`, \`new\`, \`this\` or \`true\` are not valid FratmScript. Read the full reference before writing code, then check or run it with the \`fratm\` CLI or its MCP server and fix it from the errors (Neapolitan message, line, column and hint).

## Docs

- [Full reference for LLMs](${SITE}llms-full.txt): keywords, rules that differ from JavaScript, examples for every construct, error glossary, CLI and MCP usage, and the guide's runnable examples
- [Keyword map](${SITE}keywords.json): every FratmScript keyword with its JavaScript equivalent, as JSON
- [README](${REPO}#readme): install, CLI commands, complete syntax

## Tools

- [Install the CLI](${REPO}#install): \`curl -fsSL https://raw.githubusercontent.com/kkratterf/fratmscript/main/install.sh | sh\`, or \`cargo install --git ${REPO} fratm-cli\`
- MCP server: \`fratm mcp\` (stdio) with tools fratm_reference, fratm_check, fratm_compile, fratm_run, fratm_format, fratm_gomorra; Claude Code: \`claude mcp add fratm -- fratm mcp\`
- [Claude Code plugin with a FratmScript skill](${REPO}/tree/main/packages/agent-plugin): \`/plugin marketplace add kkratterf/fratmscript\` then \`/plugin install fratmscript@fratmscript\`
- CLI for scripts: \`fratm check --json\`, \`fratm build --json\`, \`fratm lancia --json --timeout 10 -\` (reads stdin), \`fratm guida\` prints the reference

## Optional

- [Playground](${SITE}): editor in the browser, with a guide (${SITE}#guida) and a 12-lesson tour
- [Guide as web pages](${SITE}en/guide/): one page per topic, also in Italian (${SITE}guida/)
- [Example programs](${REPO}/tree/main/examples)
- [npm package](${REPO}/tree/main/packages/fratmscript): \`compile(source)\` in JavaScript, Vite plugin, \`<script type="text/fratmscript">\`
`
}

function keywordsJson(): string {
  return (
    JSON.stringify(
      {
        language: 'FratmScript',
        version: version(),
        reference: `${SITE}llms-full.txt`,
        phrases: keywordTable(),
        note: '"words" lists the words of multi-word keywords one by one (chist + è = const), with descriptions in Italian.',
        words: KEYWORDS.map((k) => ({
          fratmscript: k.napoletano,
          javascript: k.javascript,
          category: k.categoria,
          description: k.descrizione,
        })),
      },
      null,
      2,
    ) + '\n'
  )
}

const FILES: Record<string, { type: string; build: () => string }> = {
  'llms.txt': { type: 'text/plain; charset=utf-8', build: llms },
  'llms-full.txt': { type: 'text/plain; charset=utf-8', build: llmsFull },
  'keywords.json': { type: 'application/json; charset=utf-8', build: keywordsJson },
}

export default function agentFiles(): Plugin {
  return {
    name: 'fratm-agent-files',
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        const name = req.url?.split('?')[0].split('/').pop() ?? ''
        const file = FILES[name]
        if (!file) return next()
        res.setHeader('Content-Type', file.type)
        res.end(file.build())
      })
    },
    generateBundle() {
      for (const [fileName, file] of Object.entries(FILES)) {
        this.emitFile({ type: 'asset', fileName, source: file.build() })
      }
    },
  }
}
