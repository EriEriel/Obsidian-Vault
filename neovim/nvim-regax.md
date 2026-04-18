---
id: "learn: nvim-regex"
aliases: []
tags:
  - #learn
  - #nvim
  - #regax
---

2026-04-06 Init 15:53
## what is it
Vim's built-in pattern matching and substitution system. Used to search, replace, and manipulate text across a file using regex-style patterns — all without leaving the editor.

## how it works
The core command is `:s` (substitute), combined with a range and flags:

```
:[range]s/pattern/replacement/[flags]
```

- **range** — which lines to operate on (`%` = whole file, `'<,'>` = selection, `5,20` = line range)
- **pattern** — what to find (supports regex)
- **replacement** — what to replace it with
- **flags** — modifiers like `g` (global), `c` (confirm), `i` (case insensitive)

Note: Vim uses "magic" mode by default — slightly different from standard regex. Some characters like `+`, `?`, `|`, `(`, `)` need a backslash prefix (`\+`, `\?`, `\|`, `\(`, `\)`).

### In patterns
```
  .     " any character
  *     " zero or more of previous
  \+    " one or more
  \?    " zero or one (optional)
  ^     " start of line
  $     " end of line
  \w    " word character
  \s    " whitespace
```

## example
```
" Basic replace all in file
:%s/foo/bar/g

" Case insensitive
:%s/foo/bar/gi

" Whole word only
:%s/\bfoo\b/bar/g

" Replace foo OR bar
:%s/foo\|bar/baz/g

" Capture group — wrap foo in brackets → [foo]
:%s/\(foo\)/[\1]/g

" Add // to start of every line
:%s/^/\/\/ /g

" Trim trailing whitespace
:%s/ *$//g

" Only in visual selection (press : in visual mode, range auto-fills)
:'<,'>s/foo/bar/g

" Delete every line containing foo
:g/foo/d

" Delete every line NOT containing foo
:v/foo/d
```

## gotchas
- `%s` without `g` flag only replaces the **first match per line**, not all
- Vim regex is not the same as PCRE — `+`, `(`, `)`, `|` need backslash: `\+`, `\(`, `\)`, `\|`
- Special characters in the replacement like `&` mean "the whole match" — escape with `\&` if you want a literal `&`
- `\1` in replacement refers to the first capture group `\(...\)` — very useful for restructuring
- `:g/pattern/d` is destructive and has no confirmation — double check your pattern first with `/pattern` search

## links
- https://vim.fandom.com/wiki/Search_and_replace
- https://learnvim.irian.to/basics/search_and_substitute
