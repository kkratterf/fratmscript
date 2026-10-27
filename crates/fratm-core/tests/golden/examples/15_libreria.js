const a_caso = (min, max) => Math.floor(Math.random() * (max - min + 1)) + min;

const uno_a_caso = (lista) => lista[Math.floor(Math.random() * lista.length)];

const mischia = (lista) => {
  const copia = [...lista];
  for (let i = copia.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [copia[i], copia[j]] = [copia[j], copia[i]];
  }
  return copia;
};

const quanti = (cosa) =>
  cosa == null ? 0 : (cosa.size ?? cosa.length ?? Object.keys(cosa).length);

const arrotonda = (nummero, cifre = 0) =>
  Math.round(nummero * 10 ** cifre) / 10 ** cifre;

const ripeti = (volte, fa) => {
  for (let i = 0; i < volte; i++) {
    fa(i);
  }
};

const aspetta = (ms) => new Promise((risolvi) => setTimeout(risolvi, ms));

const pizze = ["margherita", "marinara", "diavola", "capricciosa"];
console.log("Ncopp'o menù ce stanno", quanti(pizze), "pizze");
console.log("Stasera me magno na", uno_a_caso(pizze));
console.log("Ordine d'a cucina:", mischia(pizze).join(", "));
ripeti(3, (giro) => {
  console.log(`Estrazione ${giro + 1}: ${a_caso(1, 90)}`);
});
const conto = 4 * 6.5 / 3;
console.log("A testa:", arrotonda(conto, 2), "euro");

async function cucina() {
  console.log("'A pizza sta int'o furno…");
  await aspetta(300);
  console.log("Pronta! 🍕");
}

cucina();
