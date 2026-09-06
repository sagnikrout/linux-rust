//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/wil6210/wil_platform.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2014-2017 Qualcomm Atheros, Inc.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wil_platform_event {
    WIL_PLATFORM_EVT_FW_CRASH = 0,
    WIL_PLATFORM_EVT_PRE_RESET = 1,
    WIL_PLATFORM_EVT_FW_RDY = 2,
    WIL_PLATFORM_EVT_PRE_SUSPEND = 3,
    WIL_PLATFORM_EVT_POST_SUSPEND = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wil_platform_features {
    WIL_PLATFORM_FEATURE_FW_EXT_CLK_CONTROL = 0,
    WIL_PLATFORM_FEATURE_TRIPLE_MSI = 1,
    WIL_PLATFORM_FEATURE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wil_platform_capa {
    WIL_PLATFORM_CAPA_RADIO_ON_IN_SUSPEND = 0,
    WIL_PLATFORM_CAPA_T_PWR_ON_0 = 1,
    WIL_PLATFORM_CAPA_EXT_CLK = 2,
    WIL_PLATFORM_CAPA_MAX,
}

//
// struct wil_platform_ops - wil platform module calls from this
// driver to platform driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_platform_ops {
    pub /): *mut *mut *mut *mut int (bus_request)(void handle, uint32_t kbps / KBytes/Sec,
    pub keep_device_power): *mut *mut *mut int (suspend)(void handle, bool,
    pub device_powered_on): *mut *mut *mut int (resume)(void handle, bool,
    pub handle): *mut *mut void (uninit)(void,
    pub evt): *mut *mut *mut int (notify)(void handle, enum wil_platform_event,
    pub handle): *mut *mut int (get_capa)(void,
    pub features): *mut *mut *mut void (set_features)(void handle, int,
}

//
// struct wil_platform_rops - wil platform module callbacks from
// platform driver to this driver
// @ramdump: store a ramdump from the wil firmware. The platform
// driver may add additional data to the ramdump to
// generate the final crash dump.
// @fw_recovery: start a firmware recovery process. Called as
// part of a crash recovery process which may include other
// related platform subsystems.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_platform_rops {
    pub size): *mut *mut *mut *mut int (ramdump)(void wil_handle, void buf, uint32_t,
    pub wil_handle): *mut *mut int (fw_recovery)(void,
}

//
// wil_platform_init - initialize the platform driver
//
// @dev - pointer to the wil6210 device
// @ops - structure with platform driver operations. Platform
// driver will fill this structure with function pointers.
// @rops - structure with callbacks from platform driver to
// this driver. The platform driver copies the structure to
// its own storage. Can be NULL if this driver does not
// support crash recovery.
// @wil_handle - context for this driver that will be passed
// when platform driver invokes one of the callbacks in
// rops. May be NULL if rops is NULL.
//
extern "C" {
    pub fn wil_platform_modinit() -> int __init;
}
extern "C" {
    pub fn wil_platform_modexit();
}
