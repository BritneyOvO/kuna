	.text
	.globl	sh_ja
	.type	sh_ja, @function
sh_ja:
	movl	4(%esp), %eax
	cmpl	$3, %eax
	ja	.Ld_ja
	jmp	*t_ja(,%eax,4)
.Lc_ja_0:
	movl	$256, %eax
	ret
.Lc_ja_1:
	movl	$257, %eax
	ret
.Lc_ja_2:
	movl	$258, %eax
	ret
.Lc_ja_3:
	movl	$259, %eax
	ret
.Ld_ja:
	movl	$-1, %eax
	ret
	.size	sh_ja, .-sh_ja
	.globl	sh_jae
	.type	sh_jae, @function
sh_jae:
	movl	4(%esp), %eax
	cmpl	$4, %eax
	jae	.Ld_jae
	jmp	*t_jae(,%eax,4)
.Lc_jae_0:
	movl	$512, %eax
	ret
.Lc_jae_1:
	movl	$513, %eax
	ret
.Lc_jae_2:
	movl	$514, %eax
	ret
.Lc_jae_3:
	movl	$515, %eax
	ret
.Ld_jae:
	movl	$-1, %eax
	ret
	.size	sh_jae, .-sh_jae
	.globl	sh_prenorm
	.type	sh_prenorm, @function
sh_prenorm:
	movl	4(%esp), %eax
	subl	$1, %eax
	cmpl	$3, %eax
	ja	.Ld_prenorm
	jmp	*t_prenorm(,%eax,4)
.Lc_prenorm_0:
	movl	$768, %eax
	ret
.Lc_prenorm_1:
	movl	$769, %eax
	ret
.Lc_prenorm_2:
	movl	$770, %eax
	ret
.Lc_prenorm_3:
	movl	$771, %eax
	ret
.Ld_prenorm:
	movl	$-1, %eax
	ret
	.size	sh_prenorm, .-sh_prenorm
	.globl	sh_sched
	.type	sh_sched, @function
sh_sched:
	movl	4(%esp), %eax
	cmpl	$3, %eax
	movl	8(%esp), %ecx
	ja	.Ld_sched
	jmp	*t_sched(,%eax,4)
.Lc_sched_0:
	movl	$1024, %eax
	ret
.Lc_sched_1:
	movl	$1025, %eax
	ret
.Lc_sched_2:
	movl	$1026, %eax
	ret
.Lc_sched_3:
	movl	$1027, %eax
	ret
.Ld_sched:
	movl	$-1, %eax
	ret
	.size	sh_sched, .-sh_sched
	.globl	sh_repeat
	.type	sh_repeat, @function
sh_repeat:
	movl	4(%esp), %eax
	cmpl	$3, %eax
	ja	.Ld_repeat
	jmp	*t_repeat(,%eax,4)
.Lc_repeat_0:
	movl	$1280, %eax
	ret
.Lc_repeat_1:
	movl	$1281, %eax
	ret
.Lc_repeat_2:
	movl	$1282, %eax
	ret
.Ld_repeat:
	movl	$-1, %eax
	ret
	.size	sh_repeat, .-sh_repeat
	.globl	sh_ceiling
	.type	sh_ceiling, @function
sh_ceiling:
	movl	4(%esp), %eax
	cmpl	$7, %eax
	ja	.Ld_ceiling
	jmp	*t_ceiling(,%eax,4)
.Lc_ceiling_0:
	movl	$1536, %eax
	ret
.Lc_ceiling_1:
	movl	$1537, %eax
	ret
.Lc_ceiling_2:
	movl	$1538, %eax
	ret
.Lc_ceiling_3:
	movl	$1539, %eax
	ret
.Lc_ceiling_4:
	movl	$1540, %eax
	ret
.Lc_ceiling_5:
	movl	$1541, %eax
	ret
.Lc_ceiling_6:
	movl	$1542, %eax
	ret
.Lc_ceiling_7:
	movl	$1543, %eax
	ret
.Ld_ceiling:
	movl	$-1, %eax
	ret
	.size	sh_ceiling, .-sh_ceiling
	.globl	sh_adjacent
	.type	sh_adjacent, @function
