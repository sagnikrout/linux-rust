//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic/hinic_dev.h
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
// Huawei HiNIC PCI Express Linux driver
// Copyright(c) 2017 Huawei Technologies Co., Ltd
//

pub const LP_PKT_CNT: c_int = 64;
pub const HINIC_MAX_JUMBO_FRAME_SIZE: c_int = 15872;

pub const HINIC_MIN_MTU_SIZE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_flags {
    HINIC_LINK_UP = BIT(0),
    HINIC_INTF_UP = BIT(1),
    HINIC_RSS_ENABLE = BIT(2),
    HINIC_LINK_DOWN = BIT(3),
    HINIC_LP_TEST = BIT(4),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rx_mode_work {
    pub work: work_struct,
    pub rx_mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_rss_type {
    pub tcp_ipv6_ext: u8,
    pub ipv6_ext: u8,
    pub tcp_ipv6: u8,
    pub ipv6: u8,
    pub tcp_ipv4: u8,
    pub ipv4: u8,
    pub udp_ipv6: u8,
    pub udp_ipv4: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_rss_hash_type {
    HINIC_RSS_HASH_ENGINE_TYPE_XOR,
    HINIC_RSS_HASH_ENGINE_TYPE_TOEP,
    HINIC_RSS_HASH_ENGINE_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_intr_coal_info {
    pub pending_limt: u8,
    pub coalesce_timer_cfg: u8,
    pub resend_timer_cfg: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hinic_dbg_type {
    HINIC_DBG_SQ_INFO,
    HINIC_DBG_RQ_INFO,
    HINIC_DBG_FUNC_TABLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_debug_priv {
    pub dev: *mut hinic_dev,
    pub object: *mut c_void,
    pub type: hinic_dbg_type,
    pub root: *mut dentry,
    pub field_id: [c_int; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_dev {
    pub netdev: *mut net_device,
    pub hwdev: *mut hinic_hwdev,
    pub msg_enable: u32,
    pub tx_weight: c_uint,
    pub rx_weight: c_uint,
    pub num_qps: u16,
    pub max_qps: u16,
    pub flags: c_uint,
    pub mgmt_lock: semaphore,
    pub vlan_bitmap: *mut c_ulong,
    pub rx_mode_work: hinic_rx_mode_work,
    pub workq: *mut workqueue_struct,
    pub txqs: *mut hinic_txq,
    pub rxqs: *mut hinic_rxq,
    pub sq_depth: u16,
    pub rq_depth: u16,
    pub rss_tmpl_idx: u8,
    pub rss_hash_engine: u8,
    pub num_rss: u16,
    pub rss_limit: u16,
    pub rss_type: hinic_rss_type,
    pub rx_intr_coalesce: *mut hinic_intr_coal_info,
    pub tx_intr_coalesce: *mut hinic_intr_coal_info,
    pub sriov_info: hinic_sriov_info,
    pub lb_test_rx_idx: c_int,
    pub lb_pkt_len: c_int,
    pub lb_test_rx_buf: *mut u8,
    pub dbgfs_root: *mut dentry,
    pub sq_dbgfs: *mut dentry,
    pub rq_dbgfs: *mut dentry,
    pub func_tbl_dbgfs: *mut dentry,
    pub dbg: *mut hinic_debug_priv,
    pub devlink: *mut devlink,
    pub cable_unplugged: bool,
    pub module_unrecognized: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_devlink_priv {
    pub hwdev: *mut hinic_hwdev,
    pub hw_fault_reporter: *mut devlink_health_reporter,
    pub fw_fault_reporter: *mut devlink_health_reporter,
}
