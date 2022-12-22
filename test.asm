.orig 0x100
add r1, r1, #100
lea r0, label
ldw r0, r0, #0
syscall halt
label: .fill 0x106
.END