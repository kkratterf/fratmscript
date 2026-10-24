// 'A scola 'e FratmScript: the interactive tour.
//
// Each lesson has a starter program (src/scola/NN_name.fratm), a solution
// (NN_name.soluzione.fratm) and the console output the solution prints
// (NN_name.out). CI checks that every starter compiles and that every
// solution prints exactly its .out file; the playground uses the .out file
// to tell when the exercise is done.

import type { Lang } from '@/lib/i18n'

const starters = import.meta.glob('../scola/*.fratm', { query: '?raw', import: 'default', eager: true }) as Record<string, string>
const outputs = import.meta.glob('../scola/*.out', { query: '?raw', import: 'default', eager: true }) as Record<string, string>

export interface LessonText {
  title: string
  /** Paragraphs; `code` in backticks is shown as code */
  body: string[]
  /** What to change in the code */
  task: string
}

export interface Lesson {
  /** File name without extension, e.g. `01_ue_munno` */
  id: string
  /** Title, explanation and exercise in each UI language */
  text: Record<Lang, LessonText>
  starter: string
  solution: string
  /** Console lines the solution prints */
  expected: string[]
}

const NAP_TEXTS: Record<string, LessonText> = {
  '01_ue_munno': {
    title: 'Uè, munno!',
    body: [
      'Benvenuto a scola! Ccà mpare FratmScript, JavaScript scritto comme si deve: in napoletano.',
      'Sotto ce sta l\'editor; \'o risultato esce a destra, int\'a console (ncopp\'o telefono, int\'a tab Console). Ogni vota ca cagne \'o codice, \'o programma riparte.',
      '`stamm a dì(…)` stampa quello che ce miette dint\'e parentesi. È \'o `console.log` \'e JavaScript.',
      'Si scrive na parola sbagliata, \'o compilatore t\'o dice in napoletano e te suggerisce chella giusta. Prova a scrivere `stama` invece \'e `stamm`.',
    ],
    task: 'Fai stampà "Uè, Napule!" invece \'e "Uè, munno!".',
  },
  '02_variabili': {
    title: 'chist è e tien',
    body: [
      '`chist è` dichiara na costante: nu valore ca nun cagna maje (`const`).',
      '`tien` dichiara na variabile ca può cagnà (`let`). Pe l\'aumentà ce stanno `+= 1` e `++`.',
      '`stamm a dì` accetta cchiù valori, separati da na virgola, e \'e stampa ncopp\'a stessa riga.',
    ],
    task: 'Gennaro se mangia n\'ata pizza: aumenta `pizze` \'e uno primma \'e stampà, accussì esce "Gennaro 3".',
  },
  '03_si_sinno': {
    title: 'si e sinnò',
    body: [
      '`si (condizione) { … }` esegue \'o blocco sulo si \'a condizione è vera (`if`).',
      '`sinnò { … }` è chello ca succede quanno nun è vera (`else`). Pe cchiù casi: `sinnò si (…) { … }`.',
      'Pe confrontà: `<`, `>`, `<=`, `>=`, `==`, `!=`. Pe combinà: `e`, `o`, `no`.',
    ],
    task: 'Aggiungi nu `sinnò` ca stampa "Bonasera!" e mietti `ora` a 20.',
  },
  '04_loop': {
    title: 'pe e mentre che',
    body: [
      '`pe (tien i = 1; i <= 3; i++) { … }` ripete \'o blocco cuntanno (`for`).',
      '`mentre che (condizione) { … }` ripete fino a quanno \'a condizione è vera (`while`). Attento a nun fà nu ciclo ca nun fernesce maje: \'o playground te ferma doppo nu poco.',
    ],
    task: 'Fai cuntà \'o primmo loop da 1 a 5, e leva \'o secondo loop.',
  },
  '05_funzioni': {
    title: 'facc e piglie',
    body: [
      '`facc nomme(parametri) { … }` crea na funzione (`function`).',
      '`piglie valore` è \'o `return`: \'a funzione se ferma e dà chillu valore a chi l\'ha chiammata.',
    ],
    task: 'Scrivi na funzione `doppio(n)` ca piglie `n * 2`, e stampa `doppio(21)` sotto \'o saluto.',
  },
  '06_frecce_e_template': {
    title: 'Frecce e template',
    body: [
      'Pe na funzione corta ce sta \'a freccia: `chist è doppio = n => n * 2`. Cu cchiù parametri: `(a, b) => a + b`.',
      'Na stringa tra backtick (`` ` ``) è nu template: tutto chello ca sta dint\'a `${…}` vene calcolato e miso dint\'o testo.',
    ],
    task: 'Cu na freccia `prezzo = n => n * 6` e nu template, stampa "3 pizze costano 18 euro".',
  },
  '07_liste': {
    title: 'Liste',
    body: [
      'Na lista se scrive tra parentesi quadre: `["margherita", "marinara"]`. `lista[0]` è \'o primmo elemento, `lista.length` quanti ce ne stanno.',
      '`pe ogni (chist è x dint lista) { … }` passa \'e elementi uno a uno (`for…of`).',
      'Tutti \'e metodi \'e JavaScript funzionano: `map`, `filter`, `join`, `toUpperCase`…',
    ],
    task: 'Stampa ogni pizza in maiuscolo (`pizza.toUpperCase()`), una pe riga, senza \'e altre righe.',
  },
  '08_oggetti': {
    title: 'Oggetti',
    body: [
      'N\'oggetto mette nzieme cchiù valori cu nu nomme: `{ nome: "Margherita", prezzo: 6 }`. Se leggono cu `pizza.nome`.',
      'Cu \'o destructuring piglie cchiù campi int\'na botta: `chist è { nome, prezzo } = pizza`.',
    ],
    task: 'Piglia `nome` e `prezzo` cu \'o destructuring e stampa "Margherita: 6 euro".',
  },
  '09_classi': {
    title: 'na famiglie',
    body: [
      '`na famiglie Nomme { … }` crea na classe. \'O `costruttore` prepara l\'oggetto, `stu cos` è l\'oggetto stesso (`this`).',
      '`nu bell Nomme(…)` crea n\'oggetto nuovo (`new`). Cu `figlio \'e` na famiglie eredita da n\'ata (`extends`).',
    ],
    task: 'Aggiungi nu metodo `presentati()` ca piglie "Songo " + \'o nomme, e stampa solo `gennaro.presentati()`.',
  },
  '10_errori': {
    title: 'pruvamm e si schiatta',
    body: [
      '`iett nu bell Error("…")` lancia n\'errore (`throw`).',
      '`pruvamm { … } e si schiatta (errore) { … }` acchiappa l\'errore e te fa continuà (`try/catch`). Cu `aropp { … }` miette codice ca gira sempe (`finally`).',
      'Si n\'errore nun \'o acchiappa nisciuno, \'o playground t\'o mostra in napoletano, cu \'a riga.',
    ],
    task: 'Dint\'a `dividi`, si `b` è zero iett nu `Error("Nun se divide pe zero!")`, e int\'o `schiatta` stampa sulo `errore.message`.',
  },
  '11_async': {
    title: 'mo vir e aspett',
    body: [
      '`mo vir facc` crea na funzione asincrona (`async`): po\' aspettà cose ca ce mettono tiempo, comme na richiesta in rete.',
      '`aspett promessa` aspetta ca na Promise fernesce (`await`). Ccà `aspetta(500)` aspetta mezzo secondo.',
    ],
    task: 'Doppo l\'`aspett`, stampa "Pronta!".',
  },
  '12_vedimm': {
    title: 'vedimm',
    body: [
      '`vedimm (valore) { caso …: … }` sceglie tra tanti casi (`switch`). `rompe` esce (`break`), `predefinit` è \'o caso ca resta (`default`).',
      'Hê fernuto \'a scola! Mo puoi pruvà \'e esempi, leggere \'a guida d\'a sintassi cu \'o tasto ? o scrivere \'o programma tuoio.',
    ],
    task: 'Aggiungi nu `caso "dummeneca"` ca stampa "Ragù!" e mietti `giorno` a "dummeneca".',
  },
}

const EN_TEXTS: Record<string, LessonText> = {
  '01_ue_munno': {
    title: 'Hello, world!',
    body: [
      'Welcome to the tour! Here you learn FratmScript: JavaScript written the way it should be, in Neapolitan.',
      'The editor is below; the result appears on the right, in the console (on a phone, in the Console tab). Every time you change the code, the program runs again.',
      '`stamm a dì(…)` prints whatever you put between the parentheses. It is JavaScript\'s `console.log`.',
      'If you misspell a word, the compiler tells you in Neapolitan and suggests the right one. Try writing `stama` instead of `stamm`.',
    ],
    task: 'Print "Uè, Napule!" instead of "Uè, munno!".',
  },
  '02_variabili': {
    title: 'chist è and tien',
    body: [
      '`chist è` declares a constant: a value that never changes (`const`).',
      '`tien` declares a variable that can change (`let`). To increase it there are `+= 1` and `++`.',
      '`stamm a dì` accepts several values separated by commas and prints them on the same line.',
    ],
    task: 'Gennaro eats another pizza: increase `pizze` by one before printing, so the output is "Gennaro 3".',
  },
  '03_si_sinno': {
    title: 'si and sinnò',
    body: [
      '`si (condition) { … }` runs the block only if the condition is true (`if`).',
      '`sinnò { … }` is what happens when it is not (`else`). For more cases: `sinnò si (…) { … }`.',
      'To compare: `<`, `>`, `<=`, `>=`, `==`, `!=`. To combine: `e`, `o`, `no`.',
    ],
    task: 'Add a `sinnò` that prints "Bonasera!" and set `ora` to 20.',
  },
  '04_loop': {
    title: 'pe and mentre che',
    body: [
      '`pe (tien i = 1; i <= 3; i++) { … }` repeats the block while counting (`for`).',
      '`mentre che (condition) { … }` repeats as long as the condition is true (`while`). Careful not to write a loop that never ends: the playground stops it after a short while.',
    ],
    task: 'Make the first loop count from 1 to 5, and remove the second loop.',
  },
  '05_funzioni': {
    title: 'facc and piglie',
    body: [
      '`facc name(parameters) { … }` creates a function (`function`).',
      '`piglie value` is `return`: the function stops and gives that value back to its caller.',
    ],
    task: 'Write a function `doppio(n)` that returns `n * 2`, and print `doppio(21)` below the greeting.',
  },
  '06_frecce_e_template': {
    title: 'Arrows and templates',
    body: [
      'For a short function there is the arrow: `chist è doppio = n => n * 2`. With more parameters: `(a, b) => a + b`.',
      'A string between backticks (`` ` ``) is a template: everything inside `${…}` is computed and put into the text.',
    ],
    task: 'With an arrow `prezzo = n => n * 6` and a template, print "3 pizze costano 18 euro".',
  },
  '07_liste': {
    title: 'Lists',
    body: [
      'A list is written between square brackets: `["margherita", "marinara"]`. `lista[0]` is the first element, `lista.length` how many there are.',
      '`pe ogni (chist è x dint lista) { … }` goes through the elements one by one (`for…of`).',
      'All JavaScript methods work: `map`, `filter`, `join`, `toUpperCase`…',
    ],
    task: 'Print each pizza in upper case (`pizza.toUpperCase()`), one per line, with no other lines.',
  },
  '08_oggetti': {
    title: 'Objects',
    body: [
      'An object groups several named values: `{ nome: "Margherita", prezzo: 6 }`. You read them with `pizza.nome`.',
      'Destructuring takes several fields in one go: `chist è { nome, prezzo } = pizza`.',
    ],
    task: 'Take `nome` and `prezzo` with destructuring and print "Margherita: 6 euro".',
  },
  '09_classi': {
    title: 'na famiglie',
    body: [
      '`na famiglie Name { … }` creates a class. The `costruttore` sets up the object, and `stu cos` is the object itself (`this`).',
      '`nu bell Name(…)` creates a new object (`new`). With `figlio \'e` a class inherits from another one (`extends`).',
    ],
    task: 'Add a method `presentati()` that returns "Songo " + the name, and print only `gennaro.presentati()`.',
  },
  '10_errori': {
    title: 'pruvamm and si schiatta',
    body: [
      '`iett nu bell Error("…")` throws an error (`throw`).',
      '`pruvamm { … } e si schiatta (errore) { … }` catches the error and lets you carry on (`try/catch`). With `aropp { … }` you add code that always runs (`finally`).',
      'If nothing catches an error, the playground shows it in Neapolitan, with its line.',
    ],
    task: 'In `dividi`, if `b` is zero throw an `Error("Nun se divide pe zero!")`, and in the `schiatta` block print only `errore.message`.',
  },
  '11_async': {
    title: 'mo vir and aspett',
    body: [
      '`mo vir facc` creates an asynchronous function (`async`): it can wait for things that take time, like a network request.',
      '`aspett promise` waits for a Promise to finish (`await`). Here `aspetta(500)` waits half a second.',
    ],
    task: 'After the `aspett`, print "Pronta!".',
  },
  '12_vedimm': {
    title: 'vedimm',
    body: [
      '`vedimm (value) { caso …: … }` picks one of many cases (`switch`). `rompe` leaves it (`break`), and `predefinit` is the case for everything else (`default`).',
      'You finished the tour! Now you can try the examples, read the syntax reference with the ? button, or write your own program.',
    ],
    task: 'Add a `caso "dummeneca"` that prints "Ragù!" and set `giorno` to "dummeneca".',
  },
}

function lessonId(path: string): string {
  return path.split('/').pop()!.replace(/\.(soluzione\.)?fratm$|\.out$/g, '')
}

export const lessons: Lesson[] = Object.keys(NAP_TEXTS)
  .sort()
  .map(id => {
    const starter = starters[`../scola/${id}.fratm`]
    const solution = starters[`../scola/${id}.soluzione.fratm`]
    const out = outputs[`../scola/${id}.out`]
    if (starter === undefined || solution === undefined || out === undefined || !EN_TEXTS[id]) {
      throw new Error(`Lesson ${id} is missing a file`)
    }
    return { id, text: { en: EN_TEXTS[id], nap: NAP_TEXTS[id] }, starter, solution, expected: out.replace(/\n$/, '').split('\n') }
  })

// Every lesson file has its text, and the other way round
if (import.meta.env.DEV) {
  const ids = new Set(Object.keys(starters).concat(Object.keys(outputs)).map(lessonId))
  for (const id of ids) if (!NAP_TEXTS[id]) console.warn(`Lesson ${id} has no text in lessons.ts`)
}
