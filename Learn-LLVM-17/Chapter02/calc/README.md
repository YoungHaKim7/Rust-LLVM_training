# Result

```bash
$ cargo r 3/3

; ModuleID = 'calc.expr'
source_filename = "calc.expr"

define i32 @main(i32 %0, ptr %1) {
entry:
  call void @calc_write(i32 1)
  ret i32 0
}

declare void @calc_write(i32)

```

# test

```bash
$ cargo t

running 14 tests
test lexer::tests::test_numbers ... ok
test lexer::tests::test_operators ... ok
test parser::tests::test_simple_arithmetic ... ok
test lexer::tests::test_with_keyword ... ok
test lexer::tests::test_identifiers ... ok
test parser::tests::test_simple_number ... ok
test parser::tests::test_with_declaration ... ok
test sema::tests::test_undeclared_variable ... ok
test sema::tests::test_duplicate_variable ... ok
test sema::tests::test_number_only ... ok
test sema::tests::test_valid_variable_use ... ok
test codegen::tests::test_codegen_arithmetic ... ok
test codegen::tests::test_codegen_with_decl ... ok
test codegen::tests::test_codegen_simple_number ... ok
```

# fix(Claude AI)
Perfect! The build completed successfully. The `calc` executable is at `build/src/calc`.

**Summary:**
- ✅ Created `build/` folder
- ✅ Fixed CMakeLists.txt issues (removed Windows-specific include, set C++17)
- ✅ Fixed CodeGen.cpp (added missing `llvm/IR/Module.h` include)
- ✅ Built successfully: `build/src/calc` (7.1 MB executable)

The project is now built and ready to run from the build folder!


# Result


```bash


```

