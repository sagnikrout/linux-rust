//! Automatically rewritten from C to Rust
//! Source: samples/ftrace/ftrace-direct-multi.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0-only

    extern void my_direct_func(unsigned long ip);
#[no_mangle]
pub unsafe extern "C" fn my_direct_func(ip: c_ulong) {
    void my_direct_func(unsigned long ip)
    {
    trace_printk("ip %lx\n", ip);
    }
    extern void my_tramp(void *);

    asm (
    "       .pushsection    .text, \"ax\", @progbits\n"
    "       .type           my_tramp, @function\n"
    "       .globl          my_tramp\n"
    "   my_tramp:\n"
    "       addi	sp,sp,-3*"SZREG"\n"
    "       "REG_S"	a0,0*"SZREG"(sp)\n"
    "       "REG_S"	t0,1*"SZREG"(sp)\n"
    "       "REG_S"	ra,2*"SZREG"(sp)\n"
    "       mv	a0,t0\n"
    "       call	my_direct_func\n"
    "       "REG_L"	a0,0*"SZREG"(sp)\n"
    "       "REG_L"	t0,1*"SZREG"(sp)\n"
    "       "REG_L"	ra,2*"SZREG"(sp)\n"
    "       addi	sp,sp,3*"SZREG"\n"
    "       jr	t0\n"
    "       .size           my_tramp, .-my_tramp\n"
    "       .popsection\n"
    );

    asm (
    "	.pushsection    .text, \"ax\", @progbits\n"
    "	.type		my_tramp, @function\n"
    "	.globl		my_tramp\n"
    "   my_tramp:"
    ASM_ENDBR
    "	pushq %rbp\n"
    "	movq %rsp, %rbp\n"
    CALL_DEPTH_ACCOUNT
    "	pushq %rdi\n"
    "	movq 8(%rbp), %rdi\n"
    "	call my_direct_func\n"
    "	popq %rdi\n"
    "	leave\n"
    ASM_RET
    "	.size		my_tramp, .-my_tramp\n"
    "	.popsection\n"
    );

    asm (
    "	.pushsection	.text, \"ax\", @progbits\n"
    "	.type		my_tramp, @function\n"
    "	.globl		my_tramp\n"
    "   my_tramp:"
    "	lgr		%r1,%r15\n"
    "	stmg		%r0,%r5,"__stringify(__SF_GPRS)"(%r15)\n"
    "	stg		%r14,"__stringify(__SF_GPRS+8*8)"(%r15)\n"
    "	aghi		%r15,"__stringify(-STACK_FRAME_OVERHEAD)"\n"
    "	stg		%r1,"__stringify(__SF_BACKCHAIN)"(%r15)\n"
    "	lgr		%r2,%r0\n"
    "	brasl		%r14,my_direct_func\n"
    "	aghi		%r15,"__stringify(STACK_FRAME_OVERHEAD)"\n"
    "	lmg		%r0,%r5,"__stringify(__SF_GPRS)"(%r15)\n"
    "	lg		%r14,"__stringify(__SF_GPRS+8*8)"(%r15)\n"
    "	lgr		%r1,%r0\n"
    "	br		%r1\n"
    "	.size		my_tramp, .-my_tramp\n"
    "	.popsection\n"
    );

    asm (
    "	.pushsection	.text, \"ax\", @progbits\n"
    "	.type		my_tramp, @function\n"
    "	.globl		my_tramp\n"
    "   my_tramp:"
    "	hint	34\n" // bti	c
    "	sub	sp, sp, #32\n"
    "	stp	x9, x30, [sp]\n"
    "	str	x0, [sp, #16]\n"
    "	mov	x0, x30\n"
    "	bl	my_direct_func\n"
    "	ldp	x30, x9, [sp]\n"
    "	ldr	x0, [sp, #16]\n"
    "	add	sp, sp, #32\n"
    "	ret	x9\n"
    "	.size		my_tramp, .-my_tramp\n"
    "	.popsection\n"
    );

    asm (
    "	.pushsection	.text, \"ax\", @progbits\n"
    "	.type		my_tramp, @function\n"
    "	.globl		my_tramp\n"
    "   my_tramp:\n"
    "	addi.d	$sp, $sp, -32\n"
    "	st.d	$a0, $sp, 0\n"
    "	st.d	$t0, $sp, 8\n"
    "	st.d	$ra, $sp, 16\n"
    "	move	$a0, $t0\n"
    "	bl	my_direct_func\n"
    "	ld.d	$a0, $sp, 0\n"
    "	ld.d	$ra, $sp, 8\n"
    "	ld.d	$t0, $sp, 16\n"
    "	addi.d	$sp, $sp, 32\n"
    "	jr	$t0\n"
    "	.size		my_tramp, .-my_tramp\n"
    "	.popsection\n"
    );

pub const STACK_FRAME_SIZE: c_int = 48;

pub const STACK_FRAME_SIZE: c_int = 24;

    "	std		2, 24(1)\n"			\
    "	bcl		20, 31, 1f\n"			\
    "   1:	mflr		12\n"				\
    "	ld		2, (99f - 1b)(12)\n"

    "	ld		2, 24(1)\n"

    "   99:	.quad		.TOC.@tocbase\n"

    PPC_LL"		0, "__stringify(PPC_LR_STKOFF)"(1)\n"	\
    "	mtlr		0\n"

    "	blr\n"

    "	lwz		8, 4(3)\n"			\
    "	li		9, 6\n"				\
    "	slw		8, 8, 9\n"			\
    "	sraw		8, 8, 9\n"			\
    "	add		3, 3, 8\n"			\
    "	addi		3, 3, 4\n"

    PPC_LL"		0, "__stringify(PPC_LR_STKOFF)"(1)\n"	\
    "	mtctr		0\n"

    "	mtlr		0\n"				\
    "	bctr\n"

    asm (
    "	.pushsection	.text, \"ax\", @progbits\n"
    "	.type		my_tramp, @function\n"
    "	.globl		my_tramp\n"
    "   my_tramp:\n"
    PPC_STL"	0, "__stringify(PPC_LR_STKOFF)"(1)\n"
    PPC_STLU"	1, -"__stringify(STACK_FRAME_MIN_SIZE)"(1)\n"
    "	mflr		0\n"
    PPC_STL"	0, "__stringify(PPC_LR_STKOFF)"(1)\n"
    PPC_STLU"	1, -"__stringify(STACK_FRAME_SIZE)"(1)\n"
    PPC64_TOC_SAVE_AND_UPDATE
    PPC_STL"	3, "__stringify(STACK_FRAME_MIN_SIZE)"(1)\n"
    "	mr		3, 0\n"
    PPC_FTRACE_RECOVER_IP
    "	bl		my_direct_func\n"
    PPC_LL"		3, "__stringify(STACK_FRAME_MIN_SIZE)"(1)\n"
    PPC64_TOC_RESTORE
    "	addi		1, 1, "__stringify(STACK_FRAME_SIZE)"\n"
    PPC_FTRACE_RESTORE_LR
    "	addi		1, 1, "__stringify(STACK_FRAME_MIN_SIZE)"\n"
    PPC_LL"		0, "__stringify(PPC_LR_STKOFF)"(1)\n"
    PPC_FTRACE_RET
    PPC64_TOC
    "	.size		my_tramp, .-my_tramp\n"
    "	.popsection\n"
    );

    static struct ftrace_ops direct;
#[no_mangle]
unsafe extern "C" fn ftrace_direct_multi_init() -> int __init {
    static int __init ftrace_direct_multi_init(void)
    {
    ftrace_set_filter_ip(&direct, (unsigned long) wake_up_process, 0, 0);
    ftrace_set_filter_ip(&direct, (unsigned long) schedule, 0, 0);
    return register_ftrace_direct(&direct, (unsigned long) my_tramp);
    }
#[no_mangle]
unsafe extern "C" fn ftrace_direct_multi_exit() -> void __exit {
    static void __exit ftrace_direct_multi_exit(void)
    {
    unregister_ftrace_direct(&direct, (unsigned long) my_tramp, true);
    }
    module_init(ftrace_direct_multi_init);
    module_exit(ftrace_direct_multi_exit);
    MODULE_AUTHOR("Jiri Olsa");
    MODULE_DESCRIPTION("Example use case of using register_ftrace_direct_multi()");
    MODULE_LICENSE("GPL");
