const saluto = "saluto";
class Squadra {
  #giocatori = [];
  ["sport"] = "pallone";
  static [(saluto + "Fisso")] = "Forza Napoli!";
  constructor(...giocatori) {
    this.#giocatori = giocatori;
  }
  *[Symbol.iterator]() {
    for (const nomme of this.#giocatori) {
      yield nomme;
    }
  }
  [saluto](chi) {
    return (("Uè " + chi) + "!");
  }
  get [(saluto + "Getter")]() {
    return (this.#giocatori.length + " giocatori");
  }
  set ["capitano"](nomme) {
    this.#giocatori.unshift(nomme);
  }
  static "nomme strano"() {
    return 10;
  }
}
const napoli = new Squadra("Careca", "Alemão");
napoli.capitano = "Maradona";
console.log([...napoli]);
console.log(napoli.saluto("Diego"), napoli.salutoGetter, napoli.sport);
console.log(Squadra.salutoFisso, Squadra["nomme strano"]());
