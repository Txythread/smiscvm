.mode text
.section "CODE"

_main:
	adrp sp, stack_end@PAGE
	add sp, stack_end@PAGEOFF
	
	mov x0, 1
	add x0, -1

	hlt


.mode data
.section "DATA"
.section "STACK"
stack_end:
