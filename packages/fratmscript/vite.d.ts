export interface FratmPluginOptions {
  /** Emit source maps for compiled files (default: true). */
  sourceMap?: boolean;
  /** Library whose JSX runtime compiles JSX in `.fratm` files (default: "react"). */
  jsxImportSource?: string;
  /**
   * Serve an API from the dev server: a module (e.g. "server/api.fratm") whose
   * default export has `fetch(request)`, such as a Hono app. It answers the
   * requests under `prefix` (default "/api").
   */
  api?: string | { entry: string; prefix?: string };
}

/** A Vite/Rollup plugin; typed loosely so it does not depend on vite's types. */
export interface FratmPlugin {
  name: string;
  enforce: 'pre';
  configureServer(server: {
    middlewares: { use(fn: (req: any, res: any, next: (error?: unknown) => void) => void): void };
    ssrLoadModule(url: string): Promise<Record<string, any>>;
    ssrFixStacktrace?(error: Error): void;
  }): void;
  transform(this: unknown, source: string, id: string): { code: string; map: unknown } | null;
}

export default function fratmscript(options?: FratmPluginOptions): FratmPlugin;
export { fratmscript };
