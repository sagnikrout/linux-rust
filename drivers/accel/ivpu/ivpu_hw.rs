//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ivpu/ivpu_hw.h
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
// Copyright (C) 2020-2026 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_addr_range {
    pub start: resource_size_t,
    pub end: resource_size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ivpu_hw_info {
    pub irq): *mut *mut *mut bool (btrs_irq_handler)(struct ivpu_device vdev, int,
    pub irq): *mut *mut *mut bool (ip_irq_handler)(struct ivpu_device vdev, int,
    pub irq: },
    pub runtime: ivpu_addr_range,
    pub global: ivpu_addr_range,
    pub user: ivpu_addr_range,
    pub shave: ivpu_addr_range,
    pub dma: ivpu_addr_range,
    pub ranges: },
// Hardware min and max pll ratio
    pub min_ratio: u8,
    pub max_ratio: u8,
//
// Pll ratio for the efficiency frequency. The VPU has optimum
// performance to power ratio at this frequency.
//
    pub pn_ratio: u8,
// Pll ratios configured via sysfs interface
    pub cfg_min_ratio: u8,
    pub cfg_max_ratio: u8,
    pub profiling_freq: u32,
    pub pll: },
    pub grace_period: [u32; VPU_HWS_NUM_PRIORITY_BANDS],
    pub process_quantum: [u32; VPU_HWS_NUM_PRIORITY_BANDS],
    pub process_grace_period: [u32; VPU_HWS_NUM_PRIORITY_BANDS],
    pub hws: },
    pub tile_fuse: u32,
    pub sku: u32,
    pub config: u16,
    pub dma_bits: c_int,
    pub d0i3_entry_host_ts: ktime_t,
    pub d0i3_entry_vpu_ts: u64,
    pub firewall_irq_counter: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn ivpu_hw_init(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_power_up(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_power_down(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_reset(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_boot_fw(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_profiling_freq_drive(vdev: *mut ivpu_device, enable: bool);
}
extern "C" {
    pub fn ivpu_irq_handlers_init(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_irq_enable(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_irq_disable(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_irq_handler(irq: c_int, ptr: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn ivpu_hw_uses_ecc_mca_signal(vdev: *mut ivpu_device) -> bool;
}
extern "C" {
    pub fn ivpu_hw_btrs_telemetry_offset_get(_arg: vdev) -> return;
}
extern "C" {
    pub fn ivpu_hw_btrs_telemetry_size_get(_arg: vdev) -> return;
}
extern "C" {
    pub fn ivpu_hw_btrs_telemetry_enable_get(_arg: vdev) -> return;
}
extern "C" {
    pub fn ivpu_hw_btrs_is_idle(_arg: vdev) -> return;
}
extern "C" {
    pub fn ivpu_hw_btrs_wait_for_idle(_arg: vdev) -> return;
}
extern "C" {
    pub fn ivpu_hw_ip_ipc_rx_addr_get(_arg: vdev) -> return;
}
extern "C" {
    pub fn ivpu_hw_ip_ipc_rx_count_get(_arg: vdev) -> return;
}
