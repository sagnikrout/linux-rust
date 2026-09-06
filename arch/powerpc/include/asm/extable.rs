//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/extable.h
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
// The exception table consists of pairs of relative addresses: the first is
// the address of an instruction that is allowed to fault, and the second is
// the address at which the program should continue.  No registers are
// modified, so it is entirely up to the continuation code to figure out what
// to do.
//
// All the routines below use bits of fixup code that are out of line with the
// main instruction path.  This means when everything is well, we don't even
// have to jump over them.  Further, they do not intrude on our cache or tlb
// entries.
//
// Macro flag: #define ARCH_HAS_RELATIVE_EXTABLE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exception_table_entry {
    pub insn: c_int,
    pub fixup: c_int,
}

//
// Helper macro for exception table entries
//

