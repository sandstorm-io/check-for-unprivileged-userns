.globl main
.text
main:
	# setuid(1), to make sure we aren't root. The numeric UID is irrelevant
	# as long as it is not zero. If we are already non-root, setuid() will
	# fail, but that's fine because we didn't need it anyway in that case.
	mov $0x1,%edi
	mov $0x69,%eax
	syscall

	# x = unshare(CLONE_NEWUSER)
	mov $0x10000000,%edi
	mov $0x110,%eax
	syscall

	# _exit(x)
	mov %eax,%edi
	mov $0x3c,%eax
	syscall

