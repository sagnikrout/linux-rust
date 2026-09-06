//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ivpu/ivpu_hw_ip.h
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
// Copyright (C) 2020-2024 Intel Corporation
//

extern "C" {
    pub fn ivpu_hw_ip_host_ss_configure(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_ip_idle_gen_enable(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_ip_idle_gen_disable(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_ip_pwr_domain_enable(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_ip_host_ss_axi_enable(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_ip_top_noc_enable(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_ip_read_perf_timer_counter(vdev: *mut ivpu_device) -> u64;
}
extern "C" {
    pub fn ivpu_hw_ip_snoop_disable(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_ip_tbu_mmu_enable(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_ip_soc_cpu_boot(vdev: *mut ivpu_device) -> c_int;
}
extern "C" {
    pub fn ivpu_hw_ip_wdt_disable(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_ip_diagnose_failure(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_ip_ipc_rx_count_get(vdev: *mut ivpu_device) -> u32;
}
extern "C" {
    pub fn ivpu_hw_ip_irq_clear(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_ip_irq_handler_37xx(vdev: *mut ivpu_device, irq: c_int) -> bool;
}
extern "C" {
    pub fn ivpu_hw_ip_irq_handler_40xx(vdev: *mut ivpu_device, irq: c_int) -> bool;
}
extern "C" {
    pub fn ivpu_hw_ip_db_set(vdev: *mut ivpu_device, db_id: u32);
}
extern "C" {
    pub fn ivpu_hw_ip_ipc_rx_addr_get(vdev: *mut ivpu_device) -> u32;
}
extern "C" {
    pub fn ivpu_hw_ip_ipc_tx_set(vdev: *mut ivpu_device, vpu_addr: u32);
}
extern "C" {
    pub fn ivpu_hw_ip_irq_enable(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_ip_irq_disable(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_ip_fabric_req_override_enable_50xx(vdev: *mut ivpu_device);
}
extern "C" {
    pub fn ivpu_hw_ip_fabric_req_override_disable_50xx(vdev: *mut ivpu_device);
}
