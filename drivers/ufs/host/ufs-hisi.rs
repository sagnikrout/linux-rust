//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ufs/host/ufs-hisi.h
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
// Copyright (c) 2017, HiSilicon. All rights reserved.
//
pub const HBRN8_POLL_TOUT_MS: c_int = 1000;
//
// ufs sysctrl specific define
//

//
// M-TX Configuration Attributes for Hixxxx
//
pub const MPHY_TX_FSM_STATE: c_uint = 0x41;
pub const TX_FSM_HIBERN8: c_uint = 0x1;
//
// Hixxxx UFS HC specific Registers
//
// AHIT - Auto-Hibernate Idle Timer
pub const UFS_AHIT_AH8ITV_MASK: c_uint = 0x3FF;
// REG UFS_REG_OCPTHRTL definition
pub const UFS_HCLKDIV_NORMAL_VALUE: c_uint = 0xE4;
// vendor specific pre-defined parameters
pub const SLOW: c_int = 1;
pub const FAST: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_hisi_host {
    pub hba: *mut ufs_hba,
    pub ufs_sys_ctrl: *mut void __iomem,
    pub rst: *mut reset_control,
    pub caps: u64,
    pub in_suspend: bool,
}

