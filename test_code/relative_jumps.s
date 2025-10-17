.mode text
.section "CODE"

_main:
	adrp sp, stack_end@PAGE
	add sp, stack_end@PAGEOFF
	mov x0, 0
	jmpr _stop@RELATIVE
	mov x0, 1

_stop:
	
	adrp x0, msg@PAGE
	add x0, msg@PAGEOFF
	mov x1, msg_end@PAGEOFF - msg@PAGEOFF
	calr _print@RELATIVE
	hlt

!include debuglib


.mode data
.section "DATA"
msg:
	.ascii "Hello, world!"
msg_end:
.section "STACK"
.section "STOP"
stack_end:
