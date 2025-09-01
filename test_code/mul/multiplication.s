.section "CODE"
main:
	# Set the stack start position
	adrp sp, stack-end@PAGE
	sub sp, 4
	
	# Load the address of the multiply function to x2
	adrp x2, mul@PAGE
	add x2, mul@PAGEOFF

	# Calculate 8 * 13
	mov x0, 13
	mov x1, 8
	cal x2
	hlt
	



!include bscmath



.section "DATA"
.section "STACK"
.section "END"
stack-end:
