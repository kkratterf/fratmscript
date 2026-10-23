function validaEta(eta) {
  if ((eta < 0)) {
    throw new Error("L'eta nun po' essere negativa");
  }
  if ((eta > 130)) {
    throw new Error("L'eta è troppo alta pe' essere vera");
  }
  return ("Eta valida: " + eta);
}
function provaValore(eta) {
  try {
    console.log(validaEta(eta));
  } catch (err) {
    console.warn(((("Valore " + eta) + " scartato: ") + err.message));
  }
}
console.log("=== Validazione eta ===");
provaValore(35);
provaValore(-2);
provaValore(200);
provaValore(90);
