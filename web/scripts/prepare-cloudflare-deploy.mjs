import { cpSync, existsSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const serverDir = join(root, ".output", "server");
const publicDir = join(root, ".output", "public");
const wranglerPath = join(serverDir, "wrangler.json");
const deployConfigPath = join(root, ".wrangler", "deploy", "config.json");

if (!existsSync(wranglerPath)) {
  console.error("Missing .output/server/wrangler.json — run npm run build first.");
  process.exit(1);
}

// Nitro generates this file; it conflicts with --cwd deploy paths in Wrangler 4.
if (existsSync(deployConfigPath)) {
  rmSync(deployConfigPath);
}

const sourcePublic = join(root, "public");
if (existsSync(sourcePublic)) {
  cpSync(sourcePublic, publicDir, { recursive: true, force: true });
}

const withDomain = process.argv.includes("--domain");

const wrangler = JSON.parse(readFileSync(wranglerPath, "utf8"));
wrangler.name = "factory-engine-web";
if (process.env.CLOUDFLARE_ACCOUNT_ID) {
  wrangler.account_id = process.env.CLOUDFLARE_ACCOUNT_ID;
}
if (withDomain) {
  wrangler.routes = [
    { pattern: "factory-engine.com", custom_domain: true },
    { pattern: "www.factory-engine.com", custom_domain: true },
  ];
} else {
  delete wrangler.routes;
}
writeFileSync(wranglerPath, JSON.stringify(wrangler, null, 2) + "\n");

console.log("Prepared Cloudflare deploy:");
console.log("  worker:", wrangler.name);
if (withDomain) {
  console.log("  routes:", wrangler.routes.map((r) => r.pattern).join(", "));
} else {
  console.log("  routes: (workers.dev only — run deploy:domain after DNS is set up)");
}
