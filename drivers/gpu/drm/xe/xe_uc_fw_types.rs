//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_uc_fw_types.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2022 Intel Corporation
//

//
// +------------+---------------------------------------------------+
// |   PHASE    |           FIRMWARE STATUS TRANSITIONS             |
// +============+===================================================+
// |            |               UNINITIALIZED                       |
// +------------+-               /   |   \                         -+
// |            |   DISABLED <--/    |    \--> NOT_SUPPORTED        |
// | init_early |                    V                              |
// |            |                 SELECTED                          |
// +------------+-               /   |   \                         -+
// |            |    MISSING <--/    |    \--> ERROR                |
// |   fetch    |                    V                              |
// |            |                 AVAILABLE                         |
// +------------+-                   |   \                         -+
// |            |                    |    \--> INIT FAIL            |
// |   init     |                    V                              |
// |            |        /------> LOADABLE <----<-----------\       |
// +------------+-       \         /    \        \           \     -+
// |            |    LOAD FAIL <--<      \--> TRANSFERRED     \     |
// |   upload   |                  \           /   \          /     |
// |            |                   \---------/     \--> RUNNING    |
// +------------+---------------------------------------------------+
//
// FIXME: Ported from the i915 and this is state machine is way too complicated.
// Circle back and simplify this.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_uc_fw_status {
    XE_UC_FIRMWARE_NOT_SUPPORTED = -1, /* no uc HW */
    XE_UC_FIRMWARE_UNINITIALIZED = 0, /* used to catch checks done too early */
    XE_UC_FIRMWARE_DISABLED, /* disabled */
    XE_UC_FIRMWARE_SELECTED, /* selected the blob we want to load */
    XE_UC_FIRMWARE_MISSING, /* blob not found on the system */
    XE_UC_FIRMWARE_ERROR, /* invalid format or version */
    XE_UC_FIRMWARE_AVAILABLE, /* blob found and copied in mem */
    XE_UC_FIRMWARE_INIT_FAIL, /* failed to prepare fw objects for load */
    XE_UC_FIRMWARE_LOADABLE, /* all fw-required objects are ready */
    XE_UC_FIRMWARE_LOAD_FAIL, /* failed to xfer or init/auth the fw */
    XE_UC_FIRMWARE_TRANSFERRED, /* dma xfer done */
    XE_UC_FIRMWARE_RUNNING, /* init/auth done */
    XE_UC_FIRMWARE_PRELOADED, /* preloaded by the PF driver */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_uc_fw_type {
    XE_UC_FW_TYPE_GUC = 0,
    XE_UC_FW_TYPE_HUC,
    XE_UC_FW_TYPE_GSC,
    XE_UC_FW_NUM_TYPES
}

//
// struct xe_uc_fw_version - Version for Xe micro controller firmware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_uc_fw_version {
// @branch: branch version of the FW (not always available)
    pub branch: u16,
// @major: major version of the FW
    pub major: u16,
// @minor: minor version of the FW
    pub minor: u16,
// @patch: patch version of the FW
    pub patch: u16,
// @build: build version of the FW (not always available)
    pub build: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_uc_fw_version_types {
    XE_UC_FW_VER_RELEASE,
    XE_UC_FW_VER_COMPATIBILITY,
    XE_UC_FW_VER_TYPE_COUNT
}

//
// struct xe_uc_fw - Xe micro controller firmware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_uc_fw {
// @type: type uC firmware
    pub type: xe_uc_fw_type,
// @status: firmware load status
    pub status: xe_uc_fw_status,
//
// @__status: private firmware load status - only to be used
// by firmware loading code
//
    pub __status: xe_uc_fw_status,
}

// @path: path to uC firmware
// @user_overridden: user provided path to uC firmware via modparam
//
// @full_ver_required: driver still under development and not ready
// for backward-compatible firmware. To be used only for **new
// platforms, i.e. still under require_force_probe protection and not
// supported by i915.
//
// @size: size of uC firmware including css header
// @bo: Xe BO for uC firmware
// @has_gsc_headers: whether the FW image starts with GSC headers
//
// The firmware build process will generate a version header file with
// major and minor version defined. The versions are built into CSS
// header of firmware. The xe kernel driver set the minimal firmware
// version required per platform.
//
// @versions: FW versions wanted and found
// @versions.wanted: firmware version wanted by platform
//
// @versions.wanted_type: type of firmware version wanted
// (release vs compatibility)
//
// @versions.found: fw versions found in firmware blob
// @rsa_size: RSA size
// @ucode_size: micro kernel size
// @css_offset: offset within the blob at which the CSS is located
// @private_data_size: size of private data found in uC css header
// @build_type: Firmware build type (see CSS_UKERNEL_INFO_BUILDTYPE for definitions)
