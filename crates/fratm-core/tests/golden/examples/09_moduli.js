import { basename, dirname } from "node:path";
import os from "node:os";
import * as percorsi from "node:path";

const dado = (min, max) => Math.floor(Math.random() * (max - min + 1)) + min;

function descriviPercorso(percorso) {
  return "file=" + basename(percorso) + ", cartella=" + dirname(percorso);
}

export function calcolaIVA(prezzo) {
  return prezzo * 1.22;
}

const aliquota = 22;
export { aliquota, descriviPercorso as descrivi };
export default { nome: "ModuloDemo", versione: "1.2", ambiente: os.platform() };

async function estensione(file) {
  const path = await import("node:path");
  return path.extname(file);
}

console.log(descriviPercorso("/tmp/fratm/esempio.js"));
console.log("IVA 100 => " + calcolaIVA(100));
console.log("Separatore: " + percorsi.sep);
console.log("Dado tra 1 e 1: " + dado(1, 1));
estensione("pizza.fratm").then((est) => console.log("Estensione: " + est));
