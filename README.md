# Overview
This is a project to visualise and evaluate matrix transformations / expressions involving 2x2 matricies, 2D vectors, and floats, such as multiplication, inversing, scaling, addition, determinant, dot product and more.

# Full list of features
## Functions
* `Mat(a, b, c, d)`
* `Vec(x, y)`
* `*` (defined for anything including a float)
* `+` `-` (defined for vec by vec, mat by mat and float by float)
* `*` `/` (defined for mat by vec, mat by mat, vec by float and mat by float)
* `^` (power, defined for float by float. This does not have a visualisation)
* `.a` `.b` `.x` `.y` (defined for vec)
* `.a` `.b` `.c` `.d` `.w` `.x` `.y.` `.z` (defined for mat)
* `.i` `.j` (defined for mat, returns i hat and j hat)
* `Left(mat)` (returns `Vec(a, c)`. This is equivalent to `.i`)
* `Right(mat)` (returns `Vec(b, d)`. This is equivalent to `.j`)
* `Top(mat)` (returns `Vec(a, b)`)
* `Bottom(mat)` (returns `Vec(c, d)`)
* `Hor(v1, v2)` (returns `Mat(x1, y1, x2, y2)`)
* `Vert(v1, v2)` (returns `Mat(x1, x2, y1, y2)`)
* `RotMat(angle)`
* `RotVec(angle)`
* `X` (defined for vec by vec, represents cross product)
* `*` (defined for vec by vec, represents dot product)
* `Det(mat)`
* `=` (variable assignment)
* `Show` (used to visualise something)
* `(` and `)`
* Any token not here is interpretted as a variable name

### Other features
* Variable assignment and use
* Automatic camera which changed based on visuals
* Displays an abstract syntax tree representing your equation
* Ability to trim down your abstract syntax tree to not show specific visuals
* Complete tokeniser and Pratt parser

# Examples

```
Show Mat(1,2,-3,3) * Mat(0.5,-1,1,0.5)
```

```
a = Mat(1,2,-3,3)
b = Mat(0.5,-1,1,0.5)
c = Mat(1.0,0.5,-2,0.5)
Show c*(a-b) + b
```

```
Show Mat(1.0,0.5,-2,0.5)*(Mat(1,2,-3,3)-Mat(0.5,-1,1,0.5))
```

```
a=Vec(12,5)
b=Vec(-2,3)
ap=Vec(-5,-7)
bp=Vec(6,-2)
m=Vert(ap, bp) * Inv(Vert(a,b))

Show m*Vert(a,b)
```

```
Show Mat(2,1,-3,(2*-2)+3)
```
