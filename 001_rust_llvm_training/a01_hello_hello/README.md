# emit llvm


```bash
cargo rustc -- --emit llvm-ir=a01_hello_hello.ll
```

# lit test

```bash
$ lit -sv a01_hello_hello.ll

Testing Time: 0.14s

Total Discovered Tests: 1
  Passed: 1 (100.00%)
```

- Rust lit on the directory

```bash
$ lit .

-- Testing: 2 tests, 2 workers --
UNRESOLVED: a01_hello_hello :: target/debug/incremental/a01_hello_hello-1eag4bkx6zfpi/s-hj1d89owrt-02yf2zs-ddvkn1k5jzzo68jkq6y61fgzj/db4j352nh99q69c1r6t4b5pam.ll (1 of 2)
PASS: a01_hello_hello :: a01_hello_hello.ll (2 of 2)
********************
Unresolved Tests (1):
  a01_hello_hello :: target/debug/incremental/a01_hello_hello-1eag4bkx6zfpi/s-hj1d89owrt-02yf2zs-ddvkn1k5jzzo68jkq6y61fgzj/db4j352nh99q69c1r6t4b5pam.ll


Testing Time: 0.13s

Total Discovered Tests: 2
  Passed    : 1 (50.00%)
  Unresolved: 1 (50.00%)
```


# `lit`
- Looking at the errors, `lit` isn't finding any tests because your files aren't formatted as lit tests. Here's how to use lit:

## Basic lit test format

Lit tests require special directives embedded in comments. For LLVM IR files, tests typically use `RUN:` and `CHECK:` directives:

```llvm
; RUN: opt %s -S -o - | FileCheck %s
; CHECK: define void @test

define void @test() {
  ret void
}
```

## For your project

To use lit with your LLVM IR, you'd need to:

1. **Create a proper test file** (not your raw `.ll` file):
```llvm
; RUN: llc %s -o -
; CHECK: @main

define i32 @main() {
  ret i32 0
}
```

2. **Create a `lit.cfg` or `lit.site.cfg`** in your test directory:
```python
import lit.formats
config.name = "my_tests"
config.test_format = lit.formats.ShTest()
config.suffixes = ['.ll']
```

3. **Run lit on the directory** (not individual files):
```bash
lit .
```

## What you're actually trying to do

If you just want to examine the LLVM IR, you don't need lit. Use:
- `llc a01_hello_hello.ll -o -` to compile the IR
- `opt a01_hello_hello.ll -S -o -` to view/transform IR
- `llvm-as a01_hello_hello.ll -o a01_hello_hello.bc` to assemble to bytecode

What's your actual goal with lit?
