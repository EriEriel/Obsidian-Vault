---
id: 1773556733-LTBM
aliases:
  - mth1101
tags:
  - mathematic
  - calculus
date:
  - 2026-03-15 14:05
---

## One line decision tree:

Plug in x
  → works?         → done
  → get 0/0?       → factor and cancel
  → has sqrt?      → conjugate
  → x→∞?          → highest degree wins
  → still 0/0 or ∞/∞? → L'Hôpital's rule


## algebraic identities used as factoring tools for limits

  **Difference of squares** a² − b² = (a − b)(a + b)
  **Difference of cubes** a³ − b³ = (a − b)(a² + ab + b²)
  **Sum of cubes** a³ + b³ = (a + b)(a² − ab + b²)
  
  √x-1 can write as (√x-1)(√x+1)
  **Because** √x-1 can rewrite as (√x)² - 1² which is the same as term a² − b²

## factoring a quadratic trinomial
 
  ax² + bx + c 

  **Example** x² - x - 6 = (x-3)(x + 2)
  - multiply to give c (-6)
  - add to give b (-1)

| c | b | meaning | bracket signs |
| --------------- | --------------- | --------------- |  --------------- |
| + | + | both same,both positive | (x+)(x+) | 
| + | - | both same,both negative | (x-)(x-) | 
| - | + | Difference,bigger is positive | (x+)(x-) | 
| - | - | Difference,bigger is negative | (x+)(x-) |

## FOIL method
(x+h)² -> (x+h)(x+h) -> x²+xh+xh+h² -> x²+2xh+h²
  F -> first x²
  O -> outer xh 
  I -> inner hx (same as xh)
  L -> last h²

# [[Limits]]
  What value does f(x) approach as x get closer to some number, The function does not actually reach it just approaching.
**Ex.**
1. Direct substution 
  lim x→3  x² + 1
  = 3² + 1 = 10  ✓  done

2. Factor and cancel - when get 0/0:
  lim x→3  (x² - x - 6) / (x - 3)
  = (x-3)(x+2) / (x-3)    ← factor
  =  x + 2                  ← cancel (x-3)
  = 3 + 2 = 5

3. multiply by conjugate - when see square roots 
  lim x→0  (√(x+1) - 1) / x

  multiply top and bottom by (√(x+1) + 1)

  = (x+1 - 1) / x(√(x+1) + 1)
  = x / x(√(x+1) + 1)
  = 1 / (√(1) + 1) = 1/2

4. Limits at infinity - look at highest degree ignore the rest:
  lim x→∞  (3x² + 5) / (x² - 2)

  same degree top and bottom
  → divide leading coefficients
  = 3/1 = 3

if top degree > bottom → infinity
if top degree < bottom → 0

5. L'Hôpital's rule - when you still get 0/0 or ∞/∞:
  differentiate top and bottom separately, then try again

  lim x→∞  ln x / eˣ
  → (1/x) / eˣ = 1/xeˣ → 0

  **Signal to use:** plug in gives 0/0 or ∞/∞
  **Do NOT use** if direct substitution or factoring already works

# [[Derivative]] 
  Slope of the curve at any point = rate of change.

  f(x) = x²
  f'(x) = 2x        ← the derivative

  at x=3, slope = 2(3) = 6

## Rules to memorize (most common on MC)
### Power rule
xⁿ → nxⁿ⁻¹

x³ → 3x²
x⁴ → 4x³
x  → 1
3x → 3 (3*1) 
5  → 0 (constant always = 0)

### Constant multiple:
3x²  → 3(2x) = 6x
5x³  → 5(3x²) = 15x²

### Sum/difference:
x³ + x² - 5x + 2 → 3x² + 2x - 5

### Product rule:
(f·g)' = f'g + fg'

x²(x+1)
f = x²   f' = 2x
g = x+1  g' = 1
→ 2x(x+1) + x²(1) = 3x² + 2x

### Quotient rule - when dividing:
(f/g)' = (f'g - fg') / g²

"low d-high minus high d-low over low squared"

x² / (x+1)
f = x²   f' = 2x
g = x+1  g' = 1
→ (2x(x+1) - x²(1)) / (x+1)²
= (x²+2x) / (x+1)²

### Chain rule - function inside function:
f(g(x))' = f'(g(x)) · g'(x)

(x²+1)³
outer: u³  → 3u²
inner: x²+1 → 2x
→ 3(x²+1)² · 2x

### Implicit differentiation - when x and y are mixed:
- differentiate both sides normally
- every y term gets × dy/dx attached
- then isolate dy/dx

  xy + y² = 8
  differentiate both sides:
  y + x(dy/dx) + 2y(dy/dx) = 0
  factor out dy/dx:
  dy/dx(x + 2y) = -y
  dy/dx = -y / (x + 2y)

  then plug in the point (x,y) to get the value

---

## Common derivatives to memorize:
  sin x    →  cos x
  cos x    → -sin x
  tan x    →  sec²x
  eˣ       →  eˣ          (same! never changes)
  e^(ax)   →  ae^(ax)     ← chain rule attached!
  ln x     →  1/x
  arcsin x →  1/√(1-x²)
  arccos x → -1/√(1-x²)
  arctan x →  1/(1+x²)

---

## One-line memory hook:
  Power rule    → bring down the power, subtract 1
  Chain rule    → outside × inside'
  Product rule  → first×second' + second×first'
  Quotient rule → (bottom×top' - top×bottom') / bottom²

---

let 𝑓 is real function then derivative of 𝑓 compare with x can write as:
  f'(x) = lim h->0 f(x+h)-f(x) / h

**Example 1.**
Let f(x)=3x+2 fine f'(x)
  f'(x) = lim h->0 f(x+h)-f(x) / h 
        = lim h->0 [3(x+h)+2]-[3x+2] / h 
        = lim h->0 3h / h  
        = 3

**Example 2.**
let y=f(x)=x² (Parabola) find f'(x)
  f'(x) = lim h->0 (x+h)²-x² / h 
        = lim h->0 (x²+2xh+h²)-x² / h
        = lim h->0 2xh+h² / h 
        = lim h->0 2x+h
        = 2x

**Example 3.**
 - dx/dy x = d/dx x¹ = 1x¹⁻¹ = 1x⁰ = 1
 - d/dx x⁵ = 5x⁵⁻¹ = 5x⁴
 - d/dx 6^√x = d/dx x^1/6 = 1/6 x^1/6-1 = 1/6 x^-5/6 = 1/6x^5/6 = 1/6^√x⁵
 - d/dx (1/x³) = d/dx x⁻³ = (-3)x³⁻¹ = 3x⁻⁴ = -3/x⁴
 - d/dx (5√x+4/x²) = 5 d/dx (√x) + 4 d/dx (1/x²) = 5(1/2)x^-1/2 + 4(-2)x⁻³ = 5/2 x^-1/2 - 8x⁻³ = 5/2√x - 8/x³

## Find the slope, tangent line, normal line equations.
  ### The formula for tangent line:
  **x1 and y1 here refer to position of (x,y)**

  y-y1 = m(x-x1)

  Where: m=f'(1) (the slope we calculate)
  (x1,y1) = The point P(1,3)

  ### The formula for normal line
  Normal line is perpendicular to the tangent, so its slope is just flipped and negated.

  m(normal) = -1/m

  **Example.**
  y = x²+2 = f(x) at point P(1,3) has the value of f'(1)

  1. Find the limit
  f'(1)  = 1²+2 = 3
  f(x+h) = (1+h)² + 2

  f(x) = [f(x+h) - f(x)] / h
  
  = (1+h)²+2-3 -> FOIL the bracket, simplify, cancel h, take limit
  = 1+2h+h²+2-3 -> combine plain numbers: 1+2-3 = 0
  = (2h+h²) / h -> factor out h
  = 2+h -> take limit h->0
  = 2

  So f'(1)=2 which is the slope of the curve at P(1,3)
  Now the tangent line - plug into y-y1 = m(x-x1):

  - m = 2
  - (x,y) = (1,3)

  y-3 = 2(x-1) -> apply 2 into bracket so 2x-2 then add 3
  y = 2x-2+3
  y = 2x+1 
 
  Now for the normal line, slope = -1/2

  y-3 = -1/2(x-1)
  y = (-1/2)x + 1/2 + 3 = (-1/2)x + 7/2

## Inflection points - where curve changes concavity:
  set f''(x) = 0, solve for x
  then verify sign of f''(x) changes around each point

  **Example** f(x) = x⁴ - 4x³
  f'(x)  = 4x³ - 12x²
  f''(x) = 12x² - 24x = 12x(x-2) = 0
  → x = 0 and x = 2  ← 2 inflection points

  verify sign changes:
  - before x=0 (try x=-1): positive
  - between 0 and 2 (try x=1): negative
  - after x=2 (try x=3): positive
  sign changes at both points ✓

## Optimization - finding max/min value:
  set f'(x) = 0 → solve for x → that's your max/min point

  **Example** (revenue problem)
  price = 50, passengers = 800
  every +1 baht → -10 passengers

  R(x) = (50+x)(800-10x)
       = 40000 + 300x - 10x²
  R'(x) = 300 - 20x = 0
  x = 15  → optimal price = 50+15 = 65 baht

# [[Integration]]
  Integration is the reverse of differentiation - instead of finding slope, we're finding the **area under the curve**.

### The rule (reverse power rule):

  xⁿ -> [(xⁿ⁺¹) / n+1] + C 

  "Raise the power by 1, divide by the new power, add C"
  C is just a constant, always add it or teacher takes marks!

**Example.**
  x² -> (x³/3) + C
  x³ -> (x⁴/4) + C
  x  -> (x²/2) + C
  5  -> 5x + C (constant just get x attached)

  ∫ x²+3x+2 dx 

  x² -> (x³/3)
  3x -> (3x²/2)
  2  -> 2x

  = (x³/3) + (3x²/2) + 2x + C

### Common integrals to memorize:
  ∫ sin x dx  = -cos x + C
  ∫ cos x dx  =  sin x + C
  ∫ sec²x dx  =  tan x + C
  ∫ eˣ dx     =  eˣ + C
  ∫ (1/x) dx  =  ln|x| + C

### Definite integrals - has limits a and b:
  ∫[a to b] f(x) dx = F(b) - F(a)

  - integrate normally (NO C this time)
  - plug in top number (b), subtract plugging in bottom number (a)

  **Example** ∫[1 to 2] x² dx
  = [x³/3] from 1 to 2
  = (2³/3) - (1³/3)
  = 8/3 - 1/3 = 7/3

  **tip:** if bottom number is 0, plugging in almost always gives 0, no messy fractions!

### U-substitution - reverse chain rule:
  use when you see a function AND its derivative inside the integral

  **Steps:**
  1. let u = inner messy part
  2. find du
  3. substitute everything in terms of u
  4. integrate
  5. substitute back

  **Example** ∫ cos(√x) / √x dx
  let u = √x
  du = 1/(2√x) dx  →  dx/√x = 2 du
  = ∫ cos(u) · 2 du
  = 2 sin(u) + C
  = 2 sin(√x) + C
