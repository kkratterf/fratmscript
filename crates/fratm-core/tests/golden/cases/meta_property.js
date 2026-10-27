function Pizza() {
  if (new.target == undefined) {
    return "chiamata senza nu bell";
  }
  this.nome = "margherita";
}

console.log(Pizza());
console.log(new Pizza().nome);
const nw = { target: "Napoli" };
console.log(nw.target);
