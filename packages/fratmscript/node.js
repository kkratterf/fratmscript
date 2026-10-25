// Lets Node.js run .fratm files: `node --import fratmscript/node main.fratm`.
// Registers module hooks that compile each .fratm file when it is loaded.
import { register } from 'node:module';

register('./lib/node-hooks.js', import.meta.url);
// Stack traces point at the .fratm lines through the inline source maps
process.setSourceMapsEnabled(true);
