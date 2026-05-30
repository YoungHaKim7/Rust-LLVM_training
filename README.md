# Rust-LLVM_training(이 책을 참고함)
- https://github.com/PacktPublishing/LLVM-Code-Generation

# Checking the results after each phase
- Colombet, Quentin. LLVM Code Generation: A deep dive into compiler backend development (p. 14). (Function). Kindle Edition. 

|To stop|Command |
|-|-|
|After the preprocessor.  |`clang -E`|
|After syntax checking.| `clang -fsyntax-only`|
| After LLVM IR code generation.| `clang -O0 -emit-llvm -S`|
|After the middle-end optimizations<br />(pick the level you want)| `clang -O<1|2|3|s|z> -emit-llvm -S`|
|After assembly generation<br/>(i.e., see the extual representation of the assembly)| `clang -S`|
|After the assembler<br />(i.e., see the object file representation)| `clang -c`|

# LLVM기초(Getting Started with the LLVM System)
- https://llvm.org/docs/GettingStarted.html

# comprehensive documentation
Colombet, Quentin. LLVM Code Generation: A deep dive into compiler backend development (p. ). (Function). Kindle Edition. 
- https://llvm.org/docs/

# (유료 책)LLVM Code Generation: A deep dive into compiler backend development
-  Quentin Colombet 
  - https://www.amazon.com/dp/B0F1331DWH/
- 깃허브 코드
  - https://github.com/PacktPublishing/LLVM-Code-Generation
 
