//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/gpmc-omap.h
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
// OMAP GPMC Platform data
//
// Copyright (C) 2014 Texas Instruments, Inc. - https://www.ti.com
// Roger Quadros <rogerq@ti.com>
//
// Maximum Number of Chip Selects
pub const GPMC_CS_NUM: c_int = 8;
// bool type time settings
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpmc_bool_timings {
    pub cycle2cyclediffcsen: bool,
    pub cycle2cyclesamecsen: bool,
    pub we_extra_delay: bool,
    pub oe_extra_delay: bool,
    pub adv_extra_delay: bool,
    pub cs_extra_delay: bool,
    pub time_para_granularity: bool,
}

//
// Note that all values in this struct are in nanoseconds except sync_clk
// (which is in picoseconds), while the register values are in gpmc_fck cycles.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpmc_timings {
// Minimum clock period for synchronous mode (in picoseconds)
    pub sync_clk: u32,
// Chip-select signal timings corresponding to GPMC_CS_CONFIG2
    pub /: *mut *mut u32 cs_on; / Assertion time,
    pub /: *mut *mut u32 cs_rd_off; / Read deassertion time,
    pub /: *mut *mut u32 cs_wr_off; / Write deassertion time,
// ADV signal timings corresponding to GPMC_CONFIG3
    pub /: *mut *mut u32 adv_on; / Assertion time,
    pub /: *mut *mut u32 adv_rd_off; / Read deassertion time,
    pub /: *mut *mut u32 adv_wr_off; / Write deassertion time,
    pub /: *mut *mut u32 adv_aad_mux_on; / ADV assertion time for AAD,
    pub /: *mut *mut u32 adv_aad_mux_rd_off; / ADV read deassertion time for AAD,
    pub /: *mut *mut u32 adv_aad_mux_wr_off; / ADV write deassertion time for AAD,
// WE signals timings corresponding to GPMC_CONFIG4
    pub /: *mut *mut u32 we_on; / WE assertion time,
    pub /: *mut *mut u32 we_off; / WE deassertion time,
// OE signals timings corresponding to GPMC_CONFIG4
    pub /: *mut *mut u32 oe_on; / OE assertion time,
    pub /: *mut *mut u32 oe_off; / OE deassertion time,
    pub /: *mut *mut u32 oe_aad_mux_on; / OE assertion time for AAD,
    pub /: *mut *mut u32 oe_aad_mux_off; / OE deassertion time for AAD,
// Access time and cycle time timings corresponding to GPMC_CONFIG5
    pub /: *mut *mut u32 page_burst_access; / Multiple access word delay,
    pub /: *mut *mut u32 access; / Start-cycle to first data valid delay,
    pub /: *mut *mut u32 rd_cycle; / Total read cycle time,
    pub /: *mut *mut u32 wr_cycle; / Total write cycle time,
    pub bus_turnaround: u32,
    pub cycle2cycle_delay: u32,
    pub wait_monitoring: u32,
    pub clk_activation: u32,
// The following are only on OMAP3430
    pub /: *mut *mut u32 wr_access; / WRACCESSTIME,
    pub /: *mut *mut u32 wr_data_mux_bus; / WRDATAONADMUXBUS,
    pub bool_timings: gpmc_bool_timings,
}

