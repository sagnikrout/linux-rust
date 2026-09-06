//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/checksum.h
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
// Checksum routines
//
// Copyright (C) 2023 Rivos Inc.
//

extern "C" {
    pub fn do_csum(buff: *const c_uchar, len: c_int) -> c_uint;
}

// Default version is sufficient for 32 bit

// Define riscv versions of functions before importing asm-generic/checksum.h

//
// Quickly compute an IP checksum with the assumption that IPv4 headers will
// always be in multiples of 32-bits, and have an ihl of at least 5.
//
// @ihl: the number of 32 bit segments and must be greater than or equal to 5.
// @iph: assumed to be word aligned given that NET_IP_ALIGN is set to 2 on
// riscv, defining IP headers to be aligned.
//
// ZBB only saves three instructions on 32-bit and five on 64-bit so not
// worth checking if supported without Alternatives.
//

extern "C" {
    pub fn csum_fold(__wsum)csum: () -> return;
}
