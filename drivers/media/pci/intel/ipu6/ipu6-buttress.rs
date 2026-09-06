//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/intel/ipu6/ipu6-buttress.h
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
// Copyright (C) 2013--2024 Intel Corporation

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_buttress_ctrl {
    pub pwr_sts_off: u32 freq_ctl, pwr_sts_shift, pwr_sts_mask, pwr_sts_on,,
    pub ratio: c_uint,
    pub qos_floor: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_buttress_ipc {
    pub send_complete: completion,
    pub recv_complete: completion,
    pub nack: u32,
    pub nack_mask: u32,
    pub recv_data: u32,
    pub csr_out: u32,
    pub csr_in: u32,
    pub db0_in: u32,
    pub db0_out: u32,
    pub data0_out: u32,
    pub data0_in: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_buttress {
    pub ipc_mutex: mutex power_mutex, auth_mutex, cons_mutex,,
    pub cse: ipu6_buttress_ipc,
    pub constraints: list_head,
    pub wdt_cached_value: u32,
    pub force_suspend: bool,
    pub ref_clk: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_ipc_buttress_bulk_msg {
    pub cmd: u32,
    pub expected_resp: u32,
    pub require_resp: bool,
    pub cmd_size: u8,
}

extern "C" {
    pub fn ipu6_buttress_get_secure_mode(isp: *mut ipu6_device) -> bool;
}
extern "C" {
    pub fn ipu6_buttress_authenticate(isp: *mut ipu6_device) -> c_int;
}
extern "C" {
    pub fn ipu6_buttress_reset_authentication(isp: *mut ipu6_device) -> c_int;
}
extern "C" {
    pub fn ipu6_buttress_auth_done(isp: *mut ipu6_device) -> bool;
}
extern "C" {
    pub fn ipu6_buttress_start_tsc_sync(isp: *mut ipu6_device) -> c_int;
}
extern "C" {
    pub fn ipu6_buttress_tsc_read(isp: *mut ipu6_device, val: *mut u64);
}
extern "C" {
    pub fn ipu6_buttress_tsc_ticks_to_ns(ticks: u64, isp: *const ipu6_device) -> u64;
}
extern "C" {
    pub fn ipu6_buttress_isr(irq: c_int, isp_ptr: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn ipu6_buttress_isr_threaded(irq: c_int, isp_ptr: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn ipu6_buttress_init(isp: *mut ipu6_device) -> c_int;
}
extern "C" {
    pub fn ipu6_buttress_exit(isp: *mut ipu6_device);
}
extern "C" {
    pub fn ipu6_buttress_restore(isp: *mut ipu6_device);
}
