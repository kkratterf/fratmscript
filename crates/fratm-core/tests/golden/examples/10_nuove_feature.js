console.log("=== Simplified For Loop ===");

for (let i = 1; i <= 5; i = i + 1) {
  console.log("Iteration " + i);
}

console.log("\n=== Logical Operators ===");
const valore1 = true;
const valore2 = false;

if (valore1 && valore1) {
  console.log("pure works as AND!");
}

if (!valore2) {
  console.log("manco works as NOT!");
}

if (!valore2) {
  console.log("! works directly!");
}

console.log("\n=== Advanced Console ===");
console.log("This is a normal log");
console.warn("This is a warning!");
console.error("This is an error!");
console.log("\n=== Delete (leva) ===");
const oggetto = { nome: "Gennaro", eta: 35, citta: "Napoli" };
console.log("Before: " + oggetto.citta);
delete oggetto.citta;
console.log("After leva: " + oggetto.citta);
console.log("\n=== Debugger ===");
console.log("The debugger is activated with 'fermete'");
console.log("\n=== Combinations ===");
const numeri = [1, 2, 3, 4, 5];
let somma = 0;

for (let idx = 0; idx < 5; idx = idx + 1) {
  if (!(numeri[idx] % 2 == 0)) {
    somma = somma + numeri[idx];
  }
}

console.log("Sum of odd numbers: " + somma);
console.log("\n=== New Syntax Summary ===");
console.log("pe (...)        -> for loop (ogni optional)");
console.log("pure            -> AND (alias for 'e')");
console.log("manco           -> NOT (alias for 'no')");
console.log("!               -> direct NOT");
console.log("avvis a di(...) -> console.warn");
console.log("scrive a di(...)-> console.error");
console.log("leva x.y        -> delete x.y");
console.log("fermete         -> debugger");
