//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/clk/tegra.h
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
// Copyright (c) 2012-2020, NVIDIA CORPORATION.  All rights reserved.
//

//
// Tegra CPU clock and reset control ops
//
// wait_for_reset:
// keep waiting until the CPU in reset state
// put_in_reset:
// put the CPU in reset state
// out_of_reset:
// release the CPU from reset state
// enable_clock:
// CPU clock un-gate
// disable_clock:
// CPU clock gate
// rail_off_ready:
// CPU is ready for rail off
// suspend:
// save the clock settings when CPU go into low-power state
// resume:
// restore the clock settings when CPU exit low-power state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_cpu_car_ops {
    pub cpu): *mut *mut void (wait_for_reset)(u32,
    pub cpu): *mut *mut void (put_in_reset)(u32,
    pub cpu): *mut *mut void (out_of_reset)(u32,
    pub cpu): *mut *mut void (enable_clock)(u32,
    pub cpu): *mut *mut void (disable_clock)(u32,

    pub (*rail_off_ready)(void): *mut bool,
    pub (*suspend)(void): *mut c_void,
    pub (*resume)(void): *mut c_void,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra210_clk_emc_config {
    pub rate: c_ulong,
    pub same_freq: bool,
    pub value: u32,
    pub parent_rate: c_ulong,
    pub parent: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra210_clk_emc_provider {
    pub owner: *mut module,
    pub dev: *mut device,
    pub configs: *mut tegra210_clk_emc_config,
    pub num_configs: c_uint,
    pub config): *const tegra210_clk_emc_config,
}

extern "C" {
    pub fn tegra20_clk_prepare_emc_mc_same_freq(emc_clk: *mut clk, same: bool) -> c_int;
}

extern "C" {
    pub fn tegra210_plle_hw_sequence_start() -> c_int;
}
extern "C" {
    pub fn tegra210_plle_hw_sequence_is_enabled() -> bool;
}
extern "C" {
    pub fn tegra210_xusb_pll_hw_control_enable();
}
extern "C" {
    pub fn tegra210_xusb_pll_hw_sequence_start();
}
extern "C" {
    pub fn tegra210_sata_pll_hw_control_enable();
}
extern "C" {
    pub fn tegra210_sata_pll_hw_sequence_start();
}
extern "C" {
    pub fn tegra210_set_sata_pll_seq_sw(state: bool);
}
extern "C" {
    pub fn tegra210_put_utmipll_in_iddq();
}
extern "C" {
    pub fn tegra210_put_utmipll_out_iddq();
}
extern "C" {
    pub fn tegra210_clk_handle_mbist_war(id: c_uint) -> c_int;
}
extern "C" {
    pub fn tegra210_clk_emc_dll_enable(flag: bool);
}
extern "C" {
    pub fn tegra210_clk_emc_dll_update_setting(emc_dll_src_value: u32);
}
extern "C" {
    pub fn tegra210_clk_emc_update_setting(emc_src_value: u32);
}
extern "C" {
    pub fn tegra210_clk_emc_detach(clk: *mut clk);
}

