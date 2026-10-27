function somma(primo, ...resto) {
  return resto.reduce((totale, n) => totale + n, primo);
}

function saluta(nome = "Gennà", punto = nome.length > 4 ? "!" : ".") {
  return "Uè " + nome + punto;
}

const conta = (base = 10, ...altri) => base + altri.length;
const a = [1, 2];
const b = [...a, 3, ...a];
const o = { x: 1 };
console.log(b.join(","), somma(...b), JSON.stringify({ ...o, y: 2 }));
console.log(saluta(), saluta("Totò"), conta(), conta(1, 2, 3));
console.log(Math.max(...b, 0));
