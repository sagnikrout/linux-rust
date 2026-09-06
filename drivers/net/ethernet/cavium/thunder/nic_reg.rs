//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/thunder/nic_reg.h
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
//
// Copyright (C) 2015 Cavium, Inc.
//
pub const NIC_PF_REG_COUNT: c_int = 29573;
pub const NIC_VF_REG_COUNT: c_int = 249;
// Physical function register offsets

pub const UDP_GENEVE_PORT_NUM: c_uint = 0x17C1ULL;

pub const IPV6_PROT: c_uint = 0x86DDULL;
pub const IPV4_PROT: c_uint = 0x800ULL;
pub const ET_PROT: c_uint = 0x6558ULL;

pub const UDP_VXLAN_PORT_NUM: c_uint = 0x12B5;

pub const IPV6_PROT_DEF: c_uint = 0x2ULL;
pub const IPV4_PROT_DEF: c_uint = 0x1ULL;
pub const ET_PROT_DEF: c_uint = 0x3ULL;

// Virtual function register offsets

// Offsets within registers
pub const NIC_MSIX_VEC_SHIFT: c_int = 4;
pub const NIC_Q_NUM_SHIFT: c_int = 18;
pub const NIC_QS_ID_SHIFT: c_int = 21;
pub const NIC_VF_NUM_SHIFT: c_int = 21;
// Port kind configuration register
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkind_cfg {

    pub reserved_42_63:22: u64,
    pub /: *mut *mut u64 hdr_sl:5; / Header skip length,
    pub /: *mut *mut u64 rx_hdr:3; / TNS Receive header present,
    pub /: *mut *mut u64 lenerr_en:1;/ L2 length error check enable,
    pub reserved_32_32:1: u64,
    pub /: *mut *mut u64 maxlen:16; / Max frame size,
    pub /: *mut *mut u64 minlen:16; / Min frame size,

    pub minlen:16: u64,
    pub maxlen:16: u64,
    pub reserved_32_32:1: u64,
    pub lenerr_en:1: u64,
    pub rx_hdr:3: u64,
    pub hdr_sl:5: u64,
    pub reserved_42_63:22: u64,

}