sh_adjacent:
	movl	4(%esp), %eax
	cmpl	$3, %eax
	ja	.Ld_adjacent
	jmp	*t_adjacent(,%eax,4)
.Lc_adjacent_0:
	movl	$1792, %eax
	ret
.Lc_adjacent_1:
	movl	$1793, %eax
	ret
.Lc_adjacent_2:
	movl	$1794, %eax
	ret
.Lc_adjacent_3:
	movl	$1795, %eax
	ret
.Ld_adjacent:
	movl	$-1, %eax
	ret
	.size	sh_adjacent, .-sh_adjacent
.Lindependent:
	xorl	%eax, %eax
	ret
	.globl	sh_noguard
	.type	sh_noguard, @function
sh_noguard:
	movl	4(%esp), %eax
	jmp	*t_noguard(,%eax,4)
.Lc_noguard_0:
	movl	$2048, %eax
	ret
.Lc_noguard_1:
	movl	$2049, %eax
	ret
.Lc_noguard_2:
	movl	$2050, %eax
	ret
.Lc_noguard_3:
	movl	$2051, %eax
	ret
.Ld_noguard:
	movl	$-1, %eax
	ret
	.size	sh_noguard, .-sh_noguard
	.globl	sh_wrongidx
	.type	sh_wrongidx, @function
sh_wrongidx:
	movl	4(%esp), %eax
	movl	8(%esp), %ecx
	cmpl	$3, %ecx
	ja	.Ld_wrongidx
	jmp	*t_wrongidx(,%eax,4)
.Lc_wrongidx_0:
	movl	$2304, %eax
	ret
.Lc_wrongidx_1:
	movl	$2305, %eax
	ret
.Lc_wrongidx_2:
	movl	$2306, %eax
	ret
.Lc_wrongidx_3:
	movl	$2307, %eax
	ret
.Ld_wrongidx:
	movl	$-1, %eax
	ret
	.size	sh_wrongidx, .-sh_wrongidx
	.globl	sh_overwritten
	.type	sh_overwritten, @function
sh_overwritten:
	movl	4(%esp), %eax
	cmpl	$3, %eax
	ja	.Ld_overwritten
	movl	8(%esp), %eax
	jmp	*t_overwritten(,%eax,4)
.Lc_overwritten_0:
	movl	$2560, %eax
	ret
.Lc_overwritten_1:
	movl	$2561, %eax
	ret
.Lc_overwritten_2:
	movl	$2562, %eax
	ret
.Lc_overwritten_3:
	movl	$2563, %eax
	ret
.Ld_overwritten:
	movl	$-1, %eax
	ret
	.size	sh_overwritten, .-sh_overwritten
	.globl	sh_partial
	.type	sh_partial, @function
sh_partial:
	movl	4(%esp), %eax
	cmpl	$3, %eax
	ja	.Ld_partial
	movb	$1, %al
	jmp	*t_partial(,%eax,4)
.Lc_partial_0:
	movl	$2816, %eax
	ret
.Lc_partial_1:
	movl	$2817, %eax
	ret
.Lc_partial_2:
	movl	$2818, %eax
	ret
.Lc_partial_3:
	movl	$2819, %eax
	ret
.Ld_partial:
	movl	$-1, %eax
	ret
	.size	sh_partial, .-sh_partial
	.globl	sh_signed
	.type	sh_signed, @function
sh_signed:
	movl	4(%esp), %eax
	cmpl	$3, %eax
	jg	.Ld_signed
	jmp	*t_signed(,%eax,4)
.Lc_signed_0:
	movl	$3072, %eax
	ret
.Lc_signed_1:
	movl	$3073, %eax
	ret
.Lc_signed_2:
	movl	$3074, %eax
	ret
.Lc_signed_3:
	movl	$3075, %eax
	ret
.Ld_signed:
	movl	$-1, %eax
	ret
	.size	sh_signed, .-sh_signed
	.globl	sh_lowerbound
	.type	sh_lowerbound, @function
sh_lowerbound:
	movl	4(%esp), %eax
	cmpl	$3, %eax
	jb	.Ld_lowerbound
	jmp	*t_lowerbound(,%eax,4)
.Lc_lowerbound_0:
	movl	$3328, %eax
	ret
.Lc_lowerbound_1:
	movl	$3329, %eax
	ret
