---
id: testing
aliases: []
tags:
  - learn
  - testing
  - javascript
  - typescript
  - stub
---

# Testing & Unit Tests

## what is it
Writing automated tests to verify code behaves correctly. Unit tests check individual functions/components in isolation. Integration tests check how pieces work together. E2E tests simulate real user flows.

> Not studied properly yet. This is a placeholder.

## topics to cover

### core concepts
- Unit vs integration vs E2E — what each tests and when to use which
- The testing pyramid — why more unit tests, fewer E2E
- Arrange / Act / Assert pattern (AAA)
- What makes a good test — isolated, deterministic, fast
- Test coverage — what it means and why 100% isn't the goal

### JavaScript / TypeScript testing tools
- **Vitest** — fast unit testing for Vite/modern projects (preferred over Jest for new projects)
- **Jest** — most widely used, good to know
- `describe()` / `it()` / `test()` blocks
- `expect()` matchers: `.toBe()`, `.toEqual()`, `.toContain()`, `.toThrow()`, etc.
- `beforeEach()` / `afterEach()` / `beforeAll()` / `afterAll()`
- Mocking: `vi.fn()`, `vi.mock()`, `vi.spyOn()`

### React component testing
- **React Testing Library** — test behavior not implementation
- `render()`, `screen`, `fireEvent`, `userEvent`
- Querying: `getByText`, `getByRole`, `getByTestId`
- Why to avoid testing implementation details

### API / backend testing
- Testing Next.js API routes
- Mocking Prisma client in tests
- Testing with a real test database vs mocks — tradeoffs

### Python testing
- **pytest** — standard Python test framework
- `assert` statements
- Fixtures
- `unittest.mock` — mock objects and patch

### E2E testing
- **Playwright** — modern, supports multiple browsers
- **Cypress** — popular alternative
- When E2E is worth the cost

### TDD (Test Driven Development)
- Red / Green / Refactor cycle
- Writing tests before implementation
- When TDD is practical vs overhead

### CI integration
- Running tests in GitHub Actions
- Failing builds on test failure

## links
- [[typescript]]
- [[javascript]]
- [[react-next.js]]
- [[http-rest-api]]
- [[playwright]]
