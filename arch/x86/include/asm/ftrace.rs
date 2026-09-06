//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/ftrace.h
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

// Ignore unused weak functions which will have non zero offsets

// Add offset for endbr64 if IBT enabled

pub const ARCH_SUPPORTS_FTRACE_OPS: c_int = 1;

extern "C" {
    pub fn __fentry__();
}
//
// addr is the address of the mcount call instruction.
// recordmcount does the necessary offset calculation.
//

// Only when FL_SAVE_REGS is set, cs will be non zero

//
// When a ftrace registered caller is tracing a function that is
// also set by a register_ftrace_direct() call, it needs to be
// differentiated in the ftrace_caller trampoline. To do this, we
// place the direct caller in the ORIG_AX part of pt_regs. This
// tells the ftrace_caller that there's a direct caller.
//
// Emulate a call

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dyn_arch_ftrace {
// No extra data needed for x86
}

extern "C" {
    pub fn set_ftrace_ops_ro();
}

// Macro flag: #define ARCH_HAS_SYSCALL_MATCH_SYM_NAME
//
// Compare the symbol name with the system call name. Skip the
// "__x64_sys", "__ia32_sys", "__do_sys" or simple "sys" prefix.
//

//
// Because ia32 syscalls do not map to x86_64 syscall numbers
// this screws up the trace output when tracing a ia32 task.
// Instead of reporting bogus syscalls, just do not trace them.
//
// If the user really wants these, then they should use the
// raw syscall tracepoints with filtering.
//
pub const ARCH_TRACE_IGNORE_COMPAT_SYSCALLS: c_int = 1;
extern "C" {
    pub fn in_32bit_syscall() -> return;
}

