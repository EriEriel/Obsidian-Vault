---
id: "learn: luasnip"
aliases: []
tags:
  - #learn
  - #neovim
  - #neovim-plugin
---

2026-04-05 Init 14:31

## what is it
**Luasnip** is a snip engine in Neovim that let's user instance generate customizable snippet of code or boiler plate out of Luasnip building block

## how it works
### Creating a snippets
Every snip go into `~/.config/nvim/lua/snippets/`

#### The Anatomy of Snippet
Every snippet follow this pattern `s("trigger", nodes)`
  * **Trigger**: The world that type in (e.g., `main`, `if`, `fn`)
  * **Nodes**: What snippet actually does

#### Building block
| Nodes | Name | Use Case | Example |
| --------------- | --------------- | --------------- | --------------- |
| s | Snippet | The whole snippet block | `s("trigger", nodes)` |
| t | Text | Static text that nerver chagne | `t("printf")` |
| i | Insert | A "Tab-stop" and can type in. | `i(1, "default")` |
| c | Choice | A list of options to cycle through | `c(1, {t("public"), t("private")})` | 
| fmt | Format | Use `{}` as placeholders for nodes | `fmt("if ({}) {{ {} }}", {i(1), i(2)})` |
| d | Dynamic | Programable for decide what it should contain in node | `d(1, function(_, snip)` |
| f | Function | Mirroring Input, Dynamic Metadata, etc | `f(function(_, snip)` |
| sn | Snippet node | The container that return object node for `d` | `return sn(nil, { i(1, selection) })` |

### Basic usage
Type the trigger 

## example
```lua
-- C struct boiler plate 
return {
  s("str", fmt([[
typedef struct {{
    {}
}} {};
  ]], {
    i(1, "// fields"), -- first jump
    i(2, "MyStruct")   -- second jump (the name)
  })),
}

-- Rust logic snippet
return {
  s("matchres", fmt([[
match {} {{
    Ok(val) => {{
        {}
    }}
    Err(e) => {{
        eprintln!("Error: {{:?}}", e);
        {}
    }}
}}
  ]], {
    i(1, "expression"),
    i(2, "todo!()"),
    i(0), -- i(0) is always the FINAL jump point (the exit)
  })),
}

-- Magic global snippet in snippets/all.lua. These snippet works in every file
return {
  s("currdate", f(function() 
    return os.date("%Y-%m-%d") 
  end)),
}
```

```lua
-- More advance Typescript programmatic snippet
return {
  s("tc", fmt([[
try {{
    {}
}} catch (err) {{
    if (isRedirectError(err)) throw err;
    console.error("[{}]", err);
    throw err;
}}
]], {
    -- Dynamic Node: allows the content to be editable
    d(1, function(_, snip)
      local selection = snip.env.TM_SELECTED_TEXT

      if selection and selection[1] and #selection[1] > 0 then
        return sn(nil, { i(1, selection) })
      end
      return sn(nil, { i(1, "// operators here") })
    end), i(2, "context_name"), -- This is now the second jump point
  }))
}
```

### Explanation of The TS programmatic snippet
**Break down of d(1, ...) logic**
```lua
  local selection = snip.env.TM_SELECTED_TEXT -- 1. Polls the text in current TM_SELECTED_TEXT buffer form NeoVim

  if selection and selection[1] and #selection[1] > 0 then -- 2. Run the logic, if there is text in buffer warp it in { i(1, selection) }   
    return sn(nil, { i(1, selection) }) -- If not, give { i(1, "// operators here") }
  end
  -- 3.Injects the Result: It places that freshly created i(1) node into try {} block
```

## gotchas
* **Function node VS Dynamic node** Function node return raw string text, Read only, Best for mirroring text while Dynamic node return `sn`, Interactive, Best for warping and generate new structure

## links
[Official LuaSnip Doc](https://github.com/L3MON4D3/LuaSnip/blob/master/DOC.md)

