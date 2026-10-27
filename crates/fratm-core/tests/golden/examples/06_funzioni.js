function somma(a, b) {
  return a + b;
}

const moltiplica = (a) => a * 2;
console.log("=== Basic Functions ===");
console.log("somma(3, 4) = " + somma(3, 4));
console.log("moltiplica(5) = " + moltiplica(5));

function calcolaTotale(prezzo, quantità, sconto) {
  const subtotale = prezzo * quantità;
  const scontoApplicato = subtotale * (sconto / 100);
  return subtotale - scontoApplicato;
}

console.log("\n=== Calculate Total ===");
console.log("Price: €10, Quantity: 5, Discount: 15%");
console.log("Total: €" + calcolaTotale(10, 5, 15));

function fattoriale(n) {
  if (n <= 1) {
    return 1;
  }
  return n * fattoriale(n - 1);
}

console.log("\n=== Factorial (Recursion) ===");
console.log("5! = " + fattoriale(5));
console.log("7! = " + fattoriale(7));

function fibonacci(n) {
  if (n <= 1) {
    return n;
  }
  return fibonacci(n - 1) + fibonacci(n - 2);
}

console.log("\n=== Fibonacci (Recursion) ===");

for (let i = 0; i < 10; i = i + 1) {
  console.log("fib(" + i + ") = " + fibonacci(i));
}

function creaMultiplicatore(fattore) {
  return (x) => x * fattore;
}

const raddoppia = creaMultiplicatore(2);
const triplica = creaMultiplicatore(3);
console.log("\n=== Higher Order Functions ===");
console.log("raddoppia(5) = " + raddoppia(5));
console.log("triplica(5) = " + triplica(5));

function salutaConStile(nome, stile) {
  if (stile == undefined) {
    stile = "normal";
  }
  if (stile == "formal") {
    return "Good day, Mr. " + nome;
  } else   if (stile == "napoletano") {
    return "Uè " + nome + ", comm staje?!";
  } else {
    return "Hello " + nome;
  }
}

console.log("\n=== Greetings with Style ===");
console.log(salutaConStile("Mario", "formal"));
console.log(salutaConStile("Gennaro", "napoletano"));
console.log(salutaConStile("Luigi", undefined));
console.log("\n=== Pizza queue (generator) ===");

function* fornata(pizze) {
  let numero = 1;
  for (const pizza of pizze) {
    yield numero + ". " + pizza;
    numero++;
  }
}

for (const turno of fornata(["Margherita", "Capricciosa"])) {
  console.log(turno);
}
