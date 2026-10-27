let readline = require("readline");
let rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});
let tavola = [" ", " ", " ", " ", " ", " ", " ", " ", " "];
let giocatore_attuale = "P";

function nome_giocatore(simbolo) {
  if (simbolo == "P") {
    return "Pomodor";
  }
  return "Muzzarel";
}

function simbolo_casella(indice) {
  if (tavola[indice] == " ") {
    return "" + (indice + 1);
  }
  return tavola[indice];
}

function stampa_tavola() {
  console.log("");
  console.log(
    " " +
      simbolo_casella(0) +
      " | " +
      simbolo_casella(1) +
      " | " +
      simbolo_casella(2),
  );
  console.log("---+---+---");
  console.log(
    " " +
      simbolo_casella(3) +
      " | " +
      simbolo_casella(4) +
      " | " +
      simbolo_casella(5),
  );
  console.log("---+---+---");
  console.log(
    " " +
      simbolo_casella(6) +
      " | " +
      simbolo_casella(7) +
      " | " +
      simbolo_casella(8),
  );
  console.log("");
}

function casella_libera(indice) {
  return tavola[indice] == " ";
}

function linea_vincente(a, b, c, simbolo) {
  return tavola[a] == simbolo && tavola[b] == simbolo && tavola[c] == simbolo;
}

function ha_vinto(simbolo) {
  if (
    linea_vincente(0, 1, 2, simbolo) ||
    linea_vincente(3, 4, 5, simbolo) ||
    linea_vincente(6, 7, 8, simbolo) ||
    linea_vincente(0, 3, 6, simbolo) ||
    linea_vincente(1, 4, 7, simbolo) ||
    linea_vincente(2, 5, 8, simbolo) ||
    linea_vincente(0, 4, 8, simbolo) ||
    linea_vincente(2, 4, 6, simbolo)
  ) {
    return true;
  }
  return false;
}

function e_pareggio() {
  for (let i = 0; i < 9; i = i + 1) {
    if (tavola[i] == " ") {
      return false;
    }
  }
  return true;
}

function cambia_giocatore() {
  if (giocatore_attuale == "P") {
    giocatore_attuale = "M";
    return null;
  }
  giocatore_attuale = "P";
}

function mostra_titolo() {
  console.log("====================================");
  console.log("      POMODOR E MUZZAREL");
  console.log("   Tic Tac Toe napulitan edition");
  console.log("====================================");
  console.log("Pomodor = P | Muzzarel = M");
}

function turno() {
  stampa_tavola();
  console.log(
    "Turno 'e " +
      nome_giocatore(giocatore_attuale) +
      " (" +
      giocatore_attuale +
      ")",
  );
  rl.question("Scegli na casella (1-9): ", (risposta) => {
    let mossa = parseInt(risposta, 10);
    if (isNaN(mossa)) {
      console.warn("Mossa nun valida. Miette nu numero tra 1 e 9.");
      turno();
      return null;
    }
    let indice = mossa - 1;
    if (indice < 0 || indice > 8) {
      console.warn("Casella fora campo. Scegli tra 1 e 9.");
      turno();
      return null;
    }
    if (!casella_libera(indice)) {
      console.warn("Chest casella è già presa.");
      turno();
      return null;
    }
    tavola[indice] = giocatore_attuale;
    if (ha_vinto(giocatore_attuale)) {
      stampa_tavola();
      console.log(
        "Ha vinto " + nome_giocatore(giocatore_attuale) + "! Brav fratm!",
      );
      rl.close();
      return null;
    }
    if (e_pareggio()) {
      stampa_tavola();
      console.log("Pareggio! Pizza margherita pe tutte e dduje.");
      rl.close();
      return null;
    }
    cambia_giocatore();
    turno();
  });
}

mostra_titolo();
turno();
