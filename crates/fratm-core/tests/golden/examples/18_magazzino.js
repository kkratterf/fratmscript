const apri_db = (file = ":memory:") => {
  const sqlite = globalThis.process?.getBuiltinModule?.("node:sqlite");
  if (!sqlite) {
    throw new Error(
      "apri_db vo' Node 22.13 o cchiù nuovo (node:sqlite): int'o browser nun se pò aprì nu database",
    );
  }
  const db = new sqlite.DatabaseSync(file);
  const prepara = (q, valori) =>
    typeof q === "string"
      ? [db.prepare(q), valori]
      : [db.prepare(q.sql), q.params];
  return {
    ripiglia: (q, ...valori) => {
      const [s, p] = prepara(q, valori);
      return s.all(...p).map((riga) => ({ ...riga }));
    },
    ripiglia_uno: (q, ...valori) => {
      const [s, p] = prepara(q, valori);
      const riga = s.get(...p);
      return riga === undefined ? null : { ...riga };
    },
    esegui: (q, ...valori) => {
      const [s, p] = prepara(q, valori);
      const r = s.run(...p);
      return {
        cambiate: Number(r.changes),
        ultimo_id: Number(r.lastInsertRowid),
      };
    },
    chiude: () => db.close(),
  };
};

const gomorra = (sql, text, params) => ({
  sql,
  params,
  text,
  values: [...params],
});

const db = apri_db();
db.esegui(
  gomorra(
    "CREATE TABLE IF NOT EXISTS pizze (id INTEGER PRIMARY KEY, nomme TEXT NOT NULL, prezzo REAL)",
    "CREATE TABLE IF NOT EXISTS pizze (id INTEGER PRIMARY KEY, nomme TEXT NOT NULL, prezzo REAL)",
    [],
  ),
);
const menù = [
  ["Margherita", 5],
  ["Marinara", 4],
  ["Diavola", 7.5],
  ["Capricciosa", 8],
];

for (const [nomme, prezzo] of menù) {
  db.esegui(
    gomorra(
      "INSERT INTO pizze (nomme, prezzo) VALUES (?, ?)",
      "INSERT INTO pizze (nomme, prezzo) VALUES ($1, $2)",
      [nomme, prezzo],
    ),
  );
}

async function menùCaro(minimo) {
  return await db.ripiglia(
    gomorra(
      "SELECT nomme, prezzo FROM pizze WHERE prezzo >= ? ORDER BY prezzo DESC",
      "SELECT nomme, prezzo FROM pizze WHERE prezzo >= $1 ORDER BY prezzo DESC",
      [minimo],
    ),
  );
}

async function main() {
  for (const pizza of await menùCaro(6)) {
    console.log(`${pizza.nomme}: ${pizza.prezzo} euro`);
  }
  const cambio = db.esegui(
    gomorra(
      "UPDATE pizze SET prezzo = prezzo + 1 WHERE nomme = ?",
      "UPDATE pizze SET prezzo = prezzo + 1 WHERE nomme = $1",
      ["Marinara"],
    ),
  );
  console.log("Pizze cagnate:", cambio.cambiate);
  const marinara = db.ripiglia_uno(
    gomorra(
      "SELECT prezzo FROM pizze WHERE nomme = ?",
      "SELECT prezzo FROM pizze WHERE nomme = $1",
      ["Marinara"],
    ),
  );
  console.log("'A marinara mo costa", marinara.prezzo);
  const nisciuna = db.ripiglia_uno(
    gomorra(
      "SELECT id FROM pizze WHERE nomme = ?",
      "SELECT id FROM pizze WHERE nomme = $1",
      ["Hawaiana"],
    ),
  );
  console.log("Hawaiana:", nisciuna);
  db.chiude();
}

main();
