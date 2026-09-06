//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/virt/virtchnl.h
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
// Copyright (C) 2022, Intel Corporation.

// Restrict number of MAC Addr and VLAN that non-trusted VF can programmed
pub const ICE_MAX_VLAN_PER_VF: c_int = 8;
pub const ICE_DFLT_QUANTA: c_int = 1024;
pub const ICE_MAX_QUANTA_SIZE: c_int = 4096;
pub const ICE_MIN_QUANTA_SIZE: c_int = 256;

// MAC filters: 1 is reserved for the VF's default/perm_addr/LAA MAC, 1 for
// broadcast, and 16 for additional unicast/multicast filters
//
pub const ICE_MAX_MACADDR_PER_VF: c_int = 18;
pub const ICE_FLEX_DESC_RXDID_MAX_NUM: c_int = 64;
// Priority to be compared against previous priority from the pipe
pub const ICE_RXDID_PRIO: c_uint = 0x03;
// VFs only get a single VSI. For ice hardware, the VF does not need to know
// its VSI index. However, the virtchnl interface requires a VSI number,
// mainly due to legacy hardware.
//
// Since the VF doesn't need this information, report a static value to the VF
// instead of leaking any information about the PF or hardware setup.
//
pub const ICE_VF_VSI_ID: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_virtchnl_ops {
    pub msg): *mut *mut *mut int (get_ver_msg)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (get_vf_res_msg)(struct ice_vf vf, u8,
    pub vf): *mut *mut void (reset_vf)(struct ice_vf,
    pub msg): *mut *mut *mut int (add_mac_addr_msg)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (del_mac_addr_msg)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (cfg_qs_msg)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (ena_qs_msg)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (dis_qs_msg)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (request_qs_msg)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (cfg_irq_map_msg)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (config_rss_key)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (config_rss_lut)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (config_rss_hfunc)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (get_stats_msg)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (cfg_promiscuous_mode_msg)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (add_vlan_msg)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (remove_vlan_msg)(struct ice_vf vf, u8,
    pub vf): *mut *mut int (query_rxdid)(struct ice_vf,
    pub vf): *mut *mut int (get_rss_hashcfg)(struct ice_vf,
    pub msg): *mut *mut *mut int (set_rss_hashcfg)(struct ice_vf vf, u8,
    pub vf): *mut *mut int (ena_vlan_stripping)(struct ice_vf,
    pub vf): *mut *mut int (dis_vlan_stripping)(struct ice_vf,
    pub add): *mut *mut *mut *mut int (handle_rss_cfg_msg)(struct ice_vf vf, u8 msg, bool,
    pub msg): *mut *mut *mut int (add_fdir_fltr_msg)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (del_fdir_fltr_msg)(struct ice_vf vf, u8,
    pub vf): *mut *mut int (get_offload_vlan_v2_caps)(struct ice_vf,
    pub msg): *mut *mut *mut int (add_vlan_v2_msg)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (remove_vlan_v2_msg)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (ena_vlan_stripping_v2_msg)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (dis_vlan_stripping_v2_msg)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (ena_vlan_insertion_v2_msg)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (dis_vlan_insertion_v2_msg)(struct ice_vf vf, u8,
    pub vf): *mut *mut int (get_qos_caps)(struct ice_vf,
    pub msg): *mut *mut *mut int (cfg_q_tc_map)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (cfg_q_bw)(struct ice_vf vf, u8,
    pub msg): *mut *mut *mut int (cfg_q_quanta)(struct ice_vf vf, u8,
    pub msg): *const virtchnl_ptp_caps,
    pub vf): *mut *mut int (get_phc_time)(struct ice_vf,
}

extern "C" {
    pub fn ice_virtchnl_set_dflt_ops(vf: *mut ice_vf);
}
extern "C" {
    pub fn ice_virtchnl_set_repr_ops(vf: *mut ice_vf);
}
extern "C" {
    pub fn ice_vc_notify_vf_link_state(vf: *mut ice_vf);
}
extern "C" {
    pub fn ice_vc_notify_link_state(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_vc_notify_reset(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_vc_isvalid_vsi_id(vf: *mut ice_vf, vsi_id: u16) -> bool;
}
extern "C" {
    pub fn ice_vf_ena_rxq_interrupt(vsi: *mut ice_vsi, q_idx: u32);
}
extern "C" {
    pub fn ice_vf_ena_txq_interrupt(vsi: *mut ice_vsi, q_idx: u32);
}
extern "C" {
    pub fn ice_is_vlan_promisc_allowed(vf: *mut ice_vf) -> bool;
}

