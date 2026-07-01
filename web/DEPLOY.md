# Deploy Factory Engine to factory-engine.com

This site deploys to **Cloudflare Workers**. Your domain is at GoDaddy; DNS is pointed through Cloudflare.

## One-time setup (about 15 minutes)

### 1. Add your domain to Cloudflare

1. Log in at [dash.cloudflare.com](https://dash.cloudflare.com)
2. Click **Add a site** → enter `factory-engine.com` → choose the **Free** plan
3. Cloudflare shows **two nameservers** (e.g. `ada.ns.cloudflare.com`)

### 2. Point GoDaddy to Cloudflare

1. Log in at [godaddy.com](https://godaddy.com) → **My Products** → **factory-engine.com**
2. **DNS** or **Manage Domain** → **Nameservers** → **Change**
3. Choose **Enter my own nameservers (Custom)**
4. Paste Cloudflare’s two nameservers → **Save**
5. Wait 15–60 minutes for DNS to propagate

Back in Cloudflare, confirm the site status is **Active**.

### 3. Create a Cloudflare API token

1. [Cloudflare API Tokens](https://dash.cloudflare.com/profile/api-tokens) → **Create Token**
2. Use template **Edit Cloudflare Workers** → **Continue to summary** → **Create Token**
3. Copy the token (shown once)

Required permissions (if creating a custom token):
- **Account** → Workers Scripts → **Edit**
- **Account** → Workers Routes → **Edit**
- **Account** → Account Settings → **Read** (recommended)

If deploy fails with `Authentication error [code: 10000]`, your token is missing Workers **Edit** permissions — create a new token using the template above.

### 4. Get your Account ID

Cloudflare dashboard → any site → **Overview** → copy **Account ID** (right sidebar)

---

## Option A — Deploy from your computer (fastest)

In a terminal:

```bash
cd web
npx wrangler login
npm run deploy:domain
```

After deploy, in Cloudflare: **Workers & Pages** → **factory-engine-web** → **Settings** → **Domains & Routes** — confirm `factory-engine.com` and `www.factory-engine.com` are listed.

Visit **https://factory-engine.com**

---

## Option B — Deploy automatically with GitHub Actions

1. Push this repo to GitHub
2. Repo **Settings** → **Secrets and variables** → **Actions** → **New repository secret**:
   - `CLOUDFLARE_API_TOKEN` — token from step 3
   - `CLOUDFLARE_ACCOUNT_ID` — from step 4
3. Push to `main` (or run the **Deploy to Cloudflare Workers** workflow manually)

---

## Commands

| Command | Purpose |
|---------|---------|
| `npm run deploy` | Build + deploy to `*.workers.dev` (no custom domain) |
| `npm run deploy:domain` | Build + deploy with `factory-engine.com` routes |
| `npm run deploy:worker` | Redeploy existing build only |

---

## Troubleshooting

- **“Not authenticated”** → run `npx wrangler login` in the `web` folder
- **Custom domain fails** → ensure `factory-engine.com` is **Active** in Cloudflare before `deploy:domain`
- **Logo missing** → logo lives in `web/public/factory-engine-logo.png` and is copied during deploy prep
