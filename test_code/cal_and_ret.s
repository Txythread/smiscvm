.section "CODE"
main:
	add x10, 1
	adrp sp, stack-end@PAGE
	sub sp, 4 # go back to leave space for an entire quad word
	adrp x3, function@PAGE
	add x3, function@PAGEOFF
	cal x3
	mov x1, 1
	hlt
	
function:
	add x0, 1
	ret

.section "DATA"

.section "STACK"
.section "END"
stack-end:
