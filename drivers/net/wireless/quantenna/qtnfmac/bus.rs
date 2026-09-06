//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/quantenna/qtnfmac/bus.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2015 Quantenna Communications. All rights reserved.

pub const QTNF_MAX_MAC: c_int = 3;
pub const HBM_FRAME_META_MAGIC_PATTERN_S: c_uint = 0xAB;
pub const HBM_FRAME_META_MAGIC_PATTERN_E: c_uint = 0xBA;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_frame_meta_info {
    pub magic_s: u8,
    pub ifidx: u8,
    pub macid: u8,
    pub magic_e: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qtnf_fw_state {
    QTNF_FW_STATE_DETACHED,
    QTNF_FW_STATE_BOOT_DONE,
    QTNF_FW_STATE_ACTIVE,
    QTNF_FW_STATE_RUNNING,
    QTNF_FW_STATE_DEAD,
}

    pub qtnf_bus: struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_bus_ops {
// mgmt methods
    pub ): *mut *mut int (preinit)(struct qtnf_bus,
    pub ): *mut *mut void (stop)(struct qtnf_bus,
// control path methods
    pub ): *mut *mut *mut int (control_tx)(struct qtnf_bus , struct sk_buff,
// data xfer methods
    pub vifid): unsigned int macid, unsigned int,
    pub ): *mut *mut *mut void (data_tx_timeout)(struct qtnf_bus , struct net_device,
    pub use_meta): *mut *mut *mut void (data_tx_use_meta_set)(struct qtnf_bus bus, bool,
    pub ): *mut *mut void (data_rx_start)(struct qtnf_bus,
    pub ): *mut *mut void (data_rx_stop)(struct qtnf_bus,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_bus {
    pub dev: *mut device,
    pub fw_state: qtnf_fw_state,
    pub chip: u32,
    pub chiprev: u32,
    pub bus_ops: *mut qtnf_bus_ops,
    pub mac: [*mut qtnf_wmac; QTNF_MAX_MAC],
    pub trans: qtnf_qlink_transport,
    pub hw_info: qtnf_hw_info,
    pub mux_napi: napi_struct,
    pub mux_dev: *mut net_device,
    pub workqueue: *mut workqueue_struct,
    pub hprio_workqueue: *mut workqueue_struct,
    pub fw_work: work_struct,
    pub event_work: work_struct,
    pub /: *mut *mut mutex bus_lock; / lock during command/event processing,
    pub dbg_dir: *mut dentry,
    pub netdev_nb: notifier_block,
    pub hw_id: [u8; ETH_ALEN],
// bus private data
    pub )): *mut char bus_priv[] __aligned(sizeof(void,
}

// callback wrappers
// interface functions from common layer
extern "C" {
    pub fn qtnf_core_attach(bus: *mut qtnf_bus) -> c_int;
}
extern "C" {
    pub fn qtnf_core_detach(bus: *mut qtnf_bus);
}
