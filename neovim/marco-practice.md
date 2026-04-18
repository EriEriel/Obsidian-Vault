---
id: marco-practice
aliases: []
tags:
  - #nivm
  - #marco
---

# marco-practice
2026-04-04  #learn

## what is it
Think of a macro as a tape recorder for your keystrokes. Anything you can do manually in Neovim, a macro can repeat perfectly.The basic workflow is: Record → Stop → Play.

## how it works
1. Start recording into register with `q{register}` Example: `qa`, `qs`, etc.
2. While in normal mode stop recording with `q`
3. Play register with `@{register}` Example: `@a`, `@s`, etc.
4. Repeat the previous macro with `@@`

## example
Put asterisk in front of what and surround with "
  apple    -> * "apple"
  banana   -> * "banana"
  cherry   -> * "cherry"

1. Press `qa` to start
2. Press `shift + I` to insert at start of the line
3. Type `*` → `esc` → move cursor into world then `ysiw` then type `"`
4. In normal mode `J` to move in to next line than `q` to stop recording
5. Now on world banana `@a`
6. Then `@@` to repeat last macro

  * "apple"
  * "banana"
  * "cherry"

## gotchas
* Always start at predictable point like starting of the line
* Use semantic motions like `e` to next world and `b` to previous world etc.
* Always end macro at the next place that macro should run like `j` for next line, `}f` for start of next function

## links

