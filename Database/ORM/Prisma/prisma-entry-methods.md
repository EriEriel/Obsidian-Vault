
# Prisma Entry Methods
2026-03-13  #learn #prisma #database #typescript

## what is it
Per-method reference for Prisma Client — what each method requires, what it returns, and whether it throws. All examples use `prisma.entry` but apply to any model.

## how it works

### READ

#### `findMany` — get multiple records
```ts
const entries = await prisma.entry.findMany({
  where:   { status: 'READING' },
  orderBy: { updatedAt: 'desc' },
  include: { tags: true },
  take:    10,
  skip:    20,
});
```
- Required: nothing
- Returns: array (empty `[]` if none found)

---

#### `findUnique` — get one by unique field
```ts
const entry = await prisma.entry.findUnique({
  where: { id: 1 }, // must be unique/primary key
});
```
- Required: `where` with a unique field
- Returns: record or `null`

---

#### `findFirst` — get first match (non-unique field ok)
```ts
const entry = await prisma.entry.findFirst({
  where:   { title: 'Some Novel' },
  orderBy: { createdAt: 'asc' },
});
```
- Required: nothing (but `where` is the point)
- Returns: record or `null`

---

### CREATE

#### `create` — insert one record
```ts
const entry = await prisma.entry.create({
  data: {
    title:    'My Novel',
    category: 'NOVEL',
    status:   'READING',
  },
});
```
- Required: `data` with all non-nullable, non-default fields
- Returns: created record

---

#### `createMany` — insert multiple records
```ts
await prisma.entry.createMany({
  data: [
    { title: 'Novel A', category: 'NOVEL' },
    { title: 'Novel B', category: 'FANFIC' },
  ],
  skipDuplicates: true,
});
```
- Required: `data` array
- Returns: `{ count: n }` — NOT the records themselves

---

### UPDATE

#### `update` — update one by unique field
```ts
const updated = await prisma.entry.update({
  where: { id: 1 },
  data:  { status: 'DROPPED' },
});
```
- Required: `where` (unique), `data`
- Returns: updated record
- **Throws** if not found

---

#### `updateMany` — update multiple records
```ts
await prisma.entry.updateMany({
  where: { status: 'READING' },
  data:  { status: 'ON_HOLD' },
});
```
- Required: `data`
- Returns: `{ count: n }`
- Does NOT throw if nothing matches

---

#### `upsert` — update if exists, create if not
```ts
const entry = await prisma.entry.upsert({
  where:  { id: 1 },
  update: { status: 'DONE' },
  create: { title: 'New Novel', category: 'NOVEL', status: 'DONE' },
});
```
- Required: `where`, `update`, `create` — all three
- Returns: the record (created or updated)

---

### DELETE

#### `delete` — delete one by unique field
```ts
await prisma.entry.delete({ where: { id: 1 } });
```
- Required: `where` (unique)
- Returns: deleted record
- **Throws** if not found

---

#### `deleteMany` — delete multiple records
```ts
await prisma.entry.deleteMany({ where: { status: 'DROPPED' } });
```
- Required: nothing (omitting `where` deletes ALL — careful)
- Returns: `{ count: n }`

---

### COUNT / AGGREGATE

```ts
const total = await prisma.entry.count({ where: { status: 'READING' } });

const result = await prisma.entry.aggregate({
  _count: true,
  _avg:   { rating: true },
  _max:   { rating: true },
  where:  { category: 'NOVEL' },
});

const groups = await prisma.entry.groupBy({
  by:       ['category'],
  _count:   { _all: true },
  orderBy:  { _count: { category: 'desc' } },
});
```

## example

| Method | Finds by | Returns | Throws if missing? |
|---|---|---|---|
| `findMany` | anything | array | no |
| `findUnique` | unique field | record \| null | no |
| `findFirst` | anything | record \| null | no |
| `create` | — | record | — |
| `createMany` | — | `{ count }` | — |
| `update` | unique field | record | **yes** |
| `updateMany` | anything | `{ count }` | no |
| `upsert` | unique field | record | no |
| `delete` | unique field | record | **yes** |
| `deleteMany` | anything | `{ count }` | no |
| `count` | anything | number | no |

## gotchas
- `update` and `delete` throw — always wrap in `try/catch` or check existence first with `findUnique`
- `deleteMany` with no `where` deletes the entire table — always double check
- `select` vs `include` — use `select` for specific scalar fields only; use `include` to add relations on top. Never mix at top level

## links
- [[prisma]]
- [[prisma-cheatsheet]]
