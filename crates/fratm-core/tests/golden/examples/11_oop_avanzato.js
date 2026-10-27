class Persona {
  constructor(nome) {
    this.nome = nome;
  }

  saluta() {
    return "Ue, so' " + this.nome;
  }

  static categoria() {
    return "Essere umano";
  }
}

class Pizzaiolo extends Persona {
  constructor(nome, specialita) {
    super(nome);
    this.specialita = specialita;
    this.pizzeFatte = 0;
  }

  faiPizza(tipo) {
    this.pizzeFatte = this.pizzeFatte + 1;
    return this.nome + " ha sfornato na " + tipo + "!";
  }

  saluta() {
    return super.saluta() + " e faccio " + this.specialita;
  }

  static salutoDiClasse() {
    return "Nuje simm' pizzaiuoli";
  }
}

const gennaro = new Pizzaiolo("Gennaro", "Margherita");
console.log(gennaro.saluta());
console.log(gennaro.faiPizza("Marinara"));
console.log(gennaro.faiPizza("Diavola"));
console.log("Pizze fatte: " + gennaro.pizzeFatte);
console.log("Categoria: " + Persona.categoria());
console.log("Static subclass: " + Pizzaiolo.salutoDiClasse());

class Forno {
  #temperatura = 20;
  static forniAccesi = 0;
  pizzeInForno = [];

  get temperatura() {
    return this.#temperatura + "°C";
  }

  set temperatura(gradi) {
    this.#temperatura = this.#limita(gradi);
  }

  #limita(gradi) {
    return Math.min(gradi, 485);
  }
}

const forno = new Forno();
forno.temperatura = 900;
console.log("Forno: " + forno.temperatura);

class Menu {
  static pizze = [];

  static {
    Menu.pizze.push("Margherita", "Marinara");
  }
}

console.log("Menu: " + Menu.pizze.join(", "));
const Ricetta = class {
  constructor(nome) {
    this.nome = nome;
  }
};
let grammi = 250;
const impasto = {
  get chili() {
    return grammi / 1000;
  },
  set chili(kg) {
    grammi = kg * 1000;
  },
};
impasto.chili = 2;
const ricetta = new Ricetta("Pizza fritta");
console.log(ricetta.nome + ": " + impasto.chili + " kg");
