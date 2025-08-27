.section "CODE"
.counter-start 5
main:
	mov x0, counter-start
	adrp x1, end@PAGE
	add x1, end@PAGEOFF
	adrp x2, main-loop@PAGE
	add x2, main-loop@PAGEOFF
main-loop:
	sub x0, 1
	jmpz x0, x1
	jmp x2
end:
	hlt
