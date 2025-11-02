
using `RUSTFLAGS=-Awarnings cargo run` to suppress warnings

# gdb debugging

正常 gdb 调试即可

# miri check undefined behavior at runtime

install:

`rustup +nightly component add miri`

run:

`cargo +nightly miri run` or `cargo +nightly miri test`