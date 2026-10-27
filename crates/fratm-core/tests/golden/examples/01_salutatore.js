const messaggio = "Uè, comme staje?";

function saluta(nome) {
  if (nome == null) {
    return "E chi si tu?";
  }
  return messaggio + " " + nome + "!";
}

console.log(saluta("Gennaro"));
console.log(saluta("Ciro"));
console.log(saluta("Totò"));
console.log(saluta(null));
