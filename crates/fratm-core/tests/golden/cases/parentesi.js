class Pizza {
  constructor(nome) {
    this.nome = nome;
  }

  descrivi() {
    return "Pizza " + this.nome;
  }
}

console.log(new Pizza("margherita").descrivi());
console.log(new Pizza("marinara").nome);

function fabbrica() {
  return Pizza;
}

console.log(new (fabbrica())("diavola").nome);
({ conto: 3 }.conto);
const doppio = () => ({ n: 2 }.n * 2);
console.log(doppio());
console.log((-2) ** 2, 2 ** -1);
console.log((5).toString() + "!", (1.25).toFixed(1));
