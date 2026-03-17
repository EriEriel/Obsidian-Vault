# Prisma ORM
2026-03-13  #learn #database #backend #nodejs

## what is it
Type-safe database ORM for Node.js/TypeScript. Sits between your code and the database with three main tools:
- **Client** — type-safe query API
- **Migrate** — migration system
- **Studio** — GUI for browsing data

## how it works
Define your database schema in `schema.prisma` — Prisma reads it to generate TypeScript types and manage migrations.

Two kinds of fields on every model:
- **Scalar field** (`authorId`) — the actual column in the DB (Foreign Key)
- **Relation field** (`author`) — virtual, used by Prisma Client for joins only

## example
```prisma
model User {
  id       Int       @id @default(autoincrement())
  email    String    @unique
  name     String?
  articles Article[] // one-to-many
}

model Article {
  id       Int     @id @default(autoincrement())
  title    String
  body     String?
  author   User    @relation(fields: [authorId], references: [id])
  authorId Int
}
```

## gotchas
- **Missing types error** — after any schema change, run `npx prisma generate` to update TS types in `node_modules`
- **Introspection vs migration** — `prisma db pull` reads DB → schema; `prisma migrate dev` reads schema → DB. Never mix in the same workflow
- **Nullable relations** — if optional, BOTH the relation field (`User?`) AND scalar field (`Int?`) need `?`

## quick reference
```bash
npx prisma migrate dev --name init  # create migration + update DB
npx prisma generate                 # update client types
npx prisma studio                   # open GUI
npx prisma db pull                  # introspect existing DB → schema
```

## links
- [[postgresql]]
