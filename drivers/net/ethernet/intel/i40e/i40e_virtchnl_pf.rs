//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/i40e/i40e_virtchnl_pf.h
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
// Copyright(c) 2013 - 2018 Intel Corporation.

pub const I40E_MAX_VLANID: c_int = 4095;
pub const I40E_VIRTCHNL_SUPPORTED_QTYPES: c_int = 2;
pub const I40E_VLAN_PRIORITY_SHIFT: c_int = 13;
pub const I40E_VLAN_MASK: c_uint = 0xFFF;
pub const I40E_PRIORITY_MASK: c_uint = 0xE000;
pub const I40E_MAX_VF_PROMISC_FLAGS: c_int = 3;
pub const I40E_VF_STATE_WAIT_COUNT: c_int = 20;
pub const I40E_VFR_WAIT_COUNT: c_int = 100;
// Various queue ctrls
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_queue_ctrl {
    I40E_QUEUE_CTRL_UNKNOWN = 0,
    I40E_QUEUE_CTRL_ENABLE,
    I40E_QUEUE_CTRL_ENABLECHECK,
    I40E_QUEUE_CTRL_DISABLE,
    I40E_QUEUE_CTRL_DISABLECHECK,
    I40E_QUEUE_CTRL_FASTDISABLE,
    I40E_QUEUE_CTRL_FASTDISABLECHECK,
}

// VF states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_vf_states {
    I40E_VF_STATE_INIT = 0,
    I40E_VF_STATE_ACTIVE,
    I40E_VF_STATE_RDMAENA,
    I40E_VF_STATE_DISABLED,
    I40E_VF_STATE_MC_PROMISC,
    I40E_VF_STATE_UC_PROMISC,
    I40E_VF_STATE_PRE_ENABLE,
    I40E_VF_STATE_RESETTING,
    I40E_VF_STATE_RESOURCES_LOADED,
}

// VF capabilities
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_vf_capabilities {
    I40E_VIRTCHNL_VF_CAP_PRIVILEGE = 0,
    I40E_VIRTCHNL_VF_CAP_L2,
    I40E_VIRTCHNL_VF_CAP_RDMA,
}

// In ADq, max 4 VSI's can be allocated per VF including primary VF VSI.
// These variables are used to store indices, id's and number of queues
// for each VSI including that of primary VF VSI. Each Traffic class is
// termed as channel and each channel can in-turn have 4 queues which
// means max 16 queues overall per VF.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40evf_channel {
    pub /: *mut *mut u16 vsi_idx; / index in PF struct for all channel VSIs,
    pub /: *mut *mut u16 vsi_id; / VSI ID used by firmware,
    pub /: *mut *mut u16 num_qps; / number of queue pairs requested by user,
    pub /: *mut *mut u64 max_tx_rate; / bandwidth rate allocation for VSIs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_mdd_vf_events {
    pub /: *mut *mut u64 count; / total count of Rx|Tx events,
// count number of the last printed event
    pub last_printed: u64,
}

// VF information structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_vf {
    pub pf: *mut i40e_pf,
// VF id in the PF space
    pub vf_id: i16,
// all VF vsis connect to the same parent
    pub parent_type: i40e_switch_element_types,
    pub vf_ver: virtchnl_version_info,
    pub /: *mut *mut u32 driver_caps; / reported by VF driver,
// VF Port Extender (PE) stag if used
    pub stag: u16,
    pub default_lan_addr: virtchnl_ether_addr,
    pub port_vlan_id: u16,
    pub /: *mut *mut bool pf_set_mac; / The VMM admin set the VF MAC address,
    pub trusted: bool,
// VSI indices - actual VSI pointers are maintained in the PF structure
// When assigned, these will be non-zero, because VSI 0 is always
// the main LAN VSI for the PF.
//
    pub /: *mut *mut u16 lan_vsi_idx; / index into PF struct,
    pub /: *mut *mut u16 lan_vsi_id; / ID as used by firmware,
    pub /: *mut *mut u8 num_queue_pairs; / num of qps assigned to VF vsis,
    pub /: *mut *mut u8 num_req_queues; / num of requested qps,
// num of mdd tx and rx events detected
    pub mdd_rx_events: i40e_mdd_vf_events,
    pub mdd_tx_events: i40e_mdd_vf_events,
    pub /: *mut *mut unsigned long vf_caps; / vf's adv. capabilities,
    pub /: *mut *mut unsigned long vf_states; / vf's runtime states,
    pub /: *mut *mut unsigned int tx_rate; / Tx bandwidth limit in Mbps,
    pub link_forced: bool,
    pub /: *mut *mut bool link_up; / only valid if VF link is forced,
    pub spoofchk: bool,
    pub /: *mut *mut bool is_disabled_from_host; / PF ctrl of VF enable/disable,
    pub num_vlan: u16,
// ADq related variables
    pub /: *mut *mut bool adq_enabled; / flag to enable adq,
    pub num_tc: u8,
    pub ch: [i40evf_channel; I40E_MAX_VF_VSI],
    pub cloud_filter_list: hlist_head,
    pub num_cloud_filters: u16,
// RDMA Client
    pub qvlist_info: *mut virtchnl_rdma_qvlist_info,
}

extern "C" {
    pub fn i40e_free_vfs(pf: *mut i40e_pf);
}
extern "C" {
    pub fn i40e_pci_sriov_configure(dev: *mut pci_dev, num_vfs: c_int) -> c_int;
}
extern "C" {
    pub fn i40e_alloc_vfs(pf: *mut i40e_pf, num_alloc_vfs: u16) -> c_int;
}
extern "C" {
    pub fn i40e_vc_process_vflr_event(pf: *mut i40e_pf) -> c_int;
}
extern "C" {
    pub fn i40e_vc_reset_vf(vf: *mut i40e_vf, notify_vf: bool);
}
extern "C" {
    pub fn i40e_reset_vf(vf: *mut i40e_vf, flr: bool) -> bool;
}
extern "C" {
    pub fn i40e_reset_all_vfs(pf: *mut i40e_pf, flr: bool) -> bool;
}
extern "C" {
    pub fn i40e_vc_notify_vf_reset(vf: *mut i40e_vf);
}
// VF configuration related iplink handlers
extern "C" {
    pub fn i40e_ndo_set_vf_mac(netdev: *mut net_device, vf_id: c_int, mac: *mut u8) -> c_int;
}
extern "C" {
    pub fn i40e_ndo_set_vf_trust(netdev: *mut net_device, vf_id: c_int, setting: bool) -> c_int;
}
extern "C" {
    pub fn i40e_ndo_set_vf_link_state(netdev: *mut net_device, vf_id: c_int, link: c_int) -> c_int;
}
extern "C" {
    pub fn i40e_ndo_set_vf_spoofchk(netdev: *mut net_device, vf_id: c_int, enable: bool) -> c_int;
}
extern "C" {
    pub fn i40e_vc_notify_link_state(pf: *mut i40e_pf);
}
extern "C" {
    pub fn i40e_vc_notify_reset(pf: *mut i40e_pf);
}

extern "C" {
    pub fn i40e_restore_all_vfs_msi_state(pdev: *mut pci_dev);
}

