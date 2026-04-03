---
id: javascript
aliases: []
tags:
  - #learn
  - #javascript
---

# JavaScript Deep Dive
2026-03-13  #learn #javascript #async #fundamentals

## what is it
Core JS mechanics — type system, Symbols, BigInt, prototypes, generators, and the async model (Event Loop, Promises, async/await).

## how it works

### Tenary operator
Syntax:
```ts
  condition ? "value if true" : "value if false"

  // Example 
  fill = filled ? "currentColor" : "none"
  // Same as normal if/else
  if (filled) {
    fill = "currentColor"
  } else {
    fill = "none"
  }
```
Just read the `?` as `if` and : as `else`.

### Nullish coalescing operator `??`
Returns the right side if the left side is null or undefined:
```ts
entry.url ?? undefined
// same as
entry.url !== null ? entry.url : undefined
```

### equality & type coercion
| Operation | Primitive | Object |
|---|---|---|
| JS `===` | match value + type | match memory reference |
| JS `==` | coerces types | coerces types |
| Python `==` | match value + type | — |
| Python `is` | — | match memory reference |

```js
5 == '5'   // true  — string coerced to number
5 === '5'  // false — different types
```

### Symbols
Unique, hidden identifiers — won't show up in `Object.keys()` or standard iteration.
```js
const character = {
  name: 'Towa',
  [Symbol('CEO')]: false,
};
Object.keys(character); // ['name']
```

### BigInt
For numbers larger than `Number.MAX_SAFE_INTEGER` (2^53 - 1). Append `n`:
```js
9007199254740991n
// Cannot mix with regular numbers in math operations
```

### prototype chain
Every object has a prototype parent. If a property isn't found on the object, JS walks up the chain. Methods like `.map` exist once on `Array.prototype`, shared by all arrays — memory efficient.

### generators
Functions that can pause (`yield`) and resume (`.next()`). Return `{ value, done }`.
```js
function* counter() {
  yield 1;
  yield 2;
  yield 3;
}
```
Good for lazy-loading data or infinite sequences.

---

### async: the Event Loop
Processing order:
1. **Call Stack** — current synchronous code
2. **Microtask Queue** (high priority) — `Promise.then()`, `await` continuations
3. **Render** — browser UI update
4. **Macrotask Queue** (low priority) — `setTimeout`, `setInterval`, I/O

```js
console.log('A');                              // 1. stack
setTimeout(() => console.log('B'), 0);        // 4. macrotask
Promise.resolve().then(() => console.log('C')); // 2. microtask
console.log('D');                              // 1. stack
// Output: A → D → C → B
```

### Promises
Object representing a value not yet available. States: `Pending` → `Fulfilled` | `Rejected`.
```js
fetch(url)
  .then(res => res.json())
  .catch(err => console.error(err))
  .finally(() => setLoading(false));
```

### async / await
Syntactic sugar over Promises. `async` functions always return a Promise. Always wrap `await` in `try/catch`.
```js
async function getData() {
  try {
    const res = await fetch(url);
    return await res.json();
  } catch (err) {
    console.error(err);
  }
}
```

### parallelism with `Promise.all`
Avoid waterfall requests — fire them simultaneously:
```js
const [pineapple, strawberry] = await Promise.all([
  getFruit('Pineapple'),
  getFruit('Strawberry'),
]);
```

## gotchas
- Long sync loops block the Event Loop — microtasks and rendering stall until it finishes
- Always handle promise rejections — unhandled rejections can crash Node.js or freeze UIs in loading state forever
- `async` functions return a Promise even if you don't explicitly return one

## links
- [[react-next.js]]
