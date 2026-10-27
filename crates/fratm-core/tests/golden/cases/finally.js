function dividi(a, b) {
  try {
    if (b == 0) {
      throw new Error("divisione pe' zero");
    }
    return a / b;
  } catch (err) {
    console.log("guaio:", err.message);
    return null;
  } finally {
    console.log("aggio diviso", a, "pe'", b);
  }
}

console.log(dividi(10, 2));
console.log(dividi(1, 0));

function apri() {
  try {
    return "aperto";
  } finally {
    console.log("chiuso");
  }
}

console.log(apri());
let aropp = "dopo";
console.log(aropp);
