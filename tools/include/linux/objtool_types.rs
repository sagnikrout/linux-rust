//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/objtool_types.h
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
// This struct is used by asm and inline asm code to manually annotate the
// location of registers on the stack.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct unwind_hint {
    pub ip: u32,
    pub sp_offset: i16,
    pub sp_reg: u8,
    pub type: u8,
    pub signal: u8,
}

//
// UNWIND_HINT_TYPE_UNDEFINED: A blind spot in ORC coverage which can result in
// a truncated and unreliable stack unwind.
//
// UNWIND_HINT_TYPE_END_OF_STACK: The end of the kernel stack unwind before
// hitting user entry, boot code, or fork entry (when there are no pt_regs
// available).
//
// UNWIND_HINT_TYPE_CALL: Indicates that sp_reg+sp_offset resolves to PREV_SP
// (the caller's SP right before it made the call).  Used for all callable
// functions, i.e. all C code and all callable asm functions.
//
// UNWIND_HINT_TYPE_REGS: Used in entry code to indicate that sp_reg+sp_offset
// points to a fully populated pt_regs from a syscall, interrupt, or exception.
//
// UNWIND_HINT_TYPE_REGS_PARTIAL: Used in entry code to indicate that
// sp_reg+sp_offset points to the iret return frame.
//
// UNWIND_HINT_TYPE_FUNC: Generate the unwind metadata of a callable function.
// Useful for code which doesn't have an ELF function annotation.
//
// UNWIND_HINT_TYPE_{SAVE,RESTORE}: Save the unwind metadata at a certain
// location so that it can be restored later.
//
pub const UNWIND_HINT_TYPE_UNDEFINED: c_int = 0;
pub const UNWIND_HINT_TYPE_END_OF_STACK: c_int = 1;
pub const UNWIND_HINT_TYPE_CALL: c_int = 2;
pub const UNWIND_HINT_TYPE_REGS: c_int = 3;
pub const UNWIND_HINT_TYPE_REGS_PARTIAL: c_int = 4;
// The below hint types don't have corresponding ORC types
pub const UNWIND_HINT_TYPE_FUNC: c_int = 5;
pub const UNWIND_HINT_TYPE_SAVE: c_int = 6;
pub const UNWIND_HINT_TYPE_RESTORE: c_int = 7;
//
// Annotate types
//
pub const ANNOTYPE_NOENDBR: c_int = 1;
pub const ANNOTYPE_RETPOLINE_SAFE: c_int = 2;
pub const ANNOTYPE_INSTR_BEGIN: c_int = 3;
pub const ANNOTYPE_INSTR_END: c_int = 4;
pub const ANNOTYPE_UNRET_BEGIN: c_int = 5;
pub const ANNOTYPE_IGNORE_ALTS: c_int = 6;
pub const ANNOTYPE_INTRA_FUNCTION_CALL: c_int = 7;
pub const ANNOTYPE_REACHABLE: c_int = 8;
pub const ANNOTYPE_NOCFI: c_int = 9;
pub const ANNOTYPE_DATA_SPECIAL: c_int = 1;
