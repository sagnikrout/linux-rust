//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/ahb.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2016 Qualcomm Atheros, Inc. All rights reserved.
// Copyright (c) 2015 The Linux Foundation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath10k_ahb {
    pub pdev: *mut platform_device,
    pub mem: *mut void __iomem,
    pub mem_len: c_ulong,
    pub gcc_mem: *mut void __iomem,
    pub tcsr_mem: *mut void __iomem,
    pub irq: c_int,
    pub cmd_clk: *mut clk,
    pub ref_clk: *mut clk,
    pub rtc_clk: *mut clk,
    pub core_cold_rst: *mut reset_control,
    pub radio_cold_rst: *mut reset_control,
    pub radio_warm_rst: *mut reset_control,
    pub radio_srif_rst: *mut reset_control,
    pub cpu_init_rst: *mut reset_control,
}

pub const ATH10K_GCC_REG_BASE: c_uint = 0x1800000;
pub const ATH10K_GCC_REG_SIZE: c_uint = 0x60000;
pub const ATH10K_TCSR_REG_BASE: c_uint = 0x1900000;
pub const ATH10K_TCSR_REG_SIZE: c_uint = 0x80000;
pub const ATH10K_AHB_GCC_FEPLL_PLL_DIV: c_uint = 0x2f020;
pub const ATH10K_AHB_WIFI_SCRATCH_5_REG: c_uint = 0x4f014;
pub const ATH10K_AHB_WLAN_CORE_ID_REG: c_uint = 0x82030;
pub const ATH10K_AHB_TCSR_WIFI0_GLB_CFG: c_uint = 0x49000;
pub const ATH10K_AHB_TCSR_WIFI1_GLB_CFG: c_uint = 0x49004;

pub const ATH10K_AHB_TCSR_WCSS0_HALTREQ: c_uint = 0x52000;
pub const ATH10K_AHB_TCSR_WCSS1_HALTREQ: c_uint = 0x52010;
pub const ATH10K_AHB_TCSR_WCSS0_HALTACK: c_uint = 0x52004;
pub const ATH10K_AHB_TCSR_WCSS1_HALTACK: c_uint = 0x52014;

pub const AHB_AXI_BUS_HALT_REQ: c_int = 1;
pub const AHB_AXI_BUS_HALT_ACK: c_int = 1;
pub const ATH10K_AHB_CORE_CTRL_CPU_INTR_MASK: c_int = 1;
extern "C" {
    pub fn ath10k_ahb_init() -> c_int;
}
extern "C" {
    pub fn ath10k_ahb_exit();
}

