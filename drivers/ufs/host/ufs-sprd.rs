//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ufs/host/ufs-sprd.h
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
// UNISOC UFS Host Controller driver
//
// Copyright (C) 2022 Unisoc, Inc.
// Author: Zhe Wang <zhe.wang1@unisoc.com>
//
// Vendor specific attributes
pub const RXSQCONTROL: c_uint = 0x8009;
pub const CBRATESEL: c_uint = 0x8114;
pub const CBCREGADDRLSB: c_uint = 0x8116;
pub const CBCREGADDRMSB: c_uint = 0x8117;
pub const CBCREGWRLSB: c_uint = 0x8118;
pub const CBCREGWRMSB: c_uint = 0x8119;
pub const CBCREGRDWRSEL: c_uint = 0x811C;
pub const CBCRCTRL: c_uint = 0x811F;
pub const CBREFCLKCTRL2: c_uint = 0x8132;
pub const VS_MPHYDISABLE: c_uint = 0xD0C1;
pub const APB_UFSDEV_REG: c_uint = 0xCE8;
pub const APB_UFSDEV_REFCLK_EN: c_uint = 0x2;
pub const APB_USB31PLL_CTRL: c_uint = 0xCFC;
pub const APB_USB31PLLV_REF2MPHY: c_uint = 0x1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SPRD_UFS_RST_INDEX {
    SPRD_UFSHCI_SOFT_RST,
    SPRD_UFS_DEV_RST,

    SPRD_UFS_RST_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SPRD_UFS_SYSCON_INDEX {
    SPRD_UFS_ANLG,
    SPRD_UFS_AON_APB,

    SPRD_UFS_SYSCON_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SPRD_UFS_VREG_INDEX {
    SPRD_UFS_VDD_MPHY,

    SPRD_UFS_VREG_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_sprd_rst {
    pub name: *const c_char,
    pub rc: *mut reset_control,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_sprd_syscon {
    pub name: *const c_char,
    pub regmap: *mut regmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_sprd_vreg {
    pub name: *const c_char,
    pub vreg: *mut regulator,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_sprd_priv {
    pub rci: [ufs_sprd_rst; SPRD_UFS_RST_MAX],
    pub sysci: [ufs_sprd_syscon; SPRD_UFS_SYSCON_MAX],
    pub vregi: [ufs_sprd_vreg; SPRD_UFS_VREG_MAX],
    pub ufs_hba_sprd_vops: ufs_hba_variant_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_sprd_host {
    pub hba: *mut ufs_hba,
    pub priv: *mut ufs_sprd_priv,
    pub ufs_dbg_mmio: *mut void __iomem,
    pub unipro_ver: ufs_unipro_ver,
}
