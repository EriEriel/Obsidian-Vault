# Prisma 7 + PostgreSQL + Next.js Setup
2026-03-13  #learn #setup #prisma #nextjs #postgresql

## what is it
Step-by-step setup for Prisma 7 with a production-ready PostgreSQL connection pool in a Next.js App Router project using Supabase as the hosted DB.

## how it works
Prisma 7 separates the connection URL from the schema — the `datasource` block has no `url`. Instead, the connection is handled in code via `pg.Pool` + `@prisma/adapter-pg`. The `DIRECT_URL` is only used by the Prisma CLI for migrations.

Two connection strings are needed:
- `DATABASE_URL` — pooled (port `6543`) — used at runtime
- `DIRECT_URL` — direct (port `5432`) — used only by `prisma migrate`

## example

### 1. Init project
```bash
npx create-next-app@latest my-app --typescript --tailwind --eslint --app --src-dir
cd my-app
```
> `--src-dir` keeps the root clean. Next.js handles `tsconfig.json` — do NOT manually set `NodeNext` like in a vanilla TS project.

### 2. Install dependencies
```bash
npm install @prisma/client pg @prisma/adapter-pg
npm install prisma @types/pg -D
```
> No need for `dotenv` or `tsx` — Next.js handles both automatically.

### 3. Init Prisma
```bash
npx prisma init
```
Rename the generated `.env` → `.env.local` (Next.js convention, already in `.gitignore`).

### 4. Configure `.env.local`
```bash
# Pooled — runtime
DATABASE_URL="postgresql://postgres.[ref]:[password]@aws-0-[region].pooler.supabase.com:6543/postgres"

# Direct — migrations only
DIRECT_URL="postgresql://postgres.[ref]:[password]@aws-0-[region].pooler.supabase.com:5432/postgres"
```

### 5. Schema (`prisma/schema.prisma`)
```prisma
generator client {
  provider = "prisma-client-js"
  output   = "../generated/prisma"
}

datasource db {
  provider  = "postgresql"
  // No url here — intentional Prisma 7 pattern
  directUrl = env("DIRECT_URL")
}
```

### 6. DB client singleton (`src/lib/db.ts`)
```typescript
import pg from "pg";
import { PrismaPg } from "@prisma/adapter-pg";
import { PrismaClient } from "../../generated/prisma";

const globalForPrisma = globalThis as unknown as { prisma: PrismaClient };

function createPrismaClient() {
  const pool = new pg.Pool({ connectionString: process.env.DATABASE_URL });
  const adapter = new PrismaPg(pool);
  return new PrismaClient({ adapter });
}

export const prisma = globalForPrisma.prisma ?? createPrismaClient();

if (process.env.NODE_ENV !== "production") {
  globalForPrisma.prisma = prisma;
}
```
> `globalThis` pattern prevents multiple PrismaClient instances during Next.js hot reload — each file save would otherwise exhaust the connection pool.

### 7. Usage in API routes
```typescript
import { prisma } from "@/lib/db";
import { NextResponse } from "next/server";

export async function GET() {
  const entries = await prisma.entry.findMany();
  return NextResponse.json(entries);
}

export async function POST(request: Request) {
  const body = await request.json();
  const entry = await prisma.entry.create({ data: body });
  return NextResponse.json(entry, { status: 201 });
}
```

### 8. Day-to-day workflow
```bash
# 1. Edit prisma/schema.prisma
# 2. Push schema + regenerate client
npx prisma migrate dev --name describe_your_change
# 3. Only if you changed schema without migrating (rare)
npx prisma generate
# 4. Start dev server
npm run dev
```

## gotchas
- Always migrate before running — if code references a model not yet in the DB you get a runtime error
- `prisma studio` opens at `localhost:5555` — useful for debugging data directly
- No `disconnect()` needed in Next.js — the framework manages the process lifecycle

## links
- [[prisma]]
- [[postgresql]]
- [[nextjs-postgres-prisma-setup-workflow]]
