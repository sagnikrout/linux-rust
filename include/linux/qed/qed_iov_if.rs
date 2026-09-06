//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/qed/qed_iov_if.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//

// Structs used by PF to control and manipulate child VFs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_iov_hv_ops {
    pub num_vfs_param): *mut *mut *mut int (configure)(struct qed_dev cdev, int,
    pub vfid): *mut *mut *mut *mut int (set_mac) (struct qed_dev cdev, u8 mac, int,
    pub vfid): *mut *mut *mut int (set_vlan) (struct qed_dev cdev, u16 vid, int,
    pub ivi): *mut ifla_vf_info,
    pub link_state): c_int,
    pub val): *mut *mut *mut int (set_spoof) (struct qed_dev cdev, int vfid, bool,
    pub max_rate): u32 min_rate, u32,
    pub trust): *mut *mut *mut int (set_trust) (struct qed_dev cdev, int vfid, bool,
}
