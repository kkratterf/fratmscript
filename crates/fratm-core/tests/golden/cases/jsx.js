import { jsx as _jsx, jsxs as _jsxs, Fragment as _Fragment } from "react/jsx-runtime";
import { Fragment } from "react";
function Voce({ nomme, children }) {
  return _jsx("li", { "data-nomme": nomme, children: children });
}
export function Lista({ voci, titolo, ...resto }) {
  return _jsxs("section", { ...resto, className: "lista", "aria-label": titolo, children: [_jsxs("h2", { children: [titolo, " & cumpagnia"] }), _jsx("ul", { children: voci.map((v, i) => _jsxs(Voce, { nomme: v, children: [(i + 1), ". ", v] }, i)) }), _jsxs(_Fragment, { children: [_jsx("input", { type: "checkbox", disabled: true, checked: (voci.length > 2) }), _jsx("label", { htmlFor: "x", children: "Uè" })] }), _jsx(Fragment, { children: _jsx("br", {}) }, "f"), ((voci.length === 0) && _jsx("p", { children: "Vacante" }))] });
}
