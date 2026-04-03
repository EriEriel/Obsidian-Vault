---
id: javscript-web-token
aliases: []
tags:
  - #learn
  - #jwt
---

# javscript-web-token
2026-03-25

## what is it
  A JSON Web Token (JWT) is an open standard [RFC 7519](https://datatracker.ietf.org/doc/html/rfc7519) for securely transmitting information between two parties — typically a client and a server.
  Each JWT is digitally signed to prevent tampering and contains claims (pieces of information) about the user or session, it is the industry standard for **stateless** authentication.
  Stateless is mean it self-contain, we don't need to hit DB for authentication to verify the user.

### Two major token types:
- **Opaque** tokens — random identifiers referencing session data on the server.
- **JWTs** — self-contained tokens that include verifiable claims directly within the token.

JWTs allow local verification without needing a database call, enabling stateless, high-performance authentication.

## how it works
### Structure of a JWT
A JWT has three parts, separated by dots (.):
1. Header
Defines the type `JWT` and the algorithm used to sign the token e.g., `HS256`.

2. Payload
Contains the actual claims — such as user ID, expiration time, and roles.

3. Signature
Ensures integrity by signing the header and payload using a secret or public/private key pair.

Example: `header.payload.signature`

### [RFC 7519](https://datatracker.ietf.org/doc/html/rfc7519) standard claim conventions
- iss — Issuer (the authority generating the token)
- sub — Subject (user identifier)
- aud — Audience (the intended recipient app)
- exp — Expiration time
- iat — Issued at time
- nbf — Not before (token validity start)
- email, email_verified, roles — Application-specific claims
Using standardized claims ensures interoperability across libraries and identity providers (e.g., Auth0, AWS Cognito, SuperTokens, Google Identity).

### How Do JWTs Work?
JWT authentication typically follows this flow:

1. User logs in — Authentication server validates credentials.
2. JWT issued — Server signs and returns a JWT containing claims.
3. Client stores token — Usually in HttpOnly cookies or secure storage.
4. Requests authenticated — Client includes JWT in headers (e.g., `Authorization: Bearer <token>`).
5. Server verifies JWT — Using its secret or public key, checks signature validity and claim expiry.
This stateless model removes the need for a centralized session store, boosting scalability.

## example
### example of JWT issued by Google sign-in:
```js
{
  "iss": "https://accounts.google.com",
  "azp": "1234987819200.apps.googleusercontent.com",
  "aud": "1234987819200.apps.googleusercontent.com",
  "sub": "10769150350006150715113082367",
  "email": "jsmith@example.com",
  "email_verified": true,
  "iat": 1353601026,
  "exp": 1353604926,
  "nonce": "0394852-3190485-2490358",
  "hd": "example.com"
}
```

## gotchas
### authentication vs authorization
**Authentication** is to verify user while **Authorization** give the level of credentials that user can do.

### Limitation
- Difficult to revoke: Tokens remain valid until expiration.
- Key compromise risk: A leaked secret allows attackers to forge tokens.
- Size overhead: Larger than opaque tokens due to embedded JSON.

## links
[JWT Reference](https://supertokens.com/blog/what-is-jwt)
[[nextauthjs]]
[[staeless-authentication]]
