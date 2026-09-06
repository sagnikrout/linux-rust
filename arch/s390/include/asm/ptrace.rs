//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/ptrace.h
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
// S390 version
// Copyright IBM Corp. 1999, 2000
// Author(s): Denis Joseph Barrow (djbarrow@de.ibm.com,barrow_dj@yahoo.com)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psw_bits {
    pub 1: unsigned long :,
    pub /: *mut *mut unsigned long per : 1; / PER-Mask,
    pub 3: unsigned long :,
    pub /: *mut *mut unsigned long dat : 1; / DAT Mode,
    pub /: *mut *mut unsigned long io : 1; / Input/Output Mask,
    pub /: *mut *mut unsigned long ext : 1; / External Mask,
    pub /: *mut *mut unsigned long key : 4; / PSW Key,
    pub 1: unsigned long :,
    pub /: *mut *mut unsigned long mcheck : 1; / Machine-Check Mask,
    pub /: *mut *mut unsigned long wait : 1; / Wait State,
    pub /: *mut *mut unsigned long pstate : 1; / Problem State,
    pub /: *mut *mut unsigned long as : 2; / Address Space Control,
    pub /: *mut *mut unsigned long cc : 2; / Condition Code,
    pub /: *mut *mut unsigned long pm : 4; / Program Mask,
    pub /: *mut *mut unsigned long ri : 1; / Runtime Instrumentation,
    pub 6: unsigned long :,
    pub /: *mut *mut unsigned long eaba : 2; / Addressing Mode,
    pub 31: unsigned long :,
    pub /: *mut *mut unsigned long ia : 64; / Instruction Address,
}

pub const PGM_INT_CODE_MASK: c_uint = 0x7f;
pub const PGM_INT_CODE_PER: c_uint = 0x80;
//
// The pt_regs struct defines the way the registers are stored on
// the stack during a system call.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_regs {
    pub user_regs: user_pt_regs,
    pub args: [c_ulong; 1],
    pub psw: psw_t,
    pub gprs: [c_ulong; NUM_GPRS],
}

//
// Program event recording (PER) register set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct per_regs {
    pub /: *mut *mut unsigned long control; / PER control bits,
    pub /: *mut *mut unsigned long start; / PER starting address,
    pub /: *mut *mut unsigned long end; / PER ending address,
}

//
// PER event contains information about the cause of the last PER exception.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct per_event {
    pub /: *mut *mut unsigned short cause; / PER code, ATMID and AI,
    pub /: *mut *mut unsigned long address; / PER address,
    pub /: *mut *mut unsigned char paid; / PER access identification,
}

//
// Simplified per_info structure used to decode the ptrace user space ABI.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct per_struct_kernel {
    pub /: *mut *mut unsigned long cr9; / PER control bits,
    pub /: *mut *mut unsigned long cr10; / PER starting address,
    pub /: *mut *mut unsigned long cr11; / PER ending address,
    pub /: *mut *mut unsigned long bits; / Obsolete software bits,
    pub /: *mut *mut unsigned long starting_addr; / User specified start address,
    pub /: *mut *mut unsigned long ending_addr; / User specified end address,
    pub /: *mut *mut unsigned short perc_atmid; / PER trap ATMID,
    pub /: *mut *mut unsigned long address; / PER trap instruction address,
    pub /: *mut *mut unsigned char access_id; / PER trap access identification,
}

pub const PER_EVENT_MASK: c_uint = 0xEB000000UL;
pub const PER_EVENT_BRANCH: c_uint = 0x80000000UL;
pub const PER_EVENT_IFETCH: c_uint = 0x40000000UL;
pub const PER_EVENT_STORE: c_uint = 0x20000000UL;
pub const PER_EVENT_STORE_REAL: c_uint = 0x08000000UL;
pub const PER_EVENT_TRANSACTION_END: c_uint = 0x02000000UL;
pub const PER_EVENT_NULLIFICATION: c_uint = 0x01000000UL;
pub const PER_CONTROL_MASK: c_uint = 0x00e00000UL;
pub const PER_CONTROL_BRANCH_ADDRESS: c_uint = 0x00800000UL;
pub const PER_CONTROL_SUSPENSION: c_uint = 0x00400000UL;
pub const PER_CONTROL_ALTERATION: c_uint = 0x00200000UL;
extern "C" {
    pub fn update_cr_regs(task: *mut task_struct);
}
//
// These are defined as per linux/ptrace.h, which see.
//

extern "C" {
    pub fn regs_query_register_offset(name: *const c_char) -> c_int;
}
//
// regs_get_kernel_stack_nth() - get Nth entry of the stack
// @regs:pt_regs which contains kernel stack pointer.
// @n:stack entry number.
//
// regs_get_kernel_stack_nth() returns @n th entry of the kernel stack which
// is specifined by @regs. If the @n th entry is NOT in the kernel stack,
// this returns 0.
//
extern "C" {
    pub fn READ_ONCE_NOCHECK()addr: *mut *mut (unsigned long) -> return;
}
//
// regs_get_kernel_argument() - get Nth function argument in kernel
// @regs:	pt_regs of that context
// @n:		function argument number (start from 0)
//
// regs_get_kernel_argument() returns @n th argument of the function call.
//
pub const NR_REG_ARGUMENTS: c_int = 5;
extern "C" {
    pub fn regs_get_register(_arg: regs, n: 2 +) -> return;
}
extern "C" {
    pub fn regs_get_kernel_stack_nth(_arg: regs, n: argoffset +) -> return;
}

