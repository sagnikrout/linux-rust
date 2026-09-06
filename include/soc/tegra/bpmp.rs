//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/tegra/bpmp.h
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
// Copyright (c) 2016, NVIDIA CORPORATION.  All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_bpmp_soc {
    pub offset: c_uint,
    pub count: c_uint,
    pub timeout: c_uint,
    pub cpu_rx: } cpu_tx, thread,,
    pub channels: },
    pub ops: *const tegra_bpmp_ops,
    pub num_resets: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_bpmp_mb_data {
    pub code: u32,
    pub flags: u32,
    pub data: [u8; MSG_DATA_MIN_SZ],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_bpmp_channel {
    pub bpmp: *mut tegra_bpmp,
    pub ib: iosys_map,
    pub ob: iosys_map,
    pub completion: completion,
    pub ivc: *mut tegra_ivc,
    pub index: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_bpmp_mrq {
    pub list: list_head,
    pub mrq: c_uint,
    pub handler: tegra_bpmp_mrq_handler_t,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_bpmp {
    pub soc: *const tegra_bpmp_soc,
    pub dev: *mut device,
    pub priv: *mut c_void,
    pub client: mbox_client,
    pub channel: *mut mbox_chan,
    pub mbox: },
    pub atomic_tx_lock: spinlock_t,
    pub threaded_channels: *mut *mut *mut tegra_bpmp_channel tx_channel, rx_channel,,
    pub allocated: *mut c_ulong,
    pub busy: *mut c_ulong,
    pub count: c_uint,
    pub lock: semaphore,
    pub threaded: },
    pub mrqs: list_head,
    pub lock: spinlock_t,
    pub clocks: *mut tegra_bpmp_clk,
    pub num_clocks: c_uint,
    pub rstc: reset_controller_dev,
    pub genpd: genpd_onecell_data,

    pub debugfs_mirror: *mut dentry,

    pub suspended: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_bpmp_message {
    pub mrq: c_uint,
    pub data: *const c_void,
    pub size: usize,
    pub tx: },
    pub data: *mut c_void,
    pub size: usize,
    pub ret: c_int,
    pub rx: },
    pub flags: c_ulong,
}

extern "C" {
    pub fn tegra_bpmp_put(bpmp: *mut tegra_bpmp);
}
extern "C" {
    pub fn tegra_bpmp_mrq_is_supported(bpmp: *mut tegra_bpmp, mrq: c_uint) -> bool;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

extern "C" {
    pub fn tegra_bpmp_handle_rx(bpmp: *mut tegra_bpmp);
}

extern "C" {
    pub fn tegra_bpmp_init_clocks(bpmp: *mut tegra_bpmp) -> c_int;
}

extern "C" {
    pub fn tegra_bpmp_init_resets(bpmp: *mut tegra_bpmp) -> c_int;
}

extern "C" {
    pub fn tegra_bpmp_init_powergates(bpmp: *mut tegra_bpmp) -> c_int;
}

extern "C" {
    pub fn tegra_bpmp_init_debugfs(bpmp: *mut tegra_bpmp) -> c_int;
}

