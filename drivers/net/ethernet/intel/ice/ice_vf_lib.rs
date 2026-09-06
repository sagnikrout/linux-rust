//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_vf_lib.h
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
// Copyright (C) 2018-2021, Intel Corporation.

pub const ICE_MAX_SRIOV_VFS: c_int = 256;
// VF resource constraints
pub const ICE_MAX_RSS_QS_PER_VF: c_int = 16;
// VF capabilities
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_virtchnl_cap {
    ICE_VIRTCHNL_VF_CAP_PRIVILEGE = 0,
}

// Specific VF states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_vf_states {
    ICE_VF_STATE_INIT = 0,		/* PF is initializing VF */
    ICE_VF_STATE_ACTIVE,		/* VF resources are allocated for use */
    ICE_VF_STATE_QS_ENA,		/* VF queue(s) enabled */
    ICE_VF_STATE_DIS,
    ICE_VF_STATE_MC_PROMISC,
    ICE_VF_STATE_UC_PROMISC,
    ICE_VF_STATES_NBITS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_time_mac {
    pub time_modified: c_ulong,
    pub addr: [u8; ETH_ALEN],
}

// VF MDD events print structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_mdd_vf_events {
    pub /: *mut *mut u16 count; / total count of Rx|Tx events,
// count number of the last printed event
    pub last_printed: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_hash_ip_ctx_type {
    ICE_HASH_IP_CTX_IP = 0,
    ICE_HASH_IP_CTX_IP_ESP,
    ICE_HASH_IP_CTX_IP_UDP_ESP,
    ICE_HASH_IP_CTX_IP_AH,
    ICE_HASH_IP_CTX_IP_PFCP,
    ICE_HASH_IP_CTX_IP_UDP,
    ICE_HASH_IP_CTX_IP_TCP,
    ICE_HASH_IP_CTX_IP_SCTP,
    ICE_HASH_IP_CTX_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_vf_hash_ip_ctx {
    pub ctx: [ice_rss_hash_cfg; ICE_HASH_IP_CTX_MAX],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_hash_gtpu_ctx_type {
    ICE_HASH_GTPU_CTX_EH_IP = 0,
    ICE_HASH_GTPU_CTX_EH_IP_UDP,
    ICE_HASH_GTPU_CTX_EH_IP_TCP,
    ICE_HASH_GTPU_CTX_UP_IP,
    ICE_HASH_GTPU_CTX_UP_IP_UDP,
    ICE_HASH_GTPU_CTX_UP_IP_TCP,
    ICE_HASH_GTPU_CTX_DW_IP,
    ICE_HASH_GTPU_CTX_DW_IP_UDP,
    ICE_HASH_GTPU_CTX_DW_IP_TCP,
    ICE_HASH_GTPU_CTX_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_vf_hash_gtpu_ctx {
    pub ctx: [ice_rss_hash_cfg; ICE_HASH_GTPU_CTX_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_vf_hash_ctx {
    pub v4: ice_vf_hash_ip_ctx,
    pub v6: ice_vf_hash_ip_ctx,
    pub ipv4: ice_vf_hash_gtpu_ctx,
    pub ipv6: ice_vf_hash_gtpu_ctx,
}

// Structure to store fdir fv entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_fdir_prof_info {
    pub prof: ice_parser_profile,
    pub fdir_active_cnt: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_vf_qs_bw {
    pub committed: u32,
    pub peak: u32,
    pub queue_id: u16,
    pub tc: u8,
}

// Structure to store RSS field vector entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_rss_prof_info {
    pub prof: ice_parser_profile,
    pub symm: bool,
}

// VF operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_vf_ops {
    pub reset_type: ice_disq_rst_src,
    pub vf): *mut *mut void (free)(struct ice_vf,
    pub vf): *mut *mut void (clear_reset_state)(struct ice_vf,
    pub vf): *mut *mut void (clear_mbx_register)(struct ice_vf,
    pub is_vflr): *mut *mut *mut void (trigger_reset_register)(struct ice_vf vf, bool,
    pub vf): *mut *mut bool (poll_reset_status)(struct ice_vf,
    pub vf): *mut *mut void (clear_reset_trigger)(struct ice_vf,
    pub vf): *mut *mut void (irq_close)(struct ice_vf,
    pub vf): *mut *mut void (post_vsi_rebuild)(struct ice_vf,
}

// Virtchnl/SR-IOV config info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_vfs {
    pub /: *mut *mut DECLARE_HASHTABLE(table, 8); / table of VF entries,
    pub /: *mut *mut mutex table_lock; / Lock for protecting the hash table,
    pub /: *mut *mut u16 num_supported; / max supported VFs on this PF,
    pub /: *mut *mut u16 num_qps_per; / number of queue pairs per VF,
    pub /: *mut *mut u16 num_msix_per; / default MSI-X vectors per VF,
    pub /: *mut *mut unsigned long last_printed_mdd_jiffies; / MDD message rate limit,
}

// VF information structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_vf {
    pub entry: hlist_node,
    pub rcu: rcu_head,
    pub refcnt: kref,
    pub pf: *mut ice_pf,
    pub vfdev: *mut pci_dev,
// Used during virtchnl message handling and NDO ops against the VF
// that will trigger a VFR
//
    pub cfg_lock: mutex,
    pub /: *mut *mut u16 vf_id; / VF ID in the PF space,
    pub /: *mut *mut u16 lan_vsi_idx; / index into PF struct,
    pub ctrl_vsi_idx: u16,
    pub fdir: ice_vf_fdir,
    pub fdir_prof_info: [ice_fdir_prof_info; ICE_MAX_PTGS],
    pub rss_prof_info: [ice_rss_prof_info; ICE_MAX_PTGS],
    pub hash_ctx: ice_vf_hash_ctx,
    pub /: *mut *mut u64 rss_hashcfg; / RSS hash configuration,
    pub /: *mut *mut *mut ice_sw vf_sw_id; / switch ID the VF VSIs connect to,
    pub vf_ver: virtchnl_version_info,
    pub /: *mut *mut u32 driver_caps; / reported by VF driver,
    pub dev_lan_addr: [u8; ETH_ALEN],
    pub hw_lan_addr: [u8; ETH_ALEN],
    pub legacy_last_added_umac: ice_time_mac,
    pub ICE_MAX_RSS_QS_PER_VF): DECLARE_BITMAP(txq_ena,,
    pub ICE_MAX_RSS_QS_PER_VF): DECLARE_BITMAP(rxq_ena,,
    pub /: *mut *mut ice_vlan port_vlan_info; / Port VLAN ID, QoS, and TPID,
    pub vlan_v2_caps: virtchnl_vlan_caps,
    pub mbx_info: ice_mbx_vf_info,
    pub /: *mut *mut u8 pf_set_mac:1; / VF MAC address set by VMM admin,
    pub trusted:1: u8,
    pub spoofchk:1: u8,
    pub link_forced:1: u8,
    pub /: *mut *mut u8 link_up:1; / only valid if VF link is forced,
    pub lldp_tx_ena:1: u8,
    pub /: *mut *mut u16 num_msix; / num of MSI-X configured on this VF,
    pub ptp_caps: u32,
    pub /: *mut *mut unsigned int min_tx_rate; / Minimum Tx bandwidth limit in Mbps,
    pub /: *mut *mut unsigned int max_tx_rate; / Maximum Tx bandwidth limit in Mbps,
// first vector index of this VF in the PF space
    pub first_vector_idx: c_int,
    pub /: *mut *mut DECLARE_BITMAP(vf_states, ICE_VF_STATES_NBITS); / VF runtime states,
    pub /: *mut *mut unsigned long vf_caps; / VF's adv. capabilities,
    pub /: *mut *mut u8 num_req_qs; / num of queue pairs requested by VF,
    pub num_mac: u16,
    pub num_mac_lldp: u16,
    pub /: *mut *mut u16 num_vf_qs; / num of queue configured per VF,
    pub /: *mut *mut u8 vlan_strip_ena; / Outer and Inner VLAN strip enable,

    pub mdd_rx_events: ice_mdd_vf_events,
    pub mdd_tx_events: ice_mdd_vf_events,
    pub VIRTCHNL_OP_MAX): DECLARE_BITMAP(opcodes_allowlist,,
    pub repr_id: c_ulong,
    pub virtchnl_ops: *const ice_virtchnl_ops,
    pub vf_ops: *const ice_vf_ops,
// devlink port data
    pub devlink_port: devlink_port,
    pub lldp_recipe_id: u16,
    pub lldp_rule_id: u16,
    pub qs_bw: [ice_vf_qs_bw; ICE_MAX_RSS_QS_PER_VF],
}

// Flags for controlling behavior of ice_reset_vf
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_vf_reset_flags {
    ICE_VF_RESET_VFLR = BIT(0), /* Indicate a VFLR reset */
    ICE_VF_RESET_NOTIFY = BIT(1), /* Notify VF prior to reset */
    ICE_VF_RESET_LOCK = BIT(2), /* Acquire the VF cfg_lock */
}

// VF Hash Table access functions
//
// These functions provide abstraction for interacting with the VF hash table.
// In general, direct access to the hash table should be avoided outside of
// these functions where possible.
//
// The VF entries in the hash table are protected by reference counting to
// track lifetime of accesses from the table. The ice_get_vf_by_id() function
// obtains a reference to the VF structure which must be dropped by using
// ice_put_vf().
//
// ice_for_each_vf - Iterate over each VF entry
// @pf: pointer to the PF private structure
// @bkt: bucket index used for iteration
// @vf: pointer to the VF entry currently being processed in the loop
//
// The bkt variable is an unsigned integer iterator used to traverse the VF
// entries. It is *not* guaranteed to be the VF's vf_id. Do not assume it is.
// Use vf->vf_id to get the id number if needed.
//
// The caller is expected to be under the table_lock mutex for the entire
// loop. Use this iterator if your loop is long or if it might sleep.
//

//
// ice_for_each_vf_rcu - Iterate over each VF entry protected by RCU
// @pf: pointer to the PF private structure
// @bkt: bucket index used for iteration
// @vf: pointer to the VF entry currently being processed in the loop
//
// The bkt variable is an unsigned integer iterator used to traverse the VF
// entries. It is *not* guaranteed to be the VF's vf_id. Do not assume it is.
// Use vf->vf_id to get the id number if needed.
//
// The caller is expected to be under rcu_read_lock() for the entire loop.
// Only use this iterator if your loop is short and you can guarantee it does
// not sleep.
//

extern "C" {
    pub fn ice_get_vf_by_id(_arg: pf, _arg: pci_iov_vf_id(vf_dev)) -> return;
}
extern "C" {
    pub fn ice_put_vf(vf: *mut ice_vf);
}
extern "C" {
    pub fn ice_has_vfs(pf: *mut ice_pf) -> bool;
}
extern "C" {
    pub fn ice_get_num_vfs(pf: *mut ice_pf) -> u16;
}
extern "C" {
    pub fn ice_is_vf_disabled(vf: *mut ice_vf) -> bool;
}
extern "C" {
    pub fn ice_check_vf_ready_for_cfg(vf: *mut ice_vf) -> c_int;
}
extern "C" {
    pub fn ice_set_vf_state_dis(vf: *mut ice_vf);
}
extern "C" {
    pub fn ice_is_any_vf_in_unicast_promisc(pf: *mut ice_pf) -> bool;
}
extern "C" {
    pub fn ice_reset_vf(vf: *mut ice_vf, flags: u32) -> c_int;
}
extern "C" {
    pub fn ice_reset_all_vfs(pf: *mut ice_pf);
}

