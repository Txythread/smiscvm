.mode text
.section "CODE"

_start:
	mov x0, 0xf8
	mov x1, 0x1f

	nand x0, x1
	
	nand x0, x0

	hlt
