//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/ptrace.h
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
//
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

//
// This struct defines the way the registers are stored on the stack during
// a system call/exception. If you add a register here, please also add it to
// regoffset_table[] in arch/loongarch/kernel/ptrace.c.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_regs {
// Main processor registers.
    pub regs: [c_ulong; 32],
// Original syscall arg0.
    pub orig_a0: c_ulong,
// Special CSR registers.
    pub csr_era: c_ulong,
    pub csr_badvaddr: c_ulong,
    pub csr_crmd: c_ulong,
    pub csr_prmd: c_ulong,
    pub csr_euen: c_ulong,
    pub csr_ecfg: c_ulong,
    pub csr_estat: c_ulong,
    pub __last: [c_ulong; ],
    pub __aligned(8): },
    pub CSR_PRMD_PIE): return !(regs->csr_prmd &,
    pub regs->regs[3]: return,
//
// Don't use asm-generic/ptrace.h it defines FP accessors that don't make
// sense on LoongArch.  We rather want an error if they get invoked.
//
    pub val: regs->csr_era =,
// Query offset/name of register from its name/offset
    pub name): *const extern int regs_query_register_offset(char,

//
// regs_get_register() - get register value from its offset
// @regs:       pt_regs from which register value is gotten.
// @offset:     offset number of the register.
//
// regs_get_register returns the value of a register. The @offset is the
// offset of the register in struct pt_regs address which specified by @regs.
// If @offset is bigger than MAX_REG_OFFSET, this returns 0.
//
    pub 0: return,
    pub offset): *mut *mut *mut return (unsigned long )((unsigned long)regs +,
//
// regs_within_kernel_stack() - check the address in the stack
// @regs:       pt_regs which contains kernel stack pointer.
// @addr:       address which is checked.
//
// regs_within_kernel_stack() checks @addr is within the kernel stack page(s).
// If @addr is within the kernel stack, it returns true. If not, returns false.
//
    pub 1))): (kernel_stack_pointer(regs) & ~(THREAD_SIZE -,
//
// regs_get_kernel_stack_nth() - get Nth entry of the stack
// @regs:       pt_regs which contains kernel stack pointer.
// @n:          stack entry number.
//
// regs_get_kernel_stack_nth() returns @n th entry of the kernel stack which
// is specified by @regs. If the @n th entry is NOT in the kernel stack,
// this returns 0.
//
    pub )kernel_stack_pointer(regs): *mut *mut unsigned long addr = (unsigned long,
    pub n: addr +=,
    pub addr: *mut return,
    pub 0: return,
    pub task_struct: struct,
//
// regs_get_kernel_argument() - get Nth function argument in kernel
// @regs:       pt_regs of that context
// @n:          function argument number (start from 0)
//
// regs_get_argument() returns @n th argument of the function call.
// Note that this chooses most probably assignment, in some case
// it can be incorrect.
// This is expected to be called from kprobes or ftrace with regs
// where the top of stack is the return address.
//
pub const NR_REG_ARGUMENTS: c_int = 8;
}

extern "C" {
    pub fn regs_get_register(_arg: regs, _arg: args[n]) -> return;
}
extern "C" {
    pub fn regs_get_kernel_stack_nth(_arg: regs, _arg: n) -> return;
}
//
// Does the process account for user or for system time?
//

extern "C" {
    pub fn die(str: *const c_char, regs: *mut pt_regs);
}

// Helpers for working with the user stack pointer