.Lc_lowerbound_2:
	movl	$3330, %eax
	ret
.Lc_lowerbound_3:
	movl	$3331, %eax
	ret
.Ld_lowerbound:
	movl	$-1, %eax
	ret
	.size	sh_lowerbound, .-sh_lowerbound
	.globl	sh_bytecmp
	.type	sh_bytecmp, @function
sh_bytecmp:
	movl	4(%esp), %eax
	cmpb	$3, %al
	ja	.Ld_bytecmp
	jmp	*t_bytecmp(,%eax,4)
.Lc_bytecmp_0:
	movl	$3584, %eax
	ret
.Lc_bytecmp_1:
	movl	$3585, %eax
	ret
.Lc_bytecmp_2:
	movl	$3586, %eax
	ret
.Lc_bytecmp_3:
	movl	$3587, %eax
	ret
.Ld_bytecmp:
	movl	$-1, %eax
	ret
	.size	sh_bytecmp, .-sh_bytecmp
	.globl	sh_postnorm
	.type	sh_postnorm, @function
sh_postnorm:
	movl	4(%esp), %eax
	cmpl	$4, %eax
	ja	.Ld_postnorm
	subl	$1, %eax
	jmp	*t_postnorm(,%eax,4)
.Lc_postnorm_0:
	movl	$3840, %eax
	ret
.Lc_postnorm_1:
	movl	$3841, %eax
	ret
.Lc_postnorm_2:
	movl	$3842, %eax
	ret
.Lc_postnorm_3:
	movl	$3843, %eax
	ret
.Ld_postnorm:
	movl	$-1, %eax
	ret
	.size	sh_postnorm, .-sh_postnorm
	.globl	sh_flagclobber
	.type	sh_flagclobber, @function
sh_flagclobber:
	movl	4(%esp), %eax
	cmpl	$3, %eax
	testl	%ecx, %ecx
	ja	.Ld_flagclobber
	jmp	*t_flagclobber(,%eax,4)
.Lc_flagclobber_0:
	movl	$4096, %eax
	ret
.Lc_flagclobber_1:
	movl	$4097, %eax
	ret
.Lc_flagclobber_2:
	movl	$4098, %eax
	ret
.Lc_flagclobber_3:
	movl	$4099, %eax
	ret
.Ld_flagclobber:
	movl	$-1, %eax
	ret
	.size	sh_flagclobber, .-sh_flagclobber
	.globl	sh_bypass
	.type	sh_bypass, @function
sh_bypass:
	movl	4(%esp), %eax
	cmpl	$10, %ecx
	je	.Lbp_disp
	cmpl	$3, %eax
	ja	.Ld_bypass
.Lbp_disp:
	jmp	*t_bypass(,%eax,4)
.Lc_bypass_0:
	movl	$4352, %eax
	ret
.Lc_bypass_1:
	movl	$4353, %eax
	ret
.Lc_bypass_2:
	movl	$4354, %eax
	ret
.Lc_bypass_3:
	movl	$4355, %eax
	ret
.Ld_bypass:
	movl	$-1, %eax
	ret
	.size	sh_bypass, .-sh_bypass
	.globl	sh_bypassmid
	.type	sh_bypassmid, @function
sh_bypassmid:
	movl	4(%esp), %eax
	cmpl	$1, %ecx
	je	.Lbm_j
	cmpl	$3, %eax
.Lbm_j:
	ja	.Ld_bypassmid
	jmp	*t_bypassmid(,%eax,4)
.Lc_bypassmid_0:
	movl	$4608, %eax
	ret
.Lc_bypassmid_1:
	movl	$4609, %eax
	ret
.Lc_bypassmid_2:
	movl	$4610, %eax
	ret
.Lc_bypassmid_3:
	movl	$4611, %eax
	ret
.Ld_bypassmid:
	movl	$-1, %eax
	ret
	.size	sh_bypassmid, .-sh_bypassmid
	.globl	sh_writable
	.type	sh_writable, @function
sh_writable:
	movl	4(%esp), %eax
	cmpl	$3, %eax
	ja	.Ld_writable
	jmp	*t_writable(,%eax,4)
.Lc_writable_0:
	movl	$4864, %eax
	ret
