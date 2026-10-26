// 'A scola 'e FratmScript: the interactive tour.
//
// Each lesson has a starter program (src/scola/NN_name.fratm), a solution
// (NN_name.soluzione.fratm) and the console output the solution prints
// (NN_name.out). CI checks that every starter compiles and that every
// solution prints exactly its .out file; the playground uses the .out file
// to tell when the exercise is done.

import type { Lang } from '@/lib/i18n'
import { EN_TEXTS, IT_TEXTS, NAP_TEXTS, type LessonText } from '@/lib/lessonTexts'

const starters = import.meta.glob('../scola/*.fratm', { query: '?raw', import: 'default', eager: true }) as Record<string, string>
const outputs = import.meta.glob('../scola/*.out', { query: '?raw', import: 'default', eager: true }) as Record<string, string>

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
