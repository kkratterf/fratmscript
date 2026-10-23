// Minimal Source Map v3 reader: maps a generated JS position back to the
// FratmScript source. Only what the playground needs (single source, no names).

interface Segment {
  genCol: number
  srcLine: number
  srcCol: number
}

const BASE64 = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/'
const BASE64_INDEX: Record<string, number> = {}
for (let i = 0; i < BASE64.length; i++) BASE64_INDEX[BASE64[i]] = i

function decodeVlq(segment: string): number[] {
  const values: number[] = []
  let value = 0
  let shift = 0
  for (const char of segment) {
    const digit = BASE64_INDEX[char]
    if (digit === undefined) return values
    value += (digit & 31) << shift
    if (digit & 32) {
      shift += 5
    } else {
      const negative = value & 1
      value >>>= 1
      values.push(negative ? -value : value)
      value = 0
      shift = 0
    }
  }
  return values
}

/** Parses the `mappings` field into per-generated-line segment lists (0-based). */
export function parseSourceMap(json: string): Segment[][] | null {
  let mappings: string
  try {
    mappings = JSON.parse(json).mappings
  } catch {
    return null
  }
  if (typeof mappings !== 'string' || !mappings) return null

  const lines: Segment[][] = []
  let srcLine = 0
  let srcCol = 0
  for (const line of mappings.split(';')) {
    const segments: Segment[] = []
    let genCol = 0
    for (const raw of line.split(',')) {
      if (!raw) continue
      const fields = decodeVlq(raw)
      genCol += fields[0] ?? 0
      if (fields.length >= 4) {
        srcLine += fields[2]
        srcCol += fields[3]
        segments.push({ genCol, srcLine, srcCol })
      }
    }
    lines.push(segments)
  }
  return lines
}

/**
 * Finds the source position for a 1-based generated line/column.
 * Falls back to the nearest mapped line above when the line has no mapping
 * (e.g. a closing brace). Returns 1-based line/column or null.
 */
export function originalPosition(
  map: Segment[][],
  line: number,
  column: number,
): { line: number; column: number } | null {
  for (let l = line - 1; l >= 0; l--) {
    const segments = map[l]
    if (!segments || segments.length === 0) continue
    let best = segments[0]
    if (l === line - 1) {
      for (const seg of segments) {
        if (seg.genCol <= column - 1) best = seg
      }
    } else {
      best = segments[segments.length - 1]
    }
    return { line: best.srcLine + 1, column: best.srcCol + 1 }
  }
  return null
}

/**
 * For each generated line (0-based index), the 1-based source line it was
 * generated from, or null when the line has no mapping.
 */
export function sourceLinePerGeneratedLine(map: Segment[][]): (number | null)[] {
  return map.map(segments => (segments.length > 0 ? segments[0].srcLine + 1 : null))
}
