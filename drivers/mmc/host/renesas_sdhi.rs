//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/host/renesas_sdhi.h
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
// Renesas Mobile SDHI
//
// Copyright (C) 2017 Horms Solutions Ltd., Simon Horman
// Copyright (C) 2017-19 Renesas Electronics Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct renesas_sdhi_scc {
    pub /: *mut *mut unsigned long clk_rate; / clock rate for SDR104,
    pub /: *mut *mut u32 tap; / sampling clock position for SDR104/HS400 (8 TAP),
    pub /: *mut *mut u32 tap_hs400_4tap; / sampling clock position for HS400 (4 TAP),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct renesas_sdhi_of_data {
    pub tmio_flags: c_ulong,
    pub tmio_ocr_mask: u32,
    pub capabilities: c_ulong,
    pub capabilities2: c_ulong,
    pub dma_buswidth: dma_slave_buswidth,
    pub dma_rx_offset: dma_addr_t,
    pub bus_shift: c_uint,
    pub scc_offset: c_int,
    pub taps: *mut renesas_sdhi_scc,
    pub taps_num: c_int,
    pub max_blk_count: c_uint,
    pub max_segs: c_ushort,
    pub sdhi_flags: c_ulong,
}

pub const SDHI_CALIB_TABLE_MAX: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct renesas_sdhi_quirks {
    pub hs400_disabled: bool,
    pub hs400_4taps: bool,
    pub fixed_addr_mode: bool,
    pub dma_one_rx_only: bool,
    pub manual_tap_correction: bool,
    pub old_info1_layout: bool,
    pub hs400_bad_taps: u32,
    pub (*hs400_calib_table)[SDHI_CALIB_TABLE_MAX]: *const u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct renesas_sdhi_of_data_with_quirks {
    pub of_data: *const renesas_sdhi_of_data,
    pub quirks: *const renesas_sdhi_quirks,
}

// We want both end_flags to be set before we mark DMA as finished
pub const SDHI_DMA_END_FLAG_DMA: c_int = 0;
pub const SDHI_DMA_END_FLAG_ACCESS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct renesas_sdhi_dma {
    pub end_flags: c_ulong,
    pub dma_buswidth: dma_slave_buswidth,
    pub filter: dma_filter_fn,
    pub enable): *mut *mut *mut void (enable)(struct tmio_mmc_host host, bool,
    pub dma_dataend: completion,
    pub dma_complete: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct renesas_sdhi {
    pub clk: *mut clk,
    pub clkh: *mut clk,
    pub clk_cd: *mut clk,
    pub mmc_data: tmio_mmc_data,
    pub dma_priv: renesas_sdhi_dma,
    pub quirks: *const renesas_sdhi_quirks,
    pub pinctrl: *mut pinctrl,
    pub pins_uhs: *mut *mut pinctrl_state pins_default,,
    pub scc_ctl: *mut void __iomem,
    pub scc_tappos: u32,
    pub scc_tappos_hs400: u32,
    pub adjust_hs400_calib_table: *const u8,
    pub needs_adjust_hs400: bool,
    pub card_is_sdio: bool,
// Tuning values: 1 for success, 0 for failure
    pub BITS_PER_LONG): DECLARE_BITMAP(taps,,
// Sampling data comparison: 1 for match, 0 for mismatch
    pub BITS_PER_LONG): DECLARE_BITMAP(smpcmp,,
    pub tap_num: c_uint,
    pub tap_set: c_uint,
    pub rstc: *mut reset_control,
    pub host: *mut tmio_mmc_host,
    pub rdev: *mut regulator_dev,
}

extern "C" {
    pub fn renesas_sdhi_remove(pdev: *mut platform_device);
}
extern "C" {
    pub fn renesas_sdhi_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn renesas_sdhi_resume(dev: *mut device) -> c_int;
}
