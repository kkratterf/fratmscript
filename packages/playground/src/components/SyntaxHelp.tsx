import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogTrigger,
  DialogPopup,
  DialogHeader,
  DialogPanel,
  DialogTitle,
  DialogDescription,
} from '@/components/ui/dialog'

function HelpIcon({ className }: { className?: string }) {
  return (
    <svg className={className} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <circle cx="12" cy="12" r="10" />
      <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" />
      <path d="M12 17h.01" />
    </svg>
  )
}

const syntaxReference = [
  { category: 'Variables', items: [
    { fratm: 'chist è', js: 'const', desc: 'Constant declaration' },
    { fratm: 'tien', js: 'let', desc: 'Variable declaration' },
  ]},
  { category: 'Functions', items: [
    { fratm: 'facc', js: 'function', desc: 'Function declaration' },
    { fratm: 'piglie', js: 'return', desc: 'Return statement' },
    { fratm: 'mo vir facc', js: 'async function', desc: 'Async function' },
    { fratm: 'aspett', js: 'await', desc: 'Await expression' },
  ]},
  { category: 'Control Flow', items: [
    { fratm: 'si', js: 'if', desc: 'If condition' },
    { fratm: 'sinnò', js: 'else', desc: 'Else branch' },
    { fratm: 'mentre che', js: 'while', desc: 'While loop' },
    { fratm: 'pe', js: 'for', desc: 'For loop' },
    { fratm: 'rompe', js: 'break', desc: 'Break loop' },
    { fratm: 'salta', js: 'continue', desc: 'Continue loop' },
  ]},
  { category: 'Classes', items: [
    { fratm: 'na famiglie', js: 'class', desc: 'Class declaration' },
    { fratm: 'nu bell', js: 'new', desc: 'New instance' },
    { fratm: 'stu cos', js: 'this', desc: 'This reference' },
    { fratm: 'costruttore', js: 'constructor', desc: 'Constructor method' },
  ]},
  { category: 'Values', items: [
    { fratm: 'overo', js: 'true', desc: 'Boolean true' },
    { fratm: 'sfòls', js: 'false', desc: 'Boolean false' },
    { fratm: 'nisciun', js: 'null', desc: 'Null value' },
    { fratm: 'boh', js: 'undefined', desc: 'Undefined value' },
  ]},
  { category: 'Operators', items: [
    { fratm: 'e / pure', js: '&&', desc: 'Logical AND' },
    { fratm: 'o', js: '||', desc: 'Logical OR' },
    { fratm: 'no / manco', js: '!', desc: 'Logical NOT' },
  ]},
  { category: 'Console', items: [
    { fratm: 'stamm a dì', js: 'console.log', desc: 'Log output' },
    { fratm: 'avvis a dì', js: 'console.warn', desc: 'Warning output' },
    { fratm: 'scrive a dì', js: 'console.error', desc: 'Error output' },
  ]},
  { category: 'Error Handling', items: [
    { fratm: 'pruvamm', js: 'try', desc: 'Try block' },
    { fratm: 'e si schiatta', js: 'catch', desc: 'Catch block' },
    { fratm: 'iett', js: 'throw', desc: 'Throw error' },
  ]},
  { category: 'Modules', items: [
    { fratm: 'chiamm ... da', js: 'import ... from', desc: 'Import module' },
    { fratm: 'mann for', js: 'export', desc: 'Export' },
  ]},
]

export function SyntaxHelp() {
  return (
    <Dialog>
      <DialogTrigger
        render={
          <Button variant="outline" size="icon-sm" aria-label="Syntax Help">
            <HelpIcon className="size-3.5" />
          </Button>
        }
      />
      <DialogPopup className="flex flex-col max-w-2xl max-h-[80vh]">
        <DialogHeader>
          <DialogTitle>FratmScript syntax reference</DialogTitle>
          <DialogDescription>
            Neapolitan dialect to JavaScript translation
          </DialogDescription>
        </DialogHeader>
        <DialogPanel className="space-y-6">
          {syntaxReference.map((section) => (
            <div key={section.category}>
              <h3 className="mb-2 font-semibold text-foreground text-sm">{section.category}</h3>
              <div className="border rounded-md overflow-hidden">
                <table className="w-full text-sm">
                  <thead className="bg-muted/50">
                    <tr>
                      <th className="px-3 py-2 font-medium text-muted-foreground text-left">FratmScript</th>
                      <th className="px-3 py-2 font-medium text-muted-foreground text-left">JavaScript</th>
                      <th className="hidden sm:table-cell px-3 py-2 font-medium text-muted-foreground text-left">Description</th>
                    </tr>
                  </thead>
                  <tbody>
                    {section.items.map((item, i) => (
                      <tr key={i} className="border-border border-t">
                        <td className="px-3 py-2 font-mono text-primary">{item.fratm}</td>
                        <td className="px-3 py-2 font-mono text-muted-foreground">{item.js}</td>
                        <td className="hidden sm:table-cell px-3 py-2 text-muted-foreground">{item.desc}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          ))}
        </DialogPanel>
      </DialogPopup>
    </Dialog>
  )
}
