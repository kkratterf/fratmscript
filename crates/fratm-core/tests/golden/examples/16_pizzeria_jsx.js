import { jsx as _jsx, jsxs as _jsxs, Fragment as _Fragment } from "react/jsx-runtime";
import { useState } from "react";
const MENU = [{ nomme: "Margherita", prezzo: 5, piccante: false }, { nomme: "Diavola", prezzo: 7, piccante: true }, { nomme: "Marinara", prezzo: 4, piccante: false }];
function Pizza({ nomme, prezzo, piccante, ordina }) {
  return _jsxs("li", { className: "pizza", children: [_jsx("strong", { children: nomme }), " · ", prezzo, " €", (piccante ? _jsx("span", { title: "Piccante", children: "🌶️" }) : null), _jsx("button", { onClick: () => ordina(nomme), children: "Ordina" })] });
}
export default function Pizzeria() {
  let [ordine, setOrdine] = useState([]);
  const totale = ordine.map((nomme) => MENU.find((p) => (p.nomme === nomme)).prezzo).reduce((a, b) => (a + b), 0);
  return _jsxs("main", { children: [_jsx("h1", { children: "Uè, benvenuto 'a pizzeria!" }), _jsx("ul", { children: MENU.map((pizza) => _jsx(Pizza, { ...pizza, ordina: (n) => setOrdine([...ordine, n]) }, pizza.nomme)) }), ((ordine.length === 0) ? _jsx("p", { children: "Ancora nisciuna pizza. Che te magne?" }) : _jsxs("p", { children: ["Hai ordinato ", ordine.length, " pizze: ", totale, " €"] }))] });
};
