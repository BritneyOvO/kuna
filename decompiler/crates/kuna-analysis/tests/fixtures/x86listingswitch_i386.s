	.text
	.globl	_start
_start:
	call	leaf00
	call	leaf01
	call	leaf02
	call	leaf03
	call	leaf04
	call	leaf05
	call	leaf06
	call	leaf07
	call	leaf08
	call	leaf09
	call	leaf10
	call	leaf11
	call	leaf12
	call	leaf13
	call	leaf14
	call	leaf15
	call	leaf16
	call	leaf17
	call	leaf18
	call	leaf19
	call	leaf20
	call	leaf21
	pushl	$2
	call	dispatch
	addl	$4, %esp
	pushl	$1
	call	sdispatch
	addl	$4, %esp
	call	lone_called
	movl	$1, %eax
	xorl	%ebx, %ebx
	int	$0x80
leaf00:
	pushl	%esi
	pushl	%ebx
	movl	$0, %eax
	popl	%ebx
	popl	%esi
	ret
leaf01:
	pushl	%esi
	pushl	%ebx
	movl	$1, %eax
	popl	%ebx
	popl	%esi
	ret
leaf02:
	pushl	%esi
	pushl	%ebx
	movl	$2, %eax
	popl	%ebx
	popl	%esi
	ret
leaf03:
	pushl	%esi
	pushl	%ebx
	movl	$3, %eax
	popl	%ebx
	popl	%esi
	ret
leaf04:
	pushl	%esi
	pushl	%ebx
	movl	$4, %eax
	popl	%ebx
	popl	%esi
	ret
leaf05:
	pushl	%esi
	pushl	%ebx
	movl	$5, %eax
	popl	%ebx
	popl	%esi
	ret
leaf06:
	pushl	%esi
	pushl	%ebx
	movl	$6, %eax
	popl	%ebx
	popl	%esi
	ret
leaf07:
	pushl	%esi
	pushl	%ebx
	movl	$7, %eax
	popl	%ebx
	popl	%esi
	ret
leaf08:
	pushl	%esi
	pushl	%ebx
	movl	$8, %eax
	popl	%ebx
	popl	%esi
	ret
leaf09:
	pushl	%esi
	pushl	%ebx
	movl	$9, %eax
	popl	%ebx
	popl	%esi
	ret
leaf10:
	pushl	%esi
	pushl	%ebx
	movl	$10, %eax
	popl	%ebx
	popl	%esi
	ret
leaf11:
	pushl	%esi
	pushl	%ebx
	movl	$11, %eax
	popl	%ebx
	popl	%esi
	ret
leaf12:
	pushl	%esi
	pushl	%ebx
	movl	$12, %eax
	popl	%ebx
	popl	%esi
	ret
leaf13:
	pushl	%esi
	pushl	%ebx
	movl	$13, %eax
	popl	%ebx
	popl	%esi
	ret
leaf14:
	pushl	%esi
	pushl	%ebx
	movl	$14, %eax
	popl	%ebx
	popl	%esi
	ret
leaf15:
	pushl	%esi
	pushl	%ebx
	movl	$15, %eax
	popl	%ebx
	popl	%esi
	ret
leaf16:
	pushl	%esi
	pushl	%ebx
	movl	$16, %eax
	popl	%ebx
	popl	%esi
	ret
leaf17:
	pushl	%esi
	pushl	%ebx
	movl	$17, %eax
	popl	%ebx
	popl	%esi
	ret
leaf18:
	pushl	%esi
	pushl	%ebx
	movl	$18, %eax
	popl	%ebx
	popl	%esi
	ret
leaf19:
	pushl	%esi
	pushl	%ebx
	movl	$19, %eax
	popl	%ebx
	popl	%esi
	ret
leaf20:
	pushl	%esi
	pushl	%ebx
	movl	$20, %eax
	popl	%ebx
	popl	%esi
	ret
leaf21:
	pushl	%esi
	pushl	%ebx
	movl	$21, %eax
	popl	%ebx
	popl	%esi
	ret
dispatch:
	movl	4(%esp), %eax
	cmpl	$4, %eax
	ja	.Ldefault
	jmp	*jt(,%eax,4)
case0:
	pushl	%esi
	pushl	%ebx
	call	callee0
	popl	%ebx
	popl	%esi
	ret
case1:
	pushl	%esi
	pushl	%ebx
	call	callee1
	popl	%ebx
	popl	%esi
	ret
lone_called:
	pushl	%esi
	pushl	%ebx
	movl	$0x77, %eax
	popl	%ebx
	popl	%esi
	ret
case2:
	pushl	%esi
	pushl	%ebx
	call	callee2
	popl	%ebx
	popl	%esi
	ret
	.fill	12, 1, 0xcc
lone_free:
	pushl	%esi
	pushl	%ebx
	movl	$0x78, %eax
	popl	%ebx
	popl	%esi
	ret
case3:
	pushl	%esi
	pushl	%ebx
	call	callee3
	popl	%ebx
	popl	%esi
	ret
case4:
	pushl	%esi
	pushl	%ebx
	call	callee4
	popl	%ebx
	popl	%esi
	ret
.Ldefault:
	movl	$-1, %eax
	ret
callee0:
	pushl	%esi
	pushl	%ebx
	movl	$100, %eax
	popl	%ebx
	popl	%esi
	ret
callee1:
	pushl	%esi
	pushl	%ebx
	movl	$101, %eax
	popl	%ebx
	popl	%esi
	ret
callee2:
	pushl	%esi
	pushl	%ebx
	movl	$102, %eax
	popl	%ebx
	popl	%esi
	ret
callee3:
	pushl	%esi
	pushl	%ebx
	movl	$103, %eax
	popl	%ebx
	popl	%esi
	ret
callee4:
	pushl	%esi
	pushl	%ebx
	movl	$104, %eax
	popl	%ebx
	popl	%esi
	ret
sdispatch:
	movl	4(%esp), %eax
	cmpl	$2, %eax
	jg	.Lsdefault
	jmp	*sjt(,%eax,4)
scase0:
	pushl	%esi
	pushl	%ebx
	movl	$10, %eax
	popl	%ebx
	popl	%esi
	ret
scase1:
	pushl	%esi
	pushl	%ebx
	movl	$11, %eax
	popl	%ebx
	popl	%esi
	ret
scase2:
	pushl	%esi
	pushl	%ebx
	movl	$12, %eax
	popl	%ebx
	popl	%esi
	ret
.Lsdefault:
	movl	$-1, %eax
	ret
independent:
	pushl	%esi
	pushl	%ebx
	xorl	%eax, %eax
	popl	%ebx
	popl	%esi
	ret
	.section	.rodata
	.align	4
jt:
	.long	case0
	.long	case1
	.long	case2
	.long	case3
	.long	case4
adjacent_callbacks:
	.long	independent
sjt:
	.long	scase0
	.long	scase1
	.long	scase2
