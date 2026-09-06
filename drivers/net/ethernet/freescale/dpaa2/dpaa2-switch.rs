//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/dpaa2/dpaa2-switch.h
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


// SPDX-License-Identifier: GPL-2.0
//
// DPAA2 Ethernet Switch declarations
//
// Copyright 2014-2016 Freescale Semiconductor Inc.
// Copyright 2017-2021 NXP
//

// Number of IRQs supported
pub const DPSW_IRQ_NUM: c_int = 2;
// Port is member of VLAN
pub const ETHSW_VLAN_MEMBER: c_int = 1;
// VLAN to be treated as untagged on egress
pub const ETHSW_VLAN_UNTAGGED: c_int = 2;
// Untagged frames will be assigned to this VLAN
pub const ETHSW_VLAN_PVID: c_int = 4;
// VLAN configured on the switch
pub const ETHSW_VLAN_GLOBAL: c_int = 8;
// Maximum Frame Length supported by HW (currently 10k)

// Number of receive queues (one RX and one TX_CONF)
pub const DPAA2_SWITCH_RX_NUM_FQS: c_int = 2;
// Hardware requires alignment for ingress/egress buffer addresses

pub const DPAA2_SWITCH_STORE_SIZE: c_int = 16;
// Buffer management
pub const BUFS_PER_CMD: c_int = 7;

// Number of times to retry DPIO portal operations while waiting
// for portal to finish executing current command and become
// available. We want to avoid being stuck in a while loop in case
// hardware becomes unresponsive, but not give up too easily if
// the portal really is busy for valid reasons
//
pub const DPAA2_SWITCH_SWP_BUSY_RETRIES: c_int = 1000;
// Hardware annotation buffer size
pub const DPAA2_SWITCH_HWA_SIZE: c_int = 64;
// Software annotation buffer size
pub const DPAA2_SWITCH_SWA_SIZE: c_int = 64;
pub const DPAA2_SWITCH_TX_BUF_ALIGN: c_int = 64;

pub const DPAA2_ETHSW_PORT_MAX_ACL_ENTRIES: c_int = 16;
pub const DPAA2_ETHSW_PORT_DEFAULT_TRAPS: c_int = 1;
pub const DPAA2_ETHSW_PORT_ACL_CMD_BUF_SIZE: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_switch_fq {
    pub ethsw: *mut ethsw_core,
    pub type: dpsw_queue_type,
    pub store: *mut dpaa2_io_store,
    pub nctx: dpaa2_io_notification_ctx,
    pub napi: napi_struct,
    pub fqid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_mac_addr {
    pub addr: [c_uchar; ETH_ALEN],
    pub vid: u16,
    pub refcount: refcount_t,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_switch_fdb {
    pub bridge_dev: *mut net_device,
    pub fdb_id: u16,
    pub in_use: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_switch_lag {
    pub ethsw: *mut ethsw_core,
    pub bond_dev: *mut net_device,
    pub in_use: bool,
    pub id: u8,
    pub primary: *mut ethsw_port_priv,
// Protects the list of fdbs installed on this LAG
    pub fdb_lock: mutex,
    pub fdbs: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_switch_acl_entry {
    pub list: list_head,
    pub prio: u16,
    pub cookie: c_ulong,
    pub cfg: dpsw_acl_entry_cfg,
    pub key: dpsw_acl_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_switch_mirror_entry {
    pub list: list_head,
    pub cfg: dpsw_reflection_cfg,
    pub cookie: c_ulong,
    pub if_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_switch_filter_block {
    pub ethsw: *mut ethsw_core,
    pub ports: u64,
    pub in_use: bool,
    pub acl_entries: list_head,
    pub acl_id: u16,
    pub num_acl_rules: u8,
    pub mirror_entries: list_head,
}

// Per port private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethsw_port_priv {
    pub netdev: *mut net_device,
    pub idx: u16,
    pub ethsw_data: *mut ethsw_core,
    pub link_state: u8,
    pub stp_state: u8,
    pub 1]: u8 vlans[VLAN_VID_MASK +,
    pub pvid: u16,
    pub tx_qdid: u16,
    pub fdb: *mut dpaa2_switch_fdb,
    pub bcast_flood: bool,
    pub ucast_flood: bool,
    pub learn_ena: bool,
    pub filter_block: *mut dpaa2_switch_filter_block,
    pub mac: *mut dpaa2_mac,
// Protects against changes to port_priv->mac
    pub mac_lock: mutex,
    pub lag: *mut dpaa2_switch_lag __rcu,
}

// Switch data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethsw_core {
    pub dev: *mut device,
    pub mc_io: *mut fsl_mc_io,
    pub dpsw_handle: u16,
    pub sw_attr: dpsw_attr,
    pub minor: u16 major,,
    pub features: c_ulong,
    pub dev_id: c_int,
    pub ports: *mut ethsw_port_priv,
    pub iommu_domain: *mut iommu_domain,
    pub 1]: u8 vlans[VLAN_VID_MASK +,
    pub workqueue: *mut workqueue_struct,
    pub fq: [dpaa2_switch_fq; DPAA2_SWITCH_RX_NUM_FQS],
    pub dpbp_dev: *mut fsl_mc_device,
    pub buf_count: c_int,
    pub bpid: u16,
    pub napi_users: c_int,
    pub fdbs: *mut dpaa2_switch_fdb,
    pub filter_blocks: *mut dpaa2_switch_filter_block,
    pub mirror_port: u16,
    pub lags: *mut dpaa2_switch_lag,
}

extern "C" {
    pub fn dpaa2_mac_is_type_phy(_arg: port_priv->mac) -> return;
}
extern "C" {
    pub fn dpaa2_switch_port_dev_check(netdev: *const net_device) -> bool;
}
// TC offload
