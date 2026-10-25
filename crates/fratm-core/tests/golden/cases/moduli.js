import React from "react";
import Vue, { ref, computed as calcola } from "vue";
import * as fs from "node:fs";
import Pippo, * as tutto from "./pippo.js";
import { default as Base, for as $for } from "./base.js";
import { a, b as bi } from "./multi.js";
import "./stile.css";
const x = 1;
let comme = 2;
export { x, comme as y, x as default };
export { c, d as e } from "./c.js";
export * from "./d.js";
export * as spazio from "./e.js";
async function carica(nome) {
  const modulo = await import((("./pagine/" + nome) + ".js"));
  return modulo.default;
}
import("./effetti.js");
