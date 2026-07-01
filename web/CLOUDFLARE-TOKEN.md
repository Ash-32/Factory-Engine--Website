# Cloudflare API token — exact setup

Your deploy failed because the token can **log in** but cannot **write Workers** (error `10000` / HTTP 403).

## Create the correct token

1. Open: https://dash.cloudflare.com/profile/api-tokens
2. Click **Create Token**
3. Click **Use template** next to **Edit Cloudflare Workers**
4. Under **Account Resources**, choose **Include** → your Cloudflare account
5. Under **Zone Resources** (if shown), choose **Include** → **Specific zone** → `factory-engine.com`
6. Click **Continue to summary** → **Create Token**
7. Copy the token into `web/.env`:
   ```
   CLOUDFLARE_API_TOKEN=paste-here
   CLOUDFLARE_ACCOUNT_ID=your-account-id
   ```

## Verify before deploy

```powershell
cd web
npx wrangler whoami
```

You should see your account **without** warnings about missing permissions.

## Deploy

```powershell
npm run deploy:domain
```

## Easier alternative (no API token)

In your own terminal (not Cursor):

```powershell
cd web
npx wrangler login
npm run deploy:domain
```

Browser login grants full Workers access automatically.

## After success

- Visit https://factory-engine.com
- **Revoke** any tokens you pasted in chat and create a fresh one kept only in `web/.env`
