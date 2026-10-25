// Runs a tour lesson that is a React component the way a learner does in
// the playground: renders its default export, clicks every button once in
// page order, and lets the component's console.log calls reach stdout.
//
// Usage: node react-lesson.mjs dir/compiled.mjs
// react, react-dom and jsdom must be installed in dir/node_modules.

import { createRequire } from 'node:module'
import { resolve } from 'node:path'
import { pathToFileURL } from 'node:url'

const file = resolve(process.argv[2])
// Load the same React copy the compiled module imports
const require = createRequire(file)
const { JSDOM } = require('jsdom')
const React = require('react')
const { createRoot } = require('react-dom/client')

const dom = new JSDOM('<!doctype html><div id="root"></div>')
globalThis.window = dom.window
globalThis.document = dom.window.document
globalThis.IS_REACT_ACT_ENVIRONMENT = true

const { default: Component } = await import(pathToFileURL(file).href)
if (typeof Component !== 'function') {
  console.error(`${process.argv[2]}: no component (mann for predefinit facc …)`)
  process.exit(1)
}

const root = createRoot(document.getElementById('root'))
await React.act(() => root.render(React.createElement(Component)))
const count = document.querySelectorAll('button').length
for (let i = 0; i < count; i++) {
  // Look the button up again: a click can re-render the page
  const button = document.querySelectorAll('button')[i]
  if (!button) break
  await React.act(() => button.dispatchEvent(new window.MouseEvent('click', { bubbles: true })))
}
await React.act(() => root.unmount())
