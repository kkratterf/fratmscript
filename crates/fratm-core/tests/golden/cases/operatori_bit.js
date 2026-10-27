const o = { nome: "pizza" };
console.log("nome" in o, "prezzo" in o);
console.log(5 & 3, 5 | 3, 5 ^ 3, ~5, 1 << 4, -16 >> 2, -16 >>> 28);
console.log(1 | 2 == 2, 1 + 2 << 1, 3 & 1 in [0, 1]);
let f = 6;
f &= 3;
f |= 8;
f ^= 1;
f <<= 2;
f >>= 1;
f >>>= 1;
console.log(f);
console.log(void 0, typeof void 0);
let grande = 9007199254740993n;
console.log(grande + 1_000n, 0xFFn, typeof 10n);
console.log(+"42", -(-3), +(+"1"), ~~3.7);
let $void = 1;
console.log($void);

class Conto {
  #saldo = 0;

  static è_conto(x) {
    return #saldo in x;
  }
}

console.log(Conto.è_conto(new Conto()), Conto.è_conto({}));

for (const k in o) {
  console.log(k);
}

for (const x of [1]) {
  console.log(x);
}

const dint = 3;
console.log(dint);