.Lc_writable_1:
	movl	$4865, %eax
	ret
.Lc_writable_2:
	movl	$4866, %eax
	ret
.Lc_writable_3:
	movl	$4867, %eax
	ret
.Ld_writable:
	movl	$-1, %eax
	ret
	.size	sh_writable, .-sh_writable
	.globl	sh_invalid
	.type	sh_invalid, @function
sh_invalid:
	movl	4(%esp), %eax
	cmpl	$3, %eax
	ja	.Ld_invalid
	jmp	*t_invalid(,%eax,4)
.Lc_invalid_0:
	movl	$5120, %eax
	ret
.Lc_invalid_1:
	movl	$5121, %eax
	ret
.Lc_invalid_2:
	movl	$5122, %eax
	ret
.Lc_invalid_3:
	movl	$5123, %eax
	ret
.Ld_invalid:
	movl	$-1, %eax
	ret
	.size	sh_invalid, .-sh_invalid
	.globl	sh_truncated
	.type	sh_truncated, @function
sh_truncated:
	movl	4(%esp), %eax
	cmpl	$3, %eax
	ja	.Ld_truncated
	jmp	*t_truncated(,%eax,4)
.Lc_truncated_0:
	movl	$5376, %eax
	ret
.Lc_truncated_1:
	movl	$5377, %eax
	ret
.Lc_truncated_2:
	movl	$5378, %eax
	ret
.Lc_truncated_3:
	movl	$5379, %eax
	ret
.Ld_truncated:
	movl	$-1, %eax
	ret
	.size	sh_truncated, .-sh_truncated
	.globl	sh_scale2
	.type	sh_scale2, @function
sh_scale2:
	movl	4(%esp), %eax
	cmpl	$3, %eax
	ja	.Ld_scale2
	jmp	*t_scale2(,%eax,2)
.Lc_scale2_0:
	movl	$5632, %eax
	ret
.Lc_scale2_1:
	movl	$5633, %eax
	ret
.Lc_scale2_2:
	movl	$5634, %eax
	ret
.Lc_scale2_3:
	movl	$5635, %eax
	ret
.Ld_scale2:
	movl	$-1, %eax
	ret
	.size	sh_scale2, .-sh_scale2
	.globl	sh_wrap
	.type	sh_wrap, @function
sh_wrap:
	movl	4(%esp), %eax
	cmpl	$3, %eax
	ja	.Ld_wrap
	jmp	*0xfffffff8(,%eax,4)
.Lc_wrap_0:
	movl	$5888, %eax
	ret
.Lc_wrap_1:
	movl	$5889, %eax
	ret
.Lc_wrap_2:
	movl	$5890, %eax
	ret
.Lc_wrap_3:
	movl	$5891, %eax
	ret
.Ld_wrap:
	movl	$-1, %eax
	ret
	.size	sh_wrap, .-sh_wrap
	.globl	sh_conflict
	.type	sh_conflict, @function
sh_conflict:
	movl	4(%esp), %eax
	cmpl	$3, %eax
	ja	.Ld_conflict
	jmp	*t_conflict(,%eax,4)
.Lc_conflict_0:
	movl	$6144, %eax
	ret
.Lc_conflict_1:
	movl	$6145, %eax
	ret
.Lc_conflict_2:
	movl	$6146, %eax
	ret
.Lc_conflict_3:
	movl	$6147, %eax
	ret
.Ld_conflict:
	movl	$-1, %eax
	ret
	.size	sh_conflict, .-sh_conflict
	.globl	sh_conflict_fn
	.type	sh_conflict_fn, @function
sh_conflict_fn:
	movl	$0x77, %eax
	ret
	.size	sh_conflict_fn, .-sh_conflict_fn
	.globl	sh_zextbyte
	.type	sh_zextbyte, @function
sh_zextbyte:
	movl	4(%esp), %eax
	subl	$0x61, %eax
	cmpb	$5, %al
	ja	.Ld_zextbyte
	movzbl	%al, %eax
	jmp	*t_zextbyte(,%eax,4)
.Lc_zextbyte_0:
	movl	$8320, %eax
	ret
.Lc_zextbyte_1:
	movl	$8321, %eax
	ret
.Lc_zextbyte_2:
	movl	$8322, %eax
	ret
