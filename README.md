## disasm

A simple disassembler inspired by [cstool](https://github.com/aquynh/capstone/tree/master/cstool), using [Capstone engine](https://github.com/aquynh/capstone).

**Example**

```
cdisasm -vv -m x32 "8b ff 55 8b ec 83 ec 10 a1 00 40 2b 01"
0x0    8b ff             mov edi, edi
0x2    55                push ebp
0x3    8b ec             mov ebp, esp
0x5    83 ec 10          sub esp, 0x10
0x8    a1 00 40 2b 01    mov eax, dword ptr [0x12b4000]
```

You may want to checkout [this branch](https://github.com/tathanhdinh/objdisasm/tree/wip_zydis) using [Zydis library](https://github.com/zyantific/zydis).
