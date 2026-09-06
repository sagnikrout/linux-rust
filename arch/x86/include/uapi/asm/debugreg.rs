//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/uapi/asm/debugreg.h
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
// Indicate the register numbers for a number of the specific

// Define a few things for the status register.  We can use this to determine
//
// Define bits in DR6 which are set to 1 by default.
//
// This is also the DR6 architectural value following Power-up, Reset or INIT.
//
// Note, with the introduction of Bus Lock Detection (BLD) and Restricted
// Transactional Memory (RTM), the DR6 register has been modified:
//
// 1) BLD flag (bit 11) is no longer reserved to 1 if the CPU supports
// Bus Lock Detection.  The assertion of a bus lock could clear it.
//
// 2) RTM flag (bit 16) is no longer reserved to 1 if the CPU supports
// restricted transactional memory.  #DB occurred inside an RTM region
// could clear it.
//
// Apparently, DR6.BLD and DR6.RTM are active low bits.
//
// As a result, DR6_RESERVED is an incorrect name now, but it is kept for
// compatibility.
//

// Now define a bunch of things for manipulating the control register.

// The low byte to the control register determine which registers are

// The second byte to the control register has a few special things.

//
// HW breakpoint additions
//
