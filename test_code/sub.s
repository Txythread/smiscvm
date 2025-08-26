.section "CODE"
main:
	mov x2, msg@PAGE
	adrp x1, x2
	add x1, msg@PAGEOFF
	lb x0, x1
	mov x1, x0

.section "DATA"
msg:
	.ascii "b"
