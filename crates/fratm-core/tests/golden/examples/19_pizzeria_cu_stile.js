import { jsx as _jsx, jsxs as _jsxs, Fragment as _Fragment } from "react/jsx-runtime";
import { useState } from "react";
const stile = (() => { const fatti = new Set(); return (css) => { if (typeof document !== "undefined" && css.includes("{") && !fatti.has(css)) { fatti.add(css); const s = document.createElement("style"); s.textContent = css; document.head.append(s); } return css; }; })();
stile(`
  main.pizzeria {
    font-family: system-ui, sans-serif;
    max-width: 420px;
    margin: 0 auto;
    padding: 16px
  }
  li {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 0;
    border-bottom: 1px solid #e2e0da
  }
  button {
    background-color: #c8321e;
    color: white;
    border: none;
    border-radius: 6px;
    padding: 6px 12px;
    cursor: pointer
  }
  button:hover { opacity: 0.85 }
  p.totale { font-weight: bold }
  @media (max-width: 480px) {
    main.pizzeria { padding: 8px }
  }
`);
const MENU = [{ nomme: "Margherita", prezzo: 5 }, { nomme: "Diavola", prezzo: 7 }, { nomme: "Marinara", prezzo: 4 }];
export default function Pizzeria() {
  let [conto, setConto] = useState(0);
  return _jsxs("main", { className: "pizzeria", children: [_jsx("h1", { children: "'A pizzeria cu stile" }), _jsx("ul", { children: MENU.map((p) => _jsxs("li", { children: [_jsxs("span", { children: [p.nomme, " · ", p.prezzo, " €"] }), _jsx("button", { onClick: () => setConto((conto + p.prezzo)), children: "Pigliala" })] }, p.nomme)) }), _jsxs("p", { className: "totale", children: ["Totale: ", conto, " €"] })] });
};
