function* contaFino(n) {
  for (let i = 1; i <= n; i++) {
    yield i;
  }
}

console.log([...contaFino(4)]);

function* squadra() {
  yield "portiere";
  yield* ["difensore", "centrocampista"];
  yield* contaFino(2);
}

for (const ruolo of squadra()) {
  console.log(ruolo);
}

function* chiacchiera() {
  const nomme = yield "comme te chiamme?";
  yield `piacere, ${nomme}!`;
}

const c = chiacchiera();
console.log(c.next().value);
console.log(c.next("Totò").value);

class Pizzeria {
  menu = ["Margherita", "Marinara"];

  *pizze() {
    yield* this.menu;
  }
}

console.log([...new Pizzeria().pizze()].join(" e "));

async function* ordini() {
  yield await Promise.resolve("ordine 1");
  yield "ordine 2";
}

async function servi() {
  for await (const o of ordini()) {
    console.log("servito:", o);
  }
}

servi();
