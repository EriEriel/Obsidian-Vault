---
id: advance-prisma
aliases: []
tags:
  - #prisma
  - #learn
---

# advance-prisma
2026-04-04  

## what is it
More advance prisma stuff that I ran into

## how it works
`$transaction` is use when want to resolve multiple DB operation in one request, it guarantee that whole operation will either success or fail as a whole (ACID properties: Atomic, Consistent, Isolated, Durable)

## example
```ts

const [posts, totalPosts] = await prisma.$transaction([
  prisma.post.findMany({ where: { title: { contains: "prisma" } } }),
  prisma.post.count(),
]);
```

## gotchas
When use `$transaction` with `Promise.all()` the queries inside the transaction will be executed serially

This may be counterintuitive because `Promise.all()` usually parallelizes the calls passed into it.

The reason for this behaviour is that:
* One transaction means that all queries inside it have to be run on the same connection.
* A database connection can only ever execute one query at a time.
* As one query blocks the connection while it is doing its work, putting a transaction into Promise.all effectively means that queries should be ran one after another.

## links
[[Prisma]]
[Prisma $transaction](https://www.prisma.io/docs/orm/prisma-client/queries/transactions)
