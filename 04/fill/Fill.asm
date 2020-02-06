// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

(LOOP)

    @KBD
    D=M
    @PRESS
    D;JNE

    A=0
    D=A
    @R0
    M=D

    @FILL
    0;JMP

(PRESS)
    A=-1

    D=A
    @R0
    M=D

    @FILL
    0;JMP

//Fills screen with value stored in R0
(FILL)
 
    @8192
    D=A
    @R1
    M=D

(FILL_LOOP)

    @R1
    D=M

    @SCREEN
    D=A+D
    
    @R2
    M=D

    @R0
    D=M

    @R2
    A=M

    M=D

    @R1
    D=M
    D=D-1
    M=D

    @FILL_LOOP
    D;JGE

    @LOOP
    0;JMP
    

    
    