# Notes

* Rust char is 4 byte length. So it could store unicode
* You could spread tuple `let (_x, y, _z) = tup` or access to element by `tup.1`
* You could create array like this `let b: [u32; 1];` or `let b: [2; 3];`. [type; length]/[default_value; length]

## Debug

Overflow cause crash

## Release

You can't overflow, rust modulo the value
