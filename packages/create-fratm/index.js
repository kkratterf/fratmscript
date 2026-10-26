#!/usr/bin/env node
// `npm create fratm@latest mia-app`: writes a new FratmScript project with
// the same templates as `fratm nuovo`, using the compiler from npm, so the
// CLI does not have to be installed.
import { mkdir, readdir, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { createInterface } from 'node:readline/promises';
import { scaffold, version } from 'fratmscript';

const TEMPLATES = {
  '--completa': 'completa',
  '--senza-react': 'senza-react',
  '--semplice': 'semplice',
};

const HELP = `npm create fratm@latest <cartella> [opzioni]

  --completa      React + API Hono + database SQLite (Node 22.13+)
  --senza-react   Vite senza React, codice DOM normale
  --semplice      sulo nu main.fratm pe 'fratm lancia'
  --tailwind      azzecca Tailwind CSS (cu React)
  -h, --help      stammo ccà
`;

/** Parses argv: the folder, the template and whether Tailwind is wanted. */
export function parseArgs(argv) {
  const args = argv.filter((arg) => arg !== '');
  if (args.includes('-h') || args.includes('--help')) return { help: true };
  let template = 'react';
  let tailwind = false;
  const rest = [];
  for (const arg of args) {
    if (arg === '--tailwind') tailwind = true;
    else if (TEMPLATES[arg]) template = TEMPLATES[arg];
    else if (arg.startsWith('-')) return { error: `Nun capisco l'opzione ${arg}` };
    else rest.push(arg);
  }
  if (rest.length > 1) return { error: 'Na cartella sola, pe piacere' };
  if (tailwind && template !== 'react' && template !== 'completa')
    return { error: '--tailwind va cu React (sulo, o cu --completa)' };
  return { dir: rest[0], template, tailwind };
}

async function ask() {
  const rl = createInterface({ input: process.stdin, output: process.stdout });
  const answer = await rl.question("Comme se chiamma 'a cartella? (pizzeria) ");
  rl.close();
  return answer.trim() || 'pizzeria';
}

async function main(argv) {
  const options = parseArgs(argv);
  if (options.help) {
    process.stdout.write(HELP);
    return;
  }
  if (options.error) {
    process.stderr.write(`✗ ${options.error}\n\n${HELP}`);
    process.exitCode = 1;
    return;
  }
  const name = options.dir ?? (await ask());
  const dir = resolve(process.cwd(), name);
  const existing = await readdir(dir).catch((e) => (e.code === 'ENOENT' ? [] : Promise.reject(e)));
  if (existing.length > 0) {
    process.stderr.write(`✗ 'a cartella ${name} esiste già e nun è vacante\n`);
    process.exitCode = 1;
    return;
  }
  const files = scaffold({ template: options.template, name, tailwind: options.tailwind });
  for (const file of files) {
    const target = resolve(dir, file.path);
    await mkdir(dirname(target), { recursive: true });
    await writeFile(target, file.contents);
    process.stdout.write(`  + ${file.path}\n`);
  }
  // The simple template has no package.json: it needs the CLI, or the
  // compiler installed by hand to run the file with Node
  const start =
    options.template === 'semplice'
      ? "fratm lancia main.fratm\n\nSenza 'a CLI: npm i fratmscript && node --import fratmscript/node main.fratm"
      : 'npm install\n  npm run dev';
  process.stdout.write(`\n🤌 Progetto pronto cu FratmScript ${version()}: ${name}\n\n  cd ${name}\n  ${start}\n`);
}

// Not when imported by the tests
if (process.argv[1] && import.meta.url === `file://${process.argv[1]}`) {
  main(process.argv.slice(2)).catch((error) => {
    process.stderr.write(`✗ ${error.message}\n`);
    process.exitCode = 1;
  });
}
