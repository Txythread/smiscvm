.section "CODE"
main:
	adrp sp, stack-end@PAGE
	sub sp, 4 # go back to leave space for an entire quad word
	adrp x3, function@PAGE
	add x3, function@PAGEOFF
	cal x3
	mov x1, 1
	hlt
	
function:
	mov x0, 1
	hlt

.section "DATA"

.section "STACK"
.section "END"
stack-end:
