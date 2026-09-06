//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thermal/intel/intel_soc_dts_iosf.h
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
// intel_soc_dts_iosf.h
// Copyright (c) 2015, Intel Corporation.
//

// DTS0 and DTS 1
pub const SOC_MAX_DTS_SENSORS: c_int = 2;
// Only 2 out of 4 is allowed for OSPM
pub const SOC_MAX_DTS_TRIPS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_soc_dts_interrupt_type {
    INTEL_SOC_DTS_INTERRUPT_NONE,
    INTEL_SOC_DTS_INTERRUPT_APIC,
    INTEL_SOC_DTS_INTERRUPT_MSI,
    INTEL_SOC_DTS_INTERRUPT_SCI,
    INTEL_SOC_DTS_INTERRUPT_SMI,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_soc_dts_sensor_entry {
    pub id: c_int,
    pub store_status: u32,
    pub tzone: *mut thermal_zone_device,
    pub sensors: *mut intel_soc_dts_sensors,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_soc_dts_sensors {
    pub tj_max: u32,
    pub intr_notify_lock: spinlock_t,
    pub dts_update_lock: mutex,
    pub intr_type: intel_soc_dts_interrupt_type,
    pub soc_dts: [intel_soc_dts_sensor_entry; SOC_MAX_DTS_SENSORS],
}

extern "C" {
    pub fn intel_soc_dts_iosf_exit(sensors: *mut intel_soc_dts_sensors);
}
