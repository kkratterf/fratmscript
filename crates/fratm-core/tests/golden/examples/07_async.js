async function caricaDati(url) {
  console.log("Loading from: " + url);
  const risposta = await fetch(url);
  const dati = await risposta.json();
  return dati;
}

function dividi(a, b) {
  try {
    if (b == 0) {
      throw new Error("Cannot divide by zero!");
    }
    return a / b;
  } catch (errore) {
    console.log("Error: " + errore.message);
    return null;
  }
}

console.log("=== Safe Division ===");
console.log("10 / 2 = " + dividi(10, 2));
console.log("10 / 0 = " + dividi(10, 0));

async function fetchUtente(id) {
  try {
    const url = "https://api.example.com/users/" + id;
    const dati = await caricaDati(url);
    return dati;
  } catch (err) {
    console.log("Unable to load user: " + err.message);
    return null;
  }
}
