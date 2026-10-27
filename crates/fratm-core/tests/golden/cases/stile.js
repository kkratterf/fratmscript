const stile = (() => {
  const fatti = new Set();
  return (css) => {
    if (
      typeof document !== "undefined" &&
      css.includes("{") &&
      !fatti.has(css)
    ) {
      fatti.add(css);
      const s = document.createElement("style");
      s.textContent = css;
      document.head.append(s);
    }
    return css;
  };
})();

const accento = "#c8321e";
const spazio = 8;
const css = stile(`
  /* 'a pizzeria */
  main.pizzeria {
    background-color: white;
    color: ${accento};
    padding: ${spazio * 2}px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  button:hover, a:focus { color: red !important; cursor: pointer }
  li:last-child::after { content: "!" }
  @media screen and (max-width: 600px) {
    div { width: 100%; padding: 0 }
  }
`);
console.log(css);
console.log(stile(`div { color: red }`));
