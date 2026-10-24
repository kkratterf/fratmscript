// UI language: English by default, Neapolitan on request. Only the
// playground's own copy switches; compiler and runtime error messages are
// always Neapolitan, and FratmScript code and examples never change.

import { createContext, useCallback, useContext, useEffect, useMemo, useState, type ReactNode } from 'react'

export type Lang = 'en' | 'nap'

export const LANGS: Lang[] = ['en', 'nap']

const LANG_KEY = 'fratm:lang'

const en = {
  'lang.name': 'English',
  'lang.short': 'EN',
  'lang.tooltip': 'Language: English',
  'lang.switch': 'Language: English. Switch to Neapolitan',

  'theme.system': 'Theme: system',
  'theme.light': 'Theme: light',
  'theme.dark': 'Theme: dark',
  'theme.switch': 'Switch to {theme}',
  'theme.name.system': 'system',
  'theme.name.light': 'light',
  'theme.name.dark': 'dark',

  'header.tour': 'Learn',
  'header.tourLabel': 'FratmScript tour',
  'header.tourOpen': 'Learn FratmScript step by step',
  'header.tourExit': 'Leave the tour',
  'header.github': 'View on GitHub',
  'header.githubTip': 'Star this repo 🤌🏻',
  'header.linkedinTip': 'Made with love by @kkratter 🤍',
  'header.xTip': 'Follow for updates 👀',
  'header.run': 'Run',
  'header.runTip': 'Run code ({shortcut})',
  'header.stop': 'Stop',
  'header.share': 'Share',

  'share.copied': 'Link copied!',
  'share.copiedDesc': 'Send this link to share your code.',
  'share.failed': 'Copy failed',

  'loading': 'Loading FratmScript…',

  'editor.edited': '(edited)',
  'editor.examples': 'Examples:',
  'editor.chooseExample': '-- Choose an example --',
  'editor.custom': 'Custom code',
  'editor.autoRun': 'Auto-run',
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
  'output.js': 'Generated JavaScript',
  'output.mapLines': 'Map lines',
  'output.copyJs': 'Copy JS',
  'output.copied': 'Copied',
  'output.jsPlaceholder': '// Run the code to see the JavaScript',
  'output.fromLine': 'From line {line}',
  'output.line': 'Line {line}',
  'output.lineColumn': 'Line {line}, column {column}',

  'error.compile': 'Compilation error',
  'error.runtime': 'Runtime error',
  'error.timeout': 'Execution timed out',
  'error.load': 'Compiler not available',
  'error.loadMessage': 'The FratmScript compiler could not be downloaded. Check your connection and reload the page.',
  'error.timeoutMessage': 'The code ran for more than {seconds}s without finishing, so it was stopped. Is there an infinite loop?',

  'notice.truncated': 'Console output truncated after {limit} messages.',
  'notice.asyncTimeout': 'Stopped after {seconds}s: timers or promises were still pending.',
  'notice.stopped': 'Execution stopped.',

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

  'syntax.button': 'Syntax help',
  'syntax.title': 'FratmScript syntax reference',
  'syntax.description': 'Neapolitan dialect to JavaScript translation. Click a row to insert it in the editor.',
  'syntax.search': 'Search: if, stamm, console…',
  'syntax.searchLabel': 'Search the syntax reference',
  'syntax.nothing': 'Nothing matches “{query}”.',
  'syntax.description.column': 'Description',
  'syntax.insert': 'Insert {code}',

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

export type MessageKey = keyof typeof en

const nap: Record<MessageKey, string> = {
  'lang.name': 'Napulitano',
  'lang.short': 'NAP',
  'lang.tooltip': 'Lengua: napulitano',
  'lang.switch': "Lengua: napulitano. Passa a l'inglese",

  'theme.system': "Tema: comm'o sistema",
  'theme.light': 'Tema: chiaro',
  'theme.dark': 'Tema: scuro',
  'theme.switch': 'Passa a {theme}',
  'theme.name.system': "comm'o sistema",
  'theme.name.light': 'chiaro',
  'theme.name.dark': 'scuro',

  'header.tour': 'Scola',
  'header.tourLabel': "'A scola 'e FratmScript",
  'header.tourOpen': "'A scola 'e FratmScript: impara passo passo",
  'header.tourExit': "Esci d'a scola",
  'header.github': "Vire ncopp'a GitHub",
  'header.githubTip': 'Miette na stella 🤌🏻',
  'header.linkedinTip': 'Fatto cu ammore da @kkratter 🤍',
  'header.xTip': "Siguitece pe sapé 'e novità 👀",
  'header.run': 'Lancia',
  'header.runTip': "Lancia 'o codice ({shortcut})",
  'header.stop': 'Ferma',
  'header.share': 'Condividi',

  'share.copied': 'Link copiato!',
  'share.copiedDesc': "Manna stu link a chi vuò pe fà vedé 'o codice tuoio.",
  'share.failed': 'Nun se pò copià',

  'loading': 'Sto caricanno FratmScript…',

  'editor.edited': '(cagnato)',
  'editor.examples': 'Esempi:',
  'editor.chooseExample': "-- Sciglie n'esempio --",
  'editor.custom': 'Codice tuoio',
  'editor.autoRun': 'Parte sulo',
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
  'output.js': 'JavaScript generato',
  'output.mapLines': "Collega 'e righe",
  'output.copyJs': 'Copia JS',
  'output.copied': 'Copiato',
  'output.jsPlaceholder': "// Lancia 'o codice pe vedé 'o JavaScript",
  'output.fromLine': "D'a riga {line}",
  'output.line': 'Riga {line}',
  'output.lineColumn': 'Riga {line}, colonna {column}',

  'error.compile': "Errore 'e compilazione",
  'error.runtime': 'Errore mentre gira',
  'error.timeout': 'Ce sta mettenno troppo',
  'error.load': "'O compilatore nun ce sta",
  'error.loadMessage': "Nun se pò scarica 'o compilatore 'e FratmScript. Controlla 'a connessione e ricarica 'a paggina.",
  'error.timeoutMessage': "'O codice è ghiuto annanze cchiù 'e {seconds}s senza fernì, e l'aggio fermato. Ce sta nu ciclo ca nun fernesce maje?",

  'notice.truncated': "'A console s'è fermata doppo {limit} messaggi.",
  'notice.asyncTimeout': 'Fermato doppo {seconds}s: ce stevano ancora timer o promise appise.',
  'notice.stopped': 'Fermato.',

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

  'syntax.button': "Guida d'a sintassi",
  'syntax.title': "Guida d'a sintassi 'e FratmScript",
  'syntax.description': "Comme se dice in JavaScript. Pigia na riga pe 'a mettere int'a l'editor.",
  'syntax.search': 'Cerca: if, stamm, console…',
  'syntax.searchLabel': "Cerca int'a guida",
  'syntax.nothing': 'Nun ce sta niente pe “{query}”.',
  'syntax.description.column': 'Che fa',
  'syntax.insert': 'Miette {code}',

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

const MESSAGES: Record<Lang, Record<MessageKey, string>> = { en, nap }

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
    return stored === 'nap' ? 'nap' : 'en'
  } catch {
    return 'en'
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
