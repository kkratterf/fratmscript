const zero = () => 42;
const somma = (a, b) => a + b;
const doppio = (x) => x * 2;
const blocco = (a, b, c) => {
  return a * b + c;
};
console.log(zero());
console.log(somma(2, 3));
console.log(doppio(21));
console.log(blocco(2, 3, 4));
console.log([1, 2, 3].map((v, i) => v * i).join(","));
