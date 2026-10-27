const nome = "Margherita";
const campo = "prezzo";
let $for = "riservato";
const pizza = {
  nome,
  for: $for,
  [campo]: 6.5,
  ["tagl" + "ie"]: 8,
  "con spazi": true,
  si: "chiave keyword",
  descrivi() {
    return this.nome + " a " + this.prezzo;
  },
  doppia() {
    return this.prezzo * 2;
  },
  async sforna() {
    return await Promise.resolve(this.nome + " sfornata");
  },
};
console.log(
  pizza.nome,
  pizza.for,
  pizza.prezzo,
  pizza.taglie,
  pizza["con spazi"],
  pizza.si,
);
console.log(pizza.descrivi(), pizza.doppia(), JSON.stringify({}));
pizza.sforna().then((m) => console.log(m));
