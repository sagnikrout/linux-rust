//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/ptrace.h
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
//
// Copyright (C) 2012 Regents of the University of California
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_regs {
    pub epc: c_ulong,
    pub ra: c_ulong,
    pub sp: c_ulong,
    pub gp: c_ulong,
    pub tp: c_ulong,
    pub t0: c_ulong,
    pub t1: c_ulong,
    pub t2: c_ulong,
    pub s0: c_ulong,
    pub s1: c_ulong,
    pub a0: c_ulong,
    pub a1: c_ulong,
    pub a2: c_ulong,
    pub a3: c_ulong,
    pub a4: c_ulong,
    pub a5: c_ulong,
    pub a6: c_ulong,
    pub a7: c_ulong,
    pub s2: c_ulong,
    pub s3: c_ulong,
    pub s4: c_ulong,
    pub s5: c_ulong,
    pub s6: c_ulong,
    pub s7: c_ulong,
    pub s8: c_ulong,
    pub s9: c_ulong,
    pub s10: c_ulong,
    pub s11: c_ulong,
    pub t3: c_ulong,
    pub t4: c_ulong,
    pub t5: c_ulong,
    pub t6: c_ulong,
// Supervisor/Machine CSRs
    pub status: c_ulong,
    pub badaddr: c_ulong,
    pub cause: c_ulong,
// a0 value before the syscall
    pub orig_a0: c_ulong,
}

pub const PTRACE_SYSEMU: c_uint = 0x1f;
pub const PTRACE_SYSEMU_SINGLESTEP: c_uint = 0x20;

// Helpers for working with the instruction pointer

// Helpers for working with the user stack pointer
// Valid only for Kernel mode traps.
// Helpers for working with the frame pointer
extern "C" {
    pub fn regs_query_register_offset(name: *const c_char) -> c_int;
}
//
// regs_get_register() - get register value from its offset
// @regs:	pt_regs from which register value is gotten
// @offset:	offset of the register.
//
// regs_get_register returns the value of a register whose offset from @regs.
// The @offset is the offset of the register in struct pt_regs.
// If @offset is bigger than MAX_REG_OFFSET, this returns 0.
//
// regs_get_kernel_argument() - get Nth function argument in kernel
// @regs:       pt_regs of that context
// @n:          function argument number (start from 0)
//
// regs_get_argument() returns @n th argument of the function call.
//
// Note you can get the parameter correctly if the function has no
// more than eight arguments.
//
extern "C" {
    pub fn regs_get_register(_arg: regs, _arg: argument_offs[n]) -> return;
}

