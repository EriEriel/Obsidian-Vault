---
id: shadcn-ui
aliases: []
tags:
  - #learn
  - #UI
  - #shadcn-ui
---

# shadcn-ui
2026-03-19  

## what is it
  Shadcn UI is a collection of highly customizable components(button, form, dialogs) designed for React, Next.js, and other framework. It not npm package, Instead we can paste the source code directly into project via CLI, allowing full control over styling using [[tailwindcss]] and behavior via Radix UI.

## how it works
- Initialization than choose default style. 
```bash
  # Initialize
  npx shadcn@latest init
  # Add dialog components for example 
  npx shadcn@latest add dialog
```
- This will also create new components directory 
- Can overwrite default style with normal TailwindCSS. 

## example

```tsx
"use client";
import { Button } from "@/components/ui/button"
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,} from "@/components/ui/dialog"

export default function AddEntryModal() {
  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button className="bg-white border border-gray-300 text-black text-lg rounded-lg p-6 hover:font-bold cursor-pointer">
          Add Entry
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Add New Entry</DialogTitle>
        </DialogHeader>
        {/* form goes here */}
      </DialogContent>
    </Dialog>
  )
}
```

## gotchas

## links
[[react-next.js]]
[[tailwindcss]]
