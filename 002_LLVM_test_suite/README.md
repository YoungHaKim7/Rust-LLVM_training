# llvm test-suite

```bash
git clone https://github.com/llvm/llvm-test-suite.git


gix clone https://github.com/llvm/llvm-test-suite.git
```

Based on the LLVM Test Suite Guide, here are the build instructions:

## Quick Build Steps

### 1. Install the lit test runner
**Option A:** Use from LLVM build:
```bash
<path to llvm build>/bin/llvm-lit --version
```

**Option B:** Install via pip (recommended):
```bash
python3 -m venv .venv
. .venv/bin/activate
pip install git+https://github.com/llvm/llvm-project.git#subdirectory=llvm/utils/lit
```

**Option C:** Install official release:
```bash
python3 -m venv .venv
. .venv/bin/activate
pip install lit
```

### 2. Configure with CMake
```bash
mkdir test-suite-build
cd test-suite-build
cmake -DCMAKE_C_COMPILER=<path to llvm build>/bin/clang \
      -C../llvm-test-suite/cmake/caches/O3.cmake \
      ../llvm-test-suite
```

**Note:** If using your built clang and want MicroBenchmarks/XRay, add `compiler-rt` to `LLVM_ENABLE_RUNTIMES`.

### 3. Build
```bash
make

# 병렬로 빠르게 빌드하기
make -j$(nproc)
```

### 4. Run Tests
```bash
llvm-lit -v -j 1 -o results.json .
```

## Common Configuration Options

- `CMAKE_C_COMPILER` - Specify the C compiler to test
- `CMAKE_C_FLAGS` - Extra flags for compiler invocations
- `TEST_SUITE_RUN_BENCHMARKS` - Set to `OFF` to only collect compile-time metrics
- `TEST_SUITE_BENCHMARKING_ONLY` - Disable tests unsuitable for performance measurements
- `TEST_SUITE_COLLECT_STATS` - Collect internal LLVM statistics

The test-suite includes various benchmark types in `SingleSource/`, `MultiSource/`, `MicroBenchmarks/`, `External/`, and `Bitcode/` directories.
