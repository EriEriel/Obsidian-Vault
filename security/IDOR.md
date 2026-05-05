---
id: "learn: IDOR"
aliases: []
tags:
  - #learn
---
2026-04-20 Init 13:57

## what is it
**Insecure Direct Object Reference (IDOR)** — a type of access control vulnerability
where an application exposes a reference to an internal object (a database ID, filename,
or URL parameter) and fails to verify that the requester is actually authorized to access it.

## how it works
The attacker modifies a reference value in a request — usually an ID in a URL, body, or
header — and the server responds with data it should have protected.

Attack flow:
1. Attacker authenticates as a legitimate user
2. They observe a resource reference in the request (e.g. `/api/shelves/42`)
3. They modify the reference (`/api/shelves/43`) to target another user's resource
4. Server processes the request **without verifying ownership**, and returns the data

The reference itself is "direct" because it maps 1:1 to an internal record (e.g. a
primary key). The vulnerability is "insecure" because access control on that reference
is missing or bypassable.

## example
### Vulnerable (before your fix)
```ts
// DELETE /api/shelf/:id
// Anyone authenticated can delete any shelf — no ownership check
export async function deleteShelf(shelfId: string) {
  await prisma.shelf.delete({ where: { id: shelfId } });
}
```

Attacker POSTs `shelfId: "abc123"` (another user's shelf) — it gets deleted.

### Fixed (what you implemented)
```ts
// Verify the requesting user owns the shelf before acting
export async function deleteShelf(shelfId: string, userId: string) {
  const shelf = await prisma.shelf.findUnique({ where: { id: shelfId } });

  if (!shelf || shelf.userId !== userId) {
    throw new Error("Forbidden");
  }

  await prisma.shelf.delete({ where: { id: shelfId } });
}
```

The fix: **always fetch the resource first, then compare its `userId` to the
session's `userId` before performing any mutation.**

## gotchas
- **Authentication ≠ authorization.** Being logged in doesn't mean you own the resource.
  These are two separate checks and IDOR exploits the gap between them.
- **Numeric sequential IDs make IDOR trivial to exploit** (just increment). UUIDs don't
  prevent IDOR — they just make guessing harder. The ownership check is still required.
- **Read endpoints are also vulnerable**, not just mutations. `GET /api/shelf/:id` leaks
  data if unauthenticated ownership isn't checked.
- **Indirect references aren't a fix.** Mapping IDs to tokens in the frontend is
  security through obscurity — the server must always enforce ownership.
- **Don't forget nested resources.** If `Entry` belongs to `Shelf` which belongs to
  `User`, deleting an entry requires checking the entry → shelf → user chain, not just
  the entry ID alone.
- In Next.js Server Actions: `session.user.id` from `auth()` is your trusted source of
  truth. Never trust a `userId` passed in from the client.

## links
- [OWASP IDOR](https://owasp.org/www-chapter-ghana/assets/slides/IDOR.pdf)
- [PortSwigger — IDOR](https://portswigger.net/web-security/access-control/idor)
- [OWASP Top 10 — Broken Access Control (A01:2021)](https://owasp.org/Top10/A01_2021-Broken_Access_Control/)
