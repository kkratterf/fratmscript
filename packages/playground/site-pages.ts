// Static pages for search engines and crawlers that do not run JavaScript:
// - guida/<id>/ and en/guide/<id>/: one page per guide section, in Italian
//   and English, built from src/guida/guida.json and its examples
// - scola/<id>/ and en/learn/<id>/: one page per tour lesson, built from
//   src/lib/lessonTexts.ts and the lesson's starter program
// - sitemap.xml: the home and every page above, with hreflang alternates
// - robots.txt: allows every crawler and points to the sitemap
// The playground stays a single-page app; each page links to its section there.
import fs from 'node:fs'
import path from 'node:path'
import type { Plugin } from 'vite'
import { REPO, SITE, guidaDir, type GuideSection } from './agent-files'
import { analyticsScript } from './analytics'
import { EN_TEXTS, IT_TEXTS, type LessonText } from './src/lib/lessonTexts'

type Lang = 'it' | 'en'

const LANGS: Lang[] = ['it', 'en']

/** Path of a guide page under the site, '' for the guide index. */
function pagePath(lang: Lang, id = ''): string {
  const base = lang === 'it' ? 'guida/' : 'en/guide/'
  return id ? `${base}${id}/` : base
}

/** Path of a tour lesson page under the site, '' for the tour index. */
function lessonPath(lang: Lang, id = ''): string {
  const base = lang === 'it' ? 'scola/' : 'en/learn/'
  return id ? `${base}${id.replace(/_/g, '-')}/` : base
}

const scolaDir = path.join(__dirname, 'src/scola')
const LESSON_TEXTS: Record<Lang, Record<string, LessonText>> = { it: IT_TEXTS, en: EN_TEXTS }

const TEXT: Record<Lang, Record<string, string>> = {
  it: {
    guide: 'Guida',
    guideTitle: 'Guida a FratmScript',
    guideIntro:
      'FratmScript è JavaScript con le parole chiave in napoletano: un compilatore scritto in Rust traduce i file .fratm in JavaScript moderno, per Node.js e per il browser. Ogni sezione ha esempi che puoi eseguire nel playground.',
    tagline: 'JavaScript ma comme si deve',
    playground: 'Apri il playground',
    tryIt: 'Prova questa sezione nel playground',
    output: 'Output',
    wrong: 'Questo esempio non compila, apposta:',
    prev: 'Precedente',
    next: 'Successiva',
    other: 'English',
    tour: 'Tour',
    tourTitle: 'Impara FratmScript: il tour in lezioni',
    tourIntro:
      'Un corso interattivo di FratmScript, il JavaScript in napoletano: ogni lezione spiega un pezzo del linguaggio e ti dà un esercizio da risolvere nel playground, senza installare niente.',
    lesson: 'Lezione',
    task: 'Esercizio',
    expected: 'Quando hai finito, il programma stampa',
    doLesson: 'Fai questa lezione nel playground',
    startTour: 'Inizia il tour nel playground',
    seeTour: 'Preferisci gli esercizi? Fai il tour in lezioni.',
    seeGuide: 'Cerchi un argomento preciso? Leggi la guida.',
  },
  en: {
    guide: 'Guide',
    guideTitle: 'FratmScript guide',
    guideIntro:
      'FratmScript is JavaScript with Neapolitan keywords: a compiler written in Rust turns .fratm files into modern JavaScript for Node.js and the browser. Every section has examples you can run in the playground.',
    tagline: 'JavaScript, the way it should be',
    playground: 'Open the playground',
    tryIt: 'Try this section in the playground',
    output: 'Output',
    wrong: 'This example does not compile, on purpose:',
    prev: 'Previous',
    next: 'Next',
    other: 'Italiano',
    tour: 'Tour',
    tourTitle: 'Learn FratmScript: the tour in lessons',
    tourIntro:
      'An interactive FratmScript course, JavaScript in Neapolitan: each lesson explains a piece of the language and gives you an exercise to solve in the playground, with nothing to install.',
    lesson: 'Lesson',
    task: 'Exercise',
    expected: 'When you are done, the program prints',
    doLesson: 'Do this lesson in the playground',
    startTour: 'Start the tour in the playground',
    seeTour: 'Rather do exercises? Take the tour in lessons.',
    seeGuide: 'Looking for a topic? Read the guide.',
  },
}

