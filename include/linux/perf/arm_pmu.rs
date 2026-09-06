//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/perf/arm_pmu.h
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
// linux/arch/arm/include/asm/pmu.h
//
// Copyright (C) 2009 picoChip Designs Ltd, Jamie Iles
//

//
// The Armv7 and Armv8.8 or less CPU PMU supports up to 32 event counters.
// The Armv8.9/9.4 CPU PMU supports up to 33 event counters.
//

pub const ARMPMU_MAX_HWEVENTS: c_int = 32;

pub const ARMPMU_MAX_HWEVENTS: c_int = 33;

//
// ARM PMU hw_event flags
//
pub const ARMPMU_EVT_64BIT: c_uint = 0x00001 /* Event uses a 64bit counter */;
pub const ARMPMU_EVT_47BIT: c_uint = 0x00002 /* Event uses a 47bit counter */;
pub const ARMPMU_EVT_63BIT: c_uint = 0x00004 /* Event uses a 63bit counter */;
pub const HW_OP_UNSUPPORTED: c_uint = 0xFFFF;

pub const CACHE_OP_UNSUPPORTED: c_uint = 0xFFFF;

// The events for a given PMU register set.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmu_hw_events {
//
// The events that are active on the PMU for the given index.
//
    pub events: [*mut perf_event; ARMPMU_MAX_HWEVENTS],
//
// A 1 bit for an index indicates that the counter is being used for
// an event. A 0 means that the counter can be used.
//
    pub ARMPMU_MAX_HWEVENTS): DECLARE_BITMAP(used_mask,,
//
// When using percpu IRQs, we need a percpu dev_id. Place it here as we
// already have to allocate this struct per cpu.
//
    pub percpu_pmu: *mut arm_pmu,
    pub irq: c_int,
    pub branch_stack: *mut perf_branch_stack,
// Active events requesting branch records
    pub branch_users: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum armpmu_attr_groups {
    ARMPMU_ATTR_GROUP_COMMON,
    ARMPMU_ATTR_GROUP_EVENTS,
    ARMPMU_ATTR_GROUP_FORMATS,
    ARMPMU_ATTR_GROUP_CAPS,
    ARMPMU_NR_ATTR_GROUPS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_pmu {
    pub pmu: pmu,
    pub supported_cpus: cpumask_t,
    pub name: *mut c_char,
    pub pmu): *mut *mut irqreturn_t (handle_irq)(struct arm_pmu,
    pub event): *mut *mut void (enable)(struct perf_event,
    pub event): *mut *mut void (disable)(struct perf_event,
    pub event): *mut perf_event,
    pub event): *mut perf_event,
    pub attr): *mut perf_event_attr,
    pub event): *mut *mut u64 (read_counter)(struct perf_event,
    pub val): *mut *mut *mut void (write_counter)(struct perf_event event, u64,
    pub ): *mut *mut void (start)(struct arm_pmu,
    pub ): *mut *mut void (stop)(struct arm_pmu,
    pub ): *mut *mut void (reset)(void,
    pub event): *mut *mut int (map_event)(struct perf_event,
//
// Called by KVM to map the PMUv3 event space onto non-PMUv3 hardware.
//
    pub eventsel): *mut *mut int (map_pmuv3_event)(unsigned int,
    pub ARMPMU_MAX_HWEVENTS): DECLARE_BITMAP(cntr_mask,,
    pub /: *mut *mut bool secure_access; / 32-bit ARM only,
    pub plat_device: *mut platform_device,
    pub hw_events: *mut pmu_hw_events __percpu,
    pub node: hlist_node,
    pub cpu_pm_nb: notifier_block,
// the attr_groups array must be NULL-terminated
    pub 1]: *const *const attribute_group attr_groups[ARMPMU_NR_ATTR_GROUPS +,
// PMUv3 only
    pub pmuver: c_int,
    pub avoid_pmccntr: bool,
    pub reg_pmmir: u64,
    pub reg_brbidr: u64,
pub const ARMV8_PMUV3_MAX_COMMON_EVENTS: c_uint = 0x40;
    pub ARMV8_PMUV3_MAX_COMMON_EVENTS): DECLARE_BITMAP(pmceid_bitmap,,
pub const ARMV8_PMUV3_EXT_COMMON_EVENT_BASE: c_uint = 0x4000;
    pub ARMV8_PMUV3_MAX_COMMON_EVENTS): DECLARE_BITMAP(pmceid_ext_bitmap,,
// Only to be used by ACPI probing code
    pub acpi_cpuid: c_ulong,
}

extern "C" {
    pub fn armpmu_event_update(event: *mut perf_event) -> u64;
}
extern "C" {
    pub fn armpmu_event_set_period(event: *mut perf_event) -> c_int;
}
extern "C" {
    pub fn int(: *mut *mut armpmu_init_fn)(struct arm_pmu) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmu_probe_info {
    pub cpuid: c_uint,
    pub mask: c_uint,
    pub init: armpmu_init_fn,
}

extern "C" {
    pub fn arm_pmu_acpi_probe(init_fn: armpmu_init_fn) -> c_int;
}

extern "C" {
    pub fn kvm_host_pmu_init(pmu: *mut arm_pmu);
}

extern "C" {
    pub fn arm_pmu_irq_is_nmi() -> bool;
}
// Internal functions only for core arm_pmu code
extern "C" {
    pub fn armpmu_free(pmu: *mut arm_pmu);
}
extern "C" {
    pub fn armpmu_register(pmu: *mut arm_pmu) -> c_int;
}
extern "C" {
    pub fn armpmu_request_irq(armpmu: *mut *mut arm_pmu  __percpu, irq: c_int, cpu: c_int) -> c_int;
}
extern "C" {
    pub fn armpmu_free_irq(armpmu: *mut *mut arm_pmu  __percpu, irq: c_int, cpu: c_int);
}

// Why does everything I do descend into this?

