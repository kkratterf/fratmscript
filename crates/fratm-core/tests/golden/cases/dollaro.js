const $ = (sel) => `truvato ${sel}`;
let $prezzo = 5;
const carrello = { $totale: $prezzo * 2 };
console.log($("#pizza"), $prezzo, carrello.$totale);
