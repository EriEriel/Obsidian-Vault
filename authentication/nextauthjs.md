---
id: nextauthjs
aliases: []
tags:
  - #learn
  - #Nextjs
  - #authentication
---

# nextauthjs
2026-03-24  

## what is it
  **Authentication** is the security process of verifying the identity of a user, device, or system before granting access to resources

## how it works
### `useSession` and `<SessionProvider />`
  If we want to use [[useSession ]] first we need to expose the session context with `<SessionProvider />`

## example
### Add API route
  For `NextAuth.js` create a file `pages/api/auth/[...nextauth].js` This file contain global configuration and route and for Nextjs 13.2 and above with new App router, we can initialize config by [route handler](https://next-auth.js.org/configuration/initialization#route-handlers-app)

```js
  import NextAuth from "next-auth"
  import GithubProvider from "next-auth/providers/github"

  export const authOptions = {
    // Configure one or more authentication providers
    providers: [
      GithubProvider({
        clientId: process.env.GITHUB_ID,
        clientSecret: process.env.GITHUB_SECRET,
      }),
      // ...add more providers here
    ],
  }

  export default NextAuth(authOptions)
```


All requests to `/api/auth/*` (`signIn`, `callback`, `signOut`, etc.) will automatically be handled by NextAuth.js.

### Frontend - Add React Hook
`Components/login-btn.jsx`
```js
  import { useSession, signIn, signOut } from "next-auth/react"

  export default function Component() {
    const { data: session } = useSession()
    if (session) {
      return (
        <>
          Signed in as {session.user.email} <br />
          <button onClick={() => signOut()}>Sign out</button>
        </>
      )
    }
    return (
      <>
        Not signed in <br />
        <button onClick={() => signIn()}>Sign in</button>
      </>
    )
  }
```

### Backend - API Route
`pages/api/restricted.js`
```js
  import { getServerSession } from "next-auth/next"
  import { authOptions } from "./auth/[...nextauth]"

  export default async (req, res) => {
    const session = await getServerSession(req, res, authOptions)

    if (session) {
      res.send({
        content:
          "This is protected content. You can access this content because you are signed in.",
      })
    } else {
      - [ ] res.send({
        error: "You must be signed in to view the protected content on this page.",
      })
    }
  }
```

## gotchas

## links
[Getting start NextAuth.js](https://next-auth.js.org/getting-started/example)
[[react-next.js]]

