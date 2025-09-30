.section "CODE"
main:
	# Load stack address
	adrp sp, stack_end@PAGE
	add sp, stack_end@PAGEOFF
	
	# Load address of msg
	adrp x0, msg@PAGE
	add x0, msg@PAGEOFF
	
	# Load length of msg
	mov x1, msg_length
	
	# Get address of and call print function
	adrp x2, print@PAGE
	add x2, print@PAGEOFF
	cal x2
	

	hlt

!include debuglib

.section "DATA"
msg:
	.ascii "Hello, world!"
.msg_length 13

.section "STACK"
.section "END"
stack_end:
