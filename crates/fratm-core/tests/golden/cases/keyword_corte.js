try {
  throw new Error("guaio");
} catch (e) {
  console.log(e.message);
}
let o = 1;
let da = 2;
console.log((o + da));
console.log((true && false));
console.log((true || false));
