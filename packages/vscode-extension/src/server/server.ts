/**
 * FratmScript Language Server
 *
 * Fornisce:
 * - Diagnostics (errori syntax)
 * - Autocompletamento keywords
 * - Hover con traduzioni JS
 * - Semantic tokens (opzionale)
 */

import {
  createConnection,
  TextDocuments,
  ProposedFeatures,
  InitializeParams,
  InitializeResult,
  TextDocumentSyncKind,
  CompletionItem,
  CompletionItemKind,
  Hover,
  MarkupKind,
  Diagnostic,
  DiagnosticSeverity,
  Position,
  Range,
  TextDocumentPositionParams,
  SemanticTokensBuilder,
  SemanticTokensLegend,
  SemanticTokensParams,
  SemanticTokens
} from 'vscode-languageserver/node';

import { TextDocument } from 'vscode-languageserver-textdocument';
import { KEYWORDS, KeywordInfo, getKeywordInfo, isKeyword } from './keywords';

// Semantic token types e modifiers
const tokenTypes = ['keyword', 'variable', 'function', 'class', 'string', 'number', 'operator', 'comment'];
const tokenModifiers = ['declaration', 'definition', 'readonly', 'async'];

const legend: SemanticTokensLegend = {
  tokenTypes,
  tokenModifiers
};

// Crea connessione LSP
const connection = createConnection(ProposedFeatures.all);

// Gestione documenti
const documents: TextDocuments<TextDocument> = new TextDocuments(TextDocument);

// Configurazione
let hasConfigurationCapability = false;
let hasWorkspaceFolderCapability = false;

interface FratmScriptSettings {
  enableDiagnostics: boolean;
  showTranslationOnHover: boolean;
}

const defaultSettings: FratmScriptSettings = {
  enableDiagnostics: true,
  showTranslationOnHover: true
};

let globalSettings: FratmScriptSettings = defaultSettings;
const documentSettings: Map<string, Thenable<FratmScriptSettings>> = new Map();

connection.onInitialize((params: InitializeParams): InitializeResult => {
  const capabilities = params.capabilities;

  hasConfigurationCapability = !!(
    capabilities.workspace && !!capabilities.workspace.configuration
  );
  hasWorkspaceFolderCapability = !!(
    capabilities.workspace && !!capabilities.workspace.workspaceFolders
  );

  const result: InitializeResult = {
    capabilities: {
      textDocumentSync: TextDocumentSyncKind.Incremental,
      completionProvider: {
        resolveProvider: true,
        triggerCharacters: ['.', ' ']
      },
      hoverProvider: true,
      semanticTokensProvider: {
        legend,
        full: true
      }
    }
  };

  if (hasWorkspaceFolderCapability) {
    result.capabilities.workspace = {
      workspaceFolders: {
        supported: true
      }
    };
  }

  return result;
});

connection.onInitialized(() => {
  // Configuration capability è già gestita automaticamente
});

// Gestione settings
function getDocumentSettings(resource: string): Thenable<FratmScriptSettings> {
  if (!hasConfigurationCapability) {
    return Promise.resolve(globalSettings);
  }
  let result = documentSettings.get(resource);
  if (!result) {
    result = connection.workspace.getConfiguration({
      scopeUri: resource,
      section: 'fratmscript'
    });
    documentSettings.set(resource, result);
  }
  return result;
}

documents.onDidClose(e => {
  documentSettings.delete(e.document.uri);
});

// === DIAGNOSTICS ===

documents.onDidChangeContent(change => {
  validateTextDocument(change.document);
});

