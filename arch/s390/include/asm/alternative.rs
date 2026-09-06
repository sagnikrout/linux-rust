//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/alternative.h
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
// Each alternative comes with a 32 bit feature field:
// union {
// u32 feature;
// struct {
// u32 ctx	 : 4;
// u32 type : 8;
// u32 data : 20;
// };
// }
//
// @ctx is a bitfield, where only one bit must be set. Each bit defines
// in which context an alternative is supposed to be applied to the
// kernel image:
//
// - from the decompressor before the kernel itself is executed
// - from early kernel code from within the kernel
//
// @type is a number which defines the type and with that the type
// specific alternative patching.
//
// @data is additional type specific information which defines if an
// alternative should be applied.
//
pub const ALT_CTX_EARLY: c_int = 1;
pub const ALT_CTX_LATE: c_int = 2;

pub const ALT_TYPE_FACILITY: c_int = 0;
pub const ALT_TYPE_FEATURE: c_int = 1;
pub const ALT_TYPE_SPEC: c_int = 2;
pub const ALT_DATA_SHIFT: c_int = 0;
pub const ALT_TYPE_SHIFT: c_int = 20;
pub const ALT_CTX_SHIFT: c_int = 28;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alt_instr {
    pub /: *mut *mut s32 instr_offset; / original instruction,
    pub /: *mut *mut s32 repl_offset; / offset to replacement instruction,
    pub /: *mut *mut u32 feature; / feature required for replacement,
    pub /: *mut *mut u32 ctx : 4; / context,
    pub /: *mut *mut u32 type : 8; / type of alternative,
    pub /: *mut *mut u32 data : 20; / patching information,
}

extern "C" {
    pub fn __apply_alternatives(start: *mut alt_instr, end: *mut alt_instr, ctx: c_uint);
}
//
// +---------------------------------+
// |661:			     |662:
// | oldinstr			     |
// +---------------------------------+
//
// .altinstr_replacement section
// +---------------------------------+
// |6641:			     |6651:
// | alternative instr 1	     |
// +---------------------------------+
// |6642:			     |6652:
// | alternative instr 2	     |
// +---------------------------------+
//
// .altinstructions section
// +---------------------------------+
// | alt_instr entries for each      |
// | alternative instr		     |
// +---------------------------------+
//

// alternative assembly primitive:

//
// Alternative instructions for different CPU types or capabilities.
//
// This allows to use optimized instructions even on generic binary
// kernels.
//
// oldinstr is padded with jump and nops at compile time if altinstr is
// longer. altinstr is padded with jump and nops at run-time during patching.
//
// For non barrier like inlines please define new variants
// without volatile and memory clobber.
//

// Alternative inline assembly with input.

// Like alternative_input, but with a single output argument

// Use this macro if more than one output parameter is needed.

// Use this macro if clobbers are needed without inputs.

//
// Issue one struct alt_instr descriptor entry (need to put it into
// the section .altinstructions, see below). This entry contains
// enough information for the alternatives patching code to patch an
// instruction. See apply_alternatives().
//
// Define an alternative between two instructions. If @feature is
// present, early code in apply_alternatives() replaces @oldinstr with
// @newinstr.
//
// Define an alternative between two instructions. If @feature is
// present, early code in apply_alternatives() replaces @oldinstr with
// @newinstr.
//

