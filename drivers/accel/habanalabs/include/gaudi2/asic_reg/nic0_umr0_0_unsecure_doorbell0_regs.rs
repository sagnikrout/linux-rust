//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/nic0_umr0_0_unsecure_doorbell0_regs.h
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
// Copyright 2016-2020 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// NIC0_UMR0_0_UNSECURE_DOORBELL0
// (Prototype: NIC_UNSEC_DBELL)
//
pub const mmNIC0_UMR0_0_UNSECURE_DOORBELL0_UNSECURE_DB_FIRST32: c_uint = 0x5400000;
pub const mmNIC0_UMR0_0_UNSECURE_DOORBELL0_UNSECURE_DB_SECOND32: c_uint = 0x5400004;
pub const mmNIC0_UMR0_0_UNSECURE_DOORBELL0_UNSECURE_DB_THIRD32: c_uint = 0x5400008;
pub const mmNIC0_UMR0_0_UNSECURE_DOORBELL0_UNSECURE_DB_FOURTH32: c_uint = 0x540000C;
