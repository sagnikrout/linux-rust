//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/alternative-macros.h
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
// Binutils 2.27.0 can't handle a 'UL' suffix on constants, so for the assembly
// macros below we must use we must use `(1 << ARM64_CB_SHIFT)`.
//
pub const ARM64_CB_SHIFT: c_int = 15;

//
// alternative assembly primitive:
//
// If any of these .org directive fail, it means that insn1 and insn2
// don't have the same length. This used to be written as
//
// .if ((664b-663b) != (662b-661b))
// .error "Alternatives instruction length mismatch"
// .endif
//
// but most assemblers die if insn1 or insn2 have a .inst. This should
// be fixed in a binutils release posterior to 2.25.51.0.2 (anything
// containing commit 4e4d08cf7399b606 or c1baaddf8861).
//
// Alternatives with callbacks do not generate replacement instructions.
//

//
// Alternative sequences
//
// The code for the case where the capability is not present will be
// assembled and linked as normal. There are no restrictions on this
// code.
//
// The code for the case where the capability is present will be
// assembled into a special section to be used for dynamic patching.
// Code for that case must:
//
// 1. Be exactly the same length (in bytes) as the default code
// sequence.
//
// 2. Not contain a branch target that is used outside of the
// alternative sequence it is defined in (branches into an
// alternative sequence are not fixed up).
//
// Begin an alternative code sequence.
//
// Provide the other half of the alternative code sequence.
//
// Complete an alternative code sequence.
//
// Callback-based alternative epilogue
//
// Provides a trivial alternative or default sequence consisting solely
// of NOPs. The number of NOPs is chosen automatically to match the
// previous case.
//

//
// Usage: asm(ALTERNATIVE(oldinstr, newinstr, cpucap));
//
// Usage: asm(ALTERNATIVE(oldinstr, newinstr, cpucap, CONFIG_FOO));
// N.B. If CONFIG_FOO is specified, but not selected, the whole block
// will be omitted, including oldinstr.
//

