//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ti/icssg/icss_iep.h
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
// Texas Instruments ICSSG Industrial Ethernet Peripheral (IEP) Driver
//
// Copyright (C) 2023 Texas Instruments Incorporated - https://www.ti.com
//

//
// struct icss_iep_plat_data - Plat data to handle SoC variants
// @config: Regmap configuration data
// @reg_offs: register offsets to capture offset differences across SoCs
// @flags: Flags to represent IEP properties
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icss_iep_plat_data {
    pub config: *const regmap_config,
    pub reg_offs: [u32; ICSS_IEP_MAX_REGS],
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icss_iep {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub plat_data: *const icss_iep_plat_data,
    pub map: *mut regmap,
    pub client_np: *mut device_node,
    pub refclk_freq: c_ulong,
    pub /: *mut *mut int clk_tick_time; / one refclk tick time in ns,
    pub ptp_info: ptp_clock_info,
    pub ptp_clock: *mut ptp_clock,
    pub /: *mut *mut mutex ptp_clk_mutex; / PHC access serializer,
    pub def_inc: u32,
    pub slow_cmp_inc: i16,
    pub slow_cmp_count: u32,
    pub ops: *const icss_iep_clockops,
    pub clockops_data: *mut c_void,
    pub cycle_time_ns: u32,
    pub perout_enabled: u32,
    pub pps_enabled: bool,
    pub cap_cmp_irq: c_int,
    pub period: u64,
    pub latch_enable: u32,
    pub work: work_struct,
}

// Firmware specific clock operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icss_iep_clockops {
    pub ns): *mut *mut *mut void (settime)(void clockops_data, u64,
    pub delta): *mut *mut *mut void (adjtime)(void clockops_data, s64,
    pub sts): *mut *mut *mut u64 (gettime)(void clockops_data, struct ptp_system_timestamp,
    pub cmp): *mut u64,
    pub on): *mut *mut *mut int (extts_enable)(void clockops_data, u32 index, int,
}

extern "C" {
    pub fn icss_iep_put(iep: *mut icss_iep);
}
extern "C" {
    pub fn icss_iep_exit(iep: *mut icss_iep) -> c_int;
}
extern "C" {
    pub fn icss_iep_get_count_low(iep: *mut icss_iep) -> c_int;
}
extern "C" {
    pub fn icss_iep_get_count_hi(iep: *mut icss_iep) -> c_int;
}
extern "C" {
    pub fn icss_iep_get_ptp_clock_idx(iep: *mut icss_iep) -> c_int;
}
extern "C" {
    pub fn icss_iep_init_fw(iep: *mut icss_iep);
}
extern "C" {
    pub fn icss_iep_exit_fw(iep: *mut icss_iep);
}
