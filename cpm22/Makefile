OS2CCP.BIN: OS2CCP.ASM
	cargo run ../diskA/ASM.COM OS2CCP.ASM
	grep ^: OS2CCP.HEX > OS2CCP_EOF.HEX # Strip ^D chars at the end of the file
	objcopy -I ihex -O binary OS2CCP_EOF.HEX OS2CCP.BIN
	rm OS2CCP_EOF.HEX

CCP: OS2CCP.BIN