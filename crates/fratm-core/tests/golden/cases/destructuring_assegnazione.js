let a = 1;
let b = 2;
[a, b] = [b, a];
console.log(a, b);
let x = 0;
let y = 1;

for (let i = 0; i < 10; i++) {
  [x, y] = [y, x + y];
}

console.log(x);
const punto = { x: 3, y: 4, z: 5 };
const copia = {};
let ato;
({ x: copia.x, y: copia.y, ...ato } = punto);
console.log(copia, ato);
let primo;
let resto;
[primo = "nisciuno", ...resto] = [undefined, 2, 3];
console.log(primo, resto);
let p;
let q;
[[p], { q }] = [[1], { q: 2 }];
console.log(p, q);
