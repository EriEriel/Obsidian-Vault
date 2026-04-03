# TypeScript
#learn #typescript #javascript #stub

## what is it
Typed superset of JavaScript — compiles to plain JS. Adds static type checking, interfaces, generics, and better tooling to JS.

> Not studied properly yet. This is a placeholder.

## topics to cover

### type system basics
- Primitive types: `string`, `number`, `boolean`, `null`, `undefined`, `symbol`
- `any`, `unknown`, `never`, `void`
- Type inference vs explicit annotation
- Union types: `string | number`
- Intersection types: `A & B`
- Literal types: `"left" | "right"`

### interfaces and types
- `interface` vs `type` alias — when to use which
- Optional properties: `name?: string`
- Readonly properties
- Extending interfaces
- Index signatures

### functions
- Typed parameters and return types
- Optional and default parameters
- Function overloads
- Rest parameters

### generics
- Generic functions: `function identity<T>(arg: T): T`
- Generic interfaces and types
- Constraints: `<T extends object>`
- Built-in utility types: `Partial<T>`, `Required<T>`, `Pick<T,K>`, `Omit<T,K>`, `Record<K,V>`, `ReturnType<T>`

### classes
- Access modifiers: `public`, `private`, `protected`
- `readonly`
- Abstract classes
- Implementing interfaces

### advanced types
- Type narrowing — `typeof`, `instanceof`, `in`
- Type guards
- Discriminated unions
- Mapped types
- Conditional types
- Template literal types

### configuration
- `tsconfig.json` key options
- `strict` mode (what it enables)
- `nodenext` module resolution
- `verbatimModuleSyntax`

### with React / Next.js
- Typing props and state
- `React.FC` vs plain function
- Event handler types
- `useState<T>()` generics
- Typing API responses

### gotchas to learn
- `any` vs `unknown` — why `unknown` is safer
- `as` type assertion — when it's a code smell
- Non-null assertion `!` — when it's acceptable
- Why `exactOptionalPropertyTypes` causes issues with `process.env`

## links
- [[javascript]]
- [[react-next.js]]
- [[testing]]
