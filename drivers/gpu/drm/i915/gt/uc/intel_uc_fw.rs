//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/uc/intel_uc_fw.h
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
// Copyright © 2014-2019 Intel Corporation
//

// Home of GuC, HuC and DMC firmwares

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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_uc_fw_status {
    INTEL_UC_FIRMWARE_NOT_SUPPORTED = -1, /* no uc HW */
    INTEL_UC_FIRMWARE_UNINITIALIZED = 0, /* used to catch checks done too early */
    INTEL_UC_FIRMWARE_DISABLED, /* disabled */
    INTEL_UC_FIRMWARE_SELECTED, /* selected the blob we want to load */
    INTEL_UC_FIRMWARE_MISSING, /* blob not found on the system */
    INTEL_UC_FIRMWARE_ERROR, /* invalid format or version */
    INTEL_UC_FIRMWARE_AVAILABLE, /* blob found and copied in mem */
    INTEL_UC_FIRMWARE_INIT_FAIL, /* failed to prepare fw objects for load */
    INTEL_UC_FIRMWARE_LOADABLE, /* all fw-required objects are ready */
    INTEL_UC_FIRMWARE_LOAD_FAIL, /* failed to xfer or init/auth the fw */
    INTEL_UC_FIRMWARE_TRANSFERRED, /* dma xfer done */
    INTEL_UC_FIRMWARE_RUNNING /* init/auth done */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_uc_fw_type {
    INTEL_UC_FW_TYPE_GUC = 0,
    INTEL_UC_FW_TYPE_HUC,
    INTEL_UC_FW_TYPE_GSC,
}

pub const INTEL_UC_FW_NUM_TYPES: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_uc_fw_ver {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub build: u32,
}

//
// The firmware build process will generate a version header file with major and
// minor version defined. The versions are built into CSS header of firmware.
// i915 kernel driver set the minimal firmware version required per platform.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_uc_fw_file {
    pub path: *const c_char,
    pub ver: intel_uc_fw_ver,
}

//
// This structure encapsulates all the data needed during the process
// of fetching, caching, and loading the firmware image into the uC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_uc_fw {
    pub type: intel_uc_fw_type,
    pub status: intel_uc_fw_status,
    pub /: *mut *mut intel_uc_fw_status __status; / no accidental overwrites,
}

//
// @needs_ggtt_mapping: indicates whether the fw object needs to be
// pinned to ggtt. If true, the fw is pinned at init time and unpinned
// during driver unload.
//
// @vma_res: A vma resource used in binding the uc fw to ggtt. The fw is
// pinned in a reserved area of the ggtt (above the maximum address
// usable by GuC); therefore, we can't use the normal vma functions to
// do the pinning and we instead use this resource to do so.
//
// When we load the uC binaries, we pin them in a reserved section at the top of
// the GGTT, which is ~18 MBs. On multi-GT systems where the GTs share the GGTT,
// we also need to make sure that each binary is pinned to a unique location
// during load, because the different GT can go through the FW load at the same
// time (see uc_fw_ggtt_offset() for details).
// Given that the available space is much greater than what is required by the
// binaries, to keep things simple instead of dynamically partitioning the
// reserved section to make space for all the blobs we can just reserve a static
// chunk for each binary.
//

// shouldn't call this before checking hw/blob availability
//
// intel_uc_fw_get_upload_size() - Get size of firmware needed to be uploaded.
// @uc_fw: uC firmware.
//
// Get the size of the firmware and header that will be uploaded to WOPCM.
//
// Return: Upload firmware size, or zero on firmware fetch failure.
//
extern "C" {
    pub fn __intel_uc_fw_get_upload_size(_arg: uc_fw) -> return;
}
extern "C" {
    pub fn intel_uc_check_file_version(uc_fw: *mut intel_uc_fw, old_ver: *mut bool) -> c_int;
}
extern "C" {
    pub fn intel_uc_fw_fetch(uc_fw: *mut intel_uc_fw) -> c_int;
}
extern "C" {
    pub fn intel_uc_fw_cleanup_fetch(uc_fw: *mut intel_uc_fw);
}
extern "C" {
    pub fn intel_uc_fw_upload(uc_fw: *mut intel_uc_fw, offset: u32, dma_flags: u32) -> c_int;
}
extern "C" {
    pub fn intel_uc_fw_init(uc_fw: *mut intel_uc_fw) -> c_int;
}
extern "C" {
    pub fn intel_uc_fw_fini(uc_fw: *mut intel_uc_fw);
}
extern "C" {
    pub fn intel_uc_fw_resume_mapping(uc_fw: *mut intel_uc_fw);
}
extern "C" {
    pub fn intel_uc_fw_copy_rsa(uc_fw: *mut intel_uc_fw, dst: *mut c_void, max_len: u32) -> usize;
}
extern "C" {
    pub fn intel_uc_fw_mark_load_failed(uc_fw: *mut intel_uc_fw, err: c_int) -> c_int;
}
extern "C" {
    pub fn intel_uc_fw_dump(uc_fw: *const intel_uc_fw, p: *mut drm_printer);
}
