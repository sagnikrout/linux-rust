//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/include/basic_asm.h
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
// Note: These macros assume that variables being stored on the stack are
// sizeof(long), while this is usually the case it may not always be the
// case for each use case.
//

// ABIv2

pub const STACK_FRAME_MIN_SIZE: c_int = 32;
pub const STACK_FRAME_TOC_POS: c_int = 24;

pub const STACK_FRAME_MIN_SIZE: c_int = 112;
pub const STACK_FRAME_TOC_POS: c_int = 40;

//
// Caveat: if a function passed more than 8 doublewords, the caller will have
// made more space... which would render the 112 incorrect.
//

// Common 64-bit
pub const STACK_FRAME_LR_POS: c_int = 16;
pub const STACK_FRAME_CR_POS: c_int = 8;

pub const STACK_FRAME_MIN_SIZE: c_int = 16;
pub const STACK_FRAME_LR_POS: c_int = 4;

// Parameter x saved to the stack

// Local variable x saved to the stack after x parameters

//
// It is very important to note here that _extra is the extra amount of
// stack space needed. This space can be accessed using STACK_FRAME_PARAM()
// or STACK_FRAME_LOCAL() macros.
//
// r1 and r2 are not defined in ppc-asm.h (instead they are defined as sp
// and toc). Kernel programmers tend to prefer rX even for r1 and r2, hence
// %1 and %r2. r0 is defined in ppc-asm.h and therefore %r0 gets
// preprocessed incorrectly, hence r0.
//

