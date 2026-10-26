import { jsx as _jsx, jsxs as _jsxs, Fragment as _Fragment } from "react/jsx-runtime";
import { useState } from "react";
export default function Ordine({ pizze, Riga }) {
  let [nomme, setNomme] = useState("");
  return _jsxs("form", { action: "/ordine", method: "post", onSubmit: (e) => e.preventDefault(), children: [_jsx("label", { htmlFor: "nomme", className: "etichetta", children: "Nomme" }), _jsx("input", { id: "nomme", name: "nomme", type: "text", placeholder: "Totonno", value: nomme, onChange: (e) => setNomme(e.target.value), required: true, autoFocus: true }), _jsx("ul", { children: pizze.map((p) => _jsx("li", { onClick: () => setNomme(p.nomme), children: p.nomme }, p.id)) }), _jsx("img", { src: "pizza.png", alt: "Na pizza", width: 120, height: 80 }), _jsx("a", { href: "/", target: "_blank", title: "Torna", children: "Torna" }), _jsxs("div", { style: { color: "red" }, hidden: (nomme === ""), role: "status", children: ["Uè ", nomme] }), _jsx("button", { type: "submit", disabled: (nomme === ""), onClick: () => console.log(nomme), children: "Manna" }), _jsx(Riga, { tipo: "pizza", "quanno-se-clicca": setNomme })] });
};
