//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/ftrace.h
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


// SPDX-License-Identifier: GPL-2.0
pub const ARCH_SUPPORTS_FTRACE_OPS: c_int = 1;
pub const MCOUNT_INSN_SIZE: c_int = 6;

extern "C" {
    pub fn ftrace_caller();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dyn_arch_ftrace {
pub const MCOUNT_ADDR: c_int = 0;

pub const KPROBE_ON_FTRACE_NOP: c_int = 0;
pub const KPROBE_ON_FTRACE_CALL: c_int = 1;
    pub module: struct,
    pub dyn_ftrace: struct,
    pub ftrace_ops: struct,
    pub ftrace_need_init_nop(void): bool,

    pub rec): *mut *mut int ftrace_init_nop(struct module mod, struct dyn_ftrace,

    pub addr: return,

    pub &arch_ftrace_regs(fregs)->regs: *mut *mut pt_regs regs =,
    pub regs: return,
    pub NULL: return,
    pub ip: arch_ftrace_regs(fregs)->regs.psw.addr =,

    pub ftrace_regs_get_stack_pointer(fregs): return,
    pub arch_ftrace_regs(fregs)->regs.gprs[14]: return,

    pub \: (_regs)->psw.mask = 0;,
    pub \: (_regs)->psw.addr = arch_ftrace_regs(fregs)->regs.psw.addr;,
    pub \: (_regs)->gprs[15] = arch_ftrace_regs(fregs)->regs.gprs[15];,

//
// When an ftrace registered caller is tracing a function that is
// also set by a register_ftrace_direct() call, it needs to be
// differentiated in the ftrace_caller trampoline. To do this,
// place the direct caller in the ORIG_GPR2 part of pt_regs. This
// tells the ftrace_caller that there's a direct caller.
//
    pub &arch_ftrace_regs(fregs)->regs: *mut *mut pt_regs regs =,
    pub addr: regs->orig_gpr2 =,

// Macro flag: #define ARCH_HAS_SYSCALL_MATCH_SYM_NAME
// Skip the __s390x_ prefix.
    pub name): return !strcmp(sym + 7, name) || !strcmp(sym + 8,,
    pub fregs): *mut *mut ftrace_ops op, ftrace_regs,

    pub \: .section __mcount_loc, "a", @progbits;,
    pub \: .quad name;,

// Macro flag: #define FTRACE_GEN_MCOUNT_RECORD(name)

// Macro flag: #define FTRACE_GEN_NOP_ASM(name)

