// Words the editor colours inside stile`…` and gomorra`…` blocks. They follow
// the compiler's tables (crates/fratm-core/src/stile.rs, html.rs, gomorra.rs),
// whose unit tests fail when a word is missing here.

/** Neapolitan CSS values (`stile::VALUES`) */
export const STILE_VALUES = [
  'russo', 'janco', 'niro', 'nero', 'verde', 'blu', 'azzurro', 'celeste', 'giallo', 'arancione',
  'rosa', 'viola', 'grigio', 'marrone', 'oro', 'argiento', 'trasparente', 'culore-corrente',
  'nisciuno', 'niente', 'blocco', 'in-linea', 'blocco-in-linea', 'flessibile', 'griglia', 'miezo',
  'mmiezo', 'manca', 'destra', 'ncoppa', 'sotto', 'inizio', 'fine', 'spartuto', 'attuorno',
  'stiracchiato', 'riga', 'colonna', 'fisso', 'fissa', 'assoluto', 'assoluta', 'relativo',
  'relativa', 'appiccecato', 'appiccecata', 'statico', 'grassetto', 'corsivo', 'normale',
  'sottolineato', 'sbarrato', 'maiuscolo', 'minuscolo', 'nascosto', 'visibile', 'scorre',
  'automatico', 'manella', 'aspetta', 'ereditato', 'iniziale', 'a-capo', 'nun-va-a-capo', 'copre',
  'contiene', 'ripete', 'nun-ripete', 'continuo', 'tratteggiato', 'puntinato', 'bordo-incluso',
  'facile', 'lineare', 'infinito', 'alternato',
]

/** Tags in selectors, Neapolitan and HTML (`html::TAGS`) */
export const STILE_TAGS = [
  'scatola', 'div', 'piezzo', 'span', 'scritta', 'p', 'titolo', 'h1', 'titolo2', 'h2', 'titolo3',
  'h3', 'titolo4', 'h4', 'titolo5', 'h5', 'titolo6', 'h6', 'buttone', 'button', 'casella', 'input',
  'quaderno', 'textarea', 'scegli', 'select', 'opzione', 'option', 'etichetta', 'label', 'modulo',
  'form', 'lista', 'ul', 'elenco', 'ol', 'voce', 'li', 'collegamento', 'a', 'immagine', 'img',
  'capa', 'header', 'pede', 'footer', 'core', 'main', 'sezione', 'section', 'articolo', 'article',
  'navigazione', 'nav', 'acapo', 'br', 'linea', 'hr', 'forte', 'strong', 'corsivo', 'em', 'codice',
  'code', 'tabella', 'table', 'riga', 'tr', 'cella', 'td', 'capocella', 'th', 'tela', 'canvas',
]

/** GomorraSQL phrases of kind Statement (`gomorra::PHRASES`) */
export const GOMORRA_STATEMENTS = [
  "facimm na' strage", "facimm 'a tavula", "abbrucimm 'a tavula", "iamme bello ia'",
  "nzipp 'ngoppa", 'ua uagliò', 'ripigliammo', 'rifacimm', 'sfaccimm',
]

/** GomorraSQL phrases of kind Clause (`gomorra::PHRASES`) */
export const GOMORRA_CLAUSES = [
  'mettimmo in fila pe', "facimm 'e mucchie pe", "nun cchiù 'e", 'pesc e pesc', "mmiez 'a",
  'accunza', 'arò', 'chist', 'zumpanno',
]

/** GomorraSQL phrases of kind Operator (`gomorra::PHRASES`) */
export const GOMORRA_OPERATORS = [
  'nun assumiglia a', 'nun è', "nun dint'a", 'assumiglia a', 'accussì', "dint'a", 'comme', 'e',
  'o', 'è', 'nun',
]

/** GomorraSQL phrases of kind Value (`gomorra::PHRASES`) */
export const GOMORRA_VALUES = [
  "tutto chillo ch'era 'o nuostro", 'nun è nisciun', 'è nisciun', 'a salì', 'a scennere',
  'nisciun', 'overo', 'sfòls',
]
