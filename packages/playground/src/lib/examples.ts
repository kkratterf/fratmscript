export const examples = {
  hello: `// Hello World in FratmScript!
chist è messaggio = "Ue, comme staje?"
stamm a dì(messaggio)

// Variables
tien contatore = 0
contatore = contatore + 1
stamm a dì("Counter: " + contatore)`,

  fibonacci: `// Fibonacci in FratmScript
facc fibonacci(n) {
    si (n <= 1) {
        piglie n
    }
    piglie fibonacci(n - 1) + fibonacci(n - 2)
}

stamm a dì("Fibonacci sequence:")
pe (tien i = 0; i < 10; i = i + 1) {
    stamm a dì("fib(" + i + ") = " + fibonacci(i))
}`,

  classe: `// Classes in FratmScript
na famiglie Pizzaiolo {
    facc costruttore(nome, specialita) {
        stu cos.nome = nome
        stu cos.specialita = specialita
        stu cos.pizzeFatte = 0
    }

    facc faiPizza(tipo) {
        stu cos.pizzeFatte = stu cos.pizzeFatte + 1
        piglie stu cos.nome + " made a " + tipo + "!"
    }

    facc presentati() {
        piglie "I am " + stu cos.nome + ", specialized in " + stu cos.specialita
    }
}

chist è gennaro = nu bell Pizzaiolo("Gennaro", "Margherita DOC")
stamm a dì(gennaro.presentati())
stamm a dì(gennaro.faiPizza("Marinara"))
stamm a dì(gennaro.faiPizza("Diavola"))
stamm a dì("Pizzas made: " + gennaro.pizzeFatte)`,

  async: `// Async/Await in FratmScript
mo vir facc caricaDati() {
    stamm a dì("Loading...")
    // Simulates a delay
    piglie "Data loaded!"
}

mo vir facc main() {
    chist è risultato = aspett caricaDati()
    stamm a dì(risultato)
}

// Note: in this playground Promises are simulated
stamm a dì("Async demo (simulated)")`,

  operatori: `// Logical Operators in FratmScript
chist è a = overo
chist è b = sfòls

// AND: "e" or "pure"
stamm a dì("overo e sfols = " + (a e b))

// OR: "o"
stamm a dì("overo o sfols = " + (a o b))

// NOT: "no", "!", or "manco"
stamm a dì("no overo = " + (no a))
stamm a dì("!sfols = " + (!b))

// Combinations
chist è x = 5
chist è y = 10
si (x < y e y < 20) {
    stamm a dì("x is less than y, and y is less than 20")
}`,

  array: `// Arrays and Objects in FratmScript

// Array
chist è pizze = ["Margherita", "Marinara", "Diavola", "Capricciosa"]
stamm a dì("Pizza menu:")
pe (tien i = 0; i < 4; i = i + 1) {
    stamm a dì("  " + (i + 1) + ". " + pizze[i])
}

// Object
chist è ordine = {
    cliente: "Mario",
    pizza: "Margherita",
    quantita: 2,
    consegna: overo
}

stamm a dì("")
stamm a dì("Order:")
stamm a dì("  Customer: " + ordine.cliente)
stamm a dì("  Pizza: " + ordine.pizza)
stamm a dì("  Quantity: " + ordine.quantita)
stamm a dì("  Delivery: " + (ordine.consegna ? "Yes" : "No"))`,
}

export const defaultCode = `// Try FratmScript!

chist è nome = "Gennaro"

facc saluta(chi) {
    si (chi == nisciun) {
        piglie "E chi si tu?"
    }
    piglie "Ue " + chi + ", comme staje?"
}

stamm a dì(saluta(nome))
stamm a dì(saluta("Ciro"))
stamm a dì(saluta(nisciun))`
