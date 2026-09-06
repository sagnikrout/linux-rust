//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/perf/arm_cspmu/arm_cspmu.h
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
// ARM CoreSight Architecture PMU driver.
// Copyright (c) 2022-2026, NVIDIA CORPORATION & AFFILIATES. All rights reserved.
//

// Default event format

// Default filter format

//
// This is the default event number for cycle count, if supported, since the
// ARM Coresight PMU specification does not define a standard event code
// for cycle count.
//

//
// The ARM Coresight PMU supports up to 256 event counters.
// If the counters are larger-than 32-bits, then the PMU includes at
// most 128 counters.
//
pub const ARM_CSPMU_MAX_HW_CNTRS: c_int = 256;
// The cycle counter, if implemented, is located at counter[31].
pub const ARM_CSPMU_CYCLE_CNTR_IDX: c_int = 31;
//
// CoreSight PMU Arch register offsets.
//
pub const PMEVCNTR_LO: c_uint = 0x0;
pub const PMEVCNTR_HI: c_uint = 0x4;
pub const PMEVTYPER: c_uint = 0x400;
pub const PMCCFILTR: c_uint = 0x47C;
pub const PMEVFILT2R: c_uint = 0x800;
pub const PMEVFILTR: c_uint = 0xA00;
pub const PMCNTENSET: c_uint = 0xC00;
pub const PMCNTEN: c_uint = 0xC10;
pub const PMCNTENCLR: c_uint = 0xC20;
pub const PMINTENSET: c_uint = 0xC40;
pub const PMINTENCLR: c_uint = 0xC60;
pub const PMOVSCLR: c_uint = 0xC80;
pub const PMOVSSET: c_uint = 0xCC0;
pub const PMIMPDEF: c_uint = 0xD80;
pub const PMCFGR: c_uint = 0xE00;
pub const PMCR: c_uint = 0xE04;
pub const PMIIDR: c_uint = 0xE08;
pub const PMCR_64: c_uint = 0xE10;
pub const PMDEVARCH: c_uint = 0xFBC;
pub const PMPIDR0: c_uint = 0xFE0;
pub const PMPIDR1: c_uint = 0xFE4;
pub const PMPIDR2: c_uint = 0xFE8;
pub const PMPIDR3: c_uint = 0xFEC;
pub const PMPIDR4: c_uint = 0xFD0;
// PMCFGR register field

// PMCR register field

// PMIIDR register field

// PMPIDR0 register field

// PMPIDR1 register field

// PMPIDR2 register field

// PMPIDR3 register field

// PMPIDR4 register field

// JEDEC-assigned JEP106 identification code
pub const ARM_CSPMU_IMPL_ID_NVIDIA: c_uint = 0x36B;
pub const ARM_CSPMU_IMPL_ID_AMPERE: c_uint = 0xA16;
// PMDEVARCH

// This tracks the events assigned to each counter in the PMU.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_cspmu_hw_events {
// The events that are active on the PMU for a given logical index.
    pub events: *mut perf_event,
//
// Each bit indicates a logical counter is being used (or not) for an
// event. If cycle counter is supported and there is a gap between
// regular and cycle counter, the last logical counter is mapped to
// cycle counter. Otherwise, logical and physical have 1-to-1 mapping.
//
    pub ARM_CSPMU_MAX_HW_CNTRS): DECLARE_BITMAP(used_ctrs,,
}

// Contains ops to query vendor/implementer specific attribute.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_cspmu_impl_ops {
// Get event attributes
    pub cspmu): *const *const *const *const attribute (get_event_attrs)(arm_cspmu,
// Get format attributes
    pub cspmu): *const *const *const *const attribute (get_format_attrs)(arm_cspmu,
// Get string identifier
    pub cspmu): *const *const *const char (get_identifier)(struct arm_cspmu,
// Get PMU name to register to core perf
    pub cspmu): *const *const *const char (get_name)(struct arm_cspmu,
// Check if the event corresponds to cycle count event
    pub event): *const *const bool (is_cycle_counter_event)(struct perf_event,
// Decode event type/id from configs
    pub event): *const *const u64 (event_type)(struct perf_event,
// Set/reset event filters
    pub event): *const perf_event,
    pub event): *const perf_event,
    pub event): *const perf_event,
// Implementation specific event validation
    pub event): *mut perf_event,
// Hide/show unsupported events
    pub unused): *mut *mut attribute attr, int,
}

// Vendor/implementer registration parameter.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_cspmu_impl_match {
// Backend module.
    pub module: *mut module,
    pub module_name: *const c_char,
// PMIIDR value/mask.
    pub pmiidr_val: u32,
    pub pmiidr_mask: u32,
// Callback to vendor backend to init arm_cspmu_impl::ops.
    pub cspmu): *mut *mut int (impl_init_ops)(struct arm_cspmu,
}

// Vendor/implementer descriptor.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_cspmu_impl {
    pub pmiidr: u32,
    pub module: *mut module,
    pub match: *mut arm_cspmu_impl_match,
    pub ops: arm_cspmu_impl_ops,
    pub ctx: *mut c_void,
}

// Coresight PMU descriptor.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_cspmu {
    pub pmu: pmu,
    pub dev: *mut device,
    pub name: *const c_char,
    pub identifier: *const c_char,
    pub base0: *mut void __iomem,
    pub base1: *mut void __iomem,
    pub associated_cpus: cpumask_t,
    pub active_cpu: cpumask_t,
    pub cpuhp_node: hlist_node,
    pub irq: c_int,
    pub has_atomic_dword: bool,
    pub has_ext64: bool,
    pub pmcfgr: u32,
    pub num_logical_ctrs: u32,
    pub num_set_clr_reg: u32,
    pub cycle_counter_logical_idx: c_int,
    pub hw_events: arm_cspmu_hw_events,
    pub attr_groups: [*const attribute_group; 5],
    pub impl: arm_cspmu_impl,
}

// Default function to show event attribute in sysfs.
// Register vendor backend.
extern "C" {
    pub fn arm_cspmu_impl_register(impl_match: *const arm_cspmu_impl_match) -> c_int;
}
// Unregister vendor backend.
extern "C" {
    pub fn arm_cspmu_impl_unregister(impl_match: *const arm_cspmu_impl_match);
}

//
// Get ACPI device associated with the PMU.
// The caller is responsible for calling acpi_dev_put() on the returned device.
//

