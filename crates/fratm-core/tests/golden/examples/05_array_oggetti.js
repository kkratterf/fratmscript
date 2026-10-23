const pizze = ["Margherita", "Marinara", "Diavola", "Quattro Formaggi", "Capricciosa"];
console.log("=== Pizza Menu ===");
for (let i = 0; (i < 5); i = (i + 1)) {
  console.log((((i + 1) + ". ") + pizze[i]));
}
const voti = [8, 9, 7, 10, 6, 9];
function calcolaMedia(arr) {
  let somma = 0;
  for (let i = 0; (i < 6); i = (i + 1)) {
    somma = (somma + arr[i]);
  }
  return (somma / 6);
}
console.log("\n=== Grade Average ===");
console.log(((((((((((("Grades: " + voti[0]) + ", ") + voti[1]) + ", ") + voti[2]) + ", ") + voti[3]) + ", ") + voti[4]) + ", ") + voti[5]));
console.log(("Average: " + calcolaMedia(voti)));
const persona = { nome: "Gennaro", cognome: "Esposito", età: 35, città: "Napoli" };
console.log("\n=== Person ===");
console.log(((("Name: " + persona.nome) + " ") + persona.cognome));
console.log(("Age: " + persona.età));
console.log(("City: " + persona.città));
const ristorante = { nome: "Trattoria da Nonna", indirizzo: "Via Toledo 123", stelleMichelin: 0, specialità: "Neapolitan Ragù" };
console.log("\n=== Restaurant ===");
console.log(("🍝 " + ristorante.nome));
console.log(("📍 " + ristorante.indirizzo));
console.log(("⭐ Michelin Stars: " + ristorante.stelleMichelin));
console.log(("👨‍🍳 Specialty: " + ristorante.specialità));
const chiave = "nome";
console.log("\n=== Dynamic Access ===");
console.log(((("Value of '" + chiave) + "': ") + ristorante[chiave]));
const squadra = [{ nome: "Diego", ruolo: "Forward", numero: 10 }, { nome: "Ciro", ruolo: "Midfielder", numero: 8 }, { nome: "Paolo", ruolo: "Defender", numero: 3 }];
console.log("\n=== Team ===");
console.log((((((squadra[0].nome + " - ") + squadra[0].ruolo) + " (#") + squadra[0].numero) + ")"));
console.log((((((squadra[1].nome + " - ") + squadra[1].ruolo) + " (#") + squadra[1].numero) + ")"));
console.log((((((squadra[2].nome + " - ") + squadra[2].ruolo) + " (#") + squadra[2].numero) + ")"));
