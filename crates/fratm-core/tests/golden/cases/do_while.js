let n = 0;

do {
  n++;
  console.log("giro", n);
} while (n < 3);

do {
  console.log("na vota sola");
} while (false);

let k = 0;

do {
  k++;
  if (k == 2) {
    continue;
  }
  if (k == 4) {
    break;
  }
  console.log("k =", k);
} while (true);

let primm = "primm", fa = "fa";
console.log(primm, fa);
