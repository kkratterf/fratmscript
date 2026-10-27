const doppio = function (x) {
  return x * 2;
};
const fattoriale = function fatt(n) {
  if (n <= 1) {
    return 1;
  }
  return n * fatt(n - 1);
};
console.log(doppio(21), fattoriale(5), fattoriale.name);
const carica = async function () {
  return 7;
};
carica().then((v) => console.log("caricato", v));
const conta = function* () {
  yield 1;
  yield 2;
};
console.log([...conta()]);
const pizza = {
  nome: "margherita",
  descrivi: function () {
    return "Pizza " + this.nome;
  },
};
console.log(pizza.descrivi());
console.log(
  (function () {
    return "IIFE";
  })(),
);
[1, 2, 3].forEach(function (n) {
  console.log(n);
});

function fatt() {
  return "declaration still works";
}

console.log(fatt());
