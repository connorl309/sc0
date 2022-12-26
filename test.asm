.orig 0x100
lea r0, const ; base number
ldb r0, r0, #0 ; r0 = 1
lshf r0, r0, #1 ; r0 = 2
lshf r1, r0, #1 ; r1 = 4
lshf r2, r1, #1 ; does this cascade fail? r2 = 8?
lshf r3, r2, #1
lshf r4, r3, #1
lshf r5, r4, #1
lshf r6, r5, #1
lshf r7, r6, #1
lshf r8, r7, #1
lshf r9, r8, #1
lshf r10, r9, #1
lshf r11, r10, #1
lshf r12, r11, #1 ; end
syscall halt
const: .fill 0x1
.END