let r = /ab+/g, s = "xabbbyab";
const targa = /^[A-Z]{2}\d{3}[A-Z]{2}$/, barre = /[/]x\//;
console.log(s.match(r).join(","), targa.test("NA123BC"), barre.test("/x/"));
console.log(s.replace(/b/gi, "c"));
let a = 10, b = 4;
console.log(a / b, [8, 2][0] / 2, (a + 2) / 3);
let c, d = 1;

for (let i = 0, j = 3; i < j; i++, j--) {
  d *= 2;
}

console.log(c, d);
