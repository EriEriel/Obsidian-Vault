# Prisma Client — CRUD Cheatsheet
2026-03-13  #learn #prisma #database #typescript

## what is it
Quick reference for all Prisma Client CRUD operations — create, read, update, delete, aggregate. All examples use a generic model but apply to any model in your schema.

## how it works

### create & update
```typescript
// Create one
await prisma.user.create({ data: { name: 'Gura' } });

// Create many (efficient bulk insert)
await prisma.user.createMany({
  data: [{ name: 'Ame' }, { name: 'Ina' }],
  skipDuplicates: true
});

// Upsert — update if exists, create if not
await prisma.user.upsert({
  where:  { email: 'biboo@hololive.net' },
  update: { name: 'Koseki Bijou' },
  create: { name: 'Koseki Bijou', email: 'biboo@hololive.net' }
});
```

### find & filter
```typescript
// Find all
await prisma.user.findMany();

// Find by unique field
await prisma.user.findUnique({ where: { id: 1 } });

// Advanced filtering
await prisma.user.findMany({
  where: {
    OR: [
      { name: { startsWith: 'G' } },
      { email: { contains: 'hololive' } }
    ],
    AND: { status: 'ACTIVE' }
  }
});

// Include relations (join)
await prisma.user.findMany({
  include: { posts: true }
});
```

### sort & paginate
```typescript
await prisma.user.findMany({
  orderBy: { name: 'asc' },
  take: 10,   // LIMIT
  skip: 20    // OFFSET
});
```

### aggregate & group
```typescript
const total = await prisma.user.count();

const stats = await prisma.user.aggregate({
  _sum: { id: true },
  _avg: { age: true }
});

const groups = await prisma.entry.groupBy({
  by: ['category'],
  _count: { _all: true },
  orderBy: { _count: { category: 'desc' } }
});
```

### transactions
```typescript
await prisma.$transaction(async (tx) => {
  const sender   = await tx.account.update({ ... });
  const receiver = await tx.account.update({ ... });
});
```

## example
See [[prisma-entry-methods]] for a full per-method breakdown with return types.

## gotchas
- `createMany` returns `{ count: n }` — not the actual records
- `update` and `delete` **throw** if record not found; `updateMany` / `deleteMany` do not
- `select` vs `include` — don't mix at the top level; `select` picks scalar fields, `include` adds relations on top of all scalars

## links
- [[prisma]]
- [[prisma-entry-methods]]
- [[postgresql]]
