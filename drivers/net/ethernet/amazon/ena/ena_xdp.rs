//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/amazon/ena/ena_xdp.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright 2015-2021 Amazon.com, Inc. or its affiliates. All rights reserved.
//

// The max MTU size is configured to be the ethernet frame size without
// the overhead of the ethernet header, which can have a VLAN header, and
// a frame check sequence (FCS).
// The buffer size we share with the device is defined to be ENA_PAGE_SIZE
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ENA_XDP_ACTIONS {
    ENA_XDP_PASS		= 0,
    ENA_XDP_TX		= BIT(0),
    ENA_XDP_REDIRECT	= BIT(1),
    ENA_XDP_DROP		= BIT(2)
}

extern "C" {
    pub fn ena_setup_and_create_all_xdp_queues(adapter: *mut ena_adapter) -> c_int;
}
extern "C" {
    pub fn ena_xdp_io_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn ena_xdp(netdev: *mut net_device, bpf: *mut netdev_bpf) -> c_int;
}
extern "C" {
    pub fn ena_xdp_register_rxq_info(rx_ring: *mut ena_ring) -> c_int;
}
extern "C" {
    pub fn ena_xdp_unregister_rxq_info(rx_ring: *mut ena_ring);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_xdp_errors_t {
    ENA_XDP_ALLOWED = 0,
    ENA_XDP_CURRENT_MTU_TOO_LARGE,
    ENA_XDP_NO_ENOUGH_QUEUES,
}

// Find xmit queue
// The XDP queues are shared between XDP_TX and XDP_REDIRECT
