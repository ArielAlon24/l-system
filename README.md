# l-system

[L-systems](https://en.wikipedia.org/wiki/L-system) implemented in Rust.

> This project is heavily based on the following [specification](http://www.paulbourke.net/fractals/lsys/)

## Example

```
[config]
line_length = 3
line_width_increment = 1.0
line_length_scale_factor = 1.1
turning_angle = 22.5
turning_angle_increment = 45.0

[rules]
F -> FF
X -> F-[[X]+X]+F[+FX]-X

[start]
axiom = X
```

![branch](./images/branch.png)
