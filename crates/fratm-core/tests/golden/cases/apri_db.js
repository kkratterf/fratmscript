const apri_db = (file = ":memory:") => { const sqlite = globalThis.process?.getBuiltinModule?.("node:sqlite"); if (!sqlite) throw new Error("apri_db vo' Node 22.13 o cchiù nuovo (node:sqlite): int'o browser nun se pò aprì nu database"); const db = new sqlite.DatabaseSync(file); const prepara = (q, valori) => typeof q === "string" ? [db.prepare(q), valori] : [db.prepare(q.sql), q.params]; return { ripiglia: (q, ...valori) => { const [s, p] = prepara(q, valori); return s.all(...p).map((riga) => ({ ...riga })); }, ripiglia_uno: (q, ...valori) => { const [s, p] = prepara(q, valori); const riga = s.get(...p); return riga === undefined ? null : { ...riga }; }, esegui: (q, ...valori) => { const [s, p] = prepara(q, valori); const r = s.run(...p); return { cambiate: Number(r.changes), ultimo_id: Number(r.lastInsertRowid) }; }, chiude: () => db.close() }; };
const gomorra = (sql, text, params) => ({ sql, params, text, values: [...params] });
const db = apri_db(":memory:");
db.esegui(gomorra("CREATE TABLE IF NOT EXISTS clienti (id INTEGER PRIMARY KEY, nomme TEXT, pizze INTEGER)", "CREATE TABLE IF NOT EXISTS clienti (id INTEGER PRIMARY KEY, nomme TEXT, pizze INTEGER)", []));
console.log(db.esegui(gomorra("INSERT INTO clienti (nomme, pizze) VALUES (?, ?)", "INSERT INTO clienti (nomme, pizze) VALUES ($1, $2)", ["Gennaro", 3])));
console.log(db.esegui(gomorra("INSERT INTO clienti (nomme, pizze) VALUES (?, ?)", "INSERT INTO clienti (nomme, pizze) VALUES ($1, $2)", ["Assunta", 5])));
console.log(db.ripiglia(gomorra("SELECT nomme FROM clienti WHERE pizze > ? ORDER BY pizze DESC", "SELECT nomme FROM clienti WHERE pizze > $1 ORDER BY pizze DESC", [2])));
console.log(db.ripiglia_uno(gomorra("SELECT COUNT(*) AS quanti FROM clienti", "SELECT COUNT(*) AS quanti FROM clienti", [])));
console.log(db.ripiglia_uno(gomorra("SELECT nomme FROM clienti WHERE id = ?", "SELECT nomme FROM clienti WHERE id = $1", [99])));
console.log(db.ripiglia("SELECT nomme FROM clienti WHERE pizze = ?", 5));
console.log(db.esegui(gomorra("DELETE FROM clienti WHERE pizze < ?", "DELETE FROM clienti WHERE pizze < $1", [4])));
db.chiude();
process.getBuiltinModule = undefined;
try {
  apri_db();
} catch (e) {
  console.log(e.message);
}
