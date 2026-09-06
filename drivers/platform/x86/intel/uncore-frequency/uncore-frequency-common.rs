//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/intel/uncore-frequency/uncore-frequency-common.h
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
// Intel Uncore Frequency Control: Common defines and prototypes
// Copyright (c) 2022, Intel Corporation.
// All rights reserved.
//

//
// Define uncore agents, which are under uncore frequency control.
// Defined in the same order as specified in the TPMI UFS Specifications.
// It is possible that there are common uncore frequency control to more than
// one hardware agents. So, these defines are used as a bit mask.
//
pub const AGENT_TYPE_CORE: c_uint = 0x01;
pub const AGENT_TYPE_CACHE: c_uint = 0x02;
pub const AGENT_TYPE_MEMORY: c_uint = 0x04;
pub const AGENT_TYPE_IO: c_uint = 0x08;
//
// struct uncore_data - Encapsulate all uncore data
// @stored_uncore_data: Last user changed MSR 620 value, which will be restored
// on system resume.
// @initial_min_freq_khz: Sampled minimum uncore frequency at driver init
// @initial_max_freq_khz: Sampled maximum uncore frequency at driver init
// @control_cpu:	Designated CPU for a die to read/write
// @valid:		Mark the data valid/invalid
// @package_id:	Package id for this instance
// @die_id:		Die id for this instance
// @domain_id:		Power domain id for this instance
// @cluster_id:		cluster id in a domain
// @seqnum_id:		Unique sequential id to append to directory name
// @instance_id:	Die indices or feature instances for a single TPMI device
// @name:		Sysfs entry name for this instance
// @agent_type_mask:	Bit mask of all hardware agents for this domain
// @uncore_attr_group:	Attribute group storage
// @max_freq_khz_kobj_attr: Storage for kobject attribute max_freq_khz
// @min_freq_khz_kobj_attr: Storage for kobject attribute min_freq_khz
// @initial_max_freq_khz_kobj_attr: Storage for kobject attribute initial_max_freq_khz
// @initial_min_freq_khz_kobj_attr: Storage for kobject attribute initial_min_freq_khz
// @current_freq_khz_kobj_attr: Storage for kobject attribute current_freq_khz
// @domain_id_kobj_attr: Storage for kobject attribute domain_id
// @fabric_cluster_id_kobj_attr: Storage for kobject attribute fabric_cluster_id
// @package_id_kobj_attr: Storage for kobject attribute package_id
// @elc_low_threshold_percent_kobj_attr:
// Storage for kobject attribute elc_low_threshold_percent
// @elc_high_threshold_percent_kobj_attr:
// Storage for kobject attribute elc_high_threshold_percent
// @elc_high_threshold_enable_kobj_attr:
// Storage for kobject attribute elc_high_threshold_enable
// @elc_floor_freq_khz_kobj_attr: Storage for kobject attribute elc_floor_freq_khz
// @agent_types_kobj_attr: Storage for kobject attribute agent_type
// @die_id_kobj_attr:	Attribute storage for die_id information
// @instance_id_kobj_attr: Attribute storage for instance_id value
// @uncore_attrs:	Attribute storage for group creation
//
// This structure is used to encapsulate all data related to uncore sysfs
// settings for a die/package.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uncore_data {
    pub stored_uncore_data: u64,
    pub initial_min_freq_khz: u32,
    pub initial_max_freq_khz: u32,
    pub control_cpu: c_int,
    pub valid: bool,
    pub package_id: c_int,
    pub die_id: c_int,
    pub domain_id: c_int,
    pub cluster_id: c_int,
    pub seqnum_id: c_int,
    pub instance_id: c_int,
    pub name: [c_char; 32],
    pub agent_type_mask: u16,
    pub uncore_attr_group: attribute_group,
    pub max_freq_khz_kobj_attr: kobj_attribute,
    pub min_freq_khz_kobj_attr: kobj_attribute,
    pub initial_max_freq_khz_kobj_attr: kobj_attribute,
    pub initial_min_freq_khz_kobj_attr: kobj_attribute,
    pub current_freq_khz_kobj_attr: kobj_attribute,
    pub domain_id_kobj_attr: kobj_attribute,
    pub fabric_cluster_id_kobj_attr: kobj_attribute,
    pub package_id_kobj_attr: kobj_attribute,
    pub elc_low_threshold_percent_kobj_attr: kobj_attribute,
    pub elc_high_threshold_percent_kobj_attr: kobj_attribute,
    pub elc_high_threshold_enable_kobj_attr: kobj_attribute,
    pub elc_floor_freq_khz_kobj_attr: kobj_attribute,
    pub agent_types_kobj_attr: kobj_attribute,
    pub die_id_kobj_attr: kobj_attribute,
    pub instance_id_kobj_attr: kobj_attribute,
    pub uncore_attrs: [*mut attribute; 16],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uncore_index {
    UNCORE_INDEX_MIN_FREQ,
    UNCORE_INDEX_MAX_FREQ,
    UNCORE_INDEX_CURRENT_FREQ,
    UNCORE_INDEX_EFF_LAT_CTRL_LOW_THRESHOLD,
    UNCORE_INDEX_EFF_LAT_CTRL_HIGH_THRESHOLD,
    UNCORE_INDEX_EFF_LAT_CTRL_HIGH_THRESHOLD_ENABLE,
    UNCORE_INDEX_EFF_LAT_CTRL_FREQ,
    UNCORE_INDEX_DIE_ID,
}

extern "C" {
    pub fn uncore_freq_common_exit();
}
extern "C" {
    pub fn uncore_freq_add_entry(data: *mut uncore_data, cpu: c_int) -> c_int;
}
extern "C" {
    pub fn uncore_freq_remove_die_entry(data: *mut uncore_data);
}
