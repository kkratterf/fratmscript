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
  'cristian', 'html', 'capa', 'head', 'panz', 'body', 'titolo', 'title', 'inciucc', 'meta',
  'attacca', 'link', 'basa', 'base', 'cess', 'style', 'fratm', 'script', 'senzafratm', 'noscript',
  'cappiell', 'header', 'pere', 'footer', 'tomtom', 'nav', 'core', 'main', 'piezz', 'section',
  'fattariello', 'article', 'allato', 'aside', 'indirizzo', 'address', 'luccolone', 'h1',
  'luccolo', 'h2', 'luccoliello', 'h3', 'luccolillo', 'h4', 'luccolino', 'h5', 'luccolicchio',
  'h6', 'cascett', 'div', 'chiacchier', 'p', 'stacc', 'hr', 'accussì', 'pre', 'accussi',
  'citazione', 'blockquote', 'list', 'ul', 'elenco', 'ol', 'voce', 'li', 'vocabolario', 'dl',
  'parola', 'dt', 'significato', 'dd', 'quadro', 'figure', 'didascalia', 'figcaption', 'vattenn',
  'a', 'scatulell', 'span', 'acap', 'br', 'fort', 'strong', 'corsivo', 'em', 'guapp', 'b',
  'stuorto', 'i', 'sottolineato', 'u', 'cancellato', 's', 'piccirill', 'small', 'evidenziato',
  'mark', 'codice', 'code', 'fonte', 'cite', 'virgulette', 'q', 'abbreviazione', 'abbr', 'tiempo',
  'time', 'sotto', 'sub', 'ncoppa', 'sup', 'tastiera', 'kbd', 'levato', 'del', 'aggiunto', 'ins',
  'cartulin', 'img', 'cornice', 'picture', 'funtana', 'source', 'filmato', 'video', 'suono',
  'audio', 'fenestella', 'iframe', 'tela', 'canvas', 'disegno', 'svg', 'tavulat', 'table',
  'cartello', 'caption', 'capotavula', 'thead', 'cuorpotavula', 'tbody', 'peretavula', 'tfoot',
  'fila', 'tr', 'casella', 'td', 'capocasella', 'th', 'modulo', 'form', 'miett', 'input',
  'citofono', 'button', 'etichetta', 'label', 'scegli', 'select', 'opzione', 'option', 'gruppo',
  'optgroup', 'ragionamento', 'textarea', 'recinto', 'fieldset', 'leggenda', 'legend',
  'suggerimenti', 'datalist', 'risultato', 'output', 'avanzamento', 'progress', 'misura', 'meter',
  'spoiler', 'details', 'anteprim', 'summary', 'dialogo', 'dialog', 'stampo', 'template',
  'fessura', 'slot', 'luccolata', 'hgroup', 'listino', 'menu', 'definizione', 'dfn', 'glossa',
  'ruby', 'schermata', 'samp', 'incognita', 'var', 'forsacap', 'wbr', 'zona', 'area', 'mappa',
  'map', 'ogget', 'object', 'traccia', 'track', 'cunte', 'math', 'culonna', 'col', 'culunnato',
  'colgroup', 'cerca', 'search', 'isolato', 'bdi', 'allevers', 'bdo', 'dato', 'data', 'parentesi',
  'rp', 'pronuncia', 'rt', 'ncastro', 'embed',
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
