// UI language: Italian by default, English or Neapolitan on request. Only the
// playground's own copy switches; compiler and runtime error messages are
// always Neapolitan, and FratmScript code and examples never change.

import { createContext, useCallback, useContext, useEffect, useMemo, useState, type ReactNode } from 'react'

export type Lang = 'it' | 'en' | 'nap'

export const LANGS: Lang[] = ['it', 'en', 'nap']

const LANG_KEY = 'fratm:lang'

const it = {
  'lang.name': 'Italiano',
  'lang.label': 'Lingua',

  'theme.label': 'Tema',
  'theme.system': 'Sistema',
  'theme.light': 'Chiaro',
  'theme.dark': 'Scuro',

  'header.tour': 'Impara',
  'header.tourLabel': 'Impara FratmScript',
  'header.tourOpen': 'Impara FratmScript passo passo',
  'header.tourExit': 'Esci dal tour',
  'header.github': 'Vedi su GitHub',
  'header.githubTip': 'Lascia una stella 🤌🏻',
  'footer.madeBy': 'Fatto con amore da',
  'header.run': 'Esegui',
  'header.runTip': 'Esegui il codice ({shortcut})',
  'header.stop': 'Ferma',
  'header.share': 'Condividi',

  'share.copied': 'Link copiato!',
  'share.copiedDesc': 'Invia questo link per condividere il tuo codice.',
  'share.failed': 'Copia non riuscita',
  'share.embed': "Copia il codice per l'embed",
  'share.embedCopied': 'Embed copiato!',
  'share.embedCopiedDesc': 'Incolla questo <iframe> in una pagina web o in un post.',
  'embed.open': 'Apri nel playground',

  'loading': 'Caricamento di FratmScript…',

  'editor.edited': '(modificato)',
  'editor.examples': 'Esempi:',
  'editor.chooseExample': '-- Scegli un esempio --',
  'editor.custom': 'Codice tuo',
  'editor.autoRun': 'Esecuzione automatica',
  'editor.label': 'Codice FratmScript',
  'editor.keyboardHint': "Tab inserisce un rientro. Premi Esc e poi Tab per uscire dall'editor.",
  'editor.format': 'Formatta il codice',
  'editor.formatTip': 'Formatta il codice (Shift+Alt+F)',

  'tabs.label': 'Pannelli del playground',
  'tabs.code': 'Codice',
  'tabs.console': 'Console',
  'tabs.js': 'JS',
  'tabs.errors': '{n} errori',
  'tabs.messages': '{n} messaggi',

  'output.title': 'Risultato',
  'output.stale': 'Il codice è cambiato dopo questo risultato',
  'output.console': 'Console',
  'output.running': 'in esecuzione…',
  'output.clear': 'Pulisci',
  'output.waiting': 'In attesa di output…',
  'output.empty': 'Nessun output in console',
  'output.view': 'Vista',
  'output.tokens': 'Token',
  'output.tree': 'Albero',
  'output.inspectHint': 'Passa sopra un elemento per vederlo nel codice.',
  'output.treeError': "Il codice non si compila: correggi l'errore per vedere l'albero.",
  'output.noTokens': 'Nessun token',
  'output.js': 'JavaScript generato',
  'output.mapLines': 'Collega le righe',
  'output.copyJs': 'Copia JS',
  'output.copied': 'Copiato',
  'output.jsPlaceholder': '// Esegui il codice per vedere il JavaScript',
  'output.jsUnavailable': "// Niente JavaScript: correggi prima l'errore qui sopra",
  'output.fromLine': 'Dalla riga {line}',
  'output.line': 'Riga {line}',
  'output.lineColumn': 'Riga {line}, colonna {column}',
  'preview.title': 'Anteprima',

  'error.compile': 'Errore di compilazione',
  'error.runtime': 'Errore in esecuzione',
  'error.timeout': 'Tempo di esecuzione scaduto',
  'error.load': 'Compilatore non disponibile',
  'error.loadMessage': 'Non è stato possibile scaricare il compilatore di FratmScript. Controlla la connessione e ricarica la pagina.',
  'error.timeoutMessage': 'Il codice è andato avanti per più di {seconds}s senza finire ed è stato fermato. C\'è un ciclo infinito?',
  'error.previewLoad': 'Anteprima non disponibile',
  'error.previewLoadMessage': 'Non è stato possibile scaricare React per l\'anteprima ({message}). Controlla la connessione e riprova.',

  'notice.truncated': 'Output della console troncato dopo {limit} messaggi.',
  'notice.asyncTimeout': 'Fermato dopo {seconds}s: c\'erano ancora timer o promise in sospeso.',
  'notice.stopped': 'Esecuzione fermata.',
  'notice.noComponent': 'Il codice usa React ma non esporta un componente: aggiungi mann for predefinit facc App() { … }.',

  'status.loading': 'Caricamento…',
  'status.ready': 'Compilatore pronto',
  'status.loadFailed': 'Il compilatore non si è caricato',
  'status.compileError': 'Errore di compilazione',
  'status.running': 'In esecuzione…',
  'status.timeout': 'Tempo di esecuzione scaduto',
  'status.runtimeError': 'Errore in esecuzione',
  'status.done': 'Fatto',
  'status.stopped': 'Fermato',
  'status.staleAuto': 'Codice cambiato, aggiorno…',
  'status.staleManual': 'Codice cambiato: il risultato non è aggiornato, premi Esegui',
  'status.compiledIn': 'Compilato in {ms}ms',
  'status.previewLoading': 'Carico l\'anteprima…',
  'status.previewReady': 'Anteprima pronta',

  'syntax.button': 'Guida alla sintassi',
  'syntax.title': 'Guida alla sintassi di FratmScript',
  'syntax.description': 'Dal napoletano a JavaScript. Clicca una riga per inserirla nell\'editor.',
  'syntax.search': 'Cerca: if, stamm, console…',
  'syntax.searchLabel': 'Cerca nella guida alla sintassi',
  'syntax.nothing': 'Nessun risultato per “{query}”.',
  'syntax.description.column': 'Descrizione',
  'syntax.insert': 'Inserisci {code}',

  'header.guide': 'Guida',
  'header.guideLabel': 'Guida al linguaggio',
  'header.guideOpen': 'Guida al linguaggio, con esempi da eseguire',
  'header.guideExit': 'Torna al playground',
  'header.playground': 'Playground',
  'guide.title': 'Guida a FratmScript',
  'guide.sections': 'Sezioni della guida',
  'guide.section': 'Sezione',
  'guide.intro': 'Gli esempi si modificano e si eseguono qui: premi Esegui o {shortcut}.',
  'guide.run': 'Esegui',
  'guide.runLabel': "Esegui l'esempio",
  'guide.stop': 'Ferma',
  'guide.reset': 'Ripristina',
  'guide.resetTip': 'Torna al codice originale',
  'guide.showJs': 'JS',
  'guide.showJsTip': 'Mostra il JavaScript generato',
  'guide.open': "Apri nell'editor",
  'guide.openTip': 'Continua con questo codice nel playground',
  'guide.console': 'Console',
  'guide.empty': 'Nessun output',
  'guide.previous': 'Indietro',
  'guide.next': 'Avanti',

  'lesson.label': 'Lezione',
  'lesson.index': 'Indice delle lezioni',
  'lesson.exit': 'Esci dal tour',
  'lesson.done': 'Bravo! 🤌',
  'lesson.exercise': 'Esercizio',
  'lesson.expected': 'La console deve stampare:',
  'lesson.reset': 'Ricomincia',
  'lesson.resetTip': 'Torna al codice iniziale',
  'lesson.solution': 'Soluzione',
  'lesson.solutionTip': 'Mostra la soluzione',
  'lesson.previous': 'Lezione precedente',
  'lesson.next': 'Avanti',
  'lesson.finish': 'Finito',
}