function sections(): GuideSection[] {
  return (JSON.parse(fs.readFileSync(path.join(guidaDir, 'guida.json'), 'utf8')) as { sections: GuideSection[] }).sections
}

function escape(text: string): string {
  return text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;')
}

/** A guide paragraph: `code` in backticks becomes <code>. */
function paragraph(text: string): string {
  return escape(text).replace(/`([^`]+)`/g, '<code>$1</code>')
}

/** Plain text for meta descriptions, at most about 160 characters. */
function summary(text: string): string {
  const plain = text.replace(/`/g, '')
  return plain.length <= 160 ? plain : plain.slice(0, plain.lastIndexOf(' ', 157)) + '…'
}

function examples(section: GuideSection, t: Record<string, string>): string {
  return section.snippets
    .map((snippet) => {
      const code = escape(fs.readFileSync(path.join(guidaDir, snippet), 'utf8').trimEnd())
      if (snippet.endsWith('.sbagliato.fratm')) {
        return `<p>${t.wrong}</p>\n<pre><code class="language-fratm">${code}</code></pre>`
      }
      const out = path.join(guidaDir, snippet.replace(/\.fratm$/, '.out'))
      const output = fs.existsSync(out)
        ? `\n<p class="label">${t.output}</p>\n<pre class="out"><code>${escape(fs.readFileSync(out, 'utf8').trimEnd())}</code></pre>`
        : ''
      return `<pre><code class="language-fratm">${code}</code></pre>${output}`
    })
    .join('\n')
}

const STYLE = `:root{color-scheme:light dark;--bg:#fff;--fg:#262626;--muted:#737373;--line:#0000001a;--code:#0000000a}
@media (prefers-color-scheme:dark){:root{--bg:#0a0a0a;--fg:#e5e5e5;--muted:#a3a3a3;--line:#ffffff1f;--code:#ffffff0f}}
*{box-sizing:border-box}body{margin:0;background:var(--bg);color:var(--fg);font:16px/1.65 system-ui,-apple-system,"Segoe UI",sans-serif}
header,main,footer{max-width:46rem;margin:0 auto;padding:0 16px}
header{display:flex;flex-wrap:wrap;gap:.5rem 1rem;align-items:center;justify-content:space-between;padding-top:1rem;padding-bottom:1rem;border-bottom:1px solid var(--line)}
header nav{display:flex;gap:1rem}a{color:inherit}.brand{font-weight:700;text-decoration:none}
h1{font-size:2rem;line-height:1.2;margin:2rem 0 1rem}h2{font-size:1.15rem;margin:2rem 0 .5rem}
code{font:.9em ui-monospace,SFMono-Regular,Menlo,monospace;background:var(--code);padding:.1em .3em;border-radius:4px}
pre{line-height:1.5;background:var(--code);border:1px solid var(--line);border-radius:8px;padding:1rem;overflow-x:auto}pre code{background:none;padding:0}
pre.out{color:var(--muted)}.label{margin-bottom:-.5rem;color:var(--muted);font-size:.875rem}
.cta{display:inline-block;margin:1rem 0;padding:.6rem 1rem;border-radius:8px;background:var(--fg);color:var(--bg);text-decoration:none;font-weight:600}
.pager{display:flex;justify-content:space-between;gap:1rem;margin:2rem 0;padding-top:1rem;border-top:1px solid var(--line)}
ol{padding-left:1.25rem}li{margin:.35rem 0}
footer{color:var(--muted);font-size:.875rem;padding-top:1rem;padding-bottom:2rem}`

interface Page {
  lang: Lang
  /** Page path under the site, e.g. `guida/cicli/` */
  path: string
  /** The same page in the other language */
  alternate: string
  title: string
  description: string
  body: string
  /** Section in the playground the page links to */
  playground: string
  /** Label of the link to the playground */
  cta: string
}

