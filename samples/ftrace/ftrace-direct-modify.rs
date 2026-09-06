//! Automatically rewritten from C to Rust
//! Source: samples/ftrace/ftrace-direct-modify.c
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

    extern void my_direct_func1(void);
    extern void my_direct_func2(void);
#[no_mangle]
pub unsafe extern "C" fn my_direct_func1() {
    void my_direct_func1(void)
    {
    trace_printk("my direct func1\n");
    }
#[no_mangle]
pub unsafe extern "C" fn my_direct_func2() {
    void my_direct_func2(void)
    {
    trace_printk("my direct func2\n");
    }
    extern void my_tramp1(void *);
    extern void my_tramp2(void *);
    let mut my_ip: static unsigned long = (unsigned long)schedule;

    asm (
    "	.pushsection    .text, \"ax\", @progbits\n"
    "	.type		my_tramp1, @function\n"
    "	.globl		my_tramp1\n"
    "   my_tramp1:\n"
    "	addi	sp,sp,-2*"SZREG"\n"
    "	"REG_S"	t0,0*"SZREG"(sp)\n"
    "	"REG_S"	ra,1*"SZREG"(sp)\n"
    "	call	my_direct_func1\n"
    "	"REG_L"	t0,0*"SZREG"(sp)\n"
    "	"REG_L"	ra,1*"SZREG"(sp)\n"
    "	addi	sp,sp,2*"SZREG"\n"
    "	jr	t0\n"
    "	.size		my_tramp1, .-my_tramp1\n"
    "	.type		my_tramp2, @function\n"
    "	.globl		my_tramp2\n"
    "   my_tramp2:\n"
    "	addi	sp,sp,-2*"SZREG"\n"
    "	"REG_S"	t0,0*"SZREG"(sp)\n"
    "	"REG_S"	ra,1*"SZREG"(sp)\n"
    "	call	my_direct_func2\n"
    "	"REG_L"	t0,0*"SZREG"(sp)\n"
    "	"REG_L"	ra,1*"SZREG"(sp)\n"
    "	addi	sp,sp,2*"SZREG"\n"
    "	jr	t0\n"
    "	.size		my_tramp2, .-my_tramp2\n"
    "	.popsection\n"
    );

    asm (
    "	.pushsection    .text, \"ax\", @progbits\n"
    "	.type		my_tramp1, @function\n"
    "	.globl		my_tramp1\n"
    "   my_tramp1:"
    ASM_ENDBR
    "	pushq %rbp\n"
    "	movq %rsp, %rbp\n"
    CALL_DEPTH_ACCOUNT
    "	call my_direct_func1\n"
    "	leave\n"
    ASM_RET
    "	.size		my_tramp1, .-my_tramp1\n"
    "	.type		my_tramp2, @function\n"
    "	.globl		my_tramp2\n"
    "   my_tramp2:"
    ASM_ENDBR
    "	pushq %rbp\n"
    "	movq %rsp, %rbp\n"
    CALL_DEPTH_ACCOUNT
    "	call my_direct_func2\n"
    "	leave\n"
    ASM_RET
    "	.size		my_tramp2, .-my_tramp2\n"
    "	.popsection\n"
    );

    asm (
    "	.pushsection	.text, \"ax\", @progbits\n"
    "	.type		my_tramp1, @function\n"
    "	.globl		my_tramp1\n"
    "   my_tramp1:"
    "	lgr		%r1,%r15\n"
    "	stmg		%r0,%r5,"__stringify(__SF_GPRS)"(%r15)\n"
    "	stg		%r14,"__stringify(__SF_GPRS+8*8)"(%r15)\n"
    "	aghi		%r15,"__stringify(-STACK_FRAME_OVERHEAD)"\n"
    "	stg		%r1,"__stringify(__SF_BACKCHAIN)"(%r15)\n"
    "	brasl		%r14,my_direct_func1\n"
    "	aghi		%r15,"__stringify(STACK_FRAME_OVERHEAD)"\n"
    "	lmg		%r0,%r5,"__stringify(__SF_GPRS)"(%r15)\n"
    "	lg		%r14,"__stringify(__SF_GPRS+8*8)"(%r15)\n"
    "	lgr		%r1,%r0\n"
    "	br		%r1\n"
    "	.size		my_tramp1, .-my_tramp1\n"
    "	.type		my_tramp2, @function\n"
    "	.globl		my_tramp2\n"
    "   my_tramp2:"
    "	lgr		%r1,%r15\n"
    "	stmg		%r0,%r5,"__stringify(__SF_GPRS)"(%r15)\n"
    "	stg		%r14,"__stringify(__SF_GPRS+8*8)"(%r15)\n"
    "	aghi		%r15,"__stringify(-STACK_FRAME_OVERHEAD)"\n"
    "	stg		%r1,"__stringify(__SF_BACKCHAIN)"(%r15)\n"
    "	brasl		%r14,my_direct_func2\n"
    "	aghi		%r15,"__stringify(STACK_FRAME_OVERHEAD)"\n"
    "	lmg		%r0,%r5,"__stringify(__SF_GPRS)"(%r15)\n"
    "	lg		%r14,"__stringify(__SF_GPRS+8*8)"(%r15)\n"
    "	lgr		%r1,%r0\n"
    "	br		%r1\n"
    "	.size		my_tramp2, .-my_tramp2\n"
    "	.popsection\n"
    );

    asm (
    "	.pushsection    .text, \"ax\", @progbits\n"
    "	.type		my_tramp1, @function\n"
    "	.globl		my_tramp1\n"
    "   my_tramp1:"
    "	hint	34\n" // bti	c
    "	sub	sp, sp, #16\n"
    "	stp	x9, x30, [sp]\n"
    "	bl	my_direct_func1\n"
    "	ldp	x30, x9, [sp]\n"
    "	add	sp, sp, #16\n"
    "	ret	x9\n"
    "	.size		my_tramp1, .-my_tramp1\n"
    "	.type		my_tramp2, @function\n"
    "	.globl		my_tramp2\n"
    "   my_tramp2:"
    "	hint	34\n" // bti	c
    "	sub	sp, sp, #16\n"
    "	stp	x9, x30, [sp]\n"
    "	bl	my_direct_func2\n"
    "	ldp	x30, x9, [sp]\n"
    "	add	sp, sp, #16\n"
    "	ret	x9\n"
    "	.size		my_tramp2, .-my_tramp2\n"
    "	.popsection\n"
    );

    asm (
    "	.pushsection    .text, \"ax\", @progbits\n"
    "	.type		my_tramp1, @function\n"
    "	.globl		my_tramp1\n"
    "   my_tramp1:\n"
    "	addi.d	$sp, $sp, -16\n"
    "	st.d	$t0, $sp, 0\n"
    "	st.d	$ra, $sp, 8\n"
    "	bl	my_direct_func1\n"
    "	ld.d	$ra, $sp, 0\n"
    "	ld.d	$t0, $sp, 8\n"
    "	addi.d	$sp, $sp, 16\n"
    "	jr	$t0\n"
    "	.size		my_tramp1, .-my_tramp1\n"
    "	.type		my_tramp2, @function\n"
    "	.globl		my_tramp2\n"
    "   my_tramp2:\n"
    "	addi.d	$sp, $sp, -16\n"
    "	st.d	$t0, $sp, 0\n"
    "	st.d	$ra, $sp, 8\n"
    "	bl	my_direct_func2\n"
    "	ld.d	$ra, $sp, 0\n"
    "	ld.d	$t0, $sp, 8\n"
    "	addi.d	$sp, $sp, 16\n"
    "	jr	$t0\n"
    "	.size		my_tramp2, .-my_tramp2\n"
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

    PPC_LL"		0, "__stringify(PPC_LR_STKOFF)"(1)\n"	\
    "	mtctr		0\n"

    "	mtlr		0\n"				\
    "	bctr\n"

    asm (
    "	.pushsection	.text, \"ax\", @progbits\n"
    "	.type		my_tramp1, @function\n"
    "	.globl		my_tramp1\n"
    "   my_tramp1:\n"
    PPC_STL"	0, "__stringify(PPC_LR_STKOFF)"(1)\n"
    PPC_STLU"	1, -"__stringify(STACK_FRAME_MIN_SIZE)"(1)\n"
    "	mflr		0\n"
    PPC_STL"	0, "__stringify(PPC_LR_STKOFF)"(1)\n"
    PPC_STLU"	1, -"__stringify(STACK_FRAME_SIZE)"(1)\n"
    PPC64_TOC_SAVE_AND_UPDATE
    "	bl		my_direct_func1\n"
    PPC64_TOC_RESTORE
    "	addi		1, 1, "__stringify(STACK_FRAME_SIZE)"\n"
    PPC_FTRACE_RESTORE_LR
    "	addi		1, 1, "__stringify(STACK_FRAME_MIN_SIZE)"\n"
    PPC_LL"		0, "__stringify(PPC_LR_STKOFF)"(1)\n"
    PPC_FTRACE_RET
    "	.size		my_tramp1, .-my_tramp1\n"
    "	.type		my_tramp2, @function\n"
    "	.globl		my_tramp2\n"
    "   my_tramp2:\n"
    PPC_STL"	0, "__stringify(PPC_LR_STKOFF)"(1)\n"
    PPC_STLU"	1, -"__stringify(STACK_FRAME_MIN_SIZE)"(1)\n"
    "	mflr		0\n"
    PPC_STL"	0, "__stringify(PPC_LR_STKOFF)"(1)\n"
    PPC_STLU"	1, -"__stringify(STACK_FRAME_SIZE)"(1)\n"
    PPC64_TOC_SAVE_AND_UPDATE
    "	bl		my_direct_func2\n"
    PPC64_TOC_RESTORE
    "	addi		1, 1, "__stringify(STACK_FRAME_SIZE)"\n"
    PPC_FTRACE_RESTORE_LR
    "	addi		1, 1, "__stringify(STACK_FRAME_MIN_SIZE)"\n"
    PPC_LL"		0, "__stringify(PPC_LR_STKOFF)"(1)\n"
    PPC_FTRACE_RET
    PPC64_TOC
    "	.size		my_tramp2, .-my_tramp2\n"
    "	.popsection\n"
    );

    static struct ftrace_ops direct;
    let mut my_tramp: static unsigned long = (unsigned long)my_tramp1;
    static unsigned long tramps[2] = {
    (unsigned long)my_tramp1,
    (unsigned long)my_tramp2,
    };
#[no_mangle]
unsafe extern "C" fn simple_thread(arg: *mut c_void) -> c_int {
    static int simple_thread(void *arg)
    {
    static int t;
    let mut ret: c_int = 0;
    while (!kthread_should_stop()) {
    set_current_state(TASK_INTERRUPTIBLE);
    schedule_timeout(2 * HZ);
    if (ret)
    continue;
    t ^= 1;
    ret = modify_ftrace_direct(&direct, tramps[t]);
    if (!ret)
    my_tramp = tramps[t];
    WARN_ON_ONCE(ret);
    }
    return 0;
    }
    static struct task_struct *simple_tsk;
#[no_mangle]
unsafe extern "C" fn ftrace_direct_init() -> int __init {
    static int __init ftrace_direct_init(void)
    {
    int ret;
    ftrace_set_filter_ip(&direct, (unsigned long) my_ip, 0, 0);
    ret = register_ftrace_direct(&direct, my_tramp);
    if (ret)
    return ret;
    simple_tsk = kthread_run(simple_thread, core::ptr::null_mut(), "event-sample-fn");
    if (IS_ERR(simple_tsk)) {
    unregister_ftrace_direct(&direct, my_tramp, true);
    return PTR_ERR(simple_tsk);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ftrace_direct_exit() -> void __exit {
    static void __exit ftrace_direct_exit(void)
    {
    kthread_stop(simple_tsk);
    unregister_ftrace_direct(&direct, my_tramp, true);
    }
    module_init(ftrace_direct_init);
    module_exit(ftrace_direct_exit);
    MODULE_AUTHOR("Steven Rostedt");
    MODULE_DESCRIPTION("Example use case of using modify_ftrace_direct()");
    MODULE_LICENSE("GPL");
