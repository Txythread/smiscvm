.section "CODE"
main:
	# Load stack address
	adrp sp, stack-end@PAGE
	add sp, stack-end@PAGEOFF
	
	# Load address of msg
	adrp x0, msg@PAGE
	add x0, msg@PAGEOFF
	
	# Load length of msg
	mov x1, msg-length
	
	# Get address of and call print function
	adrp x2, print@PAGE
	add x2, print@PAGEOFF
	cal x2
	

	hlt

!include debuglib

.section "DATA"
msg:
	.stc "Hello, world!"
.msg-length 13

.section "STACK"
.section "END"
stack-end:
