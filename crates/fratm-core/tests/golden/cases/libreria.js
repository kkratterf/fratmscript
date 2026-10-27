const a_caso = (min, max) => Math.floor(Math.random() * (max - min + 1)) + min;

const quanti = (cosa) =>
  cosa == null ? 0 : (cosa.size ?? cosa.length ?? Object.keys(cosa).length);

const arrotonda = (nummero, cifre = 0) =>
  Math.round(nummero * 10 ** cifre) / 10 ** cifre;

const ripeti = (volte, fa) => {
  for (let i = 0; i < volte; i++) {
    fa(i);
  }
};

const mischia = (lista) => {
  const copia = [...lista];
  for (let i = copia.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [copia[i], copia[j]] = [copia[j], copia[i]];
  }
  return copia;
};

const uno_a_caso = (lista) => lista[Math.floor(Math.random() * lista.length)];

const aspetta = (ms) => new Promise((risolvi) => setTimeout(risolvi, ms));

const pizze = ["margherita", "marinara", "diavola"];
console.log(quanti(pizze), quanti("Napule"), quanti({ a: 1, b: 2 }));
console.log(a_caso(7, 7), arrotonda(3.14159, 2), arrotonda(2.5));
ripeti(3, (giro) => console.log("giro", giro));
const mischiate = mischia(pizze);
console.log(
  quanti(mischiate),
  mischiate.every((p) => pizze.includes(p)),
  pizze[0],
);
console.log(pizze.includes(uno_a_caso(pizze)));

async function main() {
  await aspetta(10);
  console.log("fatto");
}

main();