async function validateTextDocument(textDocument: TextDocument): Promise<void> {
  const settings = await getDocumentSettings(textDocument.uri);

  if (!settings.enableDiagnostics) {
    connection.sendDiagnostics({ uri: textDocument.uri, diagnostics: [] });
    return;
  }

  const text = textDocument.getText();
  const diagnostics: Diagnostic[] = [];

  // Pattern per errori comuni
  const patterns: Array<{
    regex: RegExp;
    message: string;
    severity: DiagnosticSeverity;
  }> = [
    {
      regex: /\bconst\b/g,
      message: "Usa 'chist ... è' invece di 'const'",
      severity: DiagnosticSeverity.Warning
    },
    {
      regex: /\blet\b/g,
      message: "Usa 'tien' invece di 'let'",
      severity: DiagnosticSeverity.Warning
    },
    {
      regex: /\bfunction\b/g,
      message: "Usa 'facc' invece di 'function'",
      severity: DiagnosticSeverity.Warning
    },
    {
      regex: /\breturn\b/g,
      message: "Usa 'piglie' invece di 'return'",
      severity: DiagnosticSeverity.Warning
    },
    {
      regex: /\bif\b/g,
      message: "Usa 'si' invece di 'if'",
      severity: DiagnosticSeverity.Warning
    },
    {
      regex: /\belse\b/g,
      message: "Usa 'sinnò' invece di 'else'",
      severity: DiagnosticSeverity.Warning
    },
    {
      regex: /\bwhile\b/g,
      message: "Usa 'mentre che' invece di 'while'",
      severity: DiagnosticSeverity.Warning
    },
    {
      regex: /\btrue\b/g,
      message: "Usa 'overo' invece di 'true'",
      severity: DiagnosticSeverity.Warning
    },
    {
      regex: /\bfalse\b/g,
      message: "Usa 'sfòls' invece di 'false'",
      severity: DiagnosticSeverity.Warning
    },
    {
      regex: /\bnull\b/g,
      message: "Usa 'nisciun' invece di 'null'",
      severity: DiagnosticSeverity.Warning
    },
    {
      regex: /\bundefined\b/g,
      message: "Usa 'boh' invece di 'undefined'",
      severity: DiagnosticSeverity.Warning
    },
    {
      regex: /\bconsole\.log\b/g,
      message: "Usa 'stamm a dì' invece di 'console.log'",
      severity: DiagnosticSeverity.Warning
    },
    {
      regex: /\bclass\b/g,
      message: "Usa 'na famiglie' invece di 'class'",
      severity: DiagnosticSeverity.Warning
    },
    {
      regex: /\bnew\b/g,
      message: "Usa 'nu bell' invece di 'new'",
      severity: DiagnosticSeverity.Warning
    },
    {
      regex: /\bthis\b/g,
      message: "Usa 'stu cos' invece di 'this'",
      severity: DiagnosticSeverity.Warning
    },
    {
      regex: /\bimport\b/g,
      message: "Usa 'chiamm' invece di 'import'",
      severity: DiagnosticSeverity.Warning
    },
    {
      regex: /\bexport\b/g,
      message: "Usa 'mann for' invece di 'export'",
      severity: DiagnosticSeverity.Warning
    },
    {
      regex: /\basync\b/g,
      message: "Usa 'mo vir' invece di 'async'",
      severity: DiagnosticSeverity.Warning
    },
    {
      regex: /\bawait\b/g,
      message: "Usa 'aspett' invece di 'await'",
      severity: DiagnosticSeverity.Warning
    },
    {
      regex: /\btry\b/g,
      message: "Usa 'pruvamm' invece di 'try'",
      severity: DiagnosticSeverity.Warning
    },
    {
      regex: /\bcatch\b/g,
      message: "Usa 'e si schiatta' invece di 'catch'",
      severity: DiagnosticSeverity.Warning
    },
    {
      regex: /\bthrow\b/g,
      message: "Usa 'iett' invece di 'throw'",
      severity: DiagnosticSeverity.Warning
    }
  ];

  // Verifica parentesi non bilanciate
  let braceCount = 0;
  let parenCount = 0;
  let bracketCount = 0;

  for (let i = 0; i < text.length; i++) {
    const char = text[i];
    if (char === '{') braceCount++;
    else if (char === '}') braceCount--;
    else if (char === '(') parenCount++;
    else if (char === ')') parenCount--;
    else if (char === '[') bracketCount++;
    else if (char === ']') bracketCount--;
  }

  if (braceCount !== 0) {
    diagnostics.push({
      severity: DiagnosticSeverity.Error,
      range: {
        start: textDocument.positionAt(0),
        end: textDocument.positionAt(text.length)
      },
      message: braceCount > 0
        ? `Mancano ${braceCount} parentesi graffe di chiusura '}'`
        : `Ci sono ${-braceCount} parentesi graffe '}' in eccesso`,
      source: 'fratmscript'
    });
  }

  if (parenCount !== 0) {
    diagnostics.push({
      severity: DiagnosticSeverity.Error,
      range: {
        start: textDocument.positionAt(0),
        end: textDocument.positionAt(text.length)
      },
      message: parenCount > 0
        ? `Mancano ${parenCount} parentesi tonde di chiusura ')'`
        : `Ci sono ${-parenCount} parentesi tonde ')' in eccesso`,
      source: 'fratmscript'
    });
  }

  if (bracketCount !== 0) {
    diagnostics.push({
      severity: DiagnosticSeverity.Error,
      range: {
        start: textDocument.positionAt(0),
        end: textDocument.positionAt(text.length)
      },
      message: bracketCount > 0
        ? `Mancano ${bracketCount} parentesi quadre di chiusura ']'`
        : `Ci sono ${-bracketCount} parentesi quadre ']' in eccesso`,
      source: 'fratmscript'
    });
  }

  // Cerca pattern JavaScript (suggerimenti)
  for (const pattern of patterns) {
    let match;
    while ((match = pattern.regex.exec(text)) !== null) {
      const diagnostic: Diagnostic = {
        severity: pattern.severity,
        range: {
          start: textDocument.positionAt(match.index),
          end: textDocument.positionAt(match.index + match[0].length)
        },
        message: pattern.message,
        source: 'fratmscript'
      };
      diagnostics.push(diagnostic);
    }
  }

  connection.sendDiagnostics({ uri: textDocument.uri, diagnostics });
}

// === AUTOCOMPLETAMENTO ===

connection.onCompletion(
  (textDocumentPosition: TextDocumentPositionParams): CompletionItem[] => {
    const completions: CompletionItem[] = [];

    for (const keyword of KEYWORDS) {
      const item: CompletionItem = {
        label: keyword.napoletano,
        kind: getCompletionKind(keyword.categoria),
        detail: `→ ${keyword.javascript || keyword.napoletano}`,
        documentation: {
          kind: MarkupKind.Markdown,
          value: `**${keyword.napoletano}** → \`${keyword.javascript}\`\n\n${keyword.descrizione}`
        },
        insertText: keyword.snippet || keyword.napoletano,
        insertTextFormat: keyword.snippet ? 2 : 1 // 2 = Snippet, 1 = PlainText
      };
      completions.push(item);
    }

    return completions;
  }
);

function getCompletionKind(categoria: string): CompletionItemKind {
  switch (categoria) {
    case 'variabile':
      return CompletionItemKind.Variable;
    case 'funzione':
      return CompletionItemKind.Function;
    case 'controllo':
      return CompletionItemKind.Keyword;
    case 'classe':
      return CompletionItemKind.Class;
    case 'modulo':
      return CompletionItemKind.Module;
    case 'valore':
      return CompletionItemKind.Value;
    case 'operatore':
      return CompletionItemKind.Operator;
    case 'async':
      return CompletionItemKind.Keyword;
    case 'errore':
      return CompletionItemKind.Keyword;
    case 'console':
      return CompletionItemKind.Function;
    default:
      return CompletionItemKind.Text;
  }
}

connection.onCompletionResolve((item: CompletionItem): CompletionItem => {
  return item;
});

// === HOVER ===

connection.onHover((params: TextDocumentPositionParams): Hover | null => {
  const document = documents.get(params.textDocument.uri);
  if (!document) {
    return null;
  }

  const text = document.getText();
  const offset = document.offsetAt(params.position);

  // Trova la parola sotto il cursore
  const word = getWordAtPosition(text, offset);
  if (!word) {
    return null;
  }

  const keywordInfo = getKeywordInfo(word);
  if (!keywordInfo) {
    return null;
  }

  const jsEquivalent = keywordInfo.javascript
    ? `\`${keywordInfo.javascript}\``
    : '*(parte di costrutto)*';

  const markdown = [
    `## ${keywordInfo.napoletano}`,
    '',
    `**JavaScript:** ${jsEquivalent}`,
    '',
    `**Categoria:** ${keywordInfo.categoria}`,
    '',
    keywordInfo.descrizione,
    '',
    keywordInfo.snippet ? `**Snippet:**\n\`\`\`fratm\n${keywordInfo.snippet.replace(/\$\{\d+:?([^}]*)\}/g, '$1').replace(/\$\d+/g, '...')}\n\`\`\`` : ''
  ].filter(Boolean).join('\n');

  return {
    contents: {
      kind: MarkupKind.Markdown,
      value: markdown
    }
  };
});

function getWordAtPosition(text: string, offset: number): string | null {
  // Trova l'inizio della parola
  let start = offset;
  while (start > 0 && /[a-zA-ZàèéìòùÀÈÉÌÒÙ'_]/.test(text[start - 1])) {
    start--;
  }

  // Trova la fine della parola
  let end = offset;
  while (end < text.length && /[a-zA-ZàèéìòùÀÈÉÌÒÙ'_]/.test(text[end])) {
    end++;
  }

  if (start === end) {
    return null;
  }

  return text.substring(start, end);
}

// === SEMANTIC TOKENS ===

connection.languages.semanticTokens.on((params: SemanticTokensParams): SemanticTokens => {
  const document = documents.get(params.textDocument.uri);
  if (!document) {
    return { data: [] };
  }

  const builder = new SemanticTokensBuilder();
  const text = document.getText();

  // Pattern per identificare tokens
  const tokenPatterns: Array<{
    regex: RegExp;
    tokenType: number;
    modifiers?: number;
  }> = [
    // Keywords napoletane
    {
      regex: /\b(chist|è|tien|facc|piglie|si|sinnò|mentre|che|pe|ogni|rompe|salta|vir|caso|na|famiglie|costruttore|nu|bell|figlio|fisso|stu|cos|chiamm|da|mann|for|predefinit|mo|aspett|caccia|pruvamm|schiatta|iett|stamm|dì|scrive|avvis|fermete|leva|overo|sfòls|nisciun|boh|e|o|no|manco|pure)\b/g,
      tokenType: 0 // keyword
    },
    // Funzioni (identificatore seguito da parentesi)
    {
      regex: /\b([a-zA-Z_][a-zA-Z0-9_]*)\s*(?=\()/g,
      tokenType: 2 // function
    }
  ];

  for (const pattern of tokenPatterns) {
    let match;
    while ((match = pattern.regex.exec(text)) !== null) {
      const pos = document.positionAt(match.index);
      builder.push(
        pos.line,
        pos.character,
        match[0].length,
        pattern.tokenType,
        pattern.modifiers || 0
      );
    }
  }

  return builder.build();
});

// Avvia il server
documents.listen(connection);
connection.listen();
