//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/ptrace.h
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
// Based on arch/arm/include/asm/ptrace.h
//
// Copyright (C) 1996-2003 Russell King
// Copyright (C) 2012 ARM Ltd.
//

// Current Exception Level values, as contained in CurrentEL

// Additional SPSR bits not exposed in the UABI

// AArch32-specific ptrace requests
pub const COMPAT_PTRACE_GETREGS: c_int = 12;
pub const COMPAT_PTRACE_SETREGS: c_int = 13;
pub const COMPAT_PTRACE_GET_THREAD_AREA: c_int = 22;
pub const COMPAT_PTRACE_SET_SYSCALL: c_int = 23;
pub const COMPAT_PTRACE_GETVFPREGS: c_int = 27;
pub const COMPAT_PTRACE_SETVFPREGS: c_int = 28;
pub const COMPAT_PTRACE_GETHBPREGS: c_int = 29;
pub const COMPAT_PTRACE_SETHBPREGS: c_int = 30;
// SPSR_ELx bits for exceptions taken from AArch32
pub const PSR_AA32_MODE_MASK: c_uint = 0x0000001f;
pub const PSR_AA32_MODE_USR: c_uint = 0x00000010;
pub const PSR_AA32_MODE_FIQ: c_uint = 0x00000011;
pub const PSR_AA32_MODE_IRQ: c_uint = 0x00000012;
pub const PSR_AA32_MODE_SVC: c_uint = 0x00000013;
pub const PSR_AA32_MODE_ABT: c_uint = 0x00000017;
pub const PSR_AA32_MODE_HYP: c_uint = 0x0000001a;
pub const PSR_AA32_MODE_UND: c_uint = 0x0000001b;
pub const PSR_AA32_MODE_SYS: c_uint = 0x0000001f;
pub const PSR_AA32_T_BIT: c_uint = 0x00000020;
pub const PSR_AA32_F_BIT: c_uint = 0x00000040;
pub const PSR_AA32_I_BIT: c_uint = 0x00000080;
pub const PSR_AA32_A_BIT: c_uint = 0x00000100;
pub const PSR_AA32_E_BIT: c_uint = 0x00000200;
pub const PSR_AA32_PAN_BIT: c_uint = 0x00400000;
pub const PSR_AA32_SSBS_BIT: c_uint = 0x00800000;
pub const PSR_AA32_DIT_BIT: c_uint = 0x01000000;
pub const PSR_AA32_Q_BIT: c_uint = 0x08000000;
pub const PSR_AA32_V_BIT: c_uint = 0x10000000;
pub const PSR_AA32_C_BIT: c_uint = 0x20000000;
pub const PSR_AA32_Z_BIT: c_uint = 0x40000000;
pub const PSR_AA32_N_BIT: c_uint = 0x80000000;
pub const PSR_AA32_IT_MASK: c_uint = 0x0600fc00	/* If-Then execution state mask */;
pub const PSR_AA32_GE_MASK: c_uint = 0x000f0000;

pub const PSR_AA32_ENDSTATE: c_int = 0;

// AArch32 CPSR bits, as seen in AArch32
pub const COMPAT_PSR_DIT_BIT: c_uint = 0x00200000;
//
// These are 'magic' values for PTRACE_PEEKUSR that return info about where a
// process is located in memory.
//
pub const COMPAT_PT_TEXT_ADDR: c_uint = 0x10000;
pub const COMPAT_PT_DATA_ADDR: c_uint = 0x10004;
pub const COMPAT_PT_TEXT_END_ADDR: c_uint = 0x10008;
//
// If pt_regs.syscallno == NO_SYSCALL, then the thread is not executing
// a syscall -- i.e., its most recent entry into the kernel from
// userspace was not via SVC, or otherwise a tracer cancelled the syscall.
//
// This must have the value -1, for ABI compatibility with ptrace etc.
//

// sizeof(struct user) for AArch32
pub const COMPAT_USER_SZ: c_int = 296;
// Architecturally defined mapping between AArch32 and AArch64 registers

//
// This struct defines the way the registers are stored on the stack during an
// exception. struct user_pt_regs must form a prefix of struct pt_regs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_regs {
    pub user_regs: user_pt_regs,
    pub regs: [u64; 31],
    pub sp: u64,
    pub pc: u64,
    pub pstate: u64,
}

// For correct stack alignment, pt_regs has to be a multiple of 16 bytes.

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
// Read a register given an architectural register index r.
// This handles the common case where 31 means XZR, not SP.
//
// Write a register given an architectural register index r.
// This handles the common case where 31 means XZR, not SP.
//
// Valid only for Kernel mode traps.
//
// Audit currently uses regs_return_value() instead of
// syscall_get_return_value(). Apply the same sign-extension here until
// audit is updated to use syscall_get_return_value().
//
// regs_get_kernel_argument() - get Nth function argument in kernel
// @regs:	pt_regs of that context
// @n:		function argument number (start from 0)
//
// regs_get_argument() returns @n th argument of the function call.
//
// Note that this chooses the most likely register mapping. In very rare
// cases this may not return correct data, for example, if one of the
// function parameters is 16 bytes or bigger. In such cases, we cannot
// get access the parameter correctly and the register assignment of
// subsequent parameters will be shifted.
//
pub const NR_REG_ARGUMENTS: c_int = 8;
extern "C" {
    pub fn pt_regs_read_reg(_arg: regs, _arg: n) -> return;
}
// We must avoid circular header include via sched.h
extern "C" {
    pub fn valid_user_regs(regs: *mut user_pt_regs, task: *mut task_struct) -> c_int;
}

extern "C" {
    pub fn profile_pc(regs: *mut pt_regs) -> c_ulong;
}

