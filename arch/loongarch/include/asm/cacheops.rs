//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/cacheops.h
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
// Cache operations for the cache instruction.
//
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//
// Most cache ops are split into a 3 bit field identifying the cache, and a 2
// bit field identifying the cache operation.
//
pub const CacheOp_Cache: c_uint = 0x07;
pub const CacheOp_Op: c_uint = 0x18;
pub const Cache_LEAF0: c_uint = 0x00;
pub const Cache_LEAF1: c_uint = 0x01;
pub const Cache_LEAF2: c_uint = 0x02;
pub const Cache_LEAF3: c_uint = 0x03;
pub const Cache_LEAF4: c_uint = 0x04;
pub const Cache_LEAF5: c_uint = 0x05;
pub const Index_Invalidate: c_uint = 0x08;
pub const Index_Writeback_Inv: c_uint = 0x08;
pub const Hit_Invalidate: c_uint = 0x10;
pub const Hit_Writeback_Inv: c_uint = 0x10;
pub const CacheOp_User_Defined: c_uint = 0x18;

