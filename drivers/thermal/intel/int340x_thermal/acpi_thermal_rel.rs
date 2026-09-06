//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thermal/intel/int340x_thermal/acpi_thermal_rel.h
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
// ACPI_THERMAL_GET_PSVT_COUNT = Number of PSVT entries
// ACPI_THERMAL_GET_PSVT_LEN = Total return data size (PSVT count x each
// PSVT entry size)
// ACPI_THERMAL_GET_PSVT = Get the data as an array of psvt_objects
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct art {
    pub source: acpi_handle,
    pub target: acpi_handle,
    pub weight: u64,
    pub ac0_max: u64,
    pub ac1_max: u64,
    pub ac2_max: u64,
    pub ac3_max: u64,
    pub ac4_max: u64,
    pub ac5_max: u64,
    pub ac6_max: u64,
    pub ac7_max: u64,
    pub ac8_max: u64,
    pub ac9_max: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trt {
    pub source: acpi_handle,
    pub target: acpi_handle,
    pub influence: u64,
    pub sample_period: u64,
    pub reserved1: u64,
    pub reserved2: u64,
    pub reserved3: u64,
    pub reserved4: u64,
    pub __packed: },
pub const ACPI_NR_PSVT_ELEMENTS: c_int = 12;
pub const ACPI_PSVT_CONTROL_KNOB: c_int = 7;
pub const ACPI_LIMIT_STR_MAX_LEN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psvt {
    pub source: acpi_handle,
    pub target: acpi_handle,
    pub priority: u64,
    pub sample_period: u64,
    pub passive_temp: u64,
    pub source_domain: u64,
    pub control_knob: u64,
// For limit_type = ACPI_TYPE_INTEGER
    pub integer: u64,
// For limit_type = ACPI_TYPE_STRING
    pub string: [c_char; ACPI_LIMIT_STR_MAX_LEN],
    pub str_ptr: *mut c_char,
    pub limit: },
    pub step_size: u64,
    pub limit_coeff: u64,
    pub unlimit_coeff: u64,
// Spec calls this field reserved, so we borrow it for type info
    pub /: *mut *mut u64 control_knob_type; / ACPI_TYPE_STRING or ACPI_TYPE_INTEGER,
    pub __packed: },
pub const ACPI_NR_ART_ELEMENTS: c_int = 13;
// for usrspace
#[repr(C)]
#[derive(Copy, Clone)]
pub union art_object {
    pub /: *mut *mut char source_device[8]; / ACPI single name,
    pub /: *mut *mut char target_device[8]; / ACPI single name,
    pub weight: u64,
    pub ac0_max_level: u64,
    pub ac1_max_level: u64,
    pub ac2_max_level: u64,
    pub ac3_max_level: u64,
    pub ac4_max_level: u64,
    pub ac5_max_level: u64,
    pub ac6_max_level: u64,
    pub ac7_max_level: u64,
    pub ac8_max_level: u64,
    pub ac9_max_level: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union trt_object {
    pub /: *mut *mut char source_device[8]; / ACPI single name,
    pub /: *mut *mut char target_device[8]; / ACPI single name,
    pub influence: u64,
    pub sample_period: u64,
    pub reserved: [u64; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union psvt_object {
    pub source_device: [c_char; 8],
    pub target_device: [c_char; 8],
    pub priority: u64,
    pub sample_period: u64,
    pub passive_temp: u64,
    pub source_domain: u64,
    pub control_knob: u64,
    pub integer: u64,
    pub string: [c_char; ACPI_LIMIT_STR_MAX_LEN],
    pub limit: },
    pub step_size: u64,
    pub limit_coeff: u64,
    pub unlimit_coeff: u64,
    pub control_knob_type: u64,
}

extern "C" {
    pub fn acpi_thermal_rel_misc_device_add(handle: acpi_handle) -> c_int;
}
extern "C" {
    pub fn acpi_thermal_rel_misc_device_remove(handle: acpi_handle) -> c_int;
}

