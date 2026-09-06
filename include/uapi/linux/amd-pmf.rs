//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/amd-pmf.h
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


// SPDX-License-Identifier: GPL-2.0-or-later WITH Linux-syscall-note
//
// AMD Platform Management Framework (PMF) UAPI Header
//
// Copyright (c) 2026, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// This file defines the user-space API for interacting with the AMD PMF
// driver. It provides ioctl interfaces to query platform-specific metrics
// such as power source, slider position, platform type, laptop placement,
// and various BIOS input/output parameters.
//

//
// AMD_PMF_IOC_MAGIC - Magic number for AMD PMF ioctl commands
//
// This magic number uniquely identifies AMD PMF ioctl operations.
//

//
// IOCTL_AMD_PMF_POPULATE_DATA - ioctl command to retrieve PMF metrics data
//
// This ioctl command is used to populate the amd_pmf_info structure
// with the requested PMF metrics information.
//

pub const AMD_PMF_BIOS_PARAMS_MAX: c_int = 10;
// AMD PMF feature flags - bitmask indicating supported features

//
// enum amd_pmf_laptop_placement - Describes the physical placement of the laptop
// @AMD_PMF_LP_UNKNOWN: Placement cannot be determined
// @AMD_PMF_ON_TABLE: Laptop is placed on a stable surface like a table or desk
// @AMD_PMF_ON_LAP_MOTION: Laptop is on a lap with detected motion
// @AMD_PMF_IN_BAG: Laptop is detected to be inside a bag or case
// @AMD_PMF_OUT_OF_BAG: Laptop has been removed from bag or case
// @AMD_PMF_LP_UNDEFINED: Placement state is undefined
//
// This enumeration represents the physical placement state of the laptop
// as detected by platform sensors. Used for adaptive power management
// and thermal policies.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_pmf_laptop_placement {
    AMD_PMF_LP_UNKNOWN,
    AMD_PMF_ON_TABLE,
    AMD_PMF_ON_LAP_MOTION,
    AMD_PMF_IN_BAG,
    AMD_PMF_OUT_OF_BAG,
    AMD_PMF_LP_UNDEFINED,
}

//
// enum amd_pmf_ta_slider - Trusted Application power slider positions
// @AMD_PMF_TA_BEST_BATTERY: Maximum battery savings, minimal performance
// @AMD_PMF_TA_BETTER_BATTERY: Balanced towards battery life
// @AMD_PMF_TA_BETTER_PERFORMANCE: Balanced towards performance
// @AMD_PMF_TA_BEST_PERFORMANCE: Maximum performance, higher power consumption
// @AMD_PMF_TA_MAX: Sentinel value indicating maximum enum value
//
// This enumeration defines the power slider positions used by the
// AMD PMF Trusted Application for dynamic power management decisions.
// These correspond to the Windows power slider UI positions.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_pmf_ta_slider {
    AMD_PMF_TA_BEST_BATTERY,
    AMD_PMF_TA_BETTER_BATTERY,
    AMD_PMF_TA_BETTER_PERFORMANCE,
    AMD_PMF_TA_BEST_PERFORMANCE,
    AMD_PMF_TA_MAX,
}

//
// enum amd_pmf_platform_type - Describes the physical form factor orientation
// @AMD_PMF_PTYPE_UNKNOWN: Platform type cannot be determined
// @AMD_PMF_LID_CLOSE: Laptop lid is closed
// @AMD_PMF_CLAMSHELL: Traditional laptop mode with keyboard and screen
// @AMD_PMF_FLAT: Device is lying flat on a surface
// @AMD_PMF_TENT: Device is in tent mode (keyboard folded back, standing)
// @AMD_PMF_STAND: Device is propped up in stand orientation
// @AMD_PMF_TABLET: Device is in tablet mode with keyboard hidden
// @AMD_PMF_BOOK: Device is in book reading orientation
// @AMD_PMF_PRESENTATION: Device is in presentation mode
// @AMD_PMF_PULL_FWD: Screen is pulled forward towards user
// @AMD_PMF_PTYPE_INVALID: Invalid platform type marker
//
// This enumeration describes the current physical orientation or form
// factor of convertible/2-in-1 devices. Used for optimizing power and
// thermal management based on device posture.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_pmf_platform_type {
    AMD_PMF_PTYPE_UNKNOWN,
    AMD_PMF_LID_CLOSE,
    AMD_PMF_CLAMSHELL,
    AMD_PMF_FLAT,
    AMD_PMF_TENT,
    AMD_PMF_STAND,
    AMD_PMF_TABLET,
    AMD_PMF_BOOK,
    AMD_PMF_PRESENTATION,
    AMD_PMF_PULL_FWD,
    AMD_PMF_PTYPE_INVALID = 0xf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_pmf_info {
    pub size: __u64,
// Feature info
    pub features_supported: __u32,
// Power and state info
    pub platform_type: __u32,
    pub power_source: __u32,
    pub laptop_placement: __u32,
    pub lid_state: __u32,
    pub user_presence: __u32,
    pub slider_position: __u32,
// Thermal and power metrics
    pub skin_temp: __s32,
    pub gfx_busy: __u32,
    pub ambient_light: __s32,
    pub avg_c0_residency: __u32,
    pub max_c0_residency: __u32,
    pub socket_power: __u32,
// BIOS parameters
    pub bios_input: [__u32; AMD_PMF_BIOS_PARAMS_MAX],
    pub bios_output: [__u32; AMD_PMF_BIOS_PARAMS_MAX],
}
