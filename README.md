# Numbers Adventure

Calculator to perform basic arithmetic operations on raw expressions.

<p align="center">
    <img src="./icon.gif" alt="Numbers Adventure" title="Numbers Adventure"/>
<p>

## Examples

```bash
? Expression: (2 * 2) + (4 * 3)
#> 16
? Expression: (12 / 2) * [30 * 3]
#> 540
? Expression: 1.0 * 0.1
#> 0.1
? Expression: 0.1 * (100 * 14)
#> 140
? Expression: 0.1 / 20 + 14
#> 14.005
? Expression: (1 * (2 + 1)) + 1
#> 4
```

## Supported operations

```rs
enum Op {
    Plus, // '+'
    Mins, // '-'
    Multi, // '*' | 'x'
    Div, // '/'
}
```
