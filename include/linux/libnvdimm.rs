//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/libnvdimm.h
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
// libnvdimm - Non-volatile-memory Devices Subsystem
//
// Copyright(c) 2013-2015 Intel Corporation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct badrange_entry {
    pub start: u64,
    pub length: u64,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct badrange {
    pub list: list_head,
    pub lock: spinlock_t,
}

// unarmed memory devices may not persist writes
// locked memory devices should not be accessed
// memory under security wipes should not be accessed
// tracking whether or not there is a pending device reference
// dimm supports namespace labels
//
// dimm contents have changed requiring invalidation of CPU caches prior
// to activation of a region that includes this device
//
// dimm provider wants synchronous registration by __nvdimm_create()
// need to set a limit somewhere, but yes, this is likely overkill
// region flag indicating to direct-map persistent memory by default
//
// Platform ensures entire CPU store data path is flushed to pmem on
// system power loss.
//
// Platform provides mechanisms to automatically flush outstanding
// write data from memory controler to pmem on system power loss.
// (ADR)
//
// Platform provides asynchronous flush mechanism
// Region was created by CXL subsystem
// mark newly adjusted resources as requiring a label update
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvdimm_bus_descriptor {
    pub attr_groups: *const attribute_group,
    pub cmd_mask: c_ulong,
    pub dimm_family_mask: c_ulong,
    pub bus_family_mask: c_ulong,
    pub module: *mut module,
    pub provider_name: *mut c_char,
    pub of_node: *mut device_node,
    pub ndctl: ndctl_fn,
    pub nd_desc): *mut *mut int (flush_probe)(struct nvdimm_bus_descriptor,
    pub data): *mut *mut nvdimm nvdimm, unsigned int cmd, void,
    pub fw_ops: *const nvdimm_bus_fw_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_cmd_desc {
    pub in_num: c_int,
    pub out_num: c_int,
    pub in_sizes: [u32; ND_CMD_MAX_ELEM],
    pub out_sizes: [c_int; ND_CMD_MAX_ELEM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_interleave_set {
// v1.1 definition of the interleave-set-cookie algorithm
    pub cookie1: u64,
// v1.2 definition of the interleave-set-cookie algorithm
    pub cookie2: u64,
// compatibility with initial buggy Linux implementation
    pub altcookie: u64,
    pub type_guid: guid_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_mapping_desc {
    pub nvdimm: *mut nvdimm,
    pub start: u64,
    pub size: u64,
    pub position: c_int,
}

//
// Provider flush callback return values:
// 0: flush completed synchronously
// <0: flush failed
// >0: flush completion was queued and @bio will be completed later
//
pub const NVDIMM_FLUSH_ASYNC: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_region_desc {
    pub res: *mut resource,
    pub mapping: *mut nd_mapping_desc,
    pub num_mappings: u16,
    pub attr_groups: *const attribute_group,
    pub nd_set: *mut nd_interleave_set,
    pub provider_data: *mut c_void,
    pub num_lanes: c_int,
    pub numa_node: c_int,
    pub target_node: c_int,
    pub flags: c_ulong,
    pub memregion: c_int,
    pub of_node: *mut device_node,
    pub bio): *mut *mut *mut int (flush)(struct nd_region nd_region, struct bio,
}

//
// Note that separate bits for locked + unlocked are defined so that
// 'flags == 0' corresponds to an error / not-supported state.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvdimm_security_bits {
    NVDIMM_SECURITY_DISABLED,
    NVDIMM_SECURITY_UNLOCKED,
    NVDIMM_SECURITY_LOCKED,
    NVDIMM_SECURITY_FROZEN,
    NVDIMM_SECURITY_OVERWRITE,
}

pub const NVDIMM_PASSPHRASE_LEN: c_int = 32;
pub const NVDIMM_KEY_DESC_LEN: c_int = 22;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvdimm_key_data {
    pub data: [u8; NVDIMM_PASSPHRASE_LEN],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvdimm_passphrase_type {
    NVDIMM_USER,
    NVDIMM_MASTER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvdimm_security_ops {
    pub pass_type): nvdimm_passphrase_type,
    pub nvdimm): *mut *mut int (freeze)(struct nvdimm,
    pub pass_type): nvdimm_passphrase_type,
    pub key_data): *const nvdimm_key_data,
    pub key_data): *const nvdimm_key_data,
    pub pass_type): nvdimm_passphrase_type,
    pub key_data): *const nvdimm_key_data,
    pub nvdimm): *mut *mut int (query_overwrite)(struct nvdimm,
    pub key_data): *const nvdimm_key_data,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvdimm_fwa_state {
    NVDIMM_FWA_INVALID,
    NVDIMM_FWA_IDLE,
    NVDIMM_FWA_ARMED,
    NVDIMM_FWA_BUSY,
    NVDIMM_FWA_ARM_OVERFLOW,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvdimm_fwa_trigger {
    NVDIMM_FWA_ARM,
    NVDIMM_FWA_DISARM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvdimm_fwa_capability {
    NVDIMM_FWA_CAP_INVALID,
    NVDIMM_FWA_CAP_NONE,
    NVDIMM_FWA_CAP_QUIESCE,
    NVDIMM_FWA_CAP_LIVE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvdimm_fwa_result {
    NVDIMM_FWA_RESULT_INVALID,
    NVDIMM_FWA_RESULT_NONE,
    NVDIMM_FWA_RESULT_SUCCESS,
    NVDIMM_FWA_RESULT_NOTSTAGED,
    NVDIMM_FWA_RESULT_NEEDRESET,
    NVDIMM_FWA_RESULT_FAIL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvdimm_bus_fw_ops {
    pub nd_desc): *mut (struct nvdimm_bus_descriptor,
    pub nd_desc): *mut (struct nvdimm_bus_descriptor,
    pub nd_desc): *mut *mut int (activate)(struct nvdimm_bus_descriptor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvdimm_fw_ops {
    pub nvdimm): *mut *mut nvdimm_fwa_state (activate_state)(struct nvdimm,
    pub nvdimm): *mut *mut nvdimm_fwa_result (activate_result)(struct nvdimm,
    pub arg): *mut *mut *mut int (arm)(struct nvdimm nvdimm, enum nvdimm_fwa_trigger,
}

extern "C" {
    pub fn badrange_init(badrange: *mut badrange);
}
extern "C" {
    pub fn badrange_add(badrange: *mut badrange, addr: u64, length: u64) -> c_int;
}
extern "C" {
    pub fn nvdimm_bus_unregister(nvdimm_bus: *mut nvdimm_bus);
}
extern "C" {
    pub fn nvdimm_cmd_mask(nvdimm: *mut nvdimm) -> c_ulong;
}
extern "C" {
    pub fn nvdimm_delete(nvdimm: *mut nvdimm);
}
extern "C" {
    pub fn nvdimm_region_delete(nd_region: *mut nd_region);
}
extern "C" {
    pub fn nvdimm_bus_check_dimm_count(nvdimm_bus: *mut nvdimm_bus, dimm_count: c_int) -> c_int;
}
extern "C" {
    pub fn nd_region_acquire_lane(nd_region: *mut nd_region) -> c_uint;
}
extern "C" {
    pub fn nd_region_release_lane(nd_region: *mut nd_region, lane: c_uint);
}
extern "C" {
    pub fn nd_fletcher64(addr: *mut c_void, len: usize, le: bool) -> u64;
}
extern "C" {
    pub fn nvdimm_flush(nd_region: *mut nd_region, bio: *mut bio) -> c_int;
}
extern "C" {
    pub fn generic_nvdimm_flush(nd_region: *mut nd_region) -> c_int;
}
extern "C" {
    pub fn nvdimm_has_flush(nd_region: *mut nd_region) -> c_int;
}
extern "C" {
    pub fn nvdimm_has_cache(nd_region: *mut nd_region) -> c_int;
}
extern "C" {
    pub fn nvdimm_in_overwrite(nvdimm: *mut nvdimm) -> c_int;
}
extern "C" {
    pub fn is_nvdimm_sync(nd_region: *mut nd_region) -> bool;
}

extern "C" {
    pub fn arch_wb_cache_pmem(addr: *mut c_void, size: usize);
}
extern "C" {
    pub fn arch_invalidate_pmem(addr: *mut c_void, size: usize);
}

