//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/boot/dts/xilinx/xlnx-versal-net-clk.h
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
// Copyright (C) 2022, Xilinx, Inc.
// Copyright (C) 2022 - 2026, Advanced Micro Devices, Inc.
//

pub const CAN0_REF_2X: c_uint = 0x9e;
pub const CAN1_REF_2X: c_uint = 0xac;
pub const FPD_WWDT0: c_uint = 0xb5;
pub const FPD_WWDT1: c_uint = 0xb6;
pub const FPD_WWDT2: c_uint = 0xb7;
pub const FPD_WWDT3: c_uint = 0xb8;
pub const LPD_WWDT0: c_uint = 0xb9;
pub const LPD_WWDT1: c_uint = 0xba;
pub const ACPU_0: c_uint = 0x98;
pub const ACPU_1: c_uint = 0x9b;
pub const ACPU_2: c_uint = 0x9a;
pub const ACPU_3: c_uint = 0x99;
pub const I3C0_REF: c_uint = 0x9d;
pub const I3C1_REF: c_uint = 0x9f;
pub const USB1_BUS_REF: c_uint = 0xae;
pub const LPD_WWDT: c_uint = 0xad;
// Remove Versal specific node IDs

