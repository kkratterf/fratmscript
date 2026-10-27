const prezzo = 8;
const tasse = 2;
const totale = prezzo + tasse;
console.log(totale);
const cond = true;
const x = cond ? "sì" : "no";
console.log(x);
const persone = [{ nome: "Ciro", età: 30 }, { nome: "Totò", età: 12 }];
const nomi = persone.filter((p) => p.età > 18).map((p) => p.nome);
console.log(nomi);
const ok = prezzo > 5 && tasse < 3;
const ok2 = (prezzo > 5 && tasse < 3) || false;
console.log(ok, ok2);
const somma = 1 + 2 + 3;
console.log(somma);
const doppio = (x) => x * 2;
console.log(doppio(4));

if (prezzo > 1 && tasse > 1) {
  console.log("entrambi");
}

const e = 1;
console.log(e);
let y = 1;
y += 2;
console.log(y);

switch (y) {
  case 3:
    console.log("tre");
    break;
  default:
    console.log("altro");
}

const valore = null ?? "riserva";
console.log(valore);
const lunghezze = persone
  .map((p) => {
    return p.nome.length;
  })
  .filter((n) => n > 3);
console.log(lunghezze);
