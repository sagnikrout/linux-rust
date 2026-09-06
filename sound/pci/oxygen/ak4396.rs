//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/oxygen/ak4396.h
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

// Macro flag: #define AK4396_H_INCLUDED
pub const AK4396_WRITE: c_uint = 0x2000;
pub const AK4396_CONTROL_1: c_int = 0;
pub const AK4396_CONTROL_2: c_int = 1;
pub const AK4396_CONTROL_3: c_int = 2;
pub const AK4396_LCH_ATT: c_int = 3;
pub const AK4396_RCH_ATT: c_int = 4;
// control 1
pub const AK4396_RSTN: c_uint = 0x01;
pub const AK4396_DIF_MASK: c_uint = 0x0e;
pub const AK4396_DIF_16_LSB: c_uint = 0x00;
pub const AK4396_DIF_20_LSB: c_uint = 0x02;
pub const AK4396_DIF_24_MSB: c_uint = 0x04;
pub const AK4396_DIF_24_I2S: c_uint = 0x06;
pub const AK4396_DIF_24_LSB: c_uint = 0x08;
pub const AK4396_ACKS: c_uint = 0x80;
// control 2
pub const AK4396_SMUTE: c_uint = 0x01;
pub const AK4396_DEM_MASK: c_uint = 0x06;
pub const AK4396_DEM_441: c_uint = 0x00;
pub const AK4396_DEM_OFF: c_uint = 0x02;
pub const AK4396_DEM_48: c_uint = 0x04;
pub const AK4396_DEM_32: c_uint = 0x06;
pub const AK4396_DFS_MASK: c_uint = 0x18;
pub const AK4396_DFS_NORMAL: c_uint = 0x00;
pub const AK4396_DFS_DOUBLE: c_uint = 0x08;
pub const AK4396_DFS_QUAD: c_uint = 0x10;
pub const AK4396_SLOW: c_uint = 0x20;
pub const AK4396_DZFM: c_uint = 0x40;
pub const AK4396_DZFE: c_uint = 0x80;
// control 3
pub const AK4396_DZFB: c_uint = 0x04;
pub const AK4396_DCKB: c_uint = 0x10;
pub const AK4396_DCKS: c_uint = 0x20;
pub const AK4396_DSDM: c_uint = 0x40;
pub const AK4396_D_P_MASK: c_uint = 0x80;
pub const AK4396_PCM: c_uint = 0x00;
pub const AK4396_DSD: c_uint = 0x80;
