.orig 0x100

mov r0, #0 ; counter

stackLoop: add r0, r0, #1
           push r0
           cmp r13, 0xFFF0
           brp stackLoop
mov r1, r0
lea r0, stringer
syscall print
mov r0, r1
syscall display
syscall halt
stringer: .string "Number of items pushed to stack is"
.end