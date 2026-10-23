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
  SemanticTokens,
  DocumentFormattingParams,
  TextEdit
} from 'vscode-languageserver/node';

import { TextDocument } from 'vscode-languageserver-textdocument';
import { KEYWORDS, KeywordInfo, getKeywordInfo, isKeyword } from './keywords';
import { diagnose, formatSource } from './compiler';

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
      documentFormattingProvider: true,
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

  let diagnostics: Diagnostic[];
  try {
    diagnostics = diagnose(textDocument.getText());
  } catch (error) {
    connection.console.error(`FratmScript: ${error}`);
    diagnostics = [];
  }

  connection.sendDiagnostics({ uri: textDocument.uri, diagnostics });
}

// === FORMATTING ===

connection.onDocumentFormatting((params: DocumentFormattingParams): TextEdit[] => {
  const document = documents.get(params.textDocument.uri);
  if (!document) {
    return [];
  }
  const text = document.getText();
  const formatted = formatSource(text);
  if (formatted === null || formatted === text) {
    return [];
  }
  return [
    TextEdit.replace(
      { start: document.positionAt(0), end: document.positionAt(text.length) },
      formatted
    )
  ];
});

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
      regex: /\b(chist|è|tien|facc|piglie|si|sinnò|sinno|mentre|che|pe|ogni|rompe|salta|vir|caso|na|famiglie|costruttore|nu|bell|figlio|fisso|stu|cos|chiamm|da|mann|for|predefinit|mo|aspett|caccia|pruvamm|schiatta|aropp|iett|stamm|dì|di|scrive|avvis|fermete|leva|overo|sfòls|sfols|nisciun|boh|e|o|no|manco|pure)\b/g,
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
