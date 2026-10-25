// Serves a fetch-style app (`app.fetch(request) => response`, as Hono and
// other Web-standard servers have) from a Node or Connect middleware.

/** The Web `Request` for a Node `IncomingMessage`. */
export async function toRequest(req) {
  const url = new URL(req.originalUrl ?? req.url ?? '/', `http://${req.headers.host ?? 'localhost'}`);
  const headers = new Headers();
  for (const [name, value] of Object.entries(req.headers)) {
    if (value === undefined) continue;
    for (const item of Array.isArray(value) ? value : [value]) headers.append(name, item);
  }
  const init = { method: req.method, headers };
  if (req.method !== 'GET' && req.method !== 'HEAD') {
    const chunks = [];
    for await (const chunk of req) chunks.push(chunk);
    init.body = Buffer.concat(chunks);
  }
  return new Request(url, init);
}

/** Writes a Web `Response` to a Node `ServerResponse`. */
export async function sendResponse(res, response) {
  res.statusCode = response.status;
  for (const [name, value] of response.headers) {
    if (name !== 'set-cookie') res.setHeader(name, value);
  }
  const cookies = response.headers.getSetCookie?.() ?? [];
  if (cookies.length > 0) res.setHeader('set-cookie', cookies);
  res.end(Buffer.from(await response.arrayBuffer()));
}

/** Whether `url` is `prefix` or below it: `/api` matches `/api/pizze`, not `/apiario`. */
export function underPrefix(url, prefix) {
  const path = (url ?? '/').split(/[?#]/, 1)[0];
  return path === prefix || path.startsWith(prefix.endsWith('/') ? prefix : `${prefix}/`);
}