function render(page: Page): string {
  const t = TEXT[page.lang]
  const other: Lang = page.lang === 'it' ? 'en' : 'it'
  const url = SITE + page.path
  const ld = {
    '@context': 'https://schema.org',
    '@type': 'TechArticle',
    headline: page.title,
    description: page.description,
    inLanguage: page.lang,
    url,
    isPartOf: { '@type': 'WebSite', name: 'FratmScript', url: SITE },
    about: { '@type': 'ComputerLanguage', name: 'FratmScript', url: SITE },
  }
  return `<!DOCTYPE html>
<html lang="${page.lang}">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>${escape(page.title)} · FratmScript</title>
<meta name="description" content="${escape(page.description)}">
<link rel="canonical" href="${url}">
<link rel="alternate" hreflang="${page.lang}" href="${url}">
<link rel="alternate" hreflang="${other}" href="${SITE + page.alternate}">
<link rel="alternate" hreflang="x-default" href="${SITE + (page.lang === 'it' ? page.path : page.alternate)}">
<link rel="icon" type="image/png" href="${SITE}favicon.png">
<meta property="og:type" content="article">
<meta property="og:url" content="${url}">
<meta property="og:title" content="${escape(page.title)} · FratmScript">
<meta property="og:description" content="${escape(page.description)}">
<meta property="og:image" content="${SITE}og-image.png">
<meta name="twitter:card" content="summary_large_image">
<meta name="theme-color" content="#0a0a0a">
<script type="application/ld+json">${JSON.stringify(ld).replace(/</g, '\\u003c')}</script>
<style>${STYLE}</style>
${analyticsScript()}
</head>
<body>
<header>
<a class="brand" href="${SITE}">FratmScript 🤌🏻</a>
<nav><a href="${SITE + pagePath(page.lang)}">${t.guide}</a><a href="${SITE + lessonPath(page.lang)}">${t.tour}</a><a href="${SITE + page.alternate}" hreflang="${other}" lang="${other}">${t.other}</a><a href="${REPO}">GitHub</a></nav>
</header>
<main>
${page.body}
<p><a class="cta" href="${page.playground}">${page.cta}</a></p>
</main>
<footer>FratmScript · ${t.tagline} · <a href="${SITE}llms.txt">llms.txt</a> · MIT</footer>
</body>
</html>
`
}

function pages(): Page[] {
  const all = sections()
  const result: Page[] = []
  for (const lang of LANGS) {
    const t = TEXT[lang]
    const other: Lang = lang === 'it' ? 'en' : 'it'
    const list = all.map((s) => `<li><a href="${SITE + pagePath(lang, s.id)}">${escape(s.title[lang])}</a></li>`).join('\n')
    result.push({
      lang,
      path: pagePath(lang),
      alternate: pagePath(other),
      title: t.guideTitle,
      description: summary(t.guideIntro),
      body: `<h1>${t.guideTitle}</h1>\n<p>${escape(t.guideIntro)}</p>\n<ol>\n${list}\n</ol>\n<p><a href="${SITE + lessonPath(lang)}">${t.seeTour}</a></p>`,
      playground: `${SITE}#guida`,
      cta: t.playground,
    })
    all.forEach((section, i) => {
      const link = (s: GuideSection | undefined, label: string) =>
        s ? `<a href="${SITE + pagePath(lang, s.id)}" rel="${label === t.prev ? 'prev' : 'next'}">${label}: ${escape(s.title[lang])}</a>` : '<span></span>'
      result.push({
        lang,
        path: pagePath(lang, section.id),
        alternate: pagePath(other, section.id),
        title: section.title[lang],
        description: summary(section.body[lang][0]),
        body: [
          `<h1>${escape(section.title[lang])}</h1>`,
          ...section.body[lang].map((p) => `<p>${paragraph(p)}</p>`),
          examples(section, t),
          `<nav class="pager">${link(all[i - 1], t.prev)}${link(all[i + 1], t.next)}</nav>`,
        ].join('\n'),
        playground: `${SITE}#guida=${section.id}`,
        cta: t.tryIt,
      })
    })
  }
  return result.concat(lessonPages())
}

