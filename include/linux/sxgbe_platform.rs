//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sxgbe_platform.h
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
// 10G controller driver for Samsung Exynos SoCs
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Siva Reddy Kallam <siva.kallam@samsung.com>
//

// MDC Clock Selection define
pub const SXGBE_CSR_100_150M: c_uint = 0x0	/* MDC = clk_scr_i/62 */;
pub const SXGBE_CSR_150_250M: c_uint = 0x1	/* MDC = clk_scr_i/102 */;
pub const SXGBE_CSR_250_300M: c_uint = 0x2	/* MDC = clk_scr_i/122 */;
pub const SXGBE_CSR_300_350M: c_uint = 0x3	/* MDC = clk_scr_i/142 */;
pub const SXGBE_CSR_350_400M: c_uint = 0x4	/* MDC = clk_scr_i/162 */;
pub const SXGBE_CSR_400_500M: c_uint = 0x5	/* MDC = clk_scr_i/202 */;
// Platfrom data for platform device structure's
// platform_data field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sxgbe_mdio_bus_data {
    pub phy_mask: c_uint,
    pub irqs: *mut c_int,
    pub probed_phy_irq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sxgbe_dma_cfg {
    pub pbl: c_int,
    pub fixed_burst: c_int,
    pub burst_map: c_int,
    pub adv_addr_mode: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sxgbe_plat_data {
    pub phy_bus_name: *mut c_char,
    pub bus_id: c_int,
    pub phy_addr: c_int,
    pub interface: phy_interface_t,
    pub mdio_bus_data: *mut sxgbe_mdio_bus_data,
    pub dma_cfg: *mut sxgbe_dma_cfg,
    pub clk_csr: c_int,
    pub pmt: c_int,
    pub force_sf_dma_mode: c_int,
    pub force_thresh_dma_mode: c_int,
    pub riwt_off: c_int,
}
