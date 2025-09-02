# The purpose is to test if the pc ignores the last two bits as it should, in which case it should jump to hlt
.section "CODE"
main:
	jmp 5
	hlt
