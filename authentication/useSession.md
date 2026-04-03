---
id: useSession
aliases: []
tags:
  - #learn
  - #Nextjs
  - #authentication
  - #useSession
---

# useSession
2026-03-25  

## what is it
  `useSession()` is a React Hook provided by NextAuth.js (Auth.js) used to access the current session state in Client Components.
It allows you to check if a user is signed in and access their profile data (like name or email) without needing to fetch it manually

## how it works
Need `use client` and warp in `<SessionProvider />` In the App Router, this usually means creating a separate client-side "Provider" component to wrap children in the root layout.js

## example

```js
"use client";
import { useSession } from "next-auth/react";

export default function Profile() {
  const { data: session, status } = useSession();

  if (status === "loading") return <p>Loading...</p>;
  if (status === "unauthenticated") return <p>Access Denied</p>;

  return <p>Welcome, {session.user.name}!</p>;
}

\```

## gotchas

## links

