//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_rl.h
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
// Copyright(c) 2023 Intel Corporation

pub const RL_ROOT_MAX: c_int = 4;
pub const RL_CLUSTER_MAX: c_int = 16;
pub const RL_LEAF_MAX: c_int = 64;

pub const RL_RP_CNT_MAX: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rl_node_type {
    RL_ROOT,
    RL_CLUSTER,
    RL_LEAF,
}

//
// struct adf_rl_sla_input_data - ratelimiting user input data structure
// @rp_mask: 64 bit bitmask of ring pair IDs which will be assigned to SLA.
// Eg. 0x5 -> RP0 and RP2 assigned; 0xA005 -> RP0,2,13,15 assigned.
// @sla_id: ID of current SLA for operations update, rm, get. For the add
// operation, this field will be updated with the ID of the newly
// added SLA
// @parent_id: ID of the SLA to which the current one should be assigned.
// Set to -1 to refer to the default parent.
// @cir: Committed information rate. Rate guaranteed to be achieved. Input value
// is expressed in permille scale, i.e. 1000 refers to the maximum
// device throughput for a selected service.
// @pir: Peak information rate. Maximum rate available that the SLA can achieve.
// Input value is expressed in permille scale, i.e. 1000 refers to
// the maximum device throughput for a selected service.
// @type: SLA type: root, cluster, node
// @srv: Service associated to the SLA: asym, sym dc.
//
// This structure is used to perform operations on an SLA.
// Depending on the operation, some of the parameters are ignored.
// The following list reports which parameters should be set for each operation.
// - add: all except sla_id
// - update: cir, pir, sla_id
// - rm: sla_id
// - rm_all: -
// - get: sla_id
// - get_capability_rem: srv, sla_id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_rl_sla_input_data {
    pub rp_mask: u64,
    pub sla_id: c_int,
    pub parent_id: c_int,
    pub cir: c_uint,
    pub pir: c_uint,
    pub type: rl_node_type,
    pub srv: adf_base_services,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rl_slice_cnt {
    pub dcpr_cnt: u8,
    pub pke_cnt: u8,
    pub cph_cnt: u8,
    pub cpr_cnt: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_rl_interface_data {
    pub input: adf_rl_sla_input_data,
    pub cap_rem_srv: adf_base_services,
    pub lock: rw_semaphore,
    pub sysfs_added: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_rl_hw_data {
    pub scale_ref: u32,
    pub scan_interval: u32,
    pub r2l_offset: u32,
    pub l2c_offset: u32,
    pub c2s_offset: u32,
    pub pciin_tb_offset: u32,
    pub pciout_tb_offset: u32,
    pub pcie_scale_mul: u32,
    pub pcie_scale_div: u32,
    pub dcpr_correction: u32,
    pub max_tp: [u32; RL_ROOT_MAX],
    pub svc_ae_mask: [u32; SVC_BASE_COUNT],
    pub slices: rl_slice_cnt,
}

//
// struct adf_rl - ratelimiting data structure
// @accel_dev: pointer to acceleration device data
// @device_data: pointer to rate limiting data specific to a device type (or revision)
// @sla: array of pointers to SLA objects
// @root: array of pointers to root type SLAs, element number reflects node_id
// @cluster: array of pointers to cluster type SLAs, element number reflects node_id
// @leaf: array of pointers to leaf type SLAs, element number reflects node_id
// @rp_in_use: array of ring pair IDs already used in one of SLAs
// @rl_lock: mutex object which is protecting data in this structure
// @input: structure which is used for holding the data received from user
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_rl {
    pub accel_dev: *mut adf_accel_dev,
    pub device_data: *mut adf_rl_hw_data,
// mapping sla_id to SLA objects
    pub sla: [*mut rl_sla; RL_NODES_CNT_MAX],
    pub root: [*mut rl_sla; RL_ROOT_MAX],
    pub cluster: [*mut rl_sla; RL_CLUSTER_MAX],
    pub leaf: [*mut rl_sla; RL_LEAF_MAX],
    pub rp_in_use: [bool; RL_RP_CNT_MAX],
// Mutex protecting writing to SLAs lists
    pub rl_lock: mutex,
    pub user_input: adf_rl_interface_data,
}

//
// struct rl_sla - SLA object data structure
// @parent: pointer to the parent SLA (root/cluster)
// @type: SLA type
// @srv: service associated with this SLA
// @sla_id: ID of the SLA, used as element number in SLA array and as identifier
// shared with the user
// @node_id: ID of node, each of SLA type have a separate ID list
// @cir: committed information rate
// @pir: peak information rate (PIR >= CIR)
// @rem_cir: if this SLA is a parent then this field represents a remaining
// value to be used by child SLAs.
// @ring_pairs_ids: array with numeric ring pairs IDs assigned to this SLA
// @ring_pairs_cnt: number of assigned ring pairs listed in the array above
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rl_sla {
    pub parent: *mut rl_sla,
    pub type: rl_node_type,
    pub srv: adf_base_services,
    pub sla_id: u32,
    pub node_id: u32,
    pub cir: u32,
    pub pir: u32,
    pub rem_cir: u32,
    pub ring_pairs_ids: [u16; RL_RP_CNT_PER_LEAF_MAX],
    pub ring_pairs_cnt: u16,
}

extern "C" {
    pub fn adf_rl_remove_sla(accel_dev: *mut adf_accel_dev, sla_id: u32) -> c_int;
}
extern "C" {
    pub fn adf_rl_remove_sla_all(accel_dev: *mut adf_accel_dev, incl_default: bool);
}
extern "C" {
    pub fn adf_rl_init(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_rl_start(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_rl_stop(accel_dev: *mut adf_accel_dev);
}
extern "C" {
    pub fn adf_rl_exit(accel_dev: *mut adf_accel_dev);
}
