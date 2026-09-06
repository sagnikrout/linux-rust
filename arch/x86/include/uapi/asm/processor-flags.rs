//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/uapi/asm/processor-flags.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
// Various flags defined: can be included from assembler.

//
// EFLAGS bits
//

//
// Basic CPU control in CR0
//

//
// Paging options in CR3
//

pub const X86_CR3_PCID_BITS: c_int = 12;

//
// Intel CPU features in CR4
//

//
// x86-64 Task Priority Register, CR8
//

//
// AMD and Transmeta use MSRs for configuration; see <asm/msr-index.h>
//
// NSC/Cyrix CPU configuration register indexes
//
pub const CX86_PCR0: c_uint = 0x20;
pub const CX86_GCR: c_uint = 0xb8;
pub const CX86_CCR0: c_uint = 0xc0;
pub const CX86_CCR1: c_uint = 0xc1;
pub const CX86_CCR2: c_uint = 0xc2;
pub const CX86_CCR3: c_uint = 0xc3;
pub const CX86_CCR4: c_uint = 0xe8;
pub const CX86_CCR5: c_uint = 0xe9;
pub const CX86_CCR6: c_uint = 0xea;
pub const CX86_CCR7: c_uint = 0xeb;
pub const CX86_PCR1: c_uint = 0xf0;
pub const CX86_DIR0: c_uint = 0xfe;
pub const CX86_DIR1: c_uint = 0xff;
pub const CX86_ARR_BASE: c_uint = 0xc4;
pub const CX86_RCR_BASE: c_uint = 0xdc;

