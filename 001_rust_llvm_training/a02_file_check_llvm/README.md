# Result


```bash
$ FileCheck --input-file input.txt check-file.txt
```

# fix
- Perfect! ✅ FileCheck now runs successfully. The issue was that the required files (`input.txt` and `check-file.txt`) didn't exist.

## What I fixed:

**Created `input.txt`** - Contains the LLVM IR code to be checked (a simplified version of your `a02_file_check_llvm.ll`)

**Created `check-file.txt`** - Contains FileCheck directives to verify:
- Target triple is set for Apple Silicon
- "Hello, world!" string constant exists  
- The main function is properly defined
- Correct function calls are made (`Arguments::from_str`, `stdio::_print`, `lang_start`)
- Proper return types

## FileCheck behavior:
- **No output** = All checks passed ✅
- **Output with errors** = Checks failed

You can now run:
```bash
FileCheck --input-file input.txt check-file.txt
```

- And it will verify that the LLVM IR contains the expected patterns. If you want to test what happens when a check fails, try modifying `check-file.txt` to check for something that doesn't exist.
  - 그리고 LLVM IR에 예상되는 패턴이 포함되어 있는지 확인합니다. 검사에 실패했을 때 어떤 일이 일어나는지 테스트하고 싶다면 'check-file.txt'를 수정하여 존재하지 않는 것이 있는지 확인해 보세요.

