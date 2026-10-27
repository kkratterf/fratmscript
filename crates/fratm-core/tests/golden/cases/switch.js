function giudizio(voto) {
  switch (voto) {
    case 10:
      return "è nu capolavoro";
    case 8:
    case 9:
      return "bbuono assaje";
    case 6:
      return "se po' fa'";
    default:
      return "aggia studià";
  }
}

console.log(giudizio(10));
console.log(giudizio(9));
console.log(giudizio(3));
let log = [];

switch ("b") {
  case "a":
    log.push("a");
  case "b":
    {
      log.push("b");
    }
  case "c":
    log.push("c");
    break;
  default:
    log.push("mai");
}

console.log(log.join(","));

function vedimm(x) {
  return `vedimm ${x}`;
}

console.log(vedimm("nu poco"));