export type MessageKey = keyof typeof it

const en: Record<MessageKey, string> = {
  'lang.name': 'English',
  'lang.label': 'Language',

  'theme.label': 'Theme',
  'theme.system': 'System',
  'theme.light': 'Light',
  'theme.dark': 'Dark',

  'header.tour': 'Learn',
  'header.tourLabel': 'Learn FratmScript',
  'header.tourOpen': 'Learn FratmScript step by step',
  'header.tourExit': 'Leave the tour',
  'header.github': 'View on GitHub',
  'header.githubTip': 'Star this repo 🤌🏻',
  'footer.madeBy': 'Made with love by',
  'header.run': 'Run',
  'header.runTip': 'Run code ({shortcut})',
  'header.stop': 'Stop',
  'header.share': 'Share',

  'share.copied': 'Link copied!',
  'share.copiedDesc': 'Send this link to share your code.',
  'share.failed': 'Copy failed',
  'share.embed': 'Copy embed code',
  'share.embedCopied': 'Embed code copied!',
  'share.embedCopiedDesc': 'Paste this <iframe> into a web page or a blog post.',
  'embed.open': 'Open in the playground',

  'loading': 'Loading FratmScript…',

  'editor.edited': '(edited)',
  'editor.examples': 'Examples:',
  'editor.chooseExample': '-- Choose an example --',
  'editor.custom': 'Custom code',
  'editor.autoRun': 'Auto-run',
  'editor.label': 'FratmScript code',
  'editor.keyboardHint': 'Tab indents. Press Escape, then Tab, to leave the editor.',
  'editor.format': 'Format code',
  'editor.formatTip': 'Format code (Shift+Alt+F)',

  'tabs.label': 'Playground panels',
  'tabs.code': 'Code',
  'tabs.console': 'Console',
  'tabs.js': 'JS',
  'tabs.errors': '{n} errors',
  'tabs.messages': '{n} messages',

  'output.title': 'Output',
  'output.stale': 'The code changed since this output was produced',
  'output.console': 'Console',
  'output.running': 'running…',
  'output.clear': 'Clear',
  'output.waiting': 'Waiting for output…',
  'output.empty': 'No console output',
  'output.view': 'View',
  'output.tokens': 'Tokens',
  'output.tree': 'Tree',
  'output.inspectHint': 'Hover an item to see it in the code.',
  'output.treeError': "The code doesn't compile: fix the error to see the tree.",
  'output.noTokens': 'No tokens',
  'output.js': 'Generated JavaScript',
  'output.mapLines': 'Map lines',
  'output.copyJs': 'Copy JS',
  'output.copied': 'Copied',
  'output.jsPlaceholder': '// Run the code to see the JavaScript',
  'output.jsUnavailable': '// No JavaScript: fix the error above first',
  'output.fromLine': 'From line {line}',
  'output.line': 'Line {line}',
  'output.lineColumn': 'Line {line}, column {column}',
  'preview.title': 'Preview',

  'error.compile': 'Compilation error',
  'error.runtime': 'Runtime error',
  'error.timeout': 'Execution timed out',
  'error.load': 'Compiler not available',
  'error.loadMessage': 'The FratmScript compiler could not be downloaded. Check your connection and reload the page.',
  'error.timeoutMessage': 'The code ran for more than {seconds}s without finishing, so it was stopped. Is there an infinite loop?',
  'error.previewLoad': 'Preview unavailable',
  'error.previewLoadMessage': 'React could not be downloaded for the preview ({message}). Check your connection and try again.',

  'notice.truncated': 'Console output truncated after {limit} messages.',
  'notice.asyncTimeout': 'Stopped after {seconds}s: timers or promises were still pending.',
  'notice.stopped': 'Execution stopped.',
  'notice.noComponent': 'The code uses React but exports no component: add mann for predefinit facc App() { … }.',

  'status.loading': 'Loading…',
  'status.ready': 'Compiler ready',
  'status.loadFailed': 'Compiler failed to load',
  'status.compileError': 'Compilation error',
  'status.running': 'Running…',
  'status.timeout': 'Execution timed out',
  'status.runtimeError': 'Runtime error',
  'status.done': 'Done',
  'status.stopped': 'Stopped',
  'status.staleAuto': 'Code changed, updating…',
  'status.staleManual': 'Code changed: output is out of date, press Run',
  'status.compiledIn': 'Compiled in {ms}ms',
  'status.previewLoading': 'Loading the preview…',
  'status.previewReady': 'Preview ready',

  'syntax.button': 'Syntax help',
  'syntax.title': 'FratmScript syntax reference',
  'syntax.description': 'Neapolitan dialect to JavaScript translation. Click a row to insert it in the editor.',
  'syntax.search': 'Search: if, stamm, console…',
  'syntax.searchLabel': 'Search the syntax reference',
  'syntax.nothing': 'Nothing matches “{query}”.',
  'syntax.description.column': 'Description',
  'syntax.insert': 'Insert {code}',

  'header.guide': 'Guide',
  'header.guideLabel': 'Language guide',
  'header.guideOpen': 'Language guide, with examples you can run',
  'header.guideExit': 'Back to the playground',
  'header.playground': 'Playground',
  'guide.title': 'FratmScript guide',
  'guide.sections': 'Guide sections',
  'guide.section': 'Section',
  'guide.intro': 'Edit and run the examples right here: press Run or {shortcut}.',
  'guide.run': 'Run',
  'guide.runLabel': 'Run the example',
  'guide.stop': 'Stop',
  'guide.reset': 'Reset',
  'guide.resetTip': 'Back to the original code',
  'guide.showJs': 'JS',
  'guide.showJsTip': 'Show the generated JavaScript',
  'guide.open': 'Open in editor',
  'guide.openTip': 'Keep going with this code in the playground',
  'guide.console': 'Console',
  'guide.empty': 'No output',
  'guide.previous': 'Back',
  'guide.next': 'Next',

  'lesson.label': 'Lesson',
  'lesson.index': 'Lesson index',
  'lesson.exit': 'Leave the tour',
  'lesson.done': 'Well done! 🤌',
  'lesson.exercise': 'Exercise',
  'lesson.expected': 'The console must print:',
  'lesson.reset': 'Start over',
  'lesson.resetTip': 'Back to the starting code',
  'lesson.solution': 'Solution',
  'lesson.solutionTip': 'Show the solution',
  'lesson.previous': 'Previous lesson',
  'lesson.next': 'Next',
  'lesson.finish': 'Finish',
}

