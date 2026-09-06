//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/extable.h
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
// The exception table consists of two addresses relative to the
// exception table entry itself and a type selector field.
//
// The first address is of an instruction that is allowed to fault, the
// second is the target at which the program should continue.
//
// The type entry is used by fixup_exception() to select the handler to
// deal with the fault caused by the instruction in the first field.
//
// All the routines below use bits of fixup code that are out of line
// with the main instruction path.  This means when everything is well,
// we don't even have to jump over them.  Further, they do not intrude
// on our cache or tlb entries.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exception_table_entry {
    pub data: int insn, fixup,,
}

// Macro flag: #define ARCH_HAS_RELATIVE_EXTABLE

extern "C" {
    pub fn ex_get_fixup_type(ip: c_ulong) -> c_int;
}
extern "C" {
    pub fn early_fixup_exception(regs: *mut pt_regs, trapnr: c_int);
}

extern "C" {
    pub fn ex_handler_msr_mce(regs: *mut pt_regs, wrmsr: bool) -> void __noreturn;
}

extern "C" {
    pub fn ex_handler_bpf(x: *const exception_table_entry, regs: *mut pt_regs) -> bool;
}

