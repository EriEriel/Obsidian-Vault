# React + Next.js
2026-03-14  #learn #react #nextjs #frontend #javascript

## what is it
Declarative UI library (React) + full-stack framework (Next.js App Router). React manages UI as a component tree; Next.js adds routing, server components, and data fetching on top.

## how it works

### imperative vs declarative
- **Imperative** — step-by-step DOM manipulation ("create element, set text, append")
- **Declarative** — describe end state, React handles the DOM

```js
// Declarative JSX
root.render(<h1>Develop. Preview. Ship</h1>);
```

### core concepts

#### components
UI building blocks — functions that return JSX.
- Must be **Capitalized** (`Header` not `header`)
- Used like HTML tags: `<Header />`

#### props
Read-only arguments passed into components. Destructure in the signature:
```js
function Header({ title }) {
  return <h1>{title}</h1>;
}
```

#### JSX land vs JavaScript land
- JSX land: HTML-like syntax
- JavaScript land: anything inside `{ }` — ternaries, arithmetic, function calls, `.map()`

```jsx
{names.map((name) => (
  <li key={name}>{name}</li>
))}
```
> Always provide a unique `key` when mapping lists.

#### state (`useState`)
Information that changes over time, usually from user interaction.
```js
const [count, setCount] = useState(0);
<button onClick={() => setCount(count + 1)}>{count}</button>
```

---

### Next.js App Router

#### file-system routing
```
app/page.js          → /
app/about/page.js    → /about
app/layout.js        → shared UI (persists across navigation)
```

#### server vs client components
Next.js components are **Server Components by default**.

| | Server | Client |
|---|---|---|
| Directive | none (default) | `'use client'` at top |
| Data fetching | `async/await` directly | `useEffect` / SWR |
| Interactivity | none | full hooks + events |
| API keys | safe | exposed to browser |
| Bundle impact | zero | adds to JS bundle |

```js
// Server component — fetch directly
export default async function Page() {
  const data  = await fetch('https://api.example.com/posts');
  const posts = await data.json();
  return <ul>{posts.map(p => <li key={p.id}>{p.title}</li>)}</ul>;
}
```

```js
// Client component
'use client';
import { useState } from 'react';
export default function LikeButton() {
  const [likes, setLikes] = useState(0);
  return <button onClick={() => setLikes(likes + 1)}>{likes}</button>;
}
```

## gotchas
- Keep Client Components at the **leaves** of the tree — not wrapping large sections
- Always use `<Link>` from `next/link` for navigation — not `<a>` — to avoid full page reloads
- `useState` and event handlers only work in Client Components — adding them to a Server Component throws

## links
- [[javascript]]
- [[tailwindcss]]
- [[http-rest-api]]
