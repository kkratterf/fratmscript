/**
 * FratmScript Keywords Database
 *
 * Mappa completa delle keywords napoletane con traduzioni JavaScript
 * e documentazione per hover e completamento.
 */

export interface KeywordInfo {
  /** Keyword in napoletano */
  napoletano: string;
  /** Traduzione JavaScript equivalente */
  javascript: string;
  /** Descrizione in italiano */
  descrizione: string;
  /** Categoria della keyword */
  categoria: 'variabile' | 'funzione' | 'controllo' | 'classe' | 'modulo' | 'valore' | 'operatore' | 'async' | 'errore' | 'console';
  /** Snippet opzionale per l'autocompletamento */
  snippet?: string;
}

export const KEYWORDS: KeywordInfo[] = [
  // === Variabili ===
  {
    napoletano: 'chist',
    javascript: 'const',
    descrizione: 'Dichiara una costante (primo parte, seguito da "è")',
    categoria: 'variabile',
    snippet: 'chist ${1:nome} è ${2:valore}'
  },
  {
    napoletano: 'è',
    javascript: '=',
    descrizione: 'Operatore di assegnazione per costanti (dopo "chist")',
    categoria: 'variabile'
  },
  {
    napoletano: 'tien',
    javascript: 'let',
    descrizione: 'Dichiara una variabile modificabile',
    categoria: 'variabile',
    snippet: 'tien ${1:nome} = ${2:valore}'
  },

  // === Funzioni ===
  {
    napoletano: 'facc',
    javascript: 'function',
    descrizione: 'Dichiara una funzione',
    categoria: 'funzione',
    snippet: 'facc ${1:nome}(${2:parametri}) {\n\t$0\n}'
  },
  {
    napoletano: 'piglie',
    javascript: 'return',
    descrizione: 'Ritorna un valore dalla funzione',
    categoria: 'funzione',
    snippet: 'piglie ${1:valore}'
  },

  // === Controllo di flusso ===
  {
    napoletano: 'si',
    javascript: 'if',
    descrizione: 'Blocco condizionale',
    categoria: 'controllo',
    snippet: 'si (${1:condizione}) {\n\t$0\n}'
  },
  {
    napoletano: 'sinnò',
    javascript: 'else',
    descrizione: 'Blocco alternativo (else)',
    categoria: 'controllo'
  },
  {
    napoletano: 'mentre',
    javascript: 'while',
    descrizione: 'Ciclo while (prima parte, seguito da "che")',
    categoria: 'controllo',
    snippet: 'mentre che (${1:condizione}) {\n\t$0\n}'
  },
  {
    napoletano: 'che',
    javascript: '',
    descrizione: 'Parte del costrutto "mentre che" (while)',
    categoria: 'controllo'
  },
  {
    napoletano: 'pe',
    javascript: 'for',
    descrizione: 'Ciclo for (prima parte, seguito da "ogni")',
    categoria: 'controllo',
    snippet: 'pe ogni (tien ${1:i} = 0; ${1:i} < ${2:n}; ${1:i} = ${1:i} + 1) {\n\t$0\n}'
  },
  {
    napoletano: 'ogni',
    javascript: '',
    descrizione: 'Parte del costrutto "pe ogni" (for)',
    categoria: 'controllo'
  },
  {
    napoletano: 'rompe',
    javascript: 'break',
    descrizione: 'Interrompe il ciclo corrente',
    categoria: 'controllo'
  },
  {
    napoletano: 'salta',
    javascript: 'continue',
    descrizione: 'Salta alla prossima iterazione del ciclo',
    categoria: 'controllo'
  },
  {
    napoletano: 'vir',
    javascript: 'switch',
    descrizione: 'Costrutto switch (prima parte)',
    categoria: 'controllo'
  },
  {
    napoletano: 'caso',
    javascript: 'case',
    descrizione: 'Caso in un blocco switch',
    categoria: 'controllo'
  },

  // === Classi e OOP ===
  {
    napoletano: 'na',
    javascript: 'class',
    descrizione: 'Dichiara una classe (prima parte, seguito da "famiglie")',
    categoria: 'classe',
    snippet: 'na famiglie ${1:Nome} {\n\tcostruttore(${2:parametri}) {\n\t\t$0\n\t}\n}'
  },
  {
    napoletano: 'famiglie',
    javascript: '',
    descrizione: 'Parte del costrutto "na famiglie" (class)',
    categoria: 'classe'
  },
  {
    napoletano: 'costruttore',
    javascript: 'constructor',
    descrizione: 'Costruttore della classe',
    categoria: 'classe'
  },
  {
    napoletano: 'nu',
    javascript: 'new',
    descrizione: 'Crea una nuova istanza (prima parte, seguito da "bell")',
    categoria: 'classe',
    snippet: 'nu bell ${1:Classe}(${2:argomenti})'
  },
  {
    napoletano: 'bell',
    javascript: '',
    descrizione: 'Parte del costrutto "nu bell" (new)',
    categoria: 'classe'
  },
  {
    napoletano: 'figlio',
    javascript: 'extends',
    descrizione: 'Eredita da una classe (seguito da "\'e")',
    categoria: 'classe'
  },
  {
    napoletano: 'fisso',
    javascript: 'static',
    descrizione: 'Metodo o proprietà statica',
    categoria: 'classe'
  },
  {
    napoletano: 'stu',
    javascript: 'this',
    descrizione: 'Riferimento all\'istanza corrente (prima parte)',
    categoria: 'classe'
  },
  {
    napoletano: 'cos',
    javascript: '',
    descrizione: 'Parte del costrutto "stu cos" (this)',
    categoria: 'classe'
  },

  // === Moduli ===
  {
    napoletano: 'chiamm',
    javascript: 'import',
    descrizione: 'Importa un modulo',
    categoria: 'modulo',
    snippet: 'chiamm { ${1:nome} } da \'${2:modulo}\''
  },
  {
    napoletano: 'da',
    javascript: 'from',
    descrizione: 'Specifica la sorgente dell\'import',
    categoria: 'modulo'
  },
  {
    napoletano: 'mann',
    javascript: 'export',
    descrizione: 'Esporta dal modulo (prima parte, seguito da "for")',
    categoria: 'modulo',
    snippet: 'mann for { ${1:nome} }'
  },
  {
    napoletano: 'for',
    javascript: '',
    descrizione: 'Parte del costrutto "mann for" (export)',
    categoria: 'modulo'
  },
  {
    napoletano: 'predefinit',
    javascript: 'default',
    descrizione: 'Export/import di default',
    categoria: 'modulo'
  },

  // === Valori ===
  {
    napoletano: 'overo',
    javascript: 'true',
    descrizione: 'Valore booleano vero',
    categoria: 'valore'
  },
  {
    napoletano: 'sfòls',
    javascript: 'false',
    descrizione: 'Valore booleano falso',
    categoria: 'valore'
  },
  {
    napoletano: 'nisciun',
    javascript: 'null',
    descrizione: 'Valore nullo',
    categoria: 'valore'
  },
  {
    napoletano: 'boh',
    javascript: 'undefined',
    descrizione: 'Valore non definito',
    categoria: 'valore'
  },

  // === Operatori logici ===
  {
    napoletano: 'e',
    javascript: '&&',
    descrizione: 'Operatore logico AND',
    categoria: 'operatore'
  },
  {
    napoletano: 'o',
    javascript: '||',
    descrizione: 'Operatore logico OR',
    categoria: 'operatore'
  },
  {
    napoletano: 'no',
    javascript: '!',
    descrizione: 'Operatore logico NOT',
    categoria: 'operatore'
  },
  {
    napoletano: 'manco',
    javascript: '!',
    descrizione: 'Operatore logico NOT (alias napoletano)',
    categoria: 'operatore'
  },
  {
    napoletano: 'pure',
    javascript: '&&',
    descrizione: 'Operatore logico AND (alias napoletano)',
    categoria: 'operatore'
  },
  {
    napoletano: 'leva',
    javascript: 'delete',
    descrizione: 'Rimuove una proprietà da un oggetto',
    categoria: 'operatore'
  },
  {
    napoletano: "dint'a",
    javascript: 'in',
    descrizione: 'Operatore in (verifica proprietà)',
    categoria: 'operatore'
  },

  // === Async ===
  {
    napoletano: 'mo',
    javascript: 'async',
    descrizione: 'Funzione asincrona (prima parte, seguito da "vir")',
    categoria: 'async',
    snippet: 'mo vir facc ${1:nome}(${2:parametri}) {\n\t$0\n}'
  },
  {
    napoletano: 'aspett',
    javascript: 'await',
    descrizione: 'Attende una Promise',
    categoria: 'async',
    snippet: 'aspett ${1:promise}'
  },
  {
    napoletano: 'caccia',
    javascript: 'yield',
    descrizione: 'Yield in un generatore',
    categoria: 'async'
  },

  // === Gestione errori ===
  {
    napoletano: 'pruvamm',
    javascript: 'try',
    descrizione: 'Blocco try per gestione errori',
    categoria: 'errore',
    snippet: 'pruvamm {\n\t$1\n} e si schiatta (${2:errore}) {\n\t$0\n}'
  },
  {
    napoletano: 'schiatta',
    javascript: 'catch',
    descrizione: 'Blocco catch (in "e si schiatta")',
    categoria: 'errore'
  },
  {
    napoletano: 'iett',
    javascript: 'throw',
    descrizione: 'Lancia un\'eccezione',
    categoria: 'errore',
    snippet: 'iett nu bell Error(${1:messaggio})'
  },

  // === Console ===
  {
    napoletano: 'stamm',
    javascript: 'console.log',
    descrizione: 'Stampa a console (prima parte di "stamm a dì")',
    categoria: 'console',
    snippet: 'stamm a dì(${1:messaggio})'
  },
  {
    napoletano: 'dì',
    javascript: '',
    descrizione: 'Parte del costrutto "stamm a dì" (console.log)',
    categoria: 'console'
  },
  {
    napoletano: 'scrive',
    javascript: 'console.error',
    descrizione: 'Stampa errore a console',
    categoria: 'console',
    snippet: 'scrive(${1:errore})'
  },
  {
    napoletano: 'avvis',
    javascript: 'console.warn',
    descrizione: 'Stampa avviso a console',
    categoria: 'console',
    snippet: 'avvis(${1:avviso})'
  },

  // === Debug ===
  {
    napoletano: 'fermete',
    javascript: 'debugger',
    descrizione: 'Punto di interruzione per il debugger',
    categoria: 'controllo'
  }
];

/** Mappa veloce keyword -> info */
export const KEYWORD_MAP: Map<string, KeywordInfo> = new Map(
  KEYWORDS.map(k => [k.napoletano, k])
);

/** Ottieni info per una keyword */
export function getKeywordInfo(keyword: string): KeywordInfo | undefined {
  return KEYWORD_MAP.get(keyword);
}

/** Verifica se una parola è una keyword */
export function isKeyword(word: string): boolean {
  return KEYWORD_MAP.has(word);
}
