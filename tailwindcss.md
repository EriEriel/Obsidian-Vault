# Tailwind CSS
2026-03-13  #learn #setup #tailwindcss #css #frontend

## what is it
Utility-first CSS framework — style elements directly with classnames instead of writing separate CSS files.

## how it works
Instead of naming a class and writing styles in a `.css` file, you apply utility classes directly on the element:
```html
<!-- Traditional CSS -->
<p class="hello-world">Hello</p>   <!-- then .hello-world { font-size: 40px } in CSS -->

<!-- Tailwind -->
<p class="text-5xl">Hello</p>
```

Common text size classes: `text-sm`, `text-base`, `text-lg`, `text-xl`, `text-2xl` ... `text-9xl`

## example

### setup with Vite + React
```bash
npm create vite@latest my-project
# select React → JavaScript (or TypeScript)

npm install tailwindcss @tailwindcss/vite
```

In `vite.config.js`:
```js
import tailwindcss from '@tailwindcss/vite'

export default {
  plugins: [tailwindcss()]
}
```

In `index.css`:
```css
@import "tailwindcss";
```

Done — Tailwind is ready.

### setup with Next.js
Tailwind is included automatically when using `create-next-app` with the `--tailwind` flag — no manual setup needed.

## gotchas
- In React, use `className` not `class` — it's JSX
- Tailwind is responsive by default — prefix classes with breakpoints: `md:text-xl`, `lg:flex`
- Purges unused classes in production automatically — don't dynamically construct classnames with string concatenation or they'll get purged

## links
- [[react]]
- [Tailwind CSS Cheatsheet](https://www.webdevultra.com/articles/tailwindcss-cheatsheet-css-equivalents)
- [Tailwind CSS Colors](https://tailwindcss.com/docs/colors)
