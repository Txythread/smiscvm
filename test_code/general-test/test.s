.section "CODE"
main:
	# Load the stack pointer's position
	adrp sp, stack-end@PAGE
	sub sp, 4

	
	mov x0, 53 # 0x35
	add x0, 11
	mov x1, x0
	add x0, x1
	
	# Expected current state:
	# x0 ... 0x80
	# x1 ... 0x40

	adrp x3, hex-fourty-one@PAGE
	add x3, hex-fourty-one@PAGEOFF
	lb x2, x3
	add x1, x2
	
	# Expected current state:
	# x0 ... 0x80
	# x1 ... 0x81
	# x2 ... 0x41
	
	sub x1, x0
	sub x2, 1

	# Expected current state:
	# x0 ... 0x80
	# x1 ... 0x01
	# x2 ... 0x40

	sub x0, 125 # 0x7D

	# Expected current state:
	# x0 ... 0x03
	# x1 ... 0x01
	# x2 ... 0x40

	mov x3, x2
	mov x2, x1
	mov x1, x3

	# Multiply what should be 3 (x0) & 0x40 (x1)
	adrp x3, mul@PAGE
	add x3, mul@PAGEOFF
	cal x3

	# Expected current state:
	# x0 ... 0xC0

	sub x0, 184

	# x0 should now contain: 0x8
	
	# push x0 to memory add the selected region
	adrp x3, result-addr@PAGE
	add x3, result-addr@PAGEOFF
	
	sb x0, x3
	hlt

!include bscmath
.section "DATA"
# just something containing hexadecimal fourty one or uppercase 'A'
hex-fourty-one:
	.ascii "A"
.section "STACK"
.section "RESULT"
stack-end:
result-addr:
	.ascii "RES."
