//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/alternative.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alt_instr {
    pub /: *mut *mut s32 instr_offset; / offset to original instruction,
    pub /: *mut *mut s32 replace_offset; / offset to replacement instruction,
    pub /: *mut *mut u16 feature; / feature bit set for replacement,
    pub /: *mut *mut u8 instrlen; / length of original instruction,
    pub /: *mut *mut u8 replacementlen; / length of new instruction,
    pub __packed: },
//
// Debug flag that can be tested to see whether alternative
// instructions were patched in already:
//
    pub alternatives_patched: extern int,
    pub __alt_instructions_end: [extern struct alt_instr __alt_instructions[],; ],
    pub alternative_instructions(void): extern void,
    pub end): *mut *mut extern void apply_alternatives(struct alt_instr start, struct alt_instr,

//
// Pad the second replacement alternative with additional NOPs if it is
// additionally longer than the first replacement alternative.
//

// alternative assembly primitive:

//
// Alternative instructions for different CPU types or capabilities.
//
// This allows to use optimized instructions even on generic binary
// kernels.
//
// length of oldinstr must be longer or equal the length of newinstr
// It can be padded with nops as needed.
//
// For non barrier like inlines please define new variants
// without volatile and memory clobber.
//

