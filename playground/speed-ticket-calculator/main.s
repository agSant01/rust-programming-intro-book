	.file	"main.c"
	.intel_syntax noprefix
	.text
	.globl	SPEED_LIMIT
	.section	.rodata
	.align 4
	.type	SPEED_LIMIT, @object
	.size	SPEED_LIMIT, 4
SPEED_LIMIT:
	.long	60
	.globl	EXCESSIVE_SPEED
	.align 4
	.type	EXCESSIVE_SPEED, @object
	.size	EXCESSIVE_SPEED, 4
EXCESSIVE_SPEED:
	.long	80
	.globl	BIRTHDAY_INCREASE
	.align 4
	.type	BIRTHDAY_INCREASE, @object
	.size	BIRTHDAY_INCREASE, 4
BIRTHDAY_INCREASE:
	.long	5
	.text
	.globl	speedingCalculator
	.type	speedingCalculator, @function
speedingCalculator:
.LFB0:
	.cfi_startproc
	push	rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	mov	rbp, rsp
	.cfi_def_cfa_register 6
	mov	DWORD PTR -20[rbp], edi
	mov	eax, esi
	mov	BYTE PTR -24[rbp], al
	cmp	BYTE PTR -24[rbp], 0
	je	.L2
	mov	eax, 5
	jmp	.L3
.L2:
	mov	eax, 0
.L3:
	mov	DWORD PTR -4[rbp], eax
	mov	edx, 80
	mov	eax, DWORD PTR -4[rbp]
	add	eax, edx
	cmp	DWORD PTR -20[rbp], eax
	jle	.L4
	mov	eax, 2
	jmp	.L5
.L4:
	mov	edx, 60
	mov	eax, DWORD PTR -4[rbp]
	add	eax, edx
	cmp	DWORD PTR -20[rbp], eax
	jle	.L6
	mov	eax, 1
	jmp	.L5
.L6:
	mov	eax, 0
.L5:
	pop	rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE0:
	.size	speedingCalculator, .-speedingCalculator
	.section	.rodata
.LC0:
	.string	"Hello world!"
.LC1:
	.string	"%d\n"
	.text
	.globl	main
	.type	main, @function
main:
.LFB1:
	.cfi_startproc
	push	rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	mov	rbp, rsp
	.cfi_def_cfa_register 6
	sub	rsp, 16
	lea	rdi, .LC0[rip]
	call	puts@PLT
	mov	esi, 0
	mov	edi, 60
	call	speedingCalculator
	mov	DWORD PTR -4[rbp], eax
	mov	eax, DWORD PTR -4[rbp]
	mov	esi, eax
	lea	rdi, .LC1[rip]
	mov	eax, 0
	call	printf@PLT
	mov	esi, 1
	mov	edi, 65
	call	speedingCalculator
	mov	DWORD PTR -4[rbp], eax
	mov	eax, DWORD PTR -4[rbp]
	mov	esi, eax
	lea	rdi, .LC1[rip]
	mov	eax, 0
	call	printf@PLT
	mov	esi, 0
	mov	edi, 65
	call	speedingCalculator
	mov	DWORD PTR -4[rbp], eax
	mov	eax, DWORD PTR -4[rbp]
	mov	esi, eax
	lea	rdi, .LC1[rip]
	mov	eax, 0
	call	printf@PLT
	nop
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE1:
	.size	main, .-main
	.ident	"GCC: (Ubuntu 7.5.0-3ubuntu1~18.04) 7.5.0"
	.section	.note.GNU-stack,"",@progbits
