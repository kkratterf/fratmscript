const squadra = ["Maradona", "Careca", "Alemão"];

for (const nomme of squadra) {
  console.log(nomme);
}

for (const [i, nomme] of squadra.entries()) {
  console.log(i + 1, nomme);
}

const maglie = { Maradona: 10, Careca: 9 };

for (let chi in maglie) {
  console.log(chi, "porta 'o", maglie[chi]);
}

let lettera;

for (lettera of "Napule") {
  continue;
}

console.log("ultima lettera:", lettera);

async function aspetta() {
  const risultati = [Promise.resolve("uno"), Promise.resolve("doje")];
  for await (const r of risultati) {
    console.log("arrivato", r);
  }
}

aspetta();
