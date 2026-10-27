class Pizzaiolo {
  constructor(nome, specialità) {
    this.nome = nome;
    this.specialità = specialità;
    this.pizzeFatte = 0;
  }

  faiPizza(tipo) {
    this.pizzeFatte = this.pizzeFatte + 1;
    return this.nome + " made a " + tipo + "!";
  }

  presentati() {
    return "I am " + this.nome + ", specialty: " + this.specialità;
  }
}

const gennaro = new Pizzaiolo("Gennaro", "Margherita DOC");
console.log(gennaro.presentati());
console.log(gennaro.faiPizza("Marinara"));
console.log(gennaro.faiPizza("Diavola"));
console.log("Pizzas made: " + gennaro.pizzeFatte);