.Lc_zextbyte_3:
	movl	$8323, %eax
	ret
.Lc_zextbyte_4:
	movl	$8324, %eax
	ret
.Lc_zextbyte_5:
	movl	$8325, %eax
	ret
.Ld_zextbyte:
	movl	$-1, %eax
	ret
	.size	sh_zextbyte, .-sh_zextbyte
	.globl	sh_zextother
	.type	sh_zextother, @function
sh_zextother:
	movl	4(%esp), %eax
	subl	$0x61, %eax
	cmpb	$5, %al
	ja	.Ld_zextother
	movzbl	%ah, %eax
	jmp	*t_zextother(,%eax,4)
.Lc_zextother_0:
	movl	$8336, %eax
	ret
.Lc_zextother_1:
	movl	$8337, %eax
	ret
.Lc_zextother_2:
	movl	$8338, %eax
	ret
.Lc_zextother_3:
	movl	$8339, %eax
	ret
.Lc_zextother_4:
	movl	$8340, %eax
	ret
.Lc_zextother_5:
	movl	$8341, %eax
	ret
.Ld_zextother:
	movl	$-1, %eax
	ret
	.size	sh_zextother, .-sh_zextother
	.globl	sh_zextsigned
	.type	sh_zextsigned, @function
sh_zextsigned:
	movl	4(%esp), %eax
	subl	$0x61, %eax
	cmpb	$5, %al
	ja	.Ld_zextsigned
	movsbl	%al, %eax
	jmp	*t_zextsigned(,%eax,4)
.Lc_zextsigned_0:
	movl	$8352, %eax
	ret
.Lc_zextsigned_1:
	movl	$8353, %eax
	ret
.Lc_zextsigned_2:
	movl	$8354, %eax
	ret
.Lc_zextsigned_3:
	movl	$8355, %eax
	ret
.Lc_zextsigned_4:
	movl	$8356, %eax
	ret
.Lc_zextsigned_5:
	movl	$8357, %eax
	ret
.Ld_zextsigned:
	movl	$-1, %eax
	ret
	.size	sh_zextsigned, .-sh_zextsigned
	.globl	sh_zextpre
	.type	sh_zextpre, @function
sh_zextpre:
	movl	4(%esp), %esi
	movzwl	%si, %eax
	cmpw	$7, %si
	ja	.Ld_zextpre
	jmp	*t_zextpre(,%eax,4)
.Lc_zextpre_0:
	movl	$12400, %eax
	ret
.Lc_zextpre_1:
	movl	$12401, %eax
	ret
.Lc_zextpre_2:
	movl	$12402, %eax
	ret
.Lc_zextpre_3:
	movl	$12403, %eax
	ret
.Lc_zextpre_4:
	movl	$12404, %eax
	ret
.Lc_zextpre_5:
	movl	$12405, %eax
	ret
.Lc_zextpre_6:
	movl	$12406, %eax
	ret
.Lc_zextpre_7:
	movl	$12407, %eax
	ret
.Ld_zextpre:
	movl	$-1, %eax
	ret
	.size	sh_zextpre, .-sh_zextpre
	.globl	sh_zextmid
	.type	sh_zextmid, @function
sh_zextmid:
	movl	4(%esp), %eax
	cmpw	$5, %ax
	movzwl	%ax, %eax
	ja	.Ld_zextmid
	jmp	*t_zextmid(,%eax,4)
.Lc_zextmid_0:
	movl	$12400, %eax
	ret
.Lc_zextmid_1:
	movl	$12401, %eax
	ret
.Lc_zextmid_2:
	movl	$12402, %eax
	ret
.Lc_zextmid_3:
	movl	$12403, %eax
	ret
.Lc_zextmid_4:
	movl	$12404, %eax
	ret
.Lc_zextmid_5:
	movl	$12405, %eax
	ret
.Ld_zextmid:
	movl	$-1, %eax
	ret
	.size	sh_zextmid, .-sh_zextmid
	.globl	sh_zextpreal
	.type	sh_zextpreal, @function
sh_zextpreal:
	movl	4(%esp), %eax
	movzbl	%al, %eax
	cmpb	$5, %al
	ja	.Ld_zextpreal
	jmp	*t_zextpreal(,%eax,4)
