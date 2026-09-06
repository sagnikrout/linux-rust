//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/ftrace.h
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

// Ignore unused weak functions which will have larger offsets

pub const FTRACE_MCOUNT_MAX_OFFSET: c_int = 16;

pub const FTRACE_MCOUNT_MAX_OFFSET: c_int = 8;

extern "C" {
    pub fn _mcount();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dyn_arch_ftrace {

// pointer to the associated out-of-line stub
    pub ool_stub: c_ulong,

}

extern "C" {
    pub fn ftrace_init_nop(mod: *mut module, rec: *mut dyn_ftrace) -> c_int;
}

// We clear regs.msr in ftrace_call

pub const ARCH_SUPPORTS_FTRACE_OPS: c_int = 1;

//
// Some syscall entry functions on powerpc start with "ppc_" (fork and clone,
// for instance) or ppc32_/ppc64_. We should also match the sys_ variant with
// those.
//
// Macro flag: #define ARCH_HAS_SYSCALL_MATCH_SYM_NAME

// Disable ftrace on this CPU if possible (may not be implemented)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ftrace_ool_stub {

    pub ftrace_op: *mut ftrace_ops,
    pub insn: [u32; 4],
    pub long)): } __aligned(sizeof(unsigned,

    pub ftrace_free_init_tramp(void): c_void,
    pub addr): unsigned long ftrace_call_adjust(unsigned long,

//
// When an ftrace registered caller is tracing a function that is also set by a
// register_ftrace_direct() call, it needs to be differentiated in the
// ftrace_caller trampoline so that the direct call can be invoked after the
// other ftrace ops. To do this, place the direct caller in the orig_gpr3 field
// of pt_regs. This tells ftrace_caller that there's a direct caller.
//
    pub &arch_ftrace_regs(fregs)->regs: *mut *mut pt_regs regs =,
    pub addr: regs->orig_gpr3 =,

    pub }: static inline unsigned long ftrace_call_adjust(unsigned long addr) { return addr;,

