# HTTP Methods + REST API
2026-03-13  #learn #http #rest #backend #nextjs

## what is it
Backend fundamentals — HTTP verbs, REST conventions, status codes, and how they map to Next.js App Router file structure.

## how it works

### the request flow
```
Frontend (form/button)
  → fetch()
  → API route (route.ts)
  → prisma.model.method()
  → Database (PostgreSQL)
```
Frontend never touches the database directly. Everything goes through the API route.

### HTTP methods
| Method | Action | Has Body? |
|---|---|---|
| `GET` | Read / fetch | No |
| `POST` | Create | Yes |
| `PATCH` | Partial update | Yes |
| `PUT` | Full replace | Yes |
| `DELETE` | Remove | No |

> Use `PATCH` over `PUT` — it only updates the fields you send. `PUT` replaces the whole record.

### REST rules
```
# Rule 1 — nouns in URLs, not verbs
✅ POST /api/entries
❌ POST /api/createEntry   ← method IS the verb

# Rule 2 — same URL, different methods = different actions
GET    /api/entries      → fetch all
POST   /api/entries      → create one
PATCH  /api/entries/123  → update one
DELETE /api/entries/123  → delete one

# Rule 3 — stateless
Every request must contain everything the server needs.
No session memory between calls.
```

### status codes
| Code | Meaning | When |
|---|---|---|
| `200` | OK | General success |
| `201` | Created | Row inserted into DB |
| `400` | Bad Request | Client sent wrong/missing data |
| `404` | Not Found | Resource doesn't exist |
| `500` | Internal Server Error | Something broke server-side |

### Next.js App Router file structure
```
src/app/api/
  entries/
    route.ts          ← GET all, POST new
    [id]/
      route.ts        ← GET one, PATCH, DELETE
  tags/
    route.ts
```

```ts
// Export a function named after the HTTP method
export async function GET() { ... }
export async function POST(request: Request) { ... }
export async function PATCH(request: Request) { ... }
export async function DELETE(request: Request) { ... }
```

## example
```ts
// POST /api/entries
export async function POST(request: Request) {
  const body = await request.json();

  if (!body.title || typeof body.title !== 'string') {
    return NextResponse.json({ error: 'Title is required' }, { status: 400 });
  }

  const entry = await prisma.entry.create({
    data: {
      title:    body.title,
      category: body.category || 'NOVEL',
      status:   body.status   || 'READING',
    },
  });

  return NextResponse.json(entry, { status: 201 });
}
```

## gotchas
- Always validate request body before hitting the DB — return `400` early
- Don't return `200` for everything — use `201` for created resources
- `PATCH` is almost always what you want, not `PUT`

## links
- [[prisma-entry-methods]]
- [[nextjs-postgres-prisma-setup-workflow]]
