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