/** The tour index and one page per lesson, in each language. */
function lessonPages(): Page[] {
  const ids = Object.keys(IT_TEXTS).sort()
  const result: Page[] = []
  for (const lang of LANGS) {
    const t = TEXT[lang]
    const other: Lang = lang === 'it' ? 'en' : 'it'
    const texts = LESSON_TEXTS[lang]
    const list = ids.map((id) => `<li><a href="${SITE + lessonPath(lang, id)}">${escape(texts[id].title)}</a></li>`).join('\n')
    result.push({
      lang,
      path: lessonPath(lang),
      alternate: lessonPath(other),
      title: t.tourTitle,
      description: summary(t.tourIntro),
      body: `<h1>${t.tourTitle}</h1>\n<p>${escape(t.tourIntro)}</p>\n<ol>\n${list}\n</ol>\n<p><a href="${SITE + pagePath(lang)}">${t.seeGuide}</a></p>`,
      playground: `${SITE}#scola=1`,
      cta: t.startTour,
    })
    ids.forEach((id, i) => {
      const text = texts[id]
      const starter = escape(fs.readFileSync(path.join(scolaDir, `${id}.fratm`), 'utf8').trimEnd())
      const expected = escape(fs.readFileSync(path.join(scolaDir, `${id}.out`), 'utf8').trimEnd())
      const link = (next: string | undefined, label: string, rel: string) =>
        next ? `<a href="${SITE + lessonPath(lang, next)}" rel="${rel}">${label}: ${escape(texts[next].title)}</a>` : '<span></span>'
      result.push({
        lang,
        path: lessonPath(lang, id),
        alternate: lessonPath(other, id),
        title: `${t.lesson} ${i + 1}: ${text.title}`,
        description: summary(text.body[0]),
        body: [
          `<p class="label">${t.lesson} ${i + 1} / ${ids.length}</p>`,
          `<h1>${paragraph(text.title)}</h1>`,
          ...text.body.map((p) => `<p>${paragraph(p)}</p>`),
          `<h2>${t.task}</h2>`,
          `<p>${paragraph(text.task)}</p>`,
          `<pre><code class="language-fratm">${starter}</code></pre>`,
          `<p class="label">${t.expected}</p>`,
          `<pre class="out"><code>${expected}</code></pre>`,
          `<nav class="pager">${link(ids[i - 1], t.prev, 'prev')}${link(ids[i + 1], t.next, 'next')}</nav>`,
        ].join('\n'),
        playground: `${SITE}#scola=${i + 1}`,
        cta: t.doLesson,
      })
    })
  }
  return result
}

function sitemap(list: Page[]): string {
  const entry = (loc: string, alternates: [string, string][]) =>
    `  <url>\n    <loc>${loc}</loc>\n${alternates
      .map(([lang, href]) => `    <xhtml:link rel="alternate" hreflang="${lang}" href="${href}"/>`)
      .join('\n')}${alternates.length ? '\n' : ''}  </url>`
  const urls = [entry(SITE, [])]
  for (const page of list) {
    urls.push(entry(SITE + page.path, [[page.lang, SITE + page.path], [page.lang === 'it' ? 'en' : 'it', SITE + page.alternate]]))
  }
  return `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">
${urls.join('\n')}
</urlset>
`
}

/** Every file the plugin serves, by path under the site. */
function files(): Map<string, { type: string; source: string }> {
  const list = pages()
  const out = new Map<string, { type: string; source: string }>()
  for (const page of list) out.set(`${page.path}index.html`, { type: 'text/html; charset=utf-8', source: render(page) })
  out.set('sitemap.xml', { type: 'application/xml; charset=utf-8', source: sitemap(list) })
  out.set('robots.txt', { type: 'text/plain; charset=utf-8', source: `User-agent: *\nAllow: /\n\nSitemap: ${SITE}sitemap.xml\n` })
  return out
}

export default function sitePages(): Plugin {
  return {
    name: 'fratm-site-pages',
    // The playground itself gets the same analytics as the static pages
    transformIndexHtml(html) {
      return html.replace('</head>', `${analyticsScript()}\n  </head>`)
    },
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        const url = req.url?.split('?')[0] ?? ''
        const name = url.replace(/^\//, '').replace(/\/$/, '/index.html')
        const file = name !== 'index.html' ? files().get(name) : undefined
        if (!file) return next()
        res.setHeader('Content-Type', file.type)
        res.end(file.source)
      })
    },
    generateBundle() {
      for (const [fileName, file] of files()) {
        this.emitFile({ type: 'asset', fileName, source: file.source })
      }
    },
  }
}
