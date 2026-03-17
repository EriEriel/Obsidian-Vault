# Next.js + PostgreSQL + Prisma — Local Setup Workflow
2026-03-13  #learn #setup #nextjs #postgresql #prisma #arch

## what is it
Clean, repeatable workflow for initializing a Next.js project with local PostgreSQL and Prisma. Based on a real debugging session on Arch Linux (Omarchy) with Node.js v25, TypeScript strict mode, and `trust` authentication.

## how it works

### prerequisites
```bash
node -v          # Node.js installed
pg_isready       # PostgreSQL running
sudo -u postgres psql -c "\l"   # target DB exists
```

If DB doesn't exist yet:
```sql
CREATE DATABASE mydb;
GRANT ALL PRIVILEGES ON DATABASE mydb TO your_user;
```

## example

### step 1 — init Next.js
```bash
npx create-next-app@latest my-project
cd my-project
```

### step 2 — install Prisma
Both packages must be installed explicitly — `prisma` is the CLI, `@prisma/client` is the runtime.
```bash
npm install prisma --save-dev
npm install @prisma/client
```

### step 3 — init Prisma
```bash
npx prisma init
# Generates prisma/schema.prisma + .env
```

### step 4 — configure DATABASE_URL in `.env`
```env
DATABASE_URL="postgresql://postgres:yourpassword@localhost:5432/mydb?schema=public"
```
> On Arch with `trust` auth, the password is ignored by the server — but keep it non-empty to avoid connection string parsing issues.

### step 5 — declare ESM in `package.json`
Required if `tsconfig.json` uses `"module": "nodenext"` and `"verbatimModuleSyntax": true`.
```json
"type": "module"
```

### step 6 — configure schema
```prisma
generator client {
  provider = "prisma-client-js"
  // No custom output — generates to node_modules/@prisma/client
}

datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
}
```

### step 7 — define models
```prisma
model User {
  id        Int      @id @default(autoincrement())
  email     String   @unique
  name      String
  createdAt DateTime @default(now())
}
```

### step 8 — run first migration
```bash
npx prisma migrate dev --name init
# Creates tables + auto-runs prisma generate
```

### step 9 — import client
```ts
import { PrismaClient } from "@prisma/client"
const prisma = new PrismaClient()
```

## gotchas
| Issue | Cause | Fix |
|---|---|---|
| `P1010: User was denied access` | DB user doesn't exist | Create via `psql` |
| `Cannot find module '@prisma/client'` | Not explicitly installed | `npm install @prisma/client` |
| `ECMAScript imports cannot be written in a CommonJS file` | Missing `"type": "module"` | Add to `package.json` |
| Import resolves to wrong path | Custom `output` in generator | Remove `output` field |
| LSP not recognising generated types | Stale language server | `:LspRestart` in Neovim |

## links
- [[prisma]]
- [[prisma-setup-nextjs]]
- [[postgresql]]
- [[postgresql-cli-cheatsheet]]
