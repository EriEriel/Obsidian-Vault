---
id: staeless-authentication
aliases: []
tags:
  - #learn
  - #authentication
  - #staeless-authentication
---

# Stateless Authentication
2026-03-24

## What is it
It is a method where the server does not store session data, instead relying on client-side tokens (like JSON Web Tokens - JWT) to verify requests. Each token is self-contained, cryptographically signed, and contains user identity and validity information, enabling high scalability and seamless horizontal scaling by eliminating server-side session overhead.

## How it works
1. **User logs in** — client sends credentials to the server
2. **Server verifies** — checks credentials against the DB (once)
3. **Server issues a JWT** — signs it with a secret key and sends it back
4. **Client stores the token** — typically in an `HttpOnly` cookie (safer) or `localStorage`
5. **Every subsequent request** — client sends the token automatically
6. **Server verifies the signature** — no DB lookup needed, the token proves itself
7. **Server reads the payload** — extracts `userId`, `email`, expiry, etc. and handles the request
```
Client                          Server
  |                               |
  |-- POST /login {email, pw} --> |
  |                               | verify credentials (DB hit, once)
  |                               | sign JWT with SECRET
  |<-- Set-Cookie: token=... ---- |
  |                               |
  |-- GET /dashboard (cookie) --> |
  |                               | verify JWT signature (no DB)
  |                               | read payload → userId
  |<-- 200 OK ------------------- |
```

## Example

```ts
// lib/jwt.ts — sign and verify tokens
import { SignJWT, jwtVerify } from "jose";

const SECRET = new TextEncoder().encode(process.env.JWT_SECRET!);

export async function signToken(payload: { userId: string; email: string }) {
  return new SignJWT(payload)
    .setProtectedHeader({ alg: "HS256" })
    .setExpirationTime("7d")
    .sign(SECRET);
}

export async function verifyToken(token: string) {
  try {
    const { payload } = await jwtVerify(token, SECRET);
    return payload;
  } catch {
    return null; // expired or tampered
  }
}
\```

```ts
// middleware.ts — protect routes at the edge, no DB call
import { NextRequest, NextResponse } from "next/server";
import { verifyToken } from "@/lib/jwt";

export async function middleware(req: NextRequest) {
  const token = req.cookies.get("auth_token")?.value;
  const user = token ? await verifyToken(token) : null;

  if (!user) return NextResponse.redirect(new URL("/login", req.url));
  return NextResponse.next();
}
```

## Gotchas
- **You cannot invalidate a JWT early** — once issued, it's valid until expiry. If a user logs out or gets banned, the token is still technically valid. Solutions: short expiry + refresh tokens, or a token blocklist (which reintroduces some state).
- **Secret key rotation is painful** — changing `JWT_SECRET` instantly invalidates all active sessions.
- **Don't store JWTs in `localStorage`** — vulnerable to XSS. Use `HttpOnly` cookies instead.
- **JWT payload is only encoded, not encrypted** — don't put sensitive data (passwords, PII) in the payload. Use JWE if you need encryption.
- **Clock skew** — token expiry relies on system clocks being in sync. Add a small leeway (e.g. 30s) to account for drift between servers.
- **Bigger request size** — JWTs are larger than a simple session ID, which adds a small overhead to every request.

## Links
[[react-next.js]]

- [RFC 7519 — JWT Spec](https://datatracker.ietf.org/doc/html/rfc7519)
- [jose npm package](https://github.com/panva/jose)
- [jwt.io](https://jwt.io/)
- [Next.js Middleware docs](https://nextjs.org/docs/app/building-your-application/routing/middleware)
- [OWASP JWT Security Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/JSON_Web_Token_for_Java_Cheat_Sheet.html)
