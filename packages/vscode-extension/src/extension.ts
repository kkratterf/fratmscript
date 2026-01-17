/**
 * FratmScript VSCode Extension
 *
 * Entry point dell'estensione. Avvia il Language Server
 * e gestisce la comunicazione con VSCode.
 */

import * as path from 'path';
import { workspace, ExtensionContext } from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext): void {
  // Path al modulo del server
  const serverModule = context.asAbsolutePath(
    path.join('out', 'server', 'server.js')
  );

  // Opzioni di debug per il server
  const debugOptions = { execArgv: ['--nolazy', '--inspect=6009'] };

  // Opzioni del server per run e debug
  const serverOptions: ServerOptions = {
    run: {
      module: serverModule,
      transport: TransportKind.ipc
    },
    debug: {
      module: serverModule,
      transport: TransportKind.ipc,
      options: debugOptions
    }
  };

  // Opzioni del client
  const clientOptions: LanguageClientOptions = {
    // Registra il server per documenti FratmScript
    documentSelector: [{ scheme: 'file', language: 'fratmscript' }],
    synchronize: {
      // Notifica il server dei cambi ai file .fratm
      fileEvents: workspace.createFileSystemWatcher('**/*.fratm')
    }
  };

  // Crea e avvia il client
  client = new LanguageClient(
    'fratmscriptLanguageServer',
    'FratmScript Language Server',
    serverOptions,
    clientOptions
  );

  // Avvia il client (che a sua volta avvia il server)
  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
