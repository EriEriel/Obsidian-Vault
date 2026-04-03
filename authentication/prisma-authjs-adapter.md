---
id: prisma-authjs-adapter
aliases: []
tags:
  - #learn
  - #prisma
  - #prisma-adapter
  - #authentication
---

# prisma-authjs-adapter
2026-03-25  

## what is it
This is Prisma Auth.js ORM adapter is essentially a bridge between Auth.js and database and tell Auth.js how to use Postgresql database through prisma adapter-pg

## how it works
### Installation
```
  npm install @prisma/client @prisma/extension-accelerate @auth/prisma-adapter
  npm install prisma --save-dev
```

### Environment variable
For Postgresql it automatically generate setup `DATABASE_URL` during installation, for other DB it need to set manually
  **example**  
```prisma
    DATABASE_URL=postgresql://USER:PASSWORD@HOST:PORT/DATABASE?schema=SCHEMA
```

### To improve performance
We can ensure that only single Prisma instance is created throughout the project and then import it form any file as needed to avoid recreating instance of Prisma-client every time it used, so we import prisma instance from the `auth.js` file configuration
```ts
// prisma.ts
  import { PrismaClient } from "../src/generated/client"
  import { withAccelerate } from "@prisma/extension-accelerate"
  
  const globalForPrisma = globalThis as unknown as { prisma: PrismaClient }
  
  export const prisma =
    globalForPrisma.prisma || new PrismaClient().$extends(withAccelerate())
  
  if (process.env.NODE_ENV !== "production") globalForPrisma.prisma = prisma
  ```

```ts
// ./auth.js
  import NextAuth from "next-auth"
  import { PrismaAdapter } from "@auth/prisma-adapter"
  import { prisma } from "@/prisma"
  
  export const { handlers, auth, signIn, signOut } = NextAuth({
    adapter: PrismaAdapter(prisma),
    providers: [],
  })
```

## gotchas
- Not to confuse with **database prisma-adapter-pg** that use to connect prisma to Postgresql,think of it as a separate layer:
```
    Auth.js
      ↓  (@auth/prisma-adapter)
    Prisma
      ↓  (@prisma/adapter-pg)
    PostgreSQL
```

## links
[[prisma]]
[[prisma-7-postgresql-nextjs-setup]]
[Prisma adapter for Auth.js](https://authjs.dev/getting-started/adapters/prisma)