// Device timings in picoseconds
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpmc_device_timings {
    pub /: *mut *mut u32 t_ceasu; / address setup to CS valid,
    pub /: *mut *mut u32 t_avdasu; / address setup to ADV valid,
// XXX: try to combine t_avdp_r & t_avdp_w. Issue is
// of tusb using these timings even for sync whilst
// ideally for adv_rd/(wr)_off it should have considered
// t_avdh instead. This indirectly necessitates r/w
// variations of t_avdp as it is possible to have one
// sync & other async
//
    pub /: *mut *mut u32 t_avdp_r; / ADV low time (what about t_cer ?),
    pub t_avdp_w: u32,
    pub /: *mut *mut u32 t_aavdh; / address hold time,
    pub /: *mut *mut u32 t_oeasu; / address setup to OE valid,
    pub /: *mut *mut u32 t_aa; / access time from ADV assertion,
    pub /: *mut *mut u32 t_iaa; / initial access time,
    pub /: *mut *mut u32 t_oe; / access time from OE assertion,
    pub /: *mut *mut u32 t_ce; / access time from CS asertion,
    pub /: *mut *mut u32 t_rd_cycle; / read cycle time,
    pub /: *mut *mut u32 t_cez_r; / read CS deassertion to high Z,
    pub /: *mut *mut u32 t_cez_w; / write CS deassertion to high Z,
    pub /: *mut *mut u32 t_oez; / OE deassertion to high Z,
    pub /: *mut *mut u32 t_weasu; / address setup to WE valid,
    pub /: *mut *mut u32 t_wpl; / write assertion time,
    pub /: *mut *mut u32 t_wph; / write deassertion time,
    pub /: *mut *mut u32 t_wr_cycle; / write cycle time,
    pub clk: u32,
    pub /: *mut *mut u32 t_bacc; / burst access valid clock to output delay,
    pub /: *mut *mut u32 t_ces; / CS setup time to clk,
    pub /: *mut *mut u32 t_avds; / ADV setup time to clk,
    pub /: *mut *mut u32 t_avdh; / ADV hold time from clk,
    pub /: *mut *mut u32 t_ach; / address hold time from clk,
    pub /: *mut *mut u32 t_rdyo; / clk to ready valid,
    pub /: *mut *mut u32 t_ce_rdyz; / XXX: description ?, or use t_cez instead,
    pub /: *mut *mut u32 t_ce_avd; / CS on to ADV on delay,
// XXX: check the possibility of combining
// cyc_aavhd_oe & cyc_aavdh_we
//
    pub /: *mut *mut u8 cyc_aavdh_oe;/ read address hold time in cycles,
    pub /: *mut *mut u8 cyc_aavdh_we;/ write address hold time in cycles,
    pub /: *mut *mut u8 cyc_oe; / access time from OE assertion in cycles,
    pub /: *mut *mut u8 cyc_wpl; / write deassertion time in cycles,
    pub /: *mut *mut u32 cyc_iaa; / initial access time in cycles,
// extra delays
    pub ce_xdelay: bool,
    pub avd_xdelay: bool,
    pub oe_xdelay: bool,
    pub we_xdelay: bool,
}

// Wait pin polarity values

pub const GPMC_WAITPINPOLARITY_ACTIVE_LOW: c_int = 0;
pub const GPMC_WAITPINPOLARITY_ACTIVE_HIGH: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpmc_settings {
    pub /: *mut *mut bool burst_wrap; / enables wrap bursting,
    pub /: *mut *mut bool burst_read; / enables read page/burst mode,
    pub /: *mut *mut bool burst_write; / enables write page/burst mode,
    pub /: *mut *mut bool device_nand; / device is NAND,
    pub /: *mut *mut bool sync_read; / enables synchronous reads,
    pub /: *mut *mut bool sync_write; / enables synchronous writes,
    pub /: *mut *mut bool wait_on_read; / monitor wait on reads,
    pub /: *mut *mut bool wait_on_write; / monitor wait on writes,
    pub /: *mut *mut u32 burst_len; / page/burst length,
    pub /: *mut *mut u32 device_width; / device bus width (8 or 16 bit),
    pub /: *mut *mut u32 mux_add_data; / multiplex address & data,
    pub /: *mut *mut u32 wait_pin; / wait-pin to be used,
    pub wait_pin_polarity: u32,
}

// Data for each chip select
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpmc_omap_cs_data {
    pub /: *mut *mut bool valid; / data is valid,
    pub /: *mut *mut bool is_nand; / device within this CS is NAND,
    pub settings: *mut gpmc_settings,
    pub device_timings: *mut gpmc_device_timings,
    pub gpmc_timings: *mut gpmc_timings,
    pub /: *mut *mut *mut platform_device pdev; / device within this CS region,
    pub pdata_size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpmc_omap_platform_data {
    pub cs: [gpmc_omap_cs_data; GPMC_CS_NUM],
}
