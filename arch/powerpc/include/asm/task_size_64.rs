//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/task_size_64.h
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
// 64-bit user address space can have multiple limits
// For now supported values are:
//

//
// With 52 bits in the address we can support up to 4PB of range.
//

//
// For now 512TB is only supported with book3s and 64K linux page size.
//

//
// Max value currently used:
//

//
// We don't need to allocate extended context ids for 4K page size, because we
// limit the max effective address on this config to 64TB.
//

//
// 32-bit user address space is 4GB - 1 page
// (this 1 page is needed so referencing of 0xFFFFFFFF generates EFAULT
//

//
// This decides where the kernel will search for a free chunk of vm space during
// mmap's.
//

//
// Initial task size value for user applications. For book3s 64 we start
// with 128TB and conditionally enable upto 512TB
//

