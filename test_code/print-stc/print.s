.mode code
.section "CODE"


_main:
	# Load stack address
	adrp sp, stack_end@PAGE
	add sp, stack_end@PAGEOFF
	
	# Load address of msg
	adrp x0, msg@PAGE
	add x0, msg@PAGEOFF
	
	# Load length of msg
	mov x1, msg_end@PAGEOFF - msg@PAGEOFF
	
	# Get address of and call print function
	adrp x2, _print@PAGE
	add x2, _print@PAGEOFF
	cal x2
	

	hlt

!include debuglib


.mode data
.section "DATA"
msg:
	.stc "Hello, world!"
msg_end:

.section "STACK"
.section "END"
stack_end:
