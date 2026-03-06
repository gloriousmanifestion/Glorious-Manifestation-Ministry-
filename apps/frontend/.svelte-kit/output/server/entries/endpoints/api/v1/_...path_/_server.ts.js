function getApiUrl() {
  if (typeof process === "undefined") return "http://127.0.0.1:6830";
  return process.env.BACKEND_URL || process.env.PUBLIC_API_URL || "http://127.0.0.1:6830";
}
async function proxyToBackend(event, path, init = {}) {
  const url = event.url;
  const query = url.searchParams.toString();
  const fullPath = query ? `${path}${path.includes("?") ? "&" : "?"}${query}` : path;
  const method = init.method ?? event.request.method;
  const hasBody = method !== "GET" && method !== "HEAD";
  const body = init.body ?? (hasBody ? await event.request.arrayBuffer() : void 0);
  const headers = new Headers(init.headers);
  if (hasBody && body && !headers.has("Content-Type")) {
    const ct = event.request.headers.get("Content-Type");
    if (ct) headers.set("Content-Type", ct);
  }
  headers.delete("Host");
  const base = getApiUrl();
  const backendUrl = fullPath.startsWith("http") ? fullPath : `${base}${fullPath.startsWith("/") ? "" : "/"}${fullPath}`;
  const timeoutMs = Number(process.env.BACKEND_PROXY_TIMEOUT_MS) || 25e3;
  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), timeoutMs);
  let res;
  try {
    res = await fetch(backendUrl, {
      ...init,
      method,
      body,
      headers,
      signal: controller.signal
    });
  } catch (err) {
    const message = err instanceof Error && err.name === "AbortError" ? "Backend did not respond in time." : "Backend is temporarily unavailable.";
    return new Response(JSON.stringify({ error: "service_unavailable", message }), {
      status: 503,
      headers: { "Content-Type": "application/json" }
    });
  } finally {
    clearTimeout(timeoutId);
  }
  const outHeaders = new Headers();
  res.headers.forEach((value, key) => {
    const lower = key.toLowerCase();
    if (lower === "access-control-allow-origin") return;
    outHeaders.set(key, value);
  });
  return new Response(await res.arrayBuffer(), {
    status: res.status,
    statusText: res.statusText,
    headers: outHeaders
  });
}
const prerender = false;
function backendPath(path) {
  const segment = path?.trim() || "";
  return segment ? `/api/v1/${segment}` : "/api/v1";
}
const GET = (event) => {
  return proxyToBackend(event, backendPath(event.params.path));
};
const POST = (event) => {
  return proxyToBackend(event, backendPath(event.params.path));
};
const PUT = (event) => {
  return proxyToBackend(event, backendPath(event.params.path));
};
const PATCH = (event) => {
  return proxyToBackend(event, backendPath(event.params.path));
};
const DELETE = (event) => {
  return proxyToBackend(event, backendPath(event.params.path));
};
const OPTIONS = (event) => {
  return proxyToBackend(event, backendPath(event.params.path));
};
export {
  DELETE,
  GET,
  OPTIONS,
  PATCH,
  POST,
  PUT,
  prerender
};
