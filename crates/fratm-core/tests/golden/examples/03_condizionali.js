function giudicaPizza(voto) {
  if (voto >= 9) {
    return "🌟 Exceptional! Like true Neapolitan pizza!";
  } else   if (voto >= 7) {
    return "👍 Good! But there's room for improvement";
  } else   if (voto >= 5) {
    return "😐 Passable... but it's not real pizza";
  } else {
    return "🚫 Mamma mia! This is not pizza!";
  }
}

console.log("Score 10: " + giudicaPizza(10));
console.log("Score 8: " + giudicaPizza(8));
console.log("Score 5: " + giudicaPizza(5));
console.log("Score 2: " + giudicaPizza(2));

function èMaggiorenne(età) {
  return età >= 18 ? "Adult" : "Minor";
}

console.log("25 years: " + èMaggiorenne(25));
console.log("15 years: " + èMaggiorenne(15));

function puòGuidare(età, haPatente) {
  if (età >= 18 && haPatente) {
    return "Can drive! 🚗";
  } else   if (età >= 18 && !haPatente) {
    return "Adult but no license";
  } else {
    return "Too young";
  }
}

console.log(puòGuidare(25, true));
console.log(puòGuidare(25, false));
console.log(puòGuidare(16, false));

function isWeekend(giorno) {
  return giorno == "saturday" || giorno == "sunday";
}

console.log("Monday is weekend? " + isWeekend("monday"));
console.log("Saturday is weekend? " + isWeekend("saturday"));

function pizzaDelGiorno(giorno) {
  switch (giorno) {
    case "friday":
      return "Marinara";
    case "saturday":
    case "sunday":
      return "Margherita con bufala";
    default:
      return "Margherita";
  }
}

console.log("Friday pizza: " + pizzaDelGiorno("friday"));
console.log("Sunday pizza: " + pizzaDelGiorno("sunday"));
console.log("Monday pizza: " + pizzaDelGiorno("monday"));
