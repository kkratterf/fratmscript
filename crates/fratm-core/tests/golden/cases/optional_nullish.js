const ordine = {
  cliente: { nome: "Ciro" },
  righe: [{ pizza: "Margherita" }],
  totale() {
    return 12;
  },
};
const vuoto = null;
console.log(ordine.cliente?.nome, vuoto?.cliente.nome);
console.log(ordine.righe?.[0]?.pizza, vuoto?.[0]);
console.log(ordine.totale?.(), ordine.sconto?.());
console.log(vuoto ?? "default", 0 ?? 1, "" ?? "pieno", ordine.mancia ?? 0);
console.log(vuoto ?? (false || true));
let x = vuoto ? 1 : 2;
console.log(x);
