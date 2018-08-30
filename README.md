## disasm

A simple disassembler inspired by [cstool](https://github.com/aquynh/capstone/tree/master/cstool), using [Zydis](https://github.com/zyantific/zydis).

**Example**

```
zdisasm -vv -m amd64 "8b ff 55 8b ec 83 ec 10 a1 00 40 2b 01"
0x0    8b ff       mov edi, edi
0x2    55          push rbp
0x3    8b ec       mov ebp, esp
0x5    83 ec 10    sub esp, 0x10
```
