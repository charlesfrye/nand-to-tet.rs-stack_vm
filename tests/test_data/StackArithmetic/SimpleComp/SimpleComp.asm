@8
D=A
@SP
A=M
M=D
@SP
M=M+1
@8
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
D=M
A=A-1
D=M-D
@TRUE.1
D;JEQ
D=0
@OUT.1
0;JMP
(TRUE.1)
D=-1
(OUT.1)
@SP
A=M-1
M=D
@1
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
D=M
A=A-1
D=M-D
@TRUE.2
D;JEQ
D=0
@OUT.2
0;JMP
(TRUE.2)
D=-1
(OUT.2)
@SP
A=M-1
M=D
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
@1
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
D=M
A=A-1
D=M-D
@TRUE.3
D;JLT
D=0
@OUT.3
0;JMP
(TRUE.3)
D=-1
(OUT.3)
@SP
A=M-1
M=D
@1
D=A
@SP
A=M
M=D
@SP
M=M+1
@0
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
D=M
A=A-1
D=M-D
@TRUE.4
D;JGT
D=0
@OUT.4
0;JMP
(TRUE.4)
D=-1
(OUT.4)
@SP
A=M-1
M=D

