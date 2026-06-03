# Result


```bash
Kaleidoscope Lexer
Enter expressions (Ctrl+D to exit):
ready> 3d
Token: Number(3)
ready> Token: Identifier(d)
ready> 3+3
Token: Number(3)
ready> Token: ASCII('+')
ready> Token: Number(3)
```

# Fix

- Perfect! The Rust lexer implementation is now complete and working correctly. Here's a summary of what was fixed and converted:

## ✅ **Fixed and Converted Rust Lexer**

### **Key Changes Made:**

1. **Removed unsafe mutable statics** - Converted global `IDENTIFIER_STR` and `NUM_VAL` into proper struct fields within `Lexer`

2. **Fixed type mismatches** - Corrected `Some(&c)` patterns to `Some(c)` to avoid reference type errors

3. **Added missing imports** - Added `use std::io::Write` to enable `flush()` method

4. **Fixed borrow checker issues** - Split mutable borrow operations by storing intermediate values before pushing to strings

5. **Created idiomatic Rust implementation**:
   - `Lexer` struct with proper encapsulation
   - `Token` enum for recognized tokens
   - Returns `Result<Token, i32>` to distinguish tokens from ASCII values
   - Proper EOF handling

### **Functionality Verified:**

✅ **Keywords**: `"def"` → `Token::Def`, `"extern"` → `Token::Extern`  
✅ **Identifiers**: `"foo"` → `Token:: Identifier("foo")`  
✅ **Numbers**: `"42"` → `Token:: Number(42.0)`, `"3.14"` → `Token:: Number(3.14)`  
✅ **Operators**: `"+"` → `ASCII('+')` (returns ASCII value)  
✅ **EOF**: Proper end-of-file detection

The lexer now follows the LLVM tutorial's logic while using safe, idiomatic Rust instead of C-style global mutable state. It's ready to serve as the foundation for building the rest of the Kaleidoscope language frontend.
