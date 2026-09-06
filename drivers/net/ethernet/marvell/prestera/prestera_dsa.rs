//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/prestera/prestera_dsa.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Copyright (c) 2020 Marvell International Ltd. All rights reserved.

pub const PRESTERA_DSA_HLEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prestera_dsa_cmd {
// DSA command is "To CPU"
    PRESTERA_DSA_CMD_TO_CPU = 0,

// DSA command is "From CPU"
    PRESTERA_DSA_CMD_FROM_CPU,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_dsa_vlan {
    pub vid: u16,
    pub vpt: u8,
    pub cfi_bit: u8,
    pub is_tagged: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_dsa {
    pub vlan: prestera_dsa_vlan,
    pub hw_dev_num: u32,
    pub port_num: u32,
    pub cpu_code: u8,
}

extern "C" {
    pub fn prestera_dsa_parse(dsa: *mut prestera_dsa, dsa_buf: *const u8) -> c_int;
}
extern "C" {
    pub fn prestera_dsa_build(dsa: *const prestera_dsa, dsa_buf: *mut u8) -> c_int;
}
