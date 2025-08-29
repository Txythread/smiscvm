.section "CODE"
# Global register usage:
# x4 ... current position in msg
# x5 ... msg end
# x6 ... end function
# x7 ... loop function
main:
	# Register overwrites:
	# x4 ... msg start
	adrp x4, msg@PAGE
	add x4, msg@PAGEOFF
	mov x5, x4
	add x5, msg-len
	adrp x6, end@PAGE
	mov x7, x6
	add x6, end@PAGEOFF
	add x7, loop@PAGEOFF
loop:
	# Register usage overwrites:
	# x0 ... output data
	# x1 ... current position in msg (x4) - end of message (x5)
	lb x0, x4
	out x0
	# Increment position and jmpz to end if needed, else continue the loop
	mov x1, x4
	sub x1, x5
	jmpz x1, x6
	add x4, 1
	jmp x7
end:
	hlt

.section "DATA"
msg:
	.ascii "Hello, world!"
.msg-len 13
