.orig 0x100
mov r0, 0xAAFF
lshf r0, r0, #1
syscall display

looper: rshf r0, r0, #1 ; divide by 2 every time
        syscall display
        BRnp looper

syscall halt

backer: .fill 0xA
.end