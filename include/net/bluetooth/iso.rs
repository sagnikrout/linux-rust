//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/bluetooth/iso.h
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
// BlueZ - Bluetooth protocol stack for Linux
//
// Copyright (C) 2022 Intel Corporation
//
// ISO defaults
pub const ISO_DEFAULT_MTU: c_int = 251;
pub const ISO_MAX_NUM_BIS: c_uint = 0x1f;
// ISO socket broadcast address
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_iso_bc {
    pub bc_bdaddr: bdaddr_t,
    pub bc_bdaddr_type: __u8,
    pub bc_sid: __u8,
    pub bc_num_bis: __u8,
    pub bc_bis: [__u8; ISO_MAX_NUM_BIS],
}

// ISO socket address
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_iso {
    pub iso_family: sa_family_t,
    pub iso_bdaddr: bdaddr_t,
    pub iso_bdaddr_type: __u8,
    pub iso_bc: [sockaddr_iso_bc; ],
}
