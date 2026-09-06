//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/qcom/pmic_glink.h
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
// Copyright (c) 2022, Linaro Ltd
//
pub const PMIC_GLINK_OWNER_BATTMGR: c_int = 32778;
pub const PMIC_GLINK_OWNER_USBC: c_int = 32779;
pub const PMIC_GLINK_OWNER_USBC_PAN: c_int = 32780;
pub const PMIC_GLINK_REQ_RESP: c_int = 1;
pub const PMIC_GLINK_NOTIFY: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmic_glink_hdr {
    pub owner: __le32,
    pub type: __le32,
    pub opcode: __le32,
}

extern "C" {
    pub fn pmic_glink_send(client: *mut pmic_glink_client, data: *mut c_void, len: usize) -> c_int;
}
extern "C" {
    pub fn pmic_glink_client_register(client: *mut pmic_glink_client);
}
