//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/perf/hisilicon/hisi_uncore_pmu.h
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
// HiSilicon SoC Hardware event counters support
//
// Copyright (C) 2017 HiSilicon Limited
// Author: Anurup M <anurup.m@huawei.com>
// Shaokun Zhang <zhangshaokun@hisilicon.com>
//
// This code is based on the uncore PMUs like arm-cci and arm-ccn.
//

pub const HISI_PMU_V2: c_uint = 0x30;
pub const HISI_MAX_COUNTERS: c_uint = 0x18;

pub const HISI_PMU_EVTYPE_BITS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_uncore_ops {
    pub event): *mut *mut int (check_filter)(struct perf_event,
    pub u32): *mut *mut *mut void (write_evtype)(struct hisi_pmu , int,,
    pub ): *mut *mut int (get_event_idx)(struct perf_event,
    pub ): *mut *mut *mut u64 (read_counter)(struct hisi_pmu , struct hw_perf_event,
    pub u64): *mut *mut *mut *mut void (write_counter)(struct hisi_pmu , struct hw_perf_event ,,
    pub ): *mut *mut *mut void (enable_counter)(struct hisi_pmu , struct hw_perf_event,
    pub ): *mut *mut *mut void (disable_counter)(struct hisi_pmu , struct hw_perf_event,
    pub ): *mut *mut *mut void (enable_counter_int)(struct hisi_pmu , struct hw_perf_event,
    pub ): *mut *mut *mut void (disable_counter_int)(struct hisi_pmu , struct hw_perf_event,
    pub ): *mut *mut void (start_counters)(struct hisi_pmu,
    pub ): *mut *mut void (stop_counters)(struct hisi_pmu,
    pub hisi_pmu): *mut *mut u32 (get_int_status)(struct hisi_pmu,
    pub idx): *mut *mut *mut void (clear_int_status)(struct hisi_pmu hisi_pmu, int,
    pub event): *mut *mut void (enable_filter)(struct perf_event,
    pub event): *mut *mut void (disable_filter)(struct perf_event,
}

// Describes the HISI PMU chip features information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_pmu_dev_info {
    pub name: *const c_char,
    pub attr_groups: *const attribute_group,
    pub counter_bits: u32,
    pub check_event: u32,
    pub private: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_pmu_hwevents {
    pub hw_events: [*mut perf_event; HISI_MAX_COUNTERS],
    pub HISI_MAX_COUNTERS): DECLARE_BITMAP(used_mask,,
    pub attr_groups: *const attribute_group,
}

//
// struct hisi_pmu_topology - Describe the topology hierarchy on which the PMU
// is located.
// @sccl_id: ID of the SCCL on which the PMU locate is located.
// @sicl_id: ID of the SICL on which the PMU locate is located.
// @scl_id:  ID used by the core which is unaware of the SCCL/SICL.
// @ccl_id: ID of the CCL (CPU cluster) on which the PMU is located.
// @index_id: the ID of the PMU module if there're several PMUs at a
// particularly location in the topology.
// @sub_id: submodule ID of the PMU. For example we use this for DDRC PMU v2
// since each DDRC has more than one DMC
//
// The ID will be -1 if the PMU isn't located on a certain topology.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_pmu_topology {
//
// SCCL (Super CPU CLuster) and SICL (Super I/O Cluster) are parallel
// so a PMU cannot locate on a SCCL and a SICL. If the SCCL/SICL
// distinction is not relevant, use scl_id instead.
//
    pub sccl_id: c_int,
    pub sicl_id: c_int,
    pub scl_id: c_int,
}

// Generic pmu struct for different pmu types
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_pmu {
    pub pmu: pmu,
    pub ops: *const hisi_uncore_ops,
    pub dev_info: *const hisi_pmu_dev_info,
    pub pmu_events: hisi_pmu_hwevents,
    pub topo: hisi_pmu_topology,
//
// CPUs associated to the PMU and are preferred to use for counting.
// Could be empty if PMU has no association (e.g. PMU on SICL), in
// which case any online CPU will be used.
//
    pub associated_cpus: cpumask_t,
// CPU used for counting
    pub on_cpu: c_int,
    pub irq: c_int,
    pub dev: *mut device,
    pub node: hlist_node,
    pub base: *mut void __iomem,
    pub num_counters: c_int,
    pub counter_bits: c_int,
// check event code range
    pub check_event: c_int,
    pub identifier: u32,
}

// Generic implementation of cpumask/identifier group
extern "C" {
    pub fn hisi_uncore_pmu_get_event_idx(event: *mut perf_event) -> c_int;
}
extern "C" {
    pub fn hisi_uncore_pmu_read(event: *mut perf_event);
}
extern "C" {
    pub fn hisi_uncore_pmu_add(event: *mut perf_event, flags: c_int) -> c_int;
}
extern "C" {
    pub fn hisi_uncore_pmu_del(event: *mut perf_event, flags: c_int);
}
extern "C" {
    pub fn hisi_uncore_pmu_start(event: *mut perf_event, flags: c_int);
}
extern "C" {
    pub fn hisi_uncore_pmu_stop(event: *mut perf_event, flags: c_int);
}
extern "C" {
    pub fn hisi_uncore_pmu_set_event_period(event: *mut perf_event);
}
extern "C" {
    pub fn hisi_uncore_pmu_event_update(event: *mut perf_event);
}
extern "C" {
    pub fn hisi_uncore_pmu_event_init(event: *mut perf_event) -> c_int;
}
extern "C" {
    pub fn hisi_uncore_pmu_enable(pmu: *mut pmu);
}
extern "C" {
    pub fn hisi_uncore_pmu_disable(pmu: *mut pmu);
}
extern "C" {
    pub fn hisi_uncore_pmu_online_cpu(cpu: c_uint, node: *mut hlist_node) -> c_int;
}
extern "C" {
    pub fn hisi_uncore_pmu_offline_cpu(cpu: c_uint, node: *mut hlist_node) -> c_int;
}
extern "C" {
    pub fn hisi_uncore_pmu_isr(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn hisi_uncore_pmu_init_topology(hisi_pmu: *mut hisi_pmu, dev: *mut device);
}
extern "C" {
    pub fn hisi_pmu_init(hisi_pmu: *mut hisi_pmu, module: *mut module);
}
