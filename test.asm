.ORIG 1000
add r0, r0, 25 # comments test
# this should work and not be logged whatsoever
sub r0, r0, 200
mul r0, r0, r0
div r0, r1, r9
mov r0, 69
and r1, r2, 3
or abc, abc, abc # parameters do not need to be correct for testing purposes right now.
not a, abc
xor 2, 2
lshf 1, 1, 1
rshf 1, 1, 1
lea a, be
ldi 1 1 # this should work with commas OR spaces
ldb 1 1 1
ldw 1 1 1
ldd 1 1 1
sti 1 1
stb 1 1 1
stw 1 1 1
std 1 1 1
jmp hithere
call hithere2
syscall abcdefg
br 12
brnzp 100
cmp a, b
push 100
pop r0
# should be invalid opcode here.
# funny 100 100 wahoo :)
.END