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
    ],
    task: 'Aggiungi nu `caso "dummeneca"` ca stampa "Ragù!" e mietti `giorno` a "dummeneca".',
  },
  '13_magazzino': {
    title: 'apri_db e gomorra',
    body: [
      "`apri_db()` apre nu database SQLite: sta già int'a libreria `fratm`, nun serve installà niente. Int'o playground 'o database sta int'a memoria; cu Node se pò salvà ncopp'a nu file.",
      "Miette `gomorra` primma 'e na template string e scrive 'a query in napoletano (GomorraSQL): `ripigliammo … mmiez 'a …` è `SELECT … FROM …`, `arò` è `WHERE`, `mettimmo in fila pe` è `ORDER BY`. 'O compilatore 'a traduce e te dice si è sbagliata.",
      "Nu valore miso cu `${…}` passa comme parametro, maie mmiez 'o testo d'a query: accussì nisciuno te pò fà na SQL injection.",
    ],
    task: "Nzipp na Marinara a 4 euro e stampa sulo 'e pizze ca costano meno 'e 6 euro, d'a cchiù economica: `arò prezzo < ${6} mettimmo in fila pe prezzo`.",
  },
  '14_componenti': {
    title: 'nu componente React',
    body: [
      "Nu componente React è na funzione ca piglie 'e props e torna JSX: HTML scritto int'o codice. Cu `mann for predefinit facc` 'o playground 'o disegna int'a preview, e 'o React s'o piglia isso.",
      "`useState` tene nu valore ca cagna: `tien [pizze, setPizze] = useState([…])`. Quanno chiamme `setPizze(…)` cu na lista nova, 'o componente se ridisegna.",
      "`onClick={aggiungi}` chiamma na funzione quanno se clicca 'o bottone. Ccà `aggiungi` stampa int'a console e mette na pizza nova.",
      'Hê fernuto \'a scola! Mo puoi pruvà \'e esempi, leggere \'a guida d\'a sintassi cu \'o tasto ? o scrivere \'o programma tuoio.',
    ],
    task: "Aggiungi nu bottone \"Leva\" ca stampa \"Levata:\" cu l'urdema pizza (`pizze.at(-1)`) e 'a leva (`pizze.slice(0, -1)`). Po' clicca na vota \"Aggiungi\" e na vota \"Leva\".",
  },
}

