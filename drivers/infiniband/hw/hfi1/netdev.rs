//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/netdev.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// Copyright(c) 2020 Intel Corporation.
//

//
// struct hfi1_netdev_rxq - Receive Queue for HFI
// IPoIB netdevices will be working on the rx abstraction.
// @napi: napi object
// @rx: ptr to netdev_rx
// @rcd:  ptr to receive context data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_netdev_rxq {
    pub napi: napi_struct,
    pub rx: *mut hfi1_netdev_rx,
    pub rcd: *mut hfi1_ctxtdata,
}

pub const HFI1_MAX_NETDEV_CTXTS: c_int = 8;
// Number of NETDEV RSM entries

//
// struct hfi1_netdev_rx: data required to setup and run HFI netdev.
// @rx_napi:	the dummy netdevice to support "polling" the receive contexts
// @dd:		hfi1_devdata
// @rxq:	pointer to dummy netdev receive queues.
// @num_rx_q:	number of receive queues
// @rmt_index:	first free index in RMT Array
// @msix_start: first free MSI-X interrupt vector.
// @dev_tbl:	netdev table for unique identifier IPoIb VLANs.
// @enabled:	atomic counter of netdevs enabling receive queues.
// When 0 NAPI will be disabled.
// @netdevs:	atomic counter of netdevs using dummy netdev.
// When 0 receive queues will be freed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_netdev_rx {
    pub rx_napi: *mut net_device,
    pub dd: *mut hfi1_devdata,
    pub rxq: *mut hfi1_netdev_rxq,
    pub num_rx_q: c_int,
    pub rmt_start: c_int,
    pub dev_tbl: xarray,
// count of enabled napi polls
    pub enabled: core::sync::atomic::AtomicI32,
// count of netdevs on top
    pub netdevs: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn hfi1_netdev_enable_queues(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn hfi1_netdev_disable_queues(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn hfi1_netdev_rx_init(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn hfi1_netdev_rx_destroy(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn hfi1_alloc_rx(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn hfi1_free_rx(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn hfi1_netdev_add_data(dd: *mut hfi1_devdata, id: c_int, data: *mut c_void) -> c_int;
}
// chip.c
extern "C" {
    pub fn hfi1_netdev_rx_napi(napi: *mut napi_struct, budget: c_int) -> c_int;
}
