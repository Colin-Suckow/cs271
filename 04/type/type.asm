// 16 Font height
// 32 Step one row
//10 stores current horizontal character index
//11 stores vertical character index
// 32 x 16 characters
//Fonts start at 1000

//CODE

// Zero location registers
@0
D=A
@10
M=D
@11
M=D

@200
D=A

@PC
M=D


(END)
@END
0;JMP




//DATA

(LOAD_FONT)

    //Mem location index
    @0
    D=A
    @R0
    M=D

    //A
    @1920
    @1920
    @4032
    @3264
    @7392
    @14448
    @14391
    @8216
    @24588
    @57358
    @57358
    @-16381
    @-16381
    @-16381
    @-16381
