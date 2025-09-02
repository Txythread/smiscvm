.section "CODE"
# Some random un-occupied address
.address 0x1000

# The data to move, can essentially be any non-zero value
.data 0x5F
main:
	# store the data in an intermediate register
	mov x0, data
	
	# store the address in an intermediate register, too
	mov x1, address

	# store the data (just the LSB of the register)
	sb x0, x1
	
	# load the data into x5
	lb x5, x1

	hlt
