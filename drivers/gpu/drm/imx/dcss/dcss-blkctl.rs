//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/imx/dcss/dcss-blkctl.c
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
// Copyright 2019 NXP.
//

pub const DCSS_BLKCTL_RESET_CTRL: c_uint = 0x00;

pub const DCSS_BLKCTL_CONTROL0: c_uint = 0x10;

pub const DISPMIX_REFCLK_SEL_POS: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcss_blkctl {
    pub dcss: *mut dcss_dev,
    pub base_reg: *mut void __iomem,
}

#[no_mangle]
pub unsafe extern "C" fn dcss_blkctl_cfg(blkctl: *mut dcss_blkctl) {
    void dcss_blkctl_cfg(struct dcss_blkctl *blkctl)
    {
    if (blkctl.dcss.hdmi_output)
    dcss_writel(0, blkctl.base_reg + DCSS_BLKCTL_CONTROL0);
    else
    dcss_writel(DISPMIX_PIXCLK_SEL,
    blkctl.base_reg + DCSS_BLKCTL_CONTROL0);
    dcss_set(B_CLK_RESETN | APB_CLK_RESETN | P_CLK_RESETN | RTR_CLK_RESETN,
    blkctl.base_reg + DCSS_BLKCTL_RESET_CTRL);
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_blkctl_init(dcss: *mut dcss_dev, blkctl_base: c_ulong) -> c_int {
    int dcss_blkctl_init(struct dcss_dev *dcss, unsigned long blkctl_base)
    {
    struct dcss_blkctl *blkctl;
    blkctl = devm_kzalloc(dcss.dev, sizeof(*blkctl), GFP_KERNEL);
    if (!blkctl)
    return -ENOMEM;
    blkctl.base_reg = devm_ioremap(dcss.dev, blkctl_base, SZ_4K);
    if (!blkctl.base_reg) {
    dev_err(dcss.dev, "unable to remap BLK CTRL base\n");
    return -ENOMEM;
    }
    dcss.blkctl = blkctl;
    blkctl.dcss = dcss;
    dcss_blkctl_cfg(blkctl);
    return 0;
    }
