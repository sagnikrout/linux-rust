//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/bug.h
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

extern "C" {
    pub fn __WARN_trap(bug: *mut bug_entry, ...);
}

//
// Despite that some emulators terminate on UD2, we use it for WARN().
//

pub const INSN_UD2: c_uint = 0x0b0f;
pub const LEN_UD2: c_int = 2;

pub const INSN_UDB: c_uint = 0xd6;
pub const LEN_UDB: c_int = 1;
//
// In clang we have UD1s reporting UBSAN failures on X86, 64 and 32bit.
//
pub const INSN_ASOP: c_uint = 0x67;
pub const INSN_LOCK: c_uint = 0xf0;
pub const OPCODE_ESCAPE: c_uint = 0x0f;
pub const SECOND_BYTE_OPCODE_UD1: c_uint = 0xb9;
pub const SECOND_BYTE_OPCODE_UD2: c_uint = 0x0b;
pub const BUG_NONE: c_uint = 0xffff;
pub const BUG_UD2: c_uint = 0xfffe;
pub const BUG_UD1: c_uint = 0xfffd;
pub const BUG_UD1_UBSAN: c_uint = 0xfffc;
pub const BUG_UD1_WARN: c_uint = 0xfffb;
pub const BUG_UDB: c_uint = 0xffd6;
pub const BUG_LOCK: c_uint = 0xfff0;

// Macro flag: #define HAVE_ARCH_BUG_FORMAT

// Macro flag: #define __BUG_ENTRY_FORMAT(format)

// Macro flag: #define HAVE_ARCH_BUG_FORMAT_ARGS

// Macro flag: #define HAVE_ARCH_BUG

//
// This instrumentation_begin() is strictly speaking incorrect; but it
// suppresses the complaints from WARN()s in noinstr code. If such a WARN()
// were to trigger, we'd rather wreck the machine in an attempt to get the
// message out than not know about it.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysv_va_list {
    pub gp_offset: c_uint,
    pub fp_offset: c_uint,
    pub overflow_arg_area: *mut c_void,
    pub reg_save_area: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_va_list {
    pub regs: [c_ulong; 6],
    pub args: sysv_va_list,
}

