//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/hsmmc-omap.h
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
// MMC definitions for OMAP2
//
// Copyright (C) 2006 Nokia Corporation
//
// struct omap_hsmmc_dev_attr.flags possibilities
//
// OMAP_HSMMC_SUPPORTS_DUAL_VOLT: Some HSMMC controller instances can
// operate with either 1.8Vdc or 3.0Vdc card voltages; this flag
// should be set if this is the case.  See for example Section 22.5.3
// "MMC/SD/SDIO1 Bus Voltage Selection" of the OMAP34xx Multimedia
// Device Silicon Revision 3.1.x Revision ZR (July 2011) (SWPU223R).
//
// OMAP_HSMMC_BROKEN_MULTIBLOCK_READ: Multiple-block read transfers
// don't work correctly on some MMC controller instances on some
// OMAP3 SoCs; this flag should be set if this is the case.  See
// for example Advisory 2.1.1.128 "MMC: Multiple Block Read
// Operation Issue" in _OMAP3530/3525/3515/3503 Silicon Errata_
// Revision F (October 2010) (SPRZ278F).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_hsmmc_dev_attr {
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_hsmmc_platform_data {
// back-link to device
    pub dev: *mut device,
// set if your board has components or wiring that limits the
// maximum frequency on the MMC bus
    pub max_freq: c_uint,
// Integrating attributes from the omap_hwmod layer
    pub controller_flags: u8,
// Register offset deviation
    pub reg_offset: u16,
//
// 4/8 wires and any additional host capabilities
// need to OR'd all capabilities (ref. linux/mmc/host.h)
//
    pub /: *mut *mut u32 caps; / Used for the MMC driver on 2430 and later,
    pub /: *mut *mut u32 pm_caps; / PM capabilities of the mmc,
// nonremovable e.g. eMMC
    pub nonremovable:1: unsigned,
// eMMC does not handle power off when not in sleep state
    pub no_regulator_off_init:1: unsigned,
// we can put the features above into this variable

    pub features: unsigned,
// string specifying a particular variant of hardware
    pub version: *mut c_char,
    pub name: *const c_char,
    pub ocr_mask: u32,
}
