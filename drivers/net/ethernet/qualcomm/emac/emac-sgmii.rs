//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qualcomm/emac/emac-sgmii.h
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
// Copyright (c) 2015-2016, The Linux Foundation. All rights reserved.
//
// emac_sgmii - internal emac phy
// @init initialization function
// @open called when the driver is opened
// @close called when the driver is closed
// @link_change called when the link state changes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgmii_ops {
    pub adpt): *mut *mut int (init)(struct emac_adapter,
    pub adpt): *mut *mut int (open)(struct emac_adapter,
    pub adpt): *mut *mut void (close)(struct emac_adapter,
    pub link_state): *mut *mut *mut int (link_change)(struct emac_adapter adpt, bool,
    pub adpt): *mut *mut void (reset)(struct emac_adapter,
}

// emac_sgmii - internal emac phy
// @base base address
// @digital per-lane digital block
// @irq the interrupt number
// @decode_error_count reference count of consecutive decode errors
// @sgmii_ops sgmii ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_sgmii {
    pub base: *mut void __iomem,
    pub digital: *mut void __iomem,
    pub irq: c_uint,
    pub decode_error_count: core::sync::atomic::AtomicI32,
    pub sgmii_ops: *mut sgmii_ops,
}

extern "C" {
    pub fn emac_sgmii_config(pdev: *mut platform_device, adpt: *mut emac_adapter) -> c_int;
}
extern "C" {
    pub fn emac_sgmii_init_fsm9900(adpt: *mut emac_adapter) -> c_int;
}
extern "C" {
    pub fn emac_sgmii_init_qdf2432(adpt: *mut emac_adapter) -> c_int;
}
extern "C" {
    pub fn emac_sgmii_init_qdf2400(adpt: *mut emac_adapter) -> c_int;
}
extern "C" {
    pub fn emac_sgmii_init(adpt: *mut emac_adapter) -> c_int;
}
extern "C" {
    pub fn emac_sgmii_open(adpt: *mut emac_adapter) -> c_int;
}
extern "C" {
    pub fn emac_sgmii_close(adpt: *mut emac_adapter);
}
extern "C" {
    pub fn emac_sgmii_link_change(adpt: *mut emac_adapter, link_state: bool) -> c_int;
}
extern "C" {
    pub fn emac_sgmii_reset(adpt: *mut emac_adapter);
}
