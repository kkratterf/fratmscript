fore:
for (let i = 0; i < 3; i++) {
  for (let j = 0; j < 3; j++) {
    if (j == 1) {
      continue fore;
    }
    if (i == 2) {
      break fore;
    }
    console.log(i, j);
  }
}

blocco:
{
  console.log("primma");
  break blocco;
}

function evidenzia(parti, ...valori) {
  return parti.reduce((acc, parte, k) =>
    acc + "[" + valori[k - 1] + "]" + parte,
  );
}

const nomme = "Totò";
console.log(evidenzia`Uè ${nomme}, tieni ${3} messaggi`);
console.log(String.raw`a\nb`);
