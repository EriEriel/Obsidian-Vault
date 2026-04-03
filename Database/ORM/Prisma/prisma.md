---
id: prisma
aliases: []
tags:
  - #learn
  - #ORM
  - #prisma
---
# prisma
2026-03-25  

## what is it
Prisma is a **Node.js/TypeScript ORM** (Object-Relational Mapper). Instead of writing raw SQL,
you define your database schema in a `schema.prisma` file and Prisma generates a type-safe
client to query it. It sits between your app code and the database.

Supports: PostgreSQL, MySQL, SQLite, MongoDB, CockroachDB, MS SQL Server.

## how it works
Three main pieces:

**1. Prisma Schema** (`prisma/schema.prisma`)
- Single source of truth for your DB shape
- You define `model`s — each maps to a DB table
- You run `prisma migrate dev` to push changes to the DB
- You run `prisma generate` to regenerate the client

**2. Prisma Migrate**
- Tracks schema changes as SQL migration files in `prisma/migrations/`
- `prisma migrate dev` → creates migration + applies it (dev)
- `prisma migrate deploy` → applies pending migrations (prod)

**3. Prisma Client**
- Auto-generated from your schema
- Fully type-safe: argument shapes, return types, enums all come from your schema
- Import: `import { PrismaClient } from '@prisma/client'`

Typical data flow:
```
schema.prisma → prisma generate → PrismaClient → your app queries DB
```

## example
```ts
  // prisma/schema.prisma
  model Book {
    id        Int      @id @default(autoincrement())
    title     String
    author    String
    status    Status   @default(UNREAD)
    createdAt DateTime @default(now())
  }

  enum Status {
    UNREAD
    READING
    FINISHED
  }
  ```
  ```ts
  // lib/prisma.ts — singleton pattern (important in Next.js)
  import { PrismaClient } from '@prisma/client'

  const globalForPrisma = globalThis as unknown as { prisma: PrismaClient }
  export const prisma = globalForPrisma.prisma ?? new PrismaClient()
  if (process.env.NODE_ENV !== 'production') globalForPrisma.prisma = prisma
  ```
  ```ts
  // usage
  import { prisma } from '@/lib/prisma'

  // CREATE
  const book = await prisma.book.create({
    data: { title: 'Dune', author: 'Herbert', status: 'UNREAD' }
  })

  // READ (with filter)
  const unread = await prisma.book.findMany({
    where: { status: 'UNREAD' },
    orderBy: { createdAt: 'desc' }
  })

  // UPDATE
  await prisma.book.update({
    where: { id: 1 },
    data: { status: 'READING' }
  })

  // UPSERT (create if not exists, else update)
  await prisma.book.upsert({
    where: { id: 1 },
    update: { status: 'FINISHED' },
    create: { title: 'Dune', author: 'Herbert' }
  })

  // DELETE
  await prisma.book.delete({ where: { id: 1 } })
```

## gotchas
- **Always use the singleton pattern in Next.js** — HMR creates new `PrismaClient` instances on
  every hot reload and exhausts DB connections fast if you just do `new PrismaClient()` at module level.

- **`prisma generate` after every schema change** — the client is generated code. If you change
  the schema but forget to regenerate, your types are stale and TS won't catch real mismatches.

- **`update` vs `upsert`** — `update` throws if the record doesn't exist. Use `upsert` when
  you're not sure, or `findUnique` first.

- **Prisma 7 uses Driver Adapters** — the classic connection via `DATABASE_URL` changed. If you're
  on Prisma 7+, you may need `@prisma/adapter-pg` or similar depending on your DB provider.

- **Supabase free tier is IPv6-only** — direct connections from local dev will fail unless your
  machine/ISP supports IPv6, or you use the Supabase connection pooler URL instead.

- **Relations are NOT auto-loaded** — unlike some ORMs, Prisma doesn't lazy-load. You must
  explicitly `include` or `select` related models:

```ts
  prisma.book.findMany({ include: { tags: true } })
```

- **Relations are two sided** — If any data has relation to each other it has to be two sided:

```prisma
  model User {
    // ...existing fields
    images    Image[]
  }

  model Entry {
    // ...existing fields
    image     Image?
  }

  model Image {
      user      User     @relation(fields: [userId], references: [id])
      entry     Entry?   @relation(fields: [entryId], references: [id])
  }
```

- **Enums must match exactly** — when passing enum values in TypeScript, use the generated
  enum type from `@prisma/client`, not a plain string, to avoid runtime mismatches.

## links
- [Official site](https://www.prisma.io/docs)
- [Getting-started](https://www.prisma.io/docs/orm/prisma-migrate/getting-started)
- [Prisma method reference](https://www.prisma.io/docs/orm/reference/prisma-client-reference)

