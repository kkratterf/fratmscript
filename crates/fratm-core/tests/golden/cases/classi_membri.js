class Pizzeria {
  #incasso = 0;
  static aperte = 0;
  nomme;
  menu = ["Margherita", "Marinara"];

  constructor(nomme) {
    this.nomme = nomme;
    Pizzeria.aperte++;
  }

  get incasso() {
    return this.#incasso + "€";
  }

  set incasso(euro) {
    if (euro < 0) {
      throw new Error("l'incasso nun po' scennere sotto zero");
    }
    this.#incasso = euro;
  }

  vende(pizza) {
    this.#incasso += this.#prezzo(pizza);
  }

  #prezzo(pizza) {
    return pizza == "Margherita" ? 6 : 5;
  }

  static get quante() {
    return Pizzeria.aperte;
  }
}

const daMichele = new Pizzeria("Da Michele");
daMichele.vende("Margherita");
daMichele.vende("Marinara");
console.log(daMichele.nomme, daMichele.incasso, daMichele.menu.length);
daMichele.incasso = 100;
console.log(daMichele.incasso);

try {
  daMichele.incasso = -5;
} catch (e) {
  console.log(e.message);
}

new Pizzeria("Sorbillo");
console.log("pizzerie aperte:", Pizzeria.quante);
