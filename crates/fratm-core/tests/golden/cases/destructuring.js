const ordine = {
  cliente: "Ciro",
  pizze: ["Margherita", "Marinara", "Diavola"],
  dettagli: { forno: "a legna" },
  mancia: 2,
};
const { cliente, dettagli: { forno }, consegna = "al banco", ...resto } = ordine;
let [prima, , terza = "nisciuna", ...ato] = ordine.pizze;
console.log(cliente, forno, consegna, JSON.stringify(resto));
console.log(prima, terza, ato.length);

function descrivi({ cliente, pizze: [prima] }, [x, y] = [1, 2]) {
  return cliente + " vo' " + prima + " " + (x + y);
}

const conta = ({ pizze }) => pizze.length;
console.log(descrivi(ordine), conta(ordine));

try {
  throw new Error("forno spento");
} catch ({ message }) {
  console.log("Errore: " + message);
}

for (let [i, j] = [0, 3]; i < j; i++) {
  console.log(i);
}
