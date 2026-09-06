//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/host/sdhci-pltfm.h
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
// Copyright 2010 MontaVista Software, LLC.
//
// Author: Anton Vorontsov <avorontsov@ru.mvista.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdhci_pltfm_data {
    pub ops: *const sdhci_ops,
    pub quirks: c_uint,
    pub quirks2: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdhci_pltfm_host {
    pub clk: *mut clk,
// migrate from sdhci_of_host
    pub clock: c_uint,
    pub xfer_mode_shadow: u16,
    pub ____cacheline_aligned: unsigned long private[],
}

//
// These accessors are designed for big endian hosts doing I/O to
// little endian controllers incorporating a 32-bit hardware byte swapper.
//
extern "C" {
    pub fn in_be32(reg: host->ioaddr +) -> return;
}
extern "C" {
    pub fn in_be16(0x2): host->ioaddr + (reg ^) -> return;
}
extern "C" {
    pub fn in_8(0x3): host->ioaddr + (reg ^) -> return;
}
//
// Postpone this write, we must do it together with a
// command write that is down below.
//

extern "C" {
    pub fn sdhci_get_property(pdev: *mut platform_device);
}
extern "C" {
    pub fn sdhci_get_property(_arg: pdev) -> return;
}
extern "C" {
    pub fn sdhci_pltfm_remove(pdev: *mut platform_device);
}
extern "C" {
    pub fn sdhci_pltfm_clk_get_max_clock(host: *mut sdhci_host) -> c_uint;
}

extern "C" {
    pub fn sdhci_pltfm_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn sdhci_pltfm_resume(dev: *mut device) -> c_int;
}

