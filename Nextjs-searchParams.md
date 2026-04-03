---
id: Nextjs-searchParams
aliases: []
tags:
  - #Nextjs-searchParams
  - #learn
  - #searchParams
  - #TypeScript
  - #prisma
---

# Nextjs-searchParams
2026-03-23  

## searchParams in Next.js

`searchParams` is a special Next.js prop automatically passed to every page component.
It gives you access to URL query parameters.

### Example
URL: `localhost:3000/?search=biboo`
```ts
searchParams = { search: "biboo" }
```

### Usage (Next.js 15+)
⚠️ In Next.js 15, `searchParams` is now a **Promise** and must be awaited.
```ts
export default async function Home({ searchParams }: { searchParams: Promise<{ search?: string }> }) {
  const { search = "" } = await searchParams
  const entries = await getEntries(search)
}
```

### Breakdown
- `searchParams` — Next.js built-in prop, no need to import
- `Promise<{ search?: string }>` — Next.js 15+ type, must be awaited
- `{ search = "" }` — destructure with default empty string fallback

### Search form (no JS needed)
```tsx
<form method="get" action="/">
  <input name="search" placeholder="Search..." />
</form>
```
- `method="get"` — puts form values in URL as query params
- Pressing Enter navigates to `/?search=biboo`
- No Server Action needed

### Prisma filter
```ts
async function getEntries(search: string = "") {
  return await prisma.entry.findMany({
    where: {
      title: {
        contains: search,
        mode: "insensitive", // case insensitive
      }
    }
  })
}
```

### Key notes
- Only available in page components (not regular components)
- **Next.js 15 breaking change** — `searchParams` is now async, always `await` it
- `method="get"` on form puts values in URL, not Server Action
- Combine with Prisma `contains` + `mode: "insensitive"` for search filtering## gotchas

## links
[[react-next.js]]