.Lc_zextpreal_0:
	movl	$12432, %eax
	ret
.Lc_zextpreal_1:
	movl	$12433, %eax
	ret
.Lc_zextpreal_2:
	movl	$12434, %eax
	ret
.Lc_zextpreal_3:
	movl	$12435, %eax
	ret
.Lc_zextpreal_4:
	movl	$12436, %eax
	ret
.Lc_zextpreal_5:
	movl	$12437, %eax
	ret
.Ld_zextpreal:
	movl	$-1, %eax
	ret
	.size	sh_zextpreal, .-sh_zextpreal
	.globl	sh_zextpreclobber
	.type	sh_zextpreclobber, @function
sh_zextpreclobber:
	movl	4(%esp), %esi
	movzwl	%si, %eax
	addw	$1, %si
	cmpw	$7, %si
	ja	.Ld_zextpreclobber
	jmp	*t_zextpreclobber(,%eax,4)
.Lc_zextpreclobber_0:
	movl	$12512, %eax
	ret
.Lc_zextpreclobber_1:
	movl	$12513, %eax
	ret
.Lc_zextpreclobber_2:
	movl	$12514, %eax
	ret
.Lc_zextpreclobber_3:
	movl	$12515, %eax
	ret
.Lc_zextpreclobber_4:
	movl	$12516, %eax
	ret
.Lc_zextpreclobber_5:
	movl	$12517, %eax
	ret
.Lc_zextpreclobber_6:
	movl	$12518, %eax
	ret
.Lc_zextpreclobber_7:
	movl	$12519, %eax
	ret
.Ld_zextpreclobber:
	movl	$-1, %eax
	ret
	.size	sh_zextpreclobber, .-sh_zextpreclobber
	.globl	sh_veneer
	.type	sh_veneer, @function
sh_veneer:
	jmp	*veneer_slot
.Lveneer_target:
	movl	$0x55, %eax
	ret
	.size	sh_veneer, .-sh_veneer
	.globl	_start
	.type	_start, @function
_start:
	movl	$1, %eax
	xorl	%ebx, %ebx
	int	$0x80
	.size	_start, .-_start
	.section	.rodata
	.globl	t_ja
	.type	t_ja, @object
	.align	4
t_ja:
	.long	.Lc_ja_0
	.long	.Lc_ja_1
	.long	.Lc_ja_2
	.long	.Lc_ja_3
	.size	t_ja, .-t_ja
	.globl	t_jae
	.type	t_jae, @object
	.align	4
t_jae:
	.long	.Lc_jae_0
	.long	.Lc_jae_1
	.long	.Lc_jae_2
	.long	.Lc_jae_3
	.size	t_jae, .-t_jae
	.globl	t_prenorm
	.type	t_prenorm, @object
	.align	4
t_prenorm:
	.long	.Lc_prenorm_0
	.long	.Lc_prenorm_1
	.long	.Lc_prenorm_2
	.long	.Lc_prenorm_3
	.size	t_prenorm, .-t_prenorm
	.globl	t_sched
	.type	t_sched, @object
	.align	4
t_sched:
	.long	.Lc_sched_0
	.long	.Lc_sched_1
	.long	.Lc_sched_2
	.long	.Lc_sched_3
	.size	t_sched, .-t_sched
	.globl	t_repeat
	.type	t_repeat, @object
	.align	4
t_repeat:
	.long	.Lc_repeat_0
	.long	.Lc_repeat_1
	.long	.Lc_repeat_0
	.long	.Lc_repeat_2
	.size	t_repeat, .-t_repeat
	.globl	t_ceiling
	.type	t_ceiling, @object
	.align	4
t_ceiling:
	.long	.Lc_ceiling_0
	.long	.Lc_ceiling_1
	.long	.Lc_ceiling_2
	.long	.Lc_ceiling_3
	.long	.Lc_ceiling_4
	.long	.Lc_ceiling_5
	.long	.Lc_ceiling_6
	.long	.Lc_ceiling_7
	.size	t_ceiling, .-t_ceiling
	.globl	t_adjacent
	.type	t_adjacent, @object
	.align	4
