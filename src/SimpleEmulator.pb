;VerySimpleEmulator
;Flaith-27.12.2010

;-Datasection
DataSection
  _DS_Memory:
    Data.u  34                      ;Nombre de donn�es
    Data.a  $00,$21,$30             ;$00 indique une addition de $21 et de $30
    Data.a  $00,$F0,$50             ;Additionne $F0 � $50
    Data.a  $01,$33,$45             ;$01 est une soustraction
    Data.a  $01,$F0,$53
    Data.a  $02,$33,$03             ;$02 est une multiplication
    Data.a  $03,$DE,$45             ;$03 est une division
    Data.a  $EA                     ;$EA = NOP ==> ne fait rien du tout
    Data.a  $EA
    Data.a  $EA
    Data.a  $00,$01,$01
    Data.a  $01,$01,$01
    Data.a  $02,$01,$01
    Data.a  $03,$01,$01
    Data.a  $FF                     ;Fin du programme
EndDataSection

;-Globals
Global Dim Instruction.i(255)       ;Un entier car on conserve l'adresse de chaque proc�dure
Global Dim Memory.a($FFFF)          ;On cr�� une m�moire de 65536 Octets
Global PC.i                         ;Program Counter : A quel endroit est notre opcode ? (index)

;-Proc�dures Internes
; Affiche en hexa
Procedure.s Hexa(__Value.a)
  ProcedureReturn RSet(Hex(__Value,#PB_Ascii),2,"0")
EndProcedure

; R�cup�re un octet dans la m�moire incr�ment� automatiquement
Procedure.a GetMemory()
  PC + 1
  ProcedureReturn Memory(PC)
EndProcedure

;-Proc�dures li�es aux Opcodes
Procedure.a add()
  Protected.a a,b

  a = GetMemory()
  b = GetMemory()
  Debug "----Add $"+hexa(a)+" to $"+hexa(b)
  ProcedureReturn a+b
EndProcedure

Procedure.a sub()
  Protected.a a,b

  a = GetMemory()
  b = GetMemory()
  Debug "----Sub $"+hexa(a)+" to $"+hexa(b)
  ProcedureReturn a-b
EndProcedure

Procedure.a mul()
  Protected.a a,b

  a = GetMemory()
  b = GetMemory()
  Debug "----Mul $"+hexa(a)+" to $"+hexa(b)
  ProcedureReturn a*b
EndProcedure

Procedure.a div()
  Protected.a a,b

  a = GetMemory()
  b = GetMemory()
  Debug "----Div $"+hexa(a)+" to $"+hexa(b)
  ProcedureReturn a/b
EndProcedure

Procedure.a nop()
  Debug "----Nop"
  ProcedureReturn $EA
EndProcedure

;Liaison Tableau des instructions avec l'adresse de chaque proc�dure
Instruction($00) = @add()
Instruction($01) = @sub()
Instruction($02) = @mul()
Instruction($03) = @div()
Instruction($EA) = @nop()

;-Lecture des opcodes et insertion dans la m�moire
Restore _DS_Memory
Read.u longueur
For i = 0 To longueur-1
  Read.a aValue
  Memory(i) = aValue
Next i

; On commence � z�ro
PC = 0
; OBLIGATOIRE de r�cup�rer le 1ER OPCODE dans la m�moire
; avant de faire appel � "GetMemory" dans la boucle principale
Opcode = Memory(PC)

;-Boucle principale
While Opcode <> $FF
  ;On appelle les proc�dures li�es aux opcodes
  Debug "$"+Hexa(CallFunctionFast(Instruction(Opcode)))
  Opcode = GetMemory()
Wend
Debug "----End"
End

; IDE Options = PureBasic 6.00 Beta 10 (Windows - x64)
; CursorPosition = 93
; FirstLine = 75
; Folding = --
; CompileSourceDirectory
; EnableCompileCount = 52
; EnableBuildCount = 0
; EnableExeConstant