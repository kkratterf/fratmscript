console.log("=== Neapolitan Countdown ===");
let countdown = 5;
while ((countdown > 0)) {
  console.log((countdown + "..."));
  countdown--;
}
console.log("🎆 Happy New Year!");
console.log("\n=== Multiplication Table of 7 ===");
for (let i = 1; (i <= 10); i++) {
  console.log(((("7 x " + i) + " = ") + (7 * i)));
}
console.log("\n=== Find the number (break) ===");
for (let n = 1; (n <= 100); n++) {
  if ((n == 42)) {
    console.log(("Found it! The number is " + n));
    break;
  }
}
console.log("\n=== Odd numbers (continue) ===");
for (let j = 1; (j <= 10); j++) {
  if (((j % 2) == 0)) {
    continue;
  }
  console.log(j);
}
console.log("\n=== Pattern (nested loops) ===");
for (let riga = 1; (riga <= 5); riga++) {
  let stelle = "";
  for (let col = 1; (col <= riga); col++) {
    stelle += "⭐";
  }
  console.log(stelle);
}
console.log("\n=== Fibonacci ===");
let fib1 = 0;
let fib2 = 1;
let fibCount = 0;
while ((fibCount < 10)) {
  console.log(fib1);
  let temp = (fib1 + fib2);
  fib1 = fib2;
  fib2 = temp;
  fibCount = (fibCount + 1);
}
