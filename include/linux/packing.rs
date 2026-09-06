//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/packing.h
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


// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2016-2018 NXP
// Copyright (c) 2018-2019, Vladimir Oltean <olteanv@gmail.com>
//

// struct packed_field_u8. Use with bit offsets < 256, buffers < 32B and
// unpacked structures < 256B.
//
// struct packed_field_u16. Use with bit offsets < 65536, buffers < 8KB and
// unpacked structures < 64KB.
//

// Note that the packed fields may be either in ascending or descending order.
// Thus, we must check that both the first and last field wit within the
// packed buffer size.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum packing_op {
    PACK,
    UNPACK,
}

// Do not hand-edit the following packed field check macros!
//
// They are generated using scripts/gen_packed_field_checks.c, which may be
// built via "make scripts_gen_packed_field_checks". If larger macro sizes are
// needed in the future, please use this program to re-generate the macros and
// insert them here.
//

// End of generated content

