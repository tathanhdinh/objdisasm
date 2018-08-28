## disasm

A simple disassembler inspired by [cstool](https://github.com/aquynh/capstone/tree/master/cstool), using [Capstone engine](https://github.com/aquynh/capstone).

**Example**

```
disasm -vv -m x32 "8b ff 55 8b ec 83 ec 10 a1 00 40 2b 01" -a 0x4000
0x0000000000004000  8b ff                                          mov edi, edi
0x0000000000004002  55                                             push ebp
0x0000000000004003  8b ec                                          mov ebp, esp
0x0000000000004005  83 ec 10                                       sub esp, 0x10
0x0000000000004008  a1 00 40 2b 01                                 mov eax, dword ptr [0x12b4000]
```