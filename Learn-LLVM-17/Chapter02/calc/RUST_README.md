# Calc Expression Compiler - Rust Version

This is a Rust translation of the C++ LLVM expression compiler from Chapter 2 of "Learn LLVM 17".

## Features

- **Lexer**: Tokenizes input expressions using iterator-based approach
- **Parser**: Recursive descent parser with proper error handling using `Result` types
- **Semantic Analysis**: Variable declaration checking using `HashSet`
- **Code Generation**: LLVM IR generation using Inkwell bindings

## Language Syntax

The calculator supports:
- Arithmetic operations: `+`, `-`, `*`, `/`
- Number literals and identifiers
- Variable declarations with `with` keyword

### Examples

```bash
# Simple arithmetic
cargo run -- "1 + 2 * 3"

# Variable declarations
cargo run -- "with x, y: x + y * 2"

# Complex expressions
cargo run -- "with a: (a + 1) * 2"
```

## Architecture

### Modules

- **`ast.rs`**: AST definitions using Rust enums instead of C++ inheritance
- **`lexer.rs`**: Tokenizer with iterator interface
- **`parser.rs`**: Recursive descent parser with `Result`-based error handling
- **`sema.rs`**: Semantic analysis for variable checking
- **`codegen.rs`**: LLVM IR generation using Inkwell
- **`main.rs`**: CLI entry point using `clap`

### Key Design Differences from C++

1. **No Manual Memory Management**: Uses Rust's ownership system instead of `new`/`delete`
2. **Enum-based AST**: Replaces C++ inheritance with Rust enums
3. **Result Types**: Proper error handling instead of error flags
4. **Pattern Matching**: `match` expressions instead of visitor pattern
5. **No Raw Pointers**: Safe references and `Box` instead of pointers

## Building

```bash
cargo build
```

## Running

```bash
cargo run -- "<expression>"
```

## Dependencies

- `inkwell`: Safe Rust wrapper for LLVM
- `clap`: Command-line argument parsing

## Testing

```bash
cargo test
```

## Example Output

Input: `with x, y: x + y * 2`

Generated LLVM IR:
```llvm
@x.str = private constant [2 x i8] c"x\00"
@y.str = private constant [2 x i8] c"y\00"

define i32 @main(i32 %0, ptr %1) {
entry:
  %read = call i32 @calc_read(ptr @x.str)
  %read1 = call i32 @calc_read(ptr @y.str)
  %mul = mul i32 %read1, 2
  %add = add i32 %read, %mul
  call void @calc_write(i32 %add)
  ret i32 0
}
```

## Comparison with C++ Version

| Feature | C++ | Rust |
|---------|-----|------|
| AST | Inheritance hierarchy | Enums |
| Memory | `new`/`delete` | Ownership/`Box` |
| Errors | Error flags | `Result<T,E>` |
| Control flow | `goto` statements | Proper control flow |
| Visitor pattern | Virtual functions | Pattern matching |
| LLVM API | C++ API | Inkwell wrapper |

## License

Same as the original C++ version.
