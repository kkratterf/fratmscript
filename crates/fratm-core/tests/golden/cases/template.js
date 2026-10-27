const nome = "Gennaro";
let pizze = 2;
console.log(`Uè ${nome}!`);
console.log(
  `${pizze > 1 ? `${pizze} pizze` : "una pizza"} pe ${nome.toUpperCase()}`,
);
console.log(
  `riga uno
riga due: \` e \${nun è interpolato}`,
);
console.log(`${{ prezzo: 7 }.prezzo} euro`, ``);
