//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/net/intel/libie/rx.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2024 Intel Corporation

// Rx buffer management
// The largest size for a single descriptor as per HW

// "True" HW-writeable space: minimum from SW and HW values

// The maximum frame size as per HW (S/G)

// ATST, HW can chain up to 5 Rx descriptors

// Maximum frame size minus LL overhead

// O(1) converting i40e/ice/iavf's 8/10-bit hardware packet type to a parsed
// bitfield struct.
//
pub const LIBIE_RX_PT_NUM: c_int = 154;
//
// libie_rx_pt_parse - convert HW packet type to software bitfield structure
// @pt: 10-bit hardware packet type value from the descriptor
//
// ```libie_rx_pt_lut``` must be accessed only using this wrapper.
//
// Return: parsed bitfield struct corresponding to the provided ptype.
//
