//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ufs/host/ufs-mediatek-sip.h
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
// Copyright (C) 2022 MediaTek Inc.
//

//
// SiP (Slicon Partner) commands
//

//
// Multi-VCC by Numbering
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_mtk_vcc_num {
    UFS_VCC_NONE = 0,
    UFS_VCC_1,
    UFS_VCC_2,
    UFS_VCC_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_mtk_mphy_op {
    UFS_MPHY_BACKUP = 0,
    UFS_MPHY_RESTORE
}

//
// SMC call wrapper function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_mtk_smc_arg {
    pub cmd: c_ulong,
    pub res: *mut arm_smccc_res,
    pub v1: c_ulong,
    pub v2: c_ulong,
    pub v3: c_ulong,
    pub v4: c_ulong,
    pub v5: c_ulong,
    pub v6: c_ulong,
    pub v7: c_ulong,
}

// Sip kernel interface

