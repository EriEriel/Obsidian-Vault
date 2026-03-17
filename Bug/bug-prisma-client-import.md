# bug: Prisma Client import fails after generate
2026-03-13  #bug #prisma #typescript #nodejs

## symptom
`PrismaClient` cannot be imported in `index.ts` despite the client being generated. Module not found error or type errors on import.

## reproduce
1. Init a Prisma project with `tsconfig.json` using `"module": "nodenext"` and `"verbatimModuleSyntax": true`
2. Run `npx prisma generate`
3. Try to `import { PrismaClient } from "@prisma/client"` in `index.ts`

## suspected cause
Three compounding issues:
1. `@prisma/client` not explicitly installed — generated client depends on this runtime package
2. `package.json` missing `"type": "module"` — Node treats files as CommonJS, incompatible with `import/export` under `nodenext`
3. `exactOptionalPropertyTypes: true` — `process.env["DATABASE_URL"]` can be `undefined`, causing a type mismatch where `string` is expected

## tried
- Re-running `npx prisma generate` — didn't help (client was generating fine, import was the issue)

## fix
```bash
# 1. Install the missing runtime package
npm install @prisma/client

# 2. Add to package.json
"type": "module"

# 3. Fix the type error in prisma.config.ts
url: process.env["DATABASE_URL"]!   # non-null assertion

# 4. Regenerate
npx prisma generate

# 5. Verify no type errors
npx tsc --noEmit
```

## links
- [[prisma]]
- [[nextjs-postgres-prisma-setup-workflow]]
