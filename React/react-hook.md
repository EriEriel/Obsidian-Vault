---
id: react-hook
aliases: []
tags:
  - #learn
  - #javascript
  - #react
  - #react-hook
---

# react-hook
2026-03-26  

## what is it
React Hooks are special functions that allow to use state and other React features in functional components without writing a class. Introduced in version 16.8, they enable to "hook into" the React state and lifecycle directly.

## how it works
### useState()
  ```js
  const [state, setState] = useState(initialState)
```
**Parameter** is the initial state, can be values of any type.
**Return** array with two values, first is current state and second is `set` state.

### useSession() - **NextAuth.js**
`useSession` is a React Hook provided by NextAuth.js (Auth.js) to access the current session data and authentication status in Client Components.
- Provides access to the data object, which contains user information
- Automatically syncs the session state across all open browser tabs and windows.
- It can only be used in Client Components (files starting with "use client") and must be wrapped in a `<SessionProvider />`.
**Return** a status enum: "loading", "authenticated", or "unauthenticated".

### useActionState()
```tsx
  const [state, formAction, isPending] = useActionState(fn, initialState, permalink?);
```
Introduced in React 19 design to simplify state management for forma and asynchronous "Actions" 
* `state`: The current values of the state(initial State), then whatever the functions is return
* `formActtion`: A functions that pass into the <form action={formAction}> prop
* `isPending`: A boolean that is `true` while function is executing
* `fn`: Action function. It receives `(previousState, formData)` as arguments.

## example
### useState example:
```ts
import { useState } from 'react';

function MyComponent() {
  const [age, setAge] = useState(28);
  const [name, setName] = useState('Taylor');
  const [todos, setTodos] = useState(() => createTodos());
  // ...
```

### useSession() example:
```tsx
// Use the Hook
"use client";
import { useSession } from "next-auth/react";

export default function Profile() {
  const { data: session, status } = useSession();

  if (status === "loading") return <p>Loading...</p>;
  if (status === "unauthenticated") return <p>Access Denied</p>;

  return <p>Welcome, {session.user.name}</p>;
}
```

### useActionState() example:
```tsx
"use client";
import { useActionState } from "react";

async function updateName(prevState, formData) {
  const name = formData.get("name");
  // Simulate API call
  if (name === "") return { error: "Name is required" };
  return { success: true, message: `Hello, ${name}!` };
}

function ProfileForm() {
  const [state, formAction, isPending] = useActionState(updateName, { message: "" });

  return (
    <form action={formAction}>
      <input name="name" />
      <button type="submit" disabled={isPending}>
        {isPending ? "Updating..." : "Update"}
      </button>
      {state?.error && <p style={{color: 'red'}}>{state.error}</p>}
      {state?.message && <p>{state.message}</p>}
    </form>
  );
}
```

## gotchas
### useState 
It is a Hook, it can only be call at top level of component. Can't be call inside loop or conditions, in order to do that extract mew components and move state into it

### useSession
**Server Components**: For Server Components, use `getServerSession` (NextAuth v4) or `auth()` (Auth.js v5) instead of the `useSession` hook.
**Performance**: While convenient, using `useSession` adds client-side overhead. For better performance and SEO, fetching the session on the server and passing it to the client is often recommended.
## links
- [useState](https://react.dev/reference/react/useState)
* [[react-nextjs]]