const nap: Record<MessageKey, string> = {
  'lang.name': 'Napulitano',
  'lang.label': 'Lengua',

  'theme.label': 'Tema',
  'theme.system': 'Sistema',
  'theme.light': 'Chiaro',
  'theme.dark': 'Scuro',

  'header.tour': 'Scola',
  'header.tourLabel': "'A scola 'e FratmScript",
  'header.tourOpen': "'A scola 'e FratmScript: impara passo passo",
  'header.tourExit': "Esci d'a scola",
  'header.github': "Vire ncopp'a GitHub",
  'header.githubTip': 'Miette na stella 🤌🏻',
  'footer.madeBy': 'Fatto cu ammore da',
  'header.run': 'Lancia',
  'header.runTip': "Lancia 'o codice ({shortcut})",
  'header.stop': 'Ferma',
  'header.share': 'Condividi',

  'share.copied': 'Link copiato!',
  'share.copiedDesc': "Manna stu link a chi vuò pe fà vedé 'o codice tuoio.",
  'share.failed': 'Nun se pò copià',
  'share.embed': "Copia 'o codice pe ll'embed",
  'share.embedCopied': 'Embed copiato!',
  'share.embedCopiedDesc': "Azzecca stu <iframe> int'a na pagina web o int'a nu blog.",
  'embed.open': "Arape int'o playground",

  'loading': 'Sto caricanno FratmScript…',

  'editor.edited': '(cagnato)',
  'editor.examples': 'Esempi:',
  'editor.chooseExample': "-- Sciglie n'esempio --",
  'editor.custom': 'Codice tuoio',
  'editor.autoRun': 'Parte sulo',
  'editor.label': "Codice FratmScript",
  'editor.keyboardHint': "Tab mette 'o rientro. Pigia Esc e po' Tab pe ascì 'a l'editor.",
  'editor.format': "Arrecetta 'o codice",
  'editor.formatTip': "Arrecetta 'o codice (Shift+Alt+F)",

  'tabs.label': "Pannielle d'o playground",
  'tabs.code': 'Codice',
  'tabs.console': 'Console',
  'tabs.js': 'JS',
  'tabs.errors': '{n} errori',
  'tabs.messages': '{n} messaggi',

  'output.title': 'Risultato',
  'output.stale': "'O codice è cagnato doppo stu risultato",
  'output.console': 'Console',
  'output.running': 'sta giranno…',
  'output.clear': 'Pulizza',
  'output.waiting': "Aspetto 'o risultato…",
  'output.empty': "'A console è vacante",
  'output.view': 'Vista',
  'output.tokens': 'Token',
  'output.tree': 'Arbero',
  'output.inspectHint': "Passa ncopp'a na cosa pe vedé addò sta int'o codice.",
  'output.treeError': "'O codice nun se compila: aggiusta l'errore pe vedé l'arbero.",
  'output.noTokens': 'Nisciun token',
  'output.js': 'JavaScript generato',
  'output.mapLines': "Collega 'e righe",
  'output.copyJs': 'Copia JS',
  'output.copied': 'Copiato',
  'output.jsPlaceholder': "// Lancia 'o codice pe vedé 'o JavaScript",
  'output.jsUnavailable': "// Niente JavaScript: aggiusta primma l'errore 'a coppa",
  'output.fromLine': "D'a riga {line}",
  'output.line': 'Riga {line}',
  'output.lineColumn': 'Riga {line}, colonna {column}',
  'preview.title': 'Comme se vede',

  'error.compile': "Errore 'e compilazione",
  'error.runtime': 'Errore mentre gira',
  'error.timeout': 'Ce sta mettenno troppo',
  'error.load': "'O compilatore nun ce sta",
  'error.loadMessage': "Nun se pò scarica 'o compilatore 'e FratmScript. Controlla 'a connessione e ricarica 'a paggina.",
  'error.timeoutMessage': "'O codice è ghiuto annanze cchiù 'e {seconds}s senza fernì, e l'aggio fermato. Ce sta nu ciclo ca nun fernesce maje?",
  'error.previewLoad': "L'anteprima nun parte",
  'error.previewLoadMessage': "Nun so' riuscito a scarrecà React pe l'anteprima ({message}). Guarda 'a connessione e pruova n'ata vota.",

  'notice.truncated': "'A console s'è fermata doppo {limit} messaggi.",
  'notice.asyncTimeout': 'Fermato doppo {seconds}s: ce stevano ancora timer o promise appise.',
  'notice.stopped': 'Fermato.',
  'notice.noComponent': "'O codice usa React ma nun manna fore nisciun componente: miette mann for predefinit facc App() { … }.",

  'status.loading': 'Sto caricanno…',
  'status.ready': "'O compilatore è pronto",
  'status.loadFailed': "'O compilatore nun s'è caricato",
  'status.compileError': "Errore 'e compilazione",
  'status.running': 'Sta giranno…',
  'status.timeout': 'Ce sta mettenno troppo',
  'status.runtimeError': 'Errore mentre gira',
  'status.done': 'Fatto',
  'status.stopped': 'Fermato',
  'status.staleAuto': "'O codice è cagnato, sto aggiurnanno…",
  'status.staleManual': "'O codice è cagnato: 'o risultato è viecchio, pigia Lancia",
  'status.compiledIn': 'Compilato in {ms}ms',
  'status.previewLoading': "Sto carrecanno l'anteprima…",
  'status.previewReady': "L'anteprima è pronta",

  'syntax.button': "Guida d'a sintassi",
  'syntax.title': "Guida d'a sintassi 'e FratmScript",
  'syntax.description': "Comme se dice in JavaScript. Pigia na riga pe 'a mettere int'a l'editor.",
  'syntax.search': 'Cerca: if, stamm, console…',
  'syntax.searchLabel': "Cerca int'a guida",
  'syntax.nothing': 'Nun ce sta niente pe “{query}”.',
  'syntax.description.column': 'Che fa',
  'syntax.insert': 'Miette {code}',

  'header.guide': 'Guida',
  'header.guideLabel': "Guida d'o linguaggio",
  'header.guideOpen': "Guida d'o linguaggio, cu l'esempi ca puoi fà girà",
  'header.guideExit': "Torna a 'o playground",
  'header.playground': 'Playground',
  'guide.title': "Guida 'e FratmScript",
  'guide.sections': "Sezioni d'a guida",
  'guide.section': 'Sezione',
  'guide.intro': "L'esempi se cagnano e se fanno girà ccà: pigia Esegui o {shortcut}.",
  'guide.run': 'Esegui',
  'guide.runLabel': "Fa' girà l'esempio",
  'guide.stop': 'Ferma',
  'guide.reset': 'Ripristina',
  'guide.resetTip': "Torna a 'o codice 'e primma",
  'guide.showJs': 'JS',
  'guide.showJsTip': "Fa vedé 'o JavaScript ca esce",
  'guide.open': "Apri int'a l'editor",
  'guide.openTip': "Continua cu stu codice int'o playground",
  'guide.console': 'Console',
  'guide.empty': 'Niente output',
  'guide.previous': 'Arreto',
  'guide.next': 'Appriesso',

  'lesson.label': 'Lezione',
  'lesson.index': "Indice d'e lezioni",
  'lesson.exit': "Esci d'a scola",
  'lesson.done': 'Bravo! 🤌',
  'lesson.exercise': 'Esercizio',
  'lesson.expected': "'A console adda stampà:",
  'lesson.reset': 'Ricomincia',
  'lesson.resetTip': "Torna a 'o codice 'e partenza",
  'lesson.solution': 'Soluzione',
  'lesson.solutionTip': "Fa vedé 'a soluzione",
  'lesson.previous': "Lezione 'e primma",
  'lesson.next': 'Appriesso',
  'lesson.finish': 'Fernuto',
}

