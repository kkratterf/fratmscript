const Animale = class {
  constructor(nome) {
    this.nome = nome;
  }

  parla() {
    return this.nome + " fa versi";
  }
};
const Cane = class Bobi extends Animale {
  parla() {
    return this.nome + " abbaia (" + Bobi.name + ")";
  }
};
console.log(new Animale("Gatto").parla(), Animale.name);
console.log(new Cane("Rex").parla(), Cane.name);
console.log(
  (class {
    static ciao() {
      return "uè";
    }
  }).ciao(),
);

class Config {
  static valori = {};

  static {
    Config.valori.lingua = "napoletano";
    console.log("Config pronta");
  }
}

console.log(Config.valori.lingua);
let prezzoBase = 5;
const pizza = {
  nome: "margherita",
  get prezzo() {
    return prezzoBase + 3;
  },
  set prezzo(v) {
    prezzoBase = v - 3;
  },
  damme() {
    return "metodo chiamato damme";
  },
};
console.log(pizza.prezzo);
pizza.prezzo = 10;
console.log(pizza.prezzo, prezzoBase, pizza.damme());
const uno = 1;
const damme = 2;
console.log(uno + damme);
