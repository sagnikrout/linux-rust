//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/fjes/fjes.h
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
// FUJITSU Extended Socket Network Device driver
// Copyright (c) 2015 FUJITSU LIMITED
//

pub const FJES_MAX_QUEUES: c_int = 1;

// board specific private data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fjes_adapter {
    pub netdev: *mut net_device,
    pub plat_dev: *mut platform_device,
    pub napi: napi_struct,
    pub stats64: rtnl_link_stats64,
    pub tx_retry_count: c_uint,
    pub tx_start_jiffies: c_ulong,
    pub rx_last_jiffies: c_ulong,
    pub unset_rx_last: bool,
    pub force_close_task: work_struct,
    pub force_reset: bool,
    pub open_guard: bool,
    pub irq_registered: bool,
    pub txrx_wq: *mut workqueue_struct,
    pub control_wq: *mut workqueue_struct,
    pub tx_stall_task: work_struct,
    pub raise_intr_rxdata_task: work_struct,
    pub unshare_watch_task: work_struct,
    pub unshare_watch_bitmask: c_ulong,
    pub interrupt_watch_task: delayed_work,
    pub interrupt_watch_enable: bool,
    pub hw: fjes_hw,

    pub dbg_adapter: *mut dentry,

}

extern "C" {
    pub fn fjes_set_ethtool_ops(: *mut net_device);
}

extern "C" {
    pub fn fjes_dbg_adapter_init(adapter: *mut fjes_adapter);
}
extern "C" {
    pub fn fjes_dbg_adapter_exit(adapter: *mut fjes_adapter);
}
extern "C" {
    pub fn fjes_dbg_init();
}
extern "C" {
    pub fn fjes_dbg_exit();
}

