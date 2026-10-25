export interface FratmPluginOptions {
  /** Emit source maps for compiled files (default: true). */
  sourceMap?: boolean;
  /** Library whose JSX runtime compiles JSX in `.fratm` files (default: "react"). */
  jsxImportSource?: string;
}

/** A Vite/Rollup plugin; typed loosely so it does not depend on vite's types. */
export interface FratmPlugin {
  name: string;
  enforce: 'pre';
  transform(this: unknown, source: string, id: string): { code: string; map: unknown } | null;
}

export default function fratmscript(options?: FratmPluginOptions): FratmPlugin;
export { fratmscript };
