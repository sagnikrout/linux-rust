//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/stlb_regs.h
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
// Copyright 2016-2018 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// STLB (Prototype: STLB)
//
pub const mmSTLB_CACHE_INV: c_uint = 0x490010;
pub const mmSTLB_CACHE_INV_BASE_39_8: c_uint = 0x490014;
pub const mmSTLB_CACHE_INV_BASE_49_40: c_uint = 0x490018;
pub const mmSTLB_STLB_FEATURE_EN: c_uint = 0x49001C;
pub const mmSTLB_STLB_AXI_CACHE: c_uint = 0x490020;
pub const mmSTLB_HOP_CONFIGURATION: c_uint = 0x490024;
pub const mmSTLB_LINK_LIST_LOOKUP_MASK_49_32: c_uint = 0x490028;
pub const mmSTLB_LINK_LIST_LOOKUP_MASK_31_0: c_uint = 0x49002C;
pub const mmSTLB_LINK_LIST: c_uint = 0x490030;
pub const mmSTLB_INV_ALL_START: c_uint = 0x490034;
pub const mmSTLB_INV_ALL_SET: c_uint = 0x490038;
pub const mmSTLB_INV_PS: c_uint = 0x49003C;
pub const mmSTLB_INV_CONSUMER_INDEX: c_uint = 0x490040;
pub const mmSTLB_INV_HIT_COUNT: c_uint = 0x490044;
pub const mmSTLB_INV_SET: c_uint = 0x490048;
pub const mmSTLB_SRAM_INIT: c_uint = 0x49004C;
