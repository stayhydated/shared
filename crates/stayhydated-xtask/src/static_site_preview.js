import { existsSync, statSync } from "node:fs";
import { extname, join, normalize } from "node:path";

const distDir = process.env.STAYHYDATED_PREVIEW_DIST;
const basePath = process.env.STAYHYDATED_PREVIEW_BASE_PATH ?? "";
const buildHint = process.env.STAYHYDATED_PREVIEW_BUILD_HINT ?? "Build the web site first.";
const host = process.env.HOST ?? "127.0.0.1";
const requestedPort = Number(process.env.PORT ?? "8081");

if (!distDir || !existsSync(distDir)) {
  console.error(`Missing build output at ${distDir ?? "<unset>"}`);
  console.error(buildHint);
  process.exit(1);
}

function safeJoin(relativePath) {
  const normalized = normalize(relativePath).replace(/^(\.\.(\/|\\|$))+/, "");
  return join(distDir, normalized);
}

function isFile(path) {
  return existsSync(path) && statSync(path).isFile();
}

function resolveFile(pathname) {
  const relativePath = pathname.replace(/^\/+/, "");
  const directPath = safeJoin(relativePath);
  if (isFile(directPath)) {
    return directPath;
  }
  if (!extname(relativePath)) {
    const indexPath = safeJoin(join(relativePath, "index.html"));
    if (isFile(indexPath)) {
      return indexPath;
    }
  }
  return null;
}

function isAddressInUse(error) {
  return (
    typeof error === "object" &&
    error !== null &&
    (error.code === "EADDRINUSE" ||
      (typeof error.message === "string" && error.message.includes("EADDRINUSE")))
  );
}

function fetch(request) {
  const url = new URL(request.url);

  if (basePath && url.pathname === "/") {
    return Response.redirect(new URL(`${basePath}/`, url), 302);
  }

  const rootAsset = resolveFile(url.pathname);
  if (
    rootAsset &&
    (url.pathname.startsWith("/assets/") || url.pathname === "/dx-components-theme.css")
  ) {
    return new Response(Bun.file(rootAsset));
  }

  if (basePath && url.pathname !== basePath && !url.pathname.startsWith(`${basePath}/`)) {
    return new Response("Not Found", { status: 404 });
  }

  const sitePath = basePath ? url.pathname.slice(basePath.length) || "/" : url.pathname;
  const resolvedPath = resolveFile(sitePath);
  if (resolvedPath) {
    return new Response(Bun.file(resolvedPath));
  }

  const fallback = join(distDir, "404.html");
  return isFile(fallback)
    ? new Response(Bun.file(fallback), { status: 404 })
    : new Response("Not Found", { status: 404 });
}

function serve(port) {
  for (let candidatePort = port; candidatePort <= 65535; candidatePort += 1) {
    try {
      return Bun.serve({ hostname: host, port: candidatePort, fetch });
    } catch (error) {
      if (!isAddressInUse(error)) {
        throw error;
      }
    }
  }
  throw new Error(`No available port found at or above ${port}`);
}

const server = serve(requestedPort);
if (requestedPort !== 0 && server.port !== requestedPort) {
  console.warn(`Port ${requestedPort} is in use; using ${server.port} instead.`);
}
console.log(`Previewing SSG output at http://${server.hostname}:${server.port}${basePath}/`);
setInterval(() => {}, 1_000_000);
