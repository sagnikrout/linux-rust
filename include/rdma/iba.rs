//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/iba.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2020, Mellanox Technologies inc.  All rights reserved.
//

// ptr = (*ptr & ~mask) | prep_value;
extern "C" {
    pub fn be16_to_cpu(_arg: *mut ptr) -> return;
}
// ptr = cpu_to_be16((be16_to_cpu(*ptr) & ~mask) | prep_value);
extern "C" {
    pub fn be32_to_cpu(_arg: *mut ptr) -> return;
}
// ptr = cpu_to_be32((be32_to_cpu(*ptr) & ~mask) | prep_value);
//
// The mads are constructed so that 32 bit and smaller are naturally
// aligned, everything larger has a max alignment of 4 bytes.
//
extern "C" {
    pub fn be64_to_cpu(_arg: get_unaligned(ptr)) -> return;
}

// FIXME: A set should always set the entire field, meaning we should zero the trailing bytes

//
// The generated list becomes the parameters to the macros, the order is:
// - struct this applies to
// - starting offset of the max
// - GENMASK or GENMASK_ULL in CPU order
// - The width of data the mask operations should work on, in bits
//
// Extraction using a tabular description like table 106. bit_offset is from
// the Byte[Bit] notation.
//

//
// In IBTA spec, everything that is more than 64bits is multiple
// of bytes without leftover bits.
//

