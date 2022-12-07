# **S**imple **C**omputer 0

## What is this?

The Simple Computer 0 (**SC0**) is a hobby project of mine that seeks to emulate a basic computing system from scratch. Inspired by [Dr. Yale Patt's LC3(B)](https://users.ece.utexas.edu/~patt/), the SC0 is a piece of software written in Rust that implements its own instruction set and basic assembly-esque language. The SC0 parses input programs, converts them into "readable" code, and executes it.

I hope, in creating this project, that I can learn more about computer architecture as well as express the building blocks of computers in an elegant, easy to read manner.

## SC0 Details

The SC0 is an expansion of the LC3-B by extending to use 32 bits of space for each memory address, compared to the LC3-B's 16 bits. This brings multiple benefits, notably:

- more bits means for instructions -> more instructions! (everybody likes more instructions. Right, x86?)
- more bits means bigger constants! Hooray for constants.
- more bits means greater flexibility with memory
- more registers! (technically. Really, this is limited by the hardware's register file and addressability, but in software this is all arbitrary)

More concretely, the SC0 has 32kB of memory (this might be changed to be user-defined!), with 4 bytes of space available at each memory address. Via simple multiplication, that's 128 whole kilobytes of stuff to work with! That's a lotta bytes! Realistically no program will ever use this much space. Hopefully. Who knows, maybe I'll finally lose my marbles and write a C to SC0 compiler. But that's a topic for another day.

In terms of registers, the SC0 has 15 of them. Three are "reserved" by the Simple Computer (the **P**rogram **S**tatus **R**egister [PSR], the **P**rogram **C**ounter [PC], and the **S**tack **P**ointer). <ins>For table use, please note that "GPR" stands for **general purpose**</ins>. Register makeup is as follows (sorry for the table length!):

#
<p align="center">

| Register | Description / Function | Register |    Description / Function     |
| :------: | :--------------------: | :------: | :---------------------------: |
|    R0    |          GPR           |    R1    |              GPR              |
|    R2    |          GPR           |    R3    |              GPR              |
|    R4    |          GPR           |    R5    |              GPR              |
|    R6    |          GPR           |    R7    |              GPR              |
|    R8    |          GPR           |    R9    |              GPR              |
|   R10    |          GPR           |   R11    |              GPR              |
|   R12    |     Stack pointer      |   R13    |      GPR/Return register      |
|    PC    | Program Counter (R14)  |   PSR    | Program Status Register (R15) |

</p>
#

The instruction list and format is as follows. Please note that a lot of instructions can use either immediates (constants) or registers; that is what "reg/imm" denotes. The SC0 ***only*** operates on [2's complement](https://www.cs.cornell.edu/~tomf/notes/cps104/twoscomp.html) integer values, so any operations that *could* result in a floating point result (i.e., division) will truncate to an integer. Any instruction marked with ✞ set condition codes in the PSR.

#

|   Instruction    |          Format           |               Pseudocode                |                      Notes                      |
| :--------------: | :-----------------------: | :-------------------------------------: | :---------------------------------------------: |
| ADD<sup>✞</sup>  | add dest, src1, src2/imm  |         dest = src1 + src2/imm          |                       NA                        |
| SUB<sup>✞</sup>  | sub dest, src1, src2/imm  |         dest = src1 - src2/imm          |                       NA                        |
| MUL<sup>✞</sup>  | mul dest, src1, src2/imm  |         dest = src1 * src2/imm          |                       NA                        |
| DIV<sup>✞</sup>  | div dest, src1, src2/imm  |         dest = src1 ÷ src2/imm          |                       NA                        |
| MOV<sup>✞</sup>  |       mov dest, src       |               dest = src                |                       NA                        |
| AND<sup>✞</sup>  | and dest, src1, src2/imm  |         dest = src1 & src2/imm          |                       NA                        |
|  OR<sup>✞</sup>  |  or dest, src1, src2/imm  |         dest = src1 \| src2/imm         |                       NA                        |
| NOT<sup>✞</sup>  |       not dest, src       |               dest = ~src               |                       NA                        |
| LSHF<sup>✞</sup> | lshf dest, src1, src2/imm |         dest = src1 << src2/imm         |                       NA                        |
| RSHF<sup>✞</sup> | rshf dest, src1, src2/imm |         dest = src1 >> src2/imm         |                       NA                        |
|       LEA        |      lea dest, LABEL      |         dest = address of label         |                       NA                        |
| LDB<sup>✞</sup>  | ldb dest, src1, src2/imm  |    dest = BYTE(mem[src1 + src2/imm])    |                       NA                        |
| LDW<sup>✞</sup>  | ldw dest, src1, src2/imm  |    dest = WORD(mem[src1 + src2/imm])    |                       NA                        |
| LDD<sup>✞</sup>  | ldq dest, src1, src2/imm  |   dest = DWORD(mem[src1 + src2/imm])    |                       NA                        |
|       STB        |     stb dest, src/imm     |        BYTE(mem[dest]) = src/imm        |                       NA                        |
|       STW        |     stw dest, src/imm     |        WORD(mem[dest]) = src/imm        |                       NA                        |
|       STD        |     std dest, src/imm     |       DWORD(mem[dest]) = src/imm        |                       NA                        |
|       JMP        |      jmp dest/LABEL       |             PC = dest/LABEL             |                       NA                        |
|       CALL       |      call dest/LABEL      | R13 = PC<sup>inc</sup>, PC = dest/LABEL |                       NA                        |
|     BR(nzp)      |       BR(nzp) LABEL       |               PC = LABEL                |    Branch only if any condition code matches    |
| CMP<sup>✞</sup>  |    cmp src1, src2/imm     |            (src1 - src2/imm)            | Sets condition codes based on result of compare |
|       PUSH       |       push src1/imm       |        mem[sp] = src1/imm; sp++         |                       NA                        |
| POP<sup>✞</sup>  |         pop dest          |          sp--; dest = mem[sp]           |                Loads all 4 bytes                |
