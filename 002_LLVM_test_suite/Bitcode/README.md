- https://github.com/llvm/llvm-test-suite

https://github.com/llvm/llvm-test-suite/tree/main/Bitcode


- Run test
```bash
llvm-lit -v -j 1 -o results.json .
```


```bash
ASS: test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-vrp-3.test (1832 of 2120)
********** TEST 'test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-vrp-3.test' RESULTS **********
compile_maxrss: 0
compile_time: 0.0000
exec_time: 0.0006
hash: "9cb0bdf9896fcb88506deb62051bae0e"
link_maxrss: 0
link_time: 0.0383
size: 33456
size.__got: 8
size.__stubs: 12
size.__text: 44
size.__unwind_info: 96
**********
PASS: test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-vrp-4.test (1833 of 2120)
********** TEST 'test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-vrp-4.test' RESULTS **********
compile_maxrss: 0
compile_time: 0.0000
exec_time: 0.0006
hash: "ffb83f045b074f369348c301264d89ab"
link_maxrss: 0
link_time: 0.0365
size: 33504
size.__got: 16
size.__stubs: 24
size.__text: 64
size.__unwind_info: 88
**********
PASS: test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-vrp-5.test (1834 of 2120)
********** TEST 'test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-vrp-5.test' RESULTS **********
compile_maxrss: 0
compile_time: 0.0000
exec_time: 0.0006
hash: "73ae18169c33dbf8cdd3a66d8d39bbfd"
link_maxrss: 0
link_time: 0.0400
size: 33504
size.__got: 16
size.__stubs: 24
size.__text: 60
size.__unwind_info: 88
**********
PASS: test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-vrp-6.test (1835 of 2120)
********** TEST 'test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-vrp-6.test' RESULTS **********
compile_maxrss: 0
compile_time: 0.0000
exec_time: 0.0006
hash: "b5ef85b6c937ea5337f09a6d04eb5f25"
link_maxrss: 0
link_time: 0.0383
size: 33552
size.__got: 16
size.__stubs: 24
size.__text: 108
size.__unwind_info: 88
**********
PASS: test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-vrp-7.test (1836 of 2120)
********** TEST 'test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-vrp-7.test' RESULTS **********
compile_maxrss: 0
compile_time: 0.0000
exec_time: 0.0006
hash: "c1c6948eeba9bdbbf4a5872dda22b67f"
link_maxrss: 0
link_time: 0.0389
size: 33504
size.__common: 4
size.__got: 8
size.__stubs: 12
size.__text: 72
size.__unwind_info: 96
**********
PASS: test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-wchar_t-1.test (1837 of 2120)
********** TEST 'test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-wchar_t-1.test' RESULTS **********
compile_maxrss: 0
compile_time: 0.0000
exec_time: 0.0007
hash: "4a4263f32057e518603bdf2cf897ac93"
link_maxrss: 0
link_time: 0.0397
size: 50040
size.__data: 12
size.__got: 16
size.__stubs: 24
size.__text: 60
size.__unwind_info: 88
**********
PASS: test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-widechar-1.test (1838 of 2120)
********** TEST 'test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-widechar-1.test' RESULTS **********
compile_maxrss: 0
compile_time: 0.0000
exec_time: 0.0006
hash: "6685fbf8147897465b433591293cff1c"
link_maxrss: 0
link_time: 0.0366
size: 33448
size.__got: 8
size.__stubs: 12
size.__text: 16
size.__unwind_info: 88
**********
PASS: test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-widechar-2.test (1839 of 2120)
********** TEST 'test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-widechar-2.test' RESULTS **********
compile_maxrss: 0
compile_time: 0.0000
exec_time: 0.0007
hash: "393e0658f561142ad2fd79583403bae4"
link_maxrss: 0
link_time: 0.0409
size: 33464
size.__const: 16
size.__got: 8
size.__stubs: 12
size.__text: 16
size.__unwind_info: 88
**********
PASS: test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-widechar-3.test (1840 of 2120)
********** TEST 'test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-widechar-3.test' RESULTS **********
compile_maxrss: 0
compile_time: 0.0000
exec_time: 0.0006
hash: "7402ae04aa69f790a354036ff93e6d80"
link_maxrss: 0
link_time: 0.0413
size: 33464
size.__got: 8
size.__stubs: 12
size.__text: 24
size.__unwind_info: 96
**********
PASS: test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-zero-struct-1.test (1841 of 2120)
********** TEST 'test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-zero-struct-1.test' RESULTS **********
compile_maxrss: 0
compile_time: 0.0000
exec_time: 0.0007
hash: "67c2e9def3a7ae078392006e55fd5748"
link_maxrss: 0
link_time: 0.0398
size: 50088
size.__common: 3
size.__data: 16
size.__got: 8
size.__stubs: 12
size.__text: 116
size.__unwind_info: 96
**********
PASS: test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-zero-struct-2.test (1842 of 2120)
********** TEST 'test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-zero-struct-2.test' RESULTS **********
compile_maxrss: 0
compile_time: 0.0000
exec_time: 0.0006
hash: "d07427016db6eb67631d94bcab9b6974"
link_maxrss: 0
link_time: 0.0403
size: 33528
size.__common: 4
size.__got: 8
size.__stubs: 12
size.__text: 68
size.__unwind_info: 96
**********
PASS: test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-zerolen-1.test (1843 of 2120)
********** TEST 'test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-zerolen-1.test' RESULTS **********
compile_maxrss: 0
compile_time: 0.0000
exec_time: 0.0006
hash: "4062ca7f6a74ba07a1e0417c4bf10bc0"
link_maxrss: 0
link_time: 0.0386
size: 33512
size.__common: 4
size.__got: 8
size.__stubs: 12
size.__text: 44
size.__unwind_info: 96
**********
PASS: test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-zerolen-2.test (1844 of 2120)
********** TEST 'test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/GCC-C-execute-zerolen-2.test' RESULTS **********
compile_maxrss: 0
compile_time: 0.0000
exec_time: 0.0006
hash: "53c086a74db77767f421a6375a1d3926"
link_maxrss: 0
link_time: 0.0408
size: 16856
size.__text: 8
size.__unwind_info: 88
**********
PASS: test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/ieee/GCC-C-execute-ieee-20000320-1.test (1845 of 2120)
********** TEST 'test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/ieee/GCC-C-execute-ieee-20000320-1.test' RESULTS **********
compile_maxrss: 0
compile_time: 0.0000
exec_time: 0.0006
hash: "03ffc8a888764243111e623ff9ef9ec2"
link_maxrss: 0
link_time: 0.0425
size: 33616
size.__common: 20
size.__got: 16
size.__stubs: 24
size.__text: 140
size.__unwind_info: 96
**********
PASS: test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/ieee/GCC-C-execute-ieee-20001122-1.test (1846 of 2120)
********** TEST 'test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/ieee/GCC-C-execute-ieee-20001122-1.test' RESULTS **********
compile_maxrss: 0
compile_time: 0.0000
exec_time: 0.0006
hash: "f7df2120ce1f4a4f78bd1e33a854bbb6"
link_maxrss: 0
link_time: 0.0378
size: 33536
size.__common: 16
size.__got: 16
size.__stubs: 24
size.__text: 112
size.__unwind_info: 88
**********
PASS: test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/ieee/GCC-C-execute-ieee-20010114-2.test (1847 of 2120)
********** TEST 'test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/ieee/GCC-C-execute-ieee-20010114-2.test' RESULTS **********
compile_maxrss: 0
compile_time: 0.0000
exec_time: 0.0006
hash: "3acdf6da4a4c7bbf7ea9954f165faa79"
link_maxrss: 0
link_time: 0.0405
size: 33488
size.__got: 8
size.__stubs: 12
size.__text: 88
size.__unwind_info: 96
**********
PASS: test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/ieee/GCC-C-execute-ieee-20010226-1.test (1848 of 2120)
********** TEST 'test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/ieee/GCC-C-execute-ieee-20010226-1.test' RESULTS **********
compile_maxrss: 0
compile_time: 0.0000
exec_time: 0.0006
hash: "fb8a1723af270e0f9b694e21298cfd62"
link_maxrss: 0
link_time: 0.0380
size: 50112
size.__common: 24
size.__data: 8
size.__got: 8
size.__stubs: 12
size.__text: 108
size.__unwind_info: 88
**********
PASS: test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/ieee/GCC-C-execute-ieee-20011123-1.test (1849 of 2120)
********** TEST 'test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/ieee/GCC-C-execute-ieee-20011123-1.test' RESULTS **********
compile_maxrss: 0
compile_time: 0.0000
exec_time: 0.0006
hash: "f1e519a043d9fdcc8923b54815cc746c"
link_maxrss: 0
link_time: 0.0367
size: 33456
size.__got: 8
size.__stubs: 12
size.__text: 16
size.__unwind_info: 88
**********
PASS: test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/ieee/GCC-C-execute-ieee-20030331-1.test (1850 of 2120)
********** TEST 'test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/ieee/GCC-C-execute-ieee-20030331-1.test' RESULTS **********
compile_maxrss: 0
compile_time: 0.0000
exec_time: 0.0006
hash: "ba33187778dafd94cc61dcdabac29a41"
link_maxrss: 0
link_time: 0.0364
size: 50064
size.__data: 4
size.__got: 16
size.__stubs: 24
size.__text: 196
size.__unwind_info: 96
**********
PASS: test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/ieee/GCC-C-execute-ieee-20041213-1.test (1851 of 2120)
********** TEST 'test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/ieee/GCC-C-execute-ieee-20041213-1.test' RESULTS **********
compile_maxrss: 0
compile_time: 0.0000
exec_time: 0.0006
hash: "33a2749cbaf20930bcfb60d6f876df36"
link_maxrss: 0
link_time: 0.0431
size: 33552
size.__common: 16
size.__got: 8
size.__stubs: 12
size.__text: 116
size.__unwind_info: 88
**********
PASS: test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/ieee/GCC-C-execute-ieee-920518-1.test (1852 of 2120)
********** TEST 'test-suite :: SingleSource/Regression/C/gcc-c-torture/execute/ieee/GCC-C-execute-ieee-920518-1.test' RESULTS **********
compile_maxrss: 0
```
