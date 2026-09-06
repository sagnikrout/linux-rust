//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/uapi/asm/sie.h
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

//
// This is the simple interceptable instructions decoder.
//
// It will be used as userspace interface and it can be used in places
// that does not allow to use general decoder functions,
// such as trace events declarations.
//
// Some userspace tools may want to parse this code
// and would be confused by switch(), if() and other statements,
// but they can understand conditional operator.
//

//
// The macro icpt_insn_decoder() takes an intercepted instruction
// and returns a key, which can be used to find a mnemonic name
// of the instruction in the icpt_insn_codes table.
//

