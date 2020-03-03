// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)

// Put your code here.
// ----------------------------------

    //Initalize result to 0
    @R2
    M=0

   
(LOOP)
    @R1
    D=M
    @END
    D;JEQ

    //Load left argument from R0
    @R0
    D=M
    //Load running total from R2
    @R2
    A=M

    //Add left argument to running total
    D=A+D

    //Store result in running total
    @R2
    M=D

    //Load Right argument from R1
    @R1
    D=M
    //Subtract 1 to represent one loop of multiplication
    D=D-1
    //Store subtracted number back into R1
    M=D

    @LOOP
    0;JMP

(END)
    @END
    0;JMP
    