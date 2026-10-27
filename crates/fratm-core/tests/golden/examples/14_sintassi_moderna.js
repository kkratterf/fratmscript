const saluta = (nome = "Gennà") => `Uè ${nome}, comme staje?`;
console.log(saluta());
console.log(saluta("Totò"));
const nome = "Margherita";
const campo = "prezzo";
const pizza = {
  nome,
  [campo]: 6.5,
  ingredienti: ["pummarola", "muzzarella", "basilico"],
  descrivi() {
    return `${this.nome}: ${this.ingredienti.join(", ")}`;
  },
};
console.log(pizza.descrivi());
const { prezzo: costo, forno = "a legna", ...altro } = pizza;
const [primo, ...ato] = pizza.ingredienti;
console.log(
  `${costo} euro, forno ${forno}, primo: ${primo}, ato: ${ato.length}`,
);

function somma(...numeri) {
  return numeri.reduce((totale, n) => totale + n, 0);
}

const prezzi = [6.5, 8, 7];
console.log("Totale: " + somma(...prezzi, 1.5));
const ordine = { cliente: { nome: "Ciro" } };
console.log(ordine.cliente?.nome ?? "anonimo");
console.log(ordine.consegna?.indirizzo ?? "ritiro al banco");
let conto = 0, pizze = 0;

for (let i = 0; i < prezzi.length; i++) {
  conto += prezzi[i];
  pizze++;
}

console.log(`${pizze} pizze, ${conto} euro`);

try {
  throw new Error("forno spento");
} catch (e) {
  console.log("Errore: " + e.message);
}

const targa = /^[A-Z]{2}\d{3}[A-Z]{2}$/;
console.log(targa.test("NA123BC"), targa.test("pizza"));
