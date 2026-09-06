//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/mscc/ocelot_sys.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Microsemi Ocelot Switch driver
//
// Copyright (c) 2017 Microsemi Corporation
//
pub const SYS_COUNT_RX_OCTETS_RSZ: c_uint = 0x4;
pub const SYS_COUNT_TX_OCTETS_RSZ: c_uint = 0x4;
pub const SYS_FRONT_PORT_MODE_RSZ: c_uint = 0x4;

pub const SYS_SW_STATUS_RSZ: c_uint = 0x4;

pub const SYS_REW_MAC_HIGH_CFG_RSZ: c_uint = 0x4;
pub const SYS_REW_MAC_LOW_CFG_RSZ: c_uint = 0x4;

pub const SYS_ATOP_RSZ: c_uint = 0x4;
pub const SYS_MAC_FC_CFG_RSZ: c_uint = 0x4;

pub const SYS_EVENTS_DIF_RSZ: c_uint = 0x4;

pub const SYS_CNT_GSZ: c_uint = 0x4;

