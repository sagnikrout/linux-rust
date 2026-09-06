//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/net/intel/i40e_client.h
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

pub const I40E_CLIENT_STR_LENGTH: c_int = 10;
// Client interface version should be updated anytime there is a change in the
// existing APIs or data structures.
//
pub const I40E_CLIENT_VERSION_MAJOR: c_int = 0;
pub const I40E_CLIENT_VERSION_MINOR: c_int = 01;
pub const I40E_CLIENT_VERSION_BUILD: c_int = 00;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_client_version {
    pub major: u8,
    pub minor: u8,
    pub build: u8,
    pub rsvd: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_client_instance_state {
    __I40E_CLIENT_INSTANCE_NONE,
    __I40E_CLIENT_INSTANCE_OPENED,
}

pub const I40E_QUEUE_INVALID_IDX: c_uint = 0xFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_qv_info {
    pub /: *mut *mut u32 v_idx; / msix_vector,
    pub ceq_idx: u16,
    pub aeq_idx: u16,
    pub itr_idx: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_qvlist_info {
    pub num_vectors: u32,
    pub __counted_by(num_vectors): i40e_qv_info qv_info[],
}

// set of LAN parameters useful for clients managed by LAN
// Struct to hold per priority info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_prio_qos_params {
    pub /: *mut *mut u16 qs_handle; / qs handle for prio,
    pub /: *mut *mut u8 tc; / TC mapped to prio,
    pub reserved: u8,
}

pub const I40E_CLIENT_MAX_USER_PRIORITY: c_int = 8;
// Struct to hold Client QoS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_qos_params {
    pub prio_qos: [i40e_prio_qos_params; I40E_CLIENT_MAX_USER_PRIORITY],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_params {
    pub qos: i40e_qos_params,
    pub mtu: u16,
}

// Structure to hold Lan device info for a client device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_info {
    pub version: i40e_client_version,
    pub lanmac: [u8; 6],
    pub netdev: *mut net_device,
    pub pcidev: *mut pci_dev,
    pub aux_dev: *mut auxiliary_device,
    pub hw_addr: *mut u8 __iomem,
    pub /: *mut *mut u8 fid; / function id, PF id or VF id,
pub const I40E_CLIENT_FTYPE_PF: c_int = 0;
    pub /: *mut *mut u8 ftype; / function type, PF or VF,
    pub pf: *mut c_void,
// All L2 params that could change during the life span of the PF
// and needs to be communicated to the client when they change
//
    pub qvlist_info: *mut i40e_qvlist_info,
    pub params: i40e_params,
    pub ops: *mut i40e_ops,
    pub vectors*/: *mut *mut u16 msix_count; / number of msix,
// Array down below will be dynamically allocated based on msix_count
    pub msix_entries: *mut msix_entry,
    pub /: *mut *mut u16 itr_index; / Which ITR index the PE driver is suppose to use,
    pub /: *mut *mut u16 fw_maj_ver; / firmware major version,
    pub /: *mut *mut u16 fw_min_ver; / firmware minor version,
    pub /: *mut *mut u32 fw_build; / firmware build number,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_auxiliary_device {
    pub aux_dev: auxiliary_device,
    pub ldev: *mut i40e_info,
}

pub const I40E_CLIENT_RESET_LEVEL_PF: c_int = 1;
pub const I40E_CLIENT_RESET_LEVEL_CORE: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_ops {
// setup_q_vector_list enables queues with a particular vector
    pub qv_info): *mut i40e_qvlist_info,
    pub len): *mut *mut u32 vf_id, u8 msg, u16,
// If the PE Engine is unresponsive, RDMA driver can request a reset.
// The level helps determine the level of reset being requested.
//
    pub level): *mut *mut i40e_client client, u32,
// API for the RDMA driver to set certain VSI flags that control
// PE Engine.
//
    pub valid_flag): u32 flag, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_client_ops {
// Should be called from register_client() or whenever PF is ready
// to create a specific client instance.
//
    pub client): *mut *mut *mut int (open)(struct i40e_info ldev, struct i40e_client,
// Should be called when netdev is unavailable or when unregister
// call comes in. If the close is happenening due to a reset being
// triggered set the reset bit to true.
//
    pub reset): bool,
// called when some l2 managed parameters changes - mtu
    pub params): *mut i40e_params,
    pub len): *mut *mut u8 msg, u16,
// called when a VF is reset by the PF
    pub vf_id): *mut *mut i40e_client client, u32,
// called when the number of VFs changes
    pub num_vfs): *mut *mut i40e_client client, u32,
// returns true if VF is capable of specified offload
    pub vf_id): *mut *mut i40e_client client, u32,
}

// Client device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_client_instance {
    pub list: list_head,
    pub lan_info: i40e_info,
    pub client: *mut i40e_client,
    pub state: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_client {
    pub /: *mut *mut list_head list; / list of registered clients,
    pub name: [c_char; I40E_CLIENT_STR_LENGTH],
    pub version: i40e_client_version,
    pub /: *mut *mut unsigned long state; / client state,
    pub /: *mut *mut atomic_t ref_cnt; / Count of all the client devices of this kind,
    pub flags: u32,
    pub type: u8,
pub const I40E_CLIENT_IWARP: c_int = 0;
    pub /: *const *const *const i40e_client_ops ops; / client ops provided by the client,
}

extern "C" {
    pub fn i40e_client_device_register(ldev: *mut i40e_info, client: *mut i40e_client);
}
extern "C" {
    pub fn i40e_client_device_unregister(ldev: *mut i40e_info);
}
