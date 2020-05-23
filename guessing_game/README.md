# Notes

## Result type

Rust has a number of types named `Result` which is an enum (`Ok`, `Err`)
Inside `Ok` is the successfully generated value
`Err` contains information about how or why the operation failed.
`Result` also have expect method to display error and crach if `Err` is define otherwise it return `Ok` value.

## Println placeholders

`println!("x = {} and y = {}", 5, 10)`

## Cargo.tom

`^0.5` is equivalent to `any version that has a public API compatible with version 0.5`

## Type

Define return type
`number_generator.gen::<f64>();` will return float
`number_generator.gen::<u8>();` will return uint

## To read later

<https://rust-random.github.io/rand/rand/distributions/index.html>