t_adjacent:
	.long	.Lc_adjacent_0
	.long	.Lc_adjacent_1
	.long	.Lc_adjacent_2
	.long	.Lc_adjacent_3
	.long	.Lindependent
	.size	t_adjacent, .-t_adjacent
	.globl	t_noguard
	.type	t_noguard, @object
	.align	4
t_noguard:
	.long	.Lc_noguard_0
	.long	.Lc_noguard_1
	.long	.Lc_noguard_2
	.long	.Lc_noguard_3
	.size	t_noguard, .-t_noguard
	.globl	t_wrongidx
	.type	t_wrongidx, @object
	.align	4
t_wrongidx:
	.long	.Lc_wrongidx_0
	.long	.Lc_wrongidx_1
	.long	.Lc_wrongidx_2
	.long	.Lc_wrongidx_3
	.size	t_wrongidx, .-t_wrongidx
	.globl	t_overwritten
	.type	t_overwritten, @object
	.align	4
t_overwritten:
	.long	.Lc_overwritten_0
	.long	.Lc_overwritten_1
	.long	.Lc_overwritten_2
	.long	.Lc_overwritten_3
	.size	t_overwritten, .-t_overwritten
	.globl	t_partial
	.type	t_partial, @object
	.align	4
t_partial:
	.long	.Lc_partial_0
	.long	.Lc_partial_1
	.long	.Lc_partial_2
	.long	.Lc_partial_3
	.size	t_partial, .-t_partial
	.globl	t_signed
	.type	t_signed, @object
	.align	4
t_signed:
	.long	.Lc_signed_0
	.long	.Lc_signed_1
	.long	.Lc_signed_2
	.long	.Lc_signed_3
	.size	t_signed, .-t_signed
	.globl	t_lowerbound
	.type	t_lowerbound, @object
	.align	4
t_lowerbound:
	.long	.Lc_lowerbound_0
	.long	.Lc_lowerbound_1
	.long	.Lc_lowerbound_2
	.long	.Lc_lowerbound_3
	.size	t_lowerbound, .-t_lowerbound
	.globl	t_bytecmp
	.type	t_bytecmp, @object
	.align	4
t_bytecmp:
	.long	.Lc_bytecmp_0
	.long	.Lc_bytecmp_1
	.long	.Lc_bytecmp_2
	.long	.Lc_bytecmp_3
	.size	t_bytecmp, .-t_bytecmp
	.globl	t_postnorm
	.type	t_postnorm, @object
	.align	4
t_postnorm:
	.long	.Lc_postnorm_0
	.long	.Lc_postnorm_1
	.long	.Lc_postnorm_2
	.long	.Lc_postnorm_3
	.size	t_postnorm, .-t_postnorm
	.globl	t_flagclobber
	.type	t_flagclobber, @object
	.align	4
t_flagclobber:
	.long	.Lc_flagclobber_0
	.long	.Lc_flagclobber_1
	.long	.Lc_flagclobber_2
	.long	.Lc_flagclobber_3
	.size	t_flagclobber, .-t_flagclobber
	.globl	t_bypass
	.type	t_bypass, @object
	.align	4
t_bypass:
	.long	.Lc_bypass_0
	.long	.Lc_bypass_1
	.long	.Lc_bypass_2
	.long	.Lc_bypass_3
	.size	t_bypass, .-t_bypass
	.globl	t_bypassmid
	.type	t_bypassmid, @object
	.align	4
t_bypassmid:
	.long	.Lc_bypassmid_0
	.long	.Lc_bypassmid_1
	.long	.Lc_bypassmid_2
	.long	.Lc_bypassmid_3
	.size	t_bypassmid, .-t_bypassmid
	.globl	t_invalid
	.type	t_invalid, @object
	.align	4
t_invalid:
	.long	.Lc_invalid_0
	.long	.Lc_invalid_1
	.long	0
	.long	.Lc_invalid_3
	.size	t_invalid, .-t_invalid
	.globl	t_scale2
	.type	t_scale2, @object
	.align	4
t_scale2:
	.long	.Lc_scale2_0
	.long	.Lc_scale2_1
	.long	.Lc_scale2_2
	.long	.Lc_scale2_3
	.size	t_scale2, .-t_scale2
	.globl	t_wrap
	.type	t_wrap, @object
	.align	4
