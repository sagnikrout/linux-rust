//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/alibaba/eea/eea_adminq.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Driver for Alibaba Elastic Ethernet Adapter.
//
// Copyright (C) 2025 Alibaba Inc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_aq_cfg {
    pub rx_depth_max: __le32,
    pub rx_depth_def: __le32,
    pub tx_depth_max: __le32,
    pub tx_depth_def: __le32,
    pub max_tso_size: __le32,
    pub max_tso_segs: __le32,
    pub mac: [u8; ETH_ALEN],
    pub status: __le16,
    pub mtu: __le16,
    pub reserved0: __le16,
    pub reserved1: __le16,
    pub reserved2: u8,
    pub reserved3: u8,
    pub reserved4: __le16,
    pub reserved5: __le16,
    pub reserved6: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_aq_queue_status {
    pub qidx: __le16,
pub const EEA_QUEUE_STATUS_OK: c_int = 0;
pub const EEA_QUEUE_STATUS_NEED_RESET: c_int = 1;
    pub status: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __eea_aq_dev_status {
pub const EEA_LINK_DOWN_STATUS: c_int = 0;
pub const EEA_LINK_UP_STATUS: c_int = 1;
    pub link_status: __le16,
    pub reserved: __le16,
    pub q_status: [eea_aq_queue_status; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_aq_dev_status {
    pub num: u32,
    pub status: *mut __eea_aq_dev_status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_aq {
    pub ring: *mut eea_ring,
    pub num: u32,
    pub broken: bool,
    pub phase: u16,
// lock for adminq exec
    pub lock: mutex,
    pub q_req_size: u32,
    pub q_res_size: u32,
    pub q_req_buf: *mut eea_aq_create,
    pub q_res_buf: *mut __le32,
}

extern "C" {
    pub fn eea_create_adminq(enet: *mut eea_net, qid: u32) -> c_int;
}
extern "C" {
    pub fn eea_destroy_adminq(enet: *mut eea_net);
}
extern "C" {
    pub fn eea_adminq_query_cfg(enet: *mut eea_net, cfg: *mut eea_aq_cfg) -> c_int;
}
extern "C" {
    pub fn eea_adminq_create_q(enet: *mut eea_net, num: u32, flags: u32) -> c_int;
}
extern "C" {
    pub fn eea_adminq_destroy_all_q(enet: *mut eea_net) -> c_int;
}
extern "C" {
    pub fn eea_adminq_config_host_info(enet: *mut eea_net);
}
