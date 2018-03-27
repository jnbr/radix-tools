radix-tools
===========

Small set of command line tools to convert strings, representing numbers of
different radix / bases.

Only unsigned integers are supported.

Compiling
---------

This project uses `cargo`.
Compile it by running `cargo build --release`. The resulting binaries are
stored in `target/release` and can optionally be installed into
`$HOME/.cargo/bin` by running `cargo install`.


Usage
-----

convert decimal to hex:
`dec2hex <number to convert>`

convert hex to decimal:
`hex2dec <hex number to convert>`


Examples
--------

```
$ dec2hex 123
0x7b

$ dec2hex 0.0
failed to parse input

$ hex2dec 7b
123

$ hex2dec 0X7b
123

$ hex2dec 0x7B
123

$ hex2dec 0xabcdefg
failed to parse input
```