t_wrap:
	.long	.Lc_wrap_0
	.long	.Lc_wrap_1
	.long	.Lc_wrap_2
	.long	.Lc_wrap_3
	.size	t_wrap, .-t_wrap
	.globl	t_conflict
	.type	t_conflict, @object
	.align	4
t_conflict:
	.long	.Lc_conflict_0
	.long	sh_conflict_fn
	.long	.Lc_conflict_2
	.long	.Lc_conflict_3
	.size	t_conflict, .-t_conflict
	.globl	t_zextbyte
	.type	t_zextbyte, @object
	.align	4
t_zextbyte:
	.long	.Lc_zextbyte_0
	.long	.Lc_zextbyte_1
	.long	.Lc_zextbyte_2
	.long	.Lc_zextbyte_3
	.long	.Lc_zextbyte_4
	.long	.Lc_zextbyte_5
	.size	t_zextbyte, .-t_zextbyte
	.globl	t_zextother
	.type	t_zextother, @object
	.align	4
t_zextother:
	.long	.Lc_zextother_0
	.long	.Lc_zextother_1
	.long	.Lc_zextother_2
	.long	.Lc_zextother_3
	.long	.Lc_zextother_4
	.long	.Lc_zextother_5
	.size	t_zextother, .-t_zextother
	.globl	t_zextsigned
	.type	t_zextsigned, @object
	.align	4
t_zextsigned:
	.long	.Lc_zextsigned_0
	.long	.Lc_zextsigned_1
	.long	.Lc_zextsigned_2
	.long	.Lc_zextsigned_3
	.long	.Lc_zextsigned_4
	.long	.Lc_zextsigned_5
	.size	t_zextsigned, .-t_zextsigned
	.globl	t_zextpre
	.type	t_zextpre, @object
	.align	4
t_zextpre:
	.long	.Lc_zextpre_0
	.long	.Lc_zextpre_1
	.long	.Lc_zextpre_2
	.long	.Lc_zextpre_3
	.long	.Lc_zextpre_4
	.long	.Lc_zextpre_5
	.long	.Lc_zextpre_6
	.long	.Lc_zextpre_7
	.size	t_zextpre, .-t_zextpre
	.globl	t_zextmid
	.type	t_zextmid, @object
	.align	4
t_zextmid:
	.long	.Lc_zextmid_0
	.long	.Lc_zextmid_1
	.long	.Lc_zextmid_2
	.long	.Lc_zextmid_3
	.long	.Lc_zextmid_4
	.long	.Lc_zextmid_5
	.size	t_zextmid, .-t_zextmid
	.globl	t_zextpreal
	.type	t_zextpreal, @object
	.align	4
t_zextpreal:
	.long	.Lc_zextpreal_0
	.long	.Lc_zextpreal_1
	.long	.Lc_zextpreal_2
	.long	.Lc_zextpreal_3
	.long	.Lc_zextpreal_4
	.long	.Lc_zextpreal_5
	.size	t_zextpreal, .-t_zextpreal
	.globl	t_zextpreclobber
	.type	t_zextpreclobber, @object
	.align	4
t_zextpreclobber:
	.long	.Lc_zextpreclobber_0
	.long	.Lc_zextpreclobber_1
	.long	.Lc_zextpreclobber_2
	.long	.Lc_zextpreclobber_3
	.long	.Lc_zextpreclobber_4
	.long	.Lc_zextpreclobber_5
	.long	.Lc_zextpreclobber_6
	.long	.Lc_zextpreclobber_7
	.size	t_zextpreclobber, .-t_zextpreclobber
	.globl	veneer_slot
	.type	veneer_slot, @object
	.align	4
veneer_slot:
	.long	.Lveneer_target
	.size	veneer_slot, 4
	.data
	.globl	t_writable
	.type	t_writable, @object
	.align	4
t_writable:
	.long	.Lc_writable_0
	.long	.Lc_writable_1
	.long	.Lc_writable_2
	.long	.Lc_writable_3
	.size	t_writable, .-t_writable
	.section	.trunctab,"a"
	.globl	t_truncated
	.type	t_truncated, @object
	.align	4
t_truncated:
	.long	.Lc_truncated_0
	.long	.Lc_truncated_1
	.size	t_truncated, .-t_truncated
