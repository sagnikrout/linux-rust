//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/spi/spi.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note

//
// All the bits defined above should be covered by SPI_MODE_USER_MASK.
// The SPI_MODE_USER_MASK has the SPI_MODE_KERNEL_MASK counterpart in
// 'include/linux/spi/spi.h'. The bits defined here are from bit 0 upwards
// while in SPI_MODE_KERNEL_MASK they are from the other end downwards.
// These bits must not overlap. A static assert check should make sure of that.
// If adding extra bits, make sure to increase the bit index below as well.
//

