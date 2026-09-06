//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/base/firmware_loader/firmware.h
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
// enum fw_opt - options to control firmware loading behaviour
//
// @FW_OPT_UEVENT: Enables the fallback mechanism to send a kobject uevent
// when the firmware is not found. Userspace is in charge to load the
// firmware using the sysfs loading facility.
// @FW_OPT_NOWAIT: Used to describe the firmware request is asynchronous.
// @FW_OPT_USERHELPER: Enable the fallback mechanism, in case the direct
// filesystem lookup fails at finding the firmware.  For details refer to
// firmware_fallback_sysfs().
// @FW_OPT_NO_WARN: Quiet, avoid printing warning messages.
// @FW_OPT_NOCACHE: Disables firmware caching. Firmware caching is used to
// cache the firmware upon suspend, so that upon resume races against the
// firmware file lookup on storage is avoided. Used for calls where the
// file may be too big, or where the driver takes charge of its own
// firmware caching mechanism.
// @FW_OPT_NOFALLBACK_SYSFS: Disable the sysfs fallback mechanism. Takes
// precedence over &FW_OPT_UEVENT and &FW_OPT_USERHELPER.
// @FW_OPT_FALLBACK_PLATFORM: Enable fallback to device fw copy embedded in
// the platform's main firmware. If both this fallback and the sysfs
// fallback are enabled, then this fallback will be tried first.
// @FW_OPT_PARTIAL: Allow partial read of firmware instead of needing to read
// entire file.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_opt {
    FW_OPT_UEVENT			= BIT(0),
    FW_OPT_NOWAIT			= BIT(1),
    FW_OPT_USERHELPER		= BIT(2),
    FW_OPT_NO_WARN			= BIT(3),
    FW_OPT_NOCACHE			= BIT(4),
    FW_OPT_NOFALLBACK_SYSFS		= BIT(5),
    FW_OPT_FALLBACK_PLATFORM	= BIT(6),
    FW_OPT_PARTIAL			= BIT(7),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_status {
    FW_STATUS_UNKNOWN,
    FW_STATUS_LOADING,
    FW_STATUS_DONE,
    FW_STATUS_ABORTED,
}

//
// Concurrent request_firmware() for the same firmware need to be
// serialized.  struct fw_state is simple state machine which hold the
// state of the firmware loading.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_state {
    pub completion: completion,
    pub status: fw_status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_priv {
    pub ref: kref,
    pub list: list_head,
    pub fwc: *mut firmware_cache,
    pub fw_st: fw_state,
    pub data: *mut c_void,
    pub size: usize,
    pub allocated_size: usize,
    pub offset: usize,
    pub opt_flags: u32,

    pub is_paged_buf: bool,
    pub pages: *mut page,
    pub nr_pages: c_int,
    pub page_array_size: c_int,

    pub need_uevent: bool,
    pub pending_list: list_head,

    pub fw_name: *const c_char,
}

//
// Doing this here ensures that the fw_priv is deleted from
// the pending list in all abort/done paths.
//

extern "C" {
    pub fn __fw_state_check(_arg: fw_priv, _arg: FW_STATUS_ABORTED) -> return;
}
extern "C" {
    pub fn __fw_state_check(_arg: fw_priv, _arg: FW_STATUS_DONE) -> return;
}
extern "C" {
    pub fn __fw_state_check(_arg: fw_priv, _arg: FW_STATUS_LOADING) -> return;
}
extern "C" {
    pub fn assign_fw(fw: *mut firmware, device: *mut device) -> c_int;
}
extern "C" {
    pub fn free_fw_priv(fw_priv: *mut fw_priv);
}
extern "C" {
    pub fn fw_state_init(fw_priv: *mut fw_priv);
}

extern "C" {
    pub fn firmware_is_builtin(fw: *const firmware) -> bool;
}

extern "C" {
    pub fn fw_free_paged_buf(fw_priv: *mut fw_priv);
}
extern "C" {
    pub fn fw_grow_paged_buf(fw_priv: *mut fw_priv, pages_needed: c_int) -> c_int;
}
extern "C" {
    pub fn fw_map_paged_buf(fw_priv: *mut fw_priv) -> c_int;
}
extern "C" {
    pub fn fw_is_paged_buf(fw_priv: *mut fw_priv) -> bool;
}

