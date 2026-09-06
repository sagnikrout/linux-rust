//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxfw/mlxfw_mfa2_tlv.h
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
// Copyright (c) 2017-2019 Mellanox Technologies. All rights reserved

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxfw_mfa2_tlv {
    pub version: u8,
    pub type: u8,
    pub len: __be16,
    pub data: [u8; ],
    pub __packed: },
    pub NULL: return,
    pub ptr: return,
    pub tlv_top: *mut c_void,
    pub 1: *mut *mut tlv_top = (void ) tlv + be16_to_cpu(tlv->len) -,
    pub NULL: return,
    pub NULL: return,
    pub NULL: return,
    pub NULL: return,
    pub tlv->data: return,

    pub \: false);,

    pub \: true);,
