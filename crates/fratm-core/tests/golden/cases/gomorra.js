const sql = (sql, text, params) => ({
  sql,
  params,
  text,
  values: [...params],
});

function pizzePe(forno, quante) {
  return sql(
    "SELECT * FROM pizze WHERE forno = ? OR forno IS NULL LIMIT ? OFFSET ?",
    "SELECT * FROM pizze WHERE forno = $1 OR forno IS NULL LIMIT $2 OFFSET $3",
    [forno, quante, quante * 2],
  );
}

const q = pizzePe("a legna", 10);
console.log(q.sql);
console.log(q.text);
console.log(q.params.join(" | "), q.values.length);
console.log(
  sql("BEGIN", "BEGIN", []).sql,
  sql("COMMIT", "COMMIT", []).sql,
  sql("ROLLBACK", "ROLLBACK", []).sql,
);
console.log(
  sql(
    "INSERT INTO clienti VALUES (1, 'Gennaro ''o pizzaiuolo', TRUE)",
    "INSERT INTO clienti VALUES (1, 'Gennaro ''o pizzaiuolo', TRUE)",
    [],
  ).sql,
);