const MESSAGES: Record<Lang, Record<MessageKey, string>> = { it, en, nap }

export type MessageParams = Record<string, string | number>

/** A message to translate later, so it follows language switches. */
export interface Message {
  key: MessageKey
  params?: MessageParams
}

export function translate(lang: Lang, key: MessageKey, params?: MessageParams): string {
  const text = MESSAGES[lang][key]
  return params ? text.replace(/\{(\w+)\}/g, (match, name) => (name in params ? String(params[name]) : match)) : text
}

function readLang(): Lang {
  try {
    const stored = localStorage.getItem(LANG_KEY)
    return stored === 'en' || stored === 'nap' ? stored : 'it'
  } catch {
    return 'it'
  }
}

interface I18nValue {
  lang: Lang
  setLang: (lang: Lang) => void
  t: (key: MessageKey, params?: MessageParams) => string
}

const I18nContext = createContext<I18nValue | null>(null)

export function I18nProvider({ children }: { children: ReactNode }) {
  const [lang, setLangState] = useState<Lang>(readLang)

  useEffect(() => {
    document.documentElement.lang = lang
  }, [lang])

  const setLang = useCallback((next: Lang) => {
    setLangState(next)
    try {
      localStorage.setItem(LANG_KEY, next)
    } catch {
      // Storage can be unavailable (private mode): the choice lasts for this visit
    }
  }, [])

  const value = useMemo<I18nValue>(() => ({
    lang,
    setLang,
    t: (key, params) => translate(lang, key, params),
  }), [lang, setLang])

  return <I18nContext.Provider value={value}>{children}</I18nContext.Provider>
}

export function useI18n(): I18nValue {
  const value = useContext(I18nContext)
  if (!value) throw new Error('useI18n must be used inside I18nProvider')
  return value
}
