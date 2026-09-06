//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/rethook.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// x86 implementation of rethook. Mostly copied from arch/x86/kernel/kprobes/core.c.
//

    __visible void arch_rethook_trampoline_callback(struct pt_regs *regs);

// Macro flag: #define ANNOTATE_NOENDBR

//
// When a target function returns, this code saves registers and calls
// arch_rethook_trampoline_callback(), which calls the rethook handler.
//
    asm(
    ".text\n"
    ".global arch_rethook_trampoline\n"
    ".type arch_rethook_trampoline, @function\n"
    "arch_rethook_trampoline:\n"

    ANNOTATE_NOENDBR "\n"	/* This is only jumped from ret instruction */
// Push a fake return address to tell the unwinder it's a rethook.
    "	pushq $arch_rethook_trampoline\n"
    UNWIND_HINT_FUNC
    "       pushq $" __stringify(__KERNEL_DS) "\n"
// Save the 'sp - 16', this will be fixed later.
    "	pushq %rsp\n"
    "	pushfq\n"
    SAVE_REGS_STRING
    "	movq %rsp, %rdi\n"
    "	call arch_rethook_trampoline_callback\n"
    RESTORE_REGS_STRING
// In the callback function, 'regs->flags' is copied to 'regs->ss'.
    "	addq $16, %rsp\n"
    "	popfq\n"

// Push a fake return address to tell the unwinder it's a rethook.
    "	pushl $arch_rethook_trampoline\n"
    UNWIND_HINT_FUNC
    "	pushl %ss\n"
// Save the 'sp - 8', this will be fixed later.
    "	pushl %esp\n"
    "	pushfl\n"
    SAVE_REGS_STRING
    "	movl %esp, %eax\n"
    "	call arch_rethook_trampoline_callback\n"
    RESTORE_REGS_STRING
// In the callback function, 'regs->flags' is copied to 'regs->ss'.
    "	addl $8, %esp\n"
    "	popfl\n"

    ASM_RET
    ".size arch_rethook_trampoline, .-arch_rethook_trampoline\n"
    );
    NOKPROBE_SYMBOL(arch_rethook_trampoline);
//
// Called from arch_rethook_trampoline
//
#[no_mangle]
pub unsafe extern "C" fn arch_rethook_trampoline_callback(regs: *mut pt_regs) -> __used __visible void {
    __used __visible void arch_rethook_trampoline_callback(struct pt_regs *regs)
    {
    unsigned long *frame_pointer;
// fixup registers
    regs.cs = __KERNEL_CS;

    regs.gs = 0;

    regs.ip = (unsigned long)&arch_rethook_trampoline;
    regs.orig_ax = ~0UL;
    regs.sp += 2*sizeof(long);
    frame_pointer = (long *)(regs + 1);
//
// The return address at 'frame_pointer' is recovered by the
// arch_rethook_fixup_return() which called from this
// rethook_trampoline_handler().
//
    rethook_trampoline_handler(regs, (unsigned long)frame_pointer);
//
// Copy FLAGS to 'pt_regs::ss' so that arch_rethook_trapmoline()
// can do RET right after POPF.
//
// (unsigned long *)&regs->ss = regs->flags;
    }
    NOKPROBE_SYMBOL(arch_rethook_trampoline_callback);
//
// arch_rethook_trampoline() skips updating frame pointer. The frame pointer
// saved in arch_rethook_trampoline_callback() points to the real caller
// function's frame pointer. Thus the arch_rethook_trampoline() doesn't have
// a standard stack frame with CONFIG_FRAME_POINTER=y.
// Let's mark it non-standard function. Anyway, FP unwinder can correctly
// unwind without the hint.
//
    STACK_FRAME_NON_STANDARD_FP(arch_rethook_trampoline);
// This is called from rethook_trampoline_handler().
    void arch_rethook_fixup_return(struct pt_regs *regs,
    unsigned long correct_ret_addr)
    {
    unsigned long *frame_pointer = (void *)(regs + 1);
// Replace fake return address with real one.
// frame_pointer = correct_ret_addr;
    }
    NOKPROBE_SYMBOL(arch_rethook_fixup_return);
#[no_mangle]
pub unsafe extern "C" fn arch_rethook_prepare(rh: *mut rethook_node, regs: *mut pt_regs, mcount: bool) {
    void arch_rethook_prepare(struct rethook_node *rh, struct pt_regs *regs, bool mcount)
    {
    unsigned long *stack = (unsigned long *)regs.sp;
    rh.ret_addr = stack[0];
    rh.frame = regs.sp;
// Replace the return addr with trampoline addr
    stack[0] = (unsigned long) arch_rethook_trampoline;
    }
    NOKPROBE_SYMBOL(arch_rethook_prepare);
