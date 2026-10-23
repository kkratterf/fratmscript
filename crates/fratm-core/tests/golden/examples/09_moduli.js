import { basename, dirname } from "node:path";
import { platform } from "node:os";
function descriviPercorso(percorso) {
  return ((("file=" + basename(percorso)) + ", cartella=") + dirname(percorso));
}
export function calcolaIVA(prezzo) {
  return (prezzo * 1.22);
}
export default { nome: "ModuloDemo", versione: "1.1", ambiente: platform() };
console.log(descriviPercorso("/tmp/fratm/esempio.js"));
console.log(("IVA 100 => " + calcolaIVA(100)));
