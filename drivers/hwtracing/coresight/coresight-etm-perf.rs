//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/coresight/coresight-etm-perf.h
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
// Copyright(C) 2015 Linaro Limited. All rights reserved.
// Author: Mathieu Poirier <mathieu.poirier@linaro.org>
//

//
// In both ETMv3 and v4 the maximum number of address comparator implentable
// is 8.  The actual number is implementation specific and will be checked
// when filters are applied.
//
pub const ETM_ADDR_CMP_MAX: c_int = 8;

pub const ATTR_CFG_FLD_preset_LO: c_int = 0;
pub const ATTR_CFG_FLD_preset_HI: c_int = 3;

pub const ATTR_CFG_FLD_timestamp_LO: c_int = 4;
pub const ATTR_CFG_FLD_timestamp_HI: c_int = 7;

pub const ATTR_CFG_FLD_branch_broadcast_LO: c_int = 8;
pub const ATTR_CFG_FLD_branch_broadcast_HI: c_int = 8;

pub const ATTR_CFG_FLD_cycacc_LO: c_int = 12;
pub const ATTR_CFG_FLD_cycacc_HI: c_int = 12;

pub const ATTR_CFG_FLD_contextid1_LO: c_int = 14;
pub const ATTR_CFG_FLD_contextid1_HI: c_int = 14;

pub const ATTR_CFG_FLD_contextid2_LO: c_int = 15;
pub const ATTR_CFG_FLD_contextid2_HI: c_int = 15;
//
// Old position of 'timestamp' and not published in sysfs. Remove at a later
// date if necessary.
//

pub const ATTR_CFG_FLD_deprecated_timestamp_LO: c_int = 28;
pub const ATTR_CFG_FLD_deprecated_timestamp_HI: c_int = 28;

pub const ATTR_CFG_FLD_retstack_LO: c_int = 29;
pub const ATTR_CFG_FLD_retstack_HI: c_int = 29;

pub const ATTR_CFG_FLD_sinkid_LO: c_int = 0;
pub const ATTR_CFG_FLD_sinkid_HI: c_int = 31;

pub const ATTR_CFG_FLD_configid_LO: c_int = 32;
pub const ATTR_CFG_FLD_configid_HI: c_int = 63;

pub const ATTR_CFG_FLD_cc_threshold_LO: c_int = 0;
pub const ATTR_CFG_FLD_cc_threshold_HI: c_int = 11;
//
// struct etm_filter - single instruction range or start/stop configuration.
// @start_addr:	The address to start tracing on.
// @stop_addr:	The address to stop tracing on.
// @type:	Is this a range or start/stop filter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct etm_filter {
    pub start_addr: c_ulong,
    pub stop_addr: c_ulong,
    pub type: etm_addr_type,
}

//
// struct etm_filters - set of filters for a session
// @etm_filter:	All the filters for this session.
// @nr_filters:	Number of filters
// @ssstatus:	Status of the start/stop logic.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct etm_filters {
    pub etm_filter: [etm_filter; ETM_ADDR_CMP_MAX],
    pub nr_filters: c_uint,
    pub ssstatus: bool,
}

//
// struct etm_event_data - Coresight specifics associated to an event
// @work:		Handle to free allocated memory outside IRQ context.
// @mask:		Hold the CPU(s) this event was set for.
// @aux_hwid_done:	Whether a CPU has emitted the TraceID packet or not.
// @snk_config:		The sink configuration.
// @cfg_hash:		The hash id of any coresight config selected.
// @path:		An array of path, each slot for one CPU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct etm_event_data {
    pub work: work_struct,
    pub mask: cpumask_t,
    pub aux_hwid_done: cpumask_t,
    pub snk_config: *mut c_void,
    pub cfg_hash: u32,
    pub path: *mut *mut coresight_path  __percpu,
}

extern "C" {
    pub fn etm_perf_symlink(csdev: *mut coresight_device, link: bool) -> c_int;
}
extern "C" {
    pub fn etm_perf_add_symlink_sink(csdev: *mut coresight_device) -> c_int;
}
extern "C" {
    pub fn etm_perf_del_symlink_sink(csdev: *mut coresight_device);
}
extern "C" {
    pub fn etm_perf_del_symlink_cscfg(config_desc: *mut cscfg_config_desc);
}
extern "C" {
    pub fn etm_perf_init() -> int __init;
}
extern "C" {
    pub fn etm_perf_exit();
}
