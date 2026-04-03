---
id: prisma-where-clause
aliases: []
tags:
  - #learn
  - #prisma
  - #prisma-where-clause
---

# prisma-where-clause
2026-03-23  

## what is it
  `where` clause operation for more advance querry, It basically a SQL `WHERE` but write in JavaScript which is a lot more convenient.

## how it works
Logical operation
```prisma
where: {
  OR: [...]   // match if ANY condition is true
  AND: [...]  // match if ALL conditions are true
  NOT: [...]  // match if condition is false
}
```

Sorting operation
```prisma
where: {
  title: {
    contains: "biboo"     // title includes "biboo"
    startsWith: "biboo"   // title starts with "biboo"
    endsWith: "biboo"     // title ends with "biboo"
    equals: "biboo"       // exact match
    mode: "insensitive"   // case insensitive
  }
}
```

Relation/Array operation
```prisma
where: {
  tags: {
    some: { name: "vtuber" }   // at least ONE tag matches
    every: { name: "vtuber" }  // ALL tags match
    none: { name: "vtuber" }   // NO tags match
  }
}
```

Number operator
```prisma
where: {
  rating: {
    gt: 3    // greater than
    gte: 3   // greater than or equal
    lt: 3    // less than
    lte: 3   // less than or equal
    equals: 3
  }
}
```

## example
Where rating is gretaer than 3 and has title or author contains biboo.
```prisma
where: {
  AND: [
    { rating: { gte: 3 } },
    { OR: [
      { title: { contains: "biboo" } },
      { author: { contains: "biboo" } }
    ]}
  ]
}
```

## gotchas
- Empty string match every thing `contains: ""` match ALL entries. This return all entries intentionally but good to know first
- `OR` with empty search when `search` is `""` every conditions will match everything
- `some` vs `every` on empty array. This call vacuous truth where entry has no tags, `every` will return `true` because nothing contradict it :
```prisma
    tags: { every: { name: "name" } }
```
- `null` field, if column is null in DB, `contains` won't throw and also won't match. So entry with no without that specific column won't show up in search
- case insensitivity by default without `mode: insensitive`

## links
[[prisma]]