const IT_TEXTS: Record<string, LessonText> = {
  '01_ue_munno': {
    title: 'Ciao, mondo!',
    body: [
      'Benvenuto nel tour! Qui impari FratmScript: JavaScript scritto come si deve, in napoletano.',
      'Sotto c\'è l\'editor; il risultato compare a destra, nella console (sul telefono, nella scheda Console). Ogni volta che cambi il codice, il programma riparte.',
      '`stamm a dì(…)` stampa quello che metti tra le parentesi. È il `console.log` di JavaScript.',
      'Se sbagli a scrivere una parola, il compilatore te lo dice in napoletano e ti suggerisce quella giusta. Prova a scrivere `stama` invece di `stamm`.',
    ],
    task: 'Fai stampare "Uè, Napule!" invece di "Uè, munno!".',
  },
  '02_variabili': {
    title: 'chist è e tien',
    body: [
      '`chist è` dichiara una costante: un valore che non cambia mai (`const`).',
      '`tien` dichiara una variabile che può cambiare (`let`). Per aumentarla ci sono `+= 1` e `++`.',
      '`stamm a dì` accetta più valori separati da virgole e li stampa sulla stessa riga.',
    ],
    task: 'Gennaro mangia un\'altra pizza: aumenta `pizze` di uno prima di stampare, così esce "Gennaro 3".',
  },
  '03_si_sinno': {
    title: 'si e sinnò',
    body: [
      '`si (condizione) { … }` esegue il blocco solo se la condizione è vera (`if`).',
      '`sinnò { … }` è quello che succede quando non lo è (`else`). Per più casi: `sinnò si (…) { … }`.',
      'Per confrontare: `<`, `>`, `<=`, `>=`, `==`, `!=`. Per combinare: `e`, `o`, `no`.',
    ],
    task: 'Aggiungi un `sinnò` che stampa "Bonasera!" e metti `ora` a 20.',
  },
  '04_loop': {
    title: 'pe e mentre che',
    body: [
      '`pe (tien i = 1; i <= 3; i++) { … }` ripete il blocco contando (`for`).',
      '`mentre che (condizione) { … }` ripete finché la condizione è vera (`while`). Attento a non scrivere un ciclo che non finisce mai: il playground lo ferma dopo poco.',
    ],
    task: 'Fai contare il primo ciclo da 1 a 5 e togli il secondo ciclo.',
  },
  '05_funzioni': {
    title: 'facc e piglie',
    body: [
      '`facc nome(parametri) { … }` crea una funzione (`function`).',
      '`piglie valore` è il `return`: la funzione si ferma e restituisce quel valore a chi l\'ha chiamata.',
    ],
    task: 'Scrivi una funzione `doppio(n)` che restituisce `n * 2` e stampa `doppio(21)` sotto il saluto.',
  },
  '06_frecce_e_template': {
    title: 'Frecce e template',
    body: [
      'Per una funzione corta c\'è la freccia: `chist è doppio = n => n * 2`. Con più parametri: `(a, b) => a + b`.',
      'Una stringa tra backtick (`` ` ``) è un template: tutto quello che sta dentro `${…}` viene calcolato e messo nel testo.',
    ],
    task: 'Con una freccia `prezzo = n => n * 6` e un template, stampa "3 pizze costano 18 euro".',
  },
  '07_liste': {
    title: 'Liste',
    body: [
      'Una lista si scrive tra parentesi quadre: `["margherita", "marinara"]`. `lista[0]` è il primo elemento, `lista.length` quanti ce ne sono.',
      '`pe ogni (chist è x dint lista) { … }` passa gli elementi uno alla volta (`for…of`).',
      'Funzionano tutti i metodi di JavaScript: `map`, `filter`, `join`, `toUpperCase`…',
    ],
    task: 'Stampa ogni pizza in maiuscolo (`pizza.toUpperCase()`), una per riga, senza altre righe.',
  },
  '08_oggetti': {
    title: 'Oggetti',
    body: [
      'Un oggetto raccoglie più valori con un nome: `{ nome: "Margherita", prezzo: 6 }`. Si leggono con `pizza.nome`.',
      'Con il destructuring prendi più campi in un colpo solo: `chist è { nome, prezzo } = pizza`.',
    ],
    task: 'Prendi `nome` e `prezzo` con il destructuring e stampa "Margherita: 6 euro".',
  },
  '09_classi': {
    title: 'na famiglie',
    body: [
      '`na famiglie Nome { … }` crea una classe. Il `costruttore` prepara l\'oggetto, `stu cos` è l\'oggetto stesso (`this`).',
      '`nu bell Nome(…)` crea un nuovo oggetto (`new`). Con `figlio \'e` una classe eredita da un\'altra (`extends`).',
    ],
    task: 'Aggiungi un metodo `presentati()` che restituisce "Songo " + il nome, e stampa solo `gennaro.presentati()`.',
  },
  '10_errori': {
    title: 'pruvamm e si schiatta',
    body: [
      '`iett nu bell Error("…")` lancia un errore (`throw`).',
      '`pruvamm { … } e si schiatta (errore) { … }` cattura l\'errore e ti fa continuare (`try/catch`). Con `aropp { … }` aggiungi codice che gira sempre (`finally`).',
      'Se nessuno cattura un errore, il playground te lo mostra in napoletano, con la riga.',
    ],
    task: 'In `dividi`, se `b` è zero lancia un `Error("Nun se divide pe zero!")`, e nello `schiatta` stampa solo `errore.message`.',
  },
  '11_async': {
    title: 'mo vir e aspett',
    body: [
      '`mo vir facc` crea una funzione asincrona (`async`): può aspettare cose che richiedono tempo, come una richiesta di rete.',
      '`aspett promessa` aspetta che una Promise finisca (`await`). Qui `aspetta(500)` aspetta mezzo secondo.',
    ],
    task: 'Dopo l\'`aspett`, stampa "Pronta!".',
  },
  '12_vedimm': {
    title: 'vedimm',
    body: [
      '`vedimm (valore) { caso …: … }` sceglie tra tanti casi (`switch`). `rompe` esce (`break`), `predefinit` è il caso per tutto il resto (`default`).',
    ],
    task: 'Aggiungi un `caso "dummeneca"` che stampa "Ragù!" e metti `giorno` a "dummeneca".',
  },
  '13_magazzino': {
    title: 'apri_db e gomorra',
    body: [
      "`apri_db()` apre un database SQLite: è già nella libreria `fratm`, non serve installare niente. Nel playground il database vive in memoria; con Node si può salvare su un file.",
      "Metti `gomorra` davanti a una template string e scrivi la query in napoletano (GomorraSQL): `ripigliammo … mmiez 'a …` è `SELECT … FROM …`, `arò` è `WHERE`, `mettimmo in fila pe` è `ORDER BY`. Il compilatore la traduce e ti dice se è sbagliata.",
      "Un valore messo con `${…}` passa come parametro, mai dentro il testo della query: così nessuno può farti una SQL injection.",
    ],
    task: "Inserisci una Marinara a 4 euro e stampa solo le pizze che costano meno di 6 euro, dalla più economica: `arò prezzo < ${6} mettimmo in fila pe prezzo`.",
  },
  '14_componenti': {
    title: 'nu componente React',
    body: [
      "Un componente React è una funzione che riceve le props e restituisce JSX: HTML scritto nel codice. Con `mann for predefinit facc` il playground lo disegna nell'anteprima, e React lo carica da solo.",
      "`useState` tiene un valore che cambia: `tien [pizze, setPizze] = useState([…])`. Quando chiami `setPizze(…)` con una lista nuova, il componente si ridisegna.",
      "`onClick={aggiungi}` chiama una funzione quando si clicca il bottone. Qui `aggiungi` stampa nella console e aggiunge una pizza nuova.",
      'Hai finito il tour! Ora puoi provare gli esempi, leggere la guida alla sintassi con il tasto ? o scrivere il tuo programma.',
    ],
    task: "Aggiungi un bottone \"Leva\" che stampa \"Levata:\" con l'ultima pizza (`pizze.at(-1)`) e la toglie (`pizze.slice(0, -1)`). Poi clicca una volta \"Aggiungi\" e una volta \"Leva\".",
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
    ],
    task: 'Add a `caso "dummeneca"` that prints "Ragù!" and set `giorno` to "dummeneca".',
  },
  '13_magazzino': {
    title: 'apri_db e gomorra',
    body: [
      "`apri_db()` opens a SQLite database: it is already in the `fratm` library, nothing to install. In the playground the database lives in memory; with Node it can be saved to a file.",
      "Put `gomorra` in front of a template string and write the query in Neapolitan (GomorraSQL): `ripigliammo … mmiez 'a …` is `SELECT … FROM …`, `arò` is `WHERE`, `mettimmo in fila pe` is `ORDER BY`. The compiler translates it and tells you when it is wrong.",
      "A value put in with `${…}` goes in as a parameter, never inside the query text: that way nobody can pull a SQL injection on you.",
    ],
    task: "Insert a Marinara at 4 euro and print only the pizzas that cost less than 6 euro, cheapest first: `arò prezzo < ${6} mettimmo in fila pe prezzo`.",
  },
  '14_componenti': {
    title: 'nu componente React',
    body: [
      "A React component is a function that takes props and returns JSX: HTML written in the code. With `mann for predefinit facc` the playground draws it in the preview, and loads React by itself.",
      "`useState` holds a value that changes: `tien [pizze, setPizze] = useState([…])`. When you call `setPizze(…)` with a new list, the component draws itself again.",
      "`onClick={aggiungi}` calls a function when the button is clicked. Here `aggiungi` prints to the console and adds a new pizza.",
      'You finished the tour! Now you can try the examples, read the syntax reference with the ? button, or write your own program.',
    ],
    task: "Add a \"Leva\" button that prints \"Levata:\" with the last pizza (`pizze.at(-1)`) and removes it (`pizze.slice(0, -1)`). Then click \"Aggiungi\" once and \"Leva\" once.",
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
    if (starter === undefined || solution === undefined || out === undefined || !IT_TEXTS[id] || !EN_TEXTS[id]) {
      throw new Error(`Lesson ${id} is missing a file`)
    }
    return { id, text: { it: IT_TEXTS[id], en: EN_TEXTS[id], nap: NAP_TEXTS[id] }, starter, solution, expected: out.replace(/\n$/, '').split('\n') }
  })

// Every lesson file has its text, and the other way round
if (import.meta.env.DEV) {
  const ids = new Set(Object.keys(starters).concat(Object.keys(outputs)).map(lessonId))
  for (const id of ids) if (!NAP_TEXTS[id]) console.warn(`Lesson ${id} has no text in lessons.ts`)
}
