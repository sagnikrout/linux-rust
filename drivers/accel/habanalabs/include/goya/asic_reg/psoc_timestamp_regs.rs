//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/psoc_timestamp_regs.h
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
// PSOC_TIMESTAMP (Prototype: TIMESTAMP)
//
pub const mmPSOC_TIMESTAMP_CNTCR: c_uint = 0xC49000;
pub const mmPSOC_TIMESTAMP_CNTSR: c_uint = 0xC49004;
pub const mmPSOC_TIMESTAMP_CNTCVL: c_uint = 0xC49008;
pub const mmPSOC_TIMESTAMP_CNTCVU: c_uint = 0xC4900C;
pub const mmPSOC_TIMESTAMP_CNTFID0: c_uint = 0xC49020;
pub const mmPSOC_TIMESTAMP_PIDR4: c_uint = 0xC49FD0;
pub const mmPSOC_TIMESTAMP_PIDR5: c_uint = 0xC49FD4;
pub const mmPSOC_TIMESTAMP_PIDR6: c_uint = 0xC49FD8;
pub const mmPSOC_TIMESTAMP_PIDR7: c_uint = 0xC49FDC;
pub const mmPSOC_TIMESTAMP_PIDR0: c_uint = 0xC49FE0;
pub const mmPSOC_TIMESTAMP_PIDR1: c_uint = 0xC49FE4;
pub const mmPSOC_TIMESTAMP_PIDR2: c_uint = 0xC49FE8;
pub const mmPSOC_TIMESTAMP_PIDR3: c_uint = 0xC49FEC;
pub const mmPSOC_TIMESTAMP_CIDR0: c_uint = 0xC49FF0;
pub const mmPSOC_TIMESTAMP_CIDR1: c_uint = 0xC49FF4;
pub const mmPSOC_TIMESTAMP_CIDR2: c_uint = 0xC49FF8;
pub const mmPSOC_TIMESTAMP_CIDR3: c_uint = 0xC49FFC;
