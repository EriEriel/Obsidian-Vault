---
id: "learn: playwright"
aliases: []
tags:
  - learn
---
2026-04-25 Init 15:42
## what is it
**Playwright** is a modern E2E (End-to-End) testing framework by Microsoft. It
supports multiple browsers (Chromium, Firefox, WebKit) and platforms. It allows
you to write tests that simulate real user interactions with your web app,
verifying that everything works as expected. Think of it as a "robot user" that
clicks, types, and navigates your app automatically.

## how it works
To get started with Playwright, first install the package:
```bash
npm init playwright@latest -- --lang=TypeScript --gha --quiet
```
`--lang=TypeScript` sets up TypeScript support, `--gha` configures GitHub
Actions for CI, and `--quiet` skips interactive prompts.

After install, two key files are generated:
- `playwright.config.ts` — global config (baseURL, browsers, webServer, etc.)
- `tests/example.spec.ts` — starter test to learn from

A basic test looks like:
```ts
import { test, expect } from '@playwright/test';

test('homepage has footer text', async ({ page }) => {
  await page.goto('/');
  await expect(page.getByText('SOME_TEXT')).toBeVisible();
});
```

### Config essentials
In `playwright.config.ts`, set `baseURL` and `webServer` so Playwright knows
where your app lives and how to start it:
```ts
use: {
  baseURL: 'http://localhost:3000',
},
webServer: {
  command: 'npm run dev',
  url: 'http://localhost:3000',
  reuseExistingServer: !process.env.CI,
},
```

### Authentication with storageState
Instead of logging in via UI before every test (slow and brittle), log in once
in a `setup` project, save the session to a file, and reuse it:

```ts
// playwright.config.ts
projects: [
  { name: 'setup', testMatch: /.*\.setup\.ts/ },
  {
    name: 'chromium',
    use: {
      ...devices['Desktop Chrome'],
      storageState: 'playwright/.auth/user.json',
    },
    dependencies: ['setup'],
  },
],
```

```ts
// tests/auth.setup.ts
const authFile = 'playwright/.auth/user.json';

setup('authenticate', async ({ page }) => {
  await page.goto('/api/test/login?email=test@example.com');
  await expect(page).toHaveURL(/\/$/);
  await page.context().storageState({ path: authFile });
});
```

### The NextAuth testing problem & the Trapdoor pattern
NextAuth blocks headless browser logins via CSRF protection — the robot gets
rejected even with correct credentials. The solution is a test-only API route
that signs a real JWT server-side and sets the session cookie directly,
bypassing the login form entirely:

```ts
// src/app/api/test/login/route.ts
import { prisma } from "@/lib/prisma";
import { NextResponse } from "next/server";
import { encode } from "next-auth/jwt";

export async function GET(request: Request) {
  if (process.env.NODE_ENV === "production") {
    return new NextResponse("Not Allowed", { status: 403 });
  }

  const email = new URL(request.url).searchParams.get("email");
  const user = await prisma.user.findUnique({ where: { email! } });
  if (!user) return new NextResponse("User not found", { status: 404 });

  const token = await encode({
    token: { id: user.id, name: user.name, email: user.email },
    salt: "authjs.session-token",
    secret: process.env.AUTH_SECRET!,
  });

  const response = NextResponse.redirect(new URL("/", request.url));
  response.cookies.set("authjs.session-token", token, {
    httpOnly: true,
    secure: false, // only true in production
    sameSite: "lax",
    path: "/",
  });

  return response;
}
```

This pattern is safe because of the `NODE_ENV === "production"` guard.

## example
```ts
// Full CRUD lifecycle test
test('should add, edit, and delete an entry', async ({ page }) => {
  const title = `Test Book ${Date.now()}`;

  await page.goto('/archive');

  // Create
  await page.click('button:has-text("Add Entry")');
  await page.fill('#title', title);
  await page.click('button:has-text("save entry")');
  await expect(page.getByText(title, { exact: true })).toBeVisible();

  // Edit
  const card = page.locator('div.relative', { hasText: title });
  await card.locator('button:has-text("edit")').click();
  await page.fill('#title', title + ' (Edited)');
  await page.click('button:has-text("save changes")');
  await expect(page.getByText(title + ' (Edited)', { exact: true })).toBeVisible();

  // Delete
  const editedCard = page.locator('div.relative', { hasText: title + ' (Edited)' });
  await editedCard.locator('button:has-text("edit")').click();
  await page.click('button:has-text("delete")');
  await page.click('button:has-text("confirm delete")');
  await expect(page.getByText(title + ' (Edited)', { exact: true })).not.toBeVisible();
});
```

## gotchas
- **Use `{ exact: true }` when verifying text changes.** `getByText("foo")` will
  match anything *containing* "foo" — so `"foo (Edited)"` passes the check and
  your `.not.toBeVisible()` assertion silently breaks.
- **NextAuth + headless browsers don't mix.** CSRF protection blocks the robot
  from logging in via the UI. Use the Trapdoor API pattern above instead.
- **Use a pre-existing test user, not dynamic registration.** Registering then
  immediately logging in has race conditions — the session cookie may not be set
  before the redirect check runs.
- **`storageState` reuse is essential for speed.** Without it, every test that
  needs auth would have to log in fresh, making your suite slow and fragile.
- **`reuseExistingServer: !process.env.CI`** lets you run tests against your
  already-running dev server locally, while CI always spins up a fresh one.

## links
- https://playwright.dev/docs/intro
- https://dev.to/kuroski/writing-integration-tests-for-nextjs-next-auth-prisma-using-playwright-and-msw-388m
