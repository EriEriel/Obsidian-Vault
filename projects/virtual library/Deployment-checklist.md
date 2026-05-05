---
id: Deployment-checklist
aliases: []
tags: []
---

# Launch Checklist: Personal Library WebApp

## PHASE 1: EXTERNAL SERVICES (Do this now)
 - [x] **Supabase:**
   - Create a new project.
   - Go to Project Settings > Database.
   - Copy the "Transaction" connection string (Port 6543).
 - [x] **DuckDNS:**
   - Log in at duckdns.org.
   - Create a domain (e.g., `terminalshelf.duckdns.org`).
   - (Keep the tab open, you'll add the IP later).
 - [x] **OAuth Apps (GitHub/Google):**
   - Update your redirect URLs to use your new DuckDNS domain:
     `https://terminalshelf.duckdns.org/api/auth/callback/github`

## PHASE 2: DATABASE MIGRATION (From your Local PC)
 - [x] Open your terminal in your project folder.
 - [x] Run: `export DATABASE_URL="your-supabase-connection-string"`
 - [x] Run: `npx prisma migrate deploy`
   - *This creates the tables in Supabase so your app doesn't crash on start.*

## PHASE 3: VPS SETUP (Hetzner)
- [x] **Create Server:** Ubuntu 24.04 (CX22).
- [x] **SSH Login:** `ssh root@your_vps_ip`
- [x] **Install Docker:**
   ```bash
   curl -fsSL https://get.docker.com | sh
   ```
 - [x] **Update DuckDNS:** Go to the DuckDNS website and put your VPS IP next to your domain name.

## PHASE 4: DEPLOYMENT
- [x] **Transfer Files:** Use Git to clone your repo or `scp` to move files.
- [x] **Create .env:** `nano .env` and paste your production variables.
- [x] **Create Caddyfile:** `nano Caddyfile` (See configuration below).
- [x] **Launch:**
   ```bash
   docker compose up -d --build
   ```

## PHASE 5: VERIFICATION
- [x] Visit `https://yourdomain.duckdns.org`.
- [x] Check if Login works.
- [x] Check if Image uploads (Cloudinary) work.
