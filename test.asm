.orig 0x20
LABEL: add r0, r0, #100 ; comment
       mul r0, r0, #2
       xor r0, r0, 0xA
       BRp LABEL
       syscall halt
.END