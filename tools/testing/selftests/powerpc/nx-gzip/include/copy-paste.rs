//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/nx-gzip/include/copy-paste.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// From asm-compat.h

//
// Macros taken from arch/powerpc/include/asm/ppc-opcode.h and other
// header files.
//

pub const PPC_INST_COPY: c_uint = 0x7c20060c;
pub const PPC_INST_PASTE: c_uint = 0x7c20070d;

pub const CR0_SHIFT: c_int = 28;
pub const CR0_MASK: c_uint = 0xF;
//
// Copy/paste instructions:
//
// copy RA,RB
// Copy contents of address (RA) + effective_address(RB)
// to internal copy-buffer.
//
// paste RA,RB
// Paste contents of internal copy-buffer to the address
// (RA) + effective_address(RB)
//
