//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/display/drm_dp_aux_bus.h
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
// Copyright 2021 Google Inc.
//
// The DP AUX bus is used for devices that are connected over a DisplayPort
// AUX bus. The devices on the far side of the bus are referred to as
// endpoints in this code.
//

//
// struct dp_aux_ep_device - Main dev structure for DP AUX endpoints
//
// This is used to instantiate devices that are connected via a DP AUX
// bus. Usually the device is a panel, but conceivable other devices could
// be hooked up there.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_aux_ep_device {
// @dev: The normal dev pointer
    pub dev: device,
// @aux: Pointer to the aux bus
    pub aux: *mut drm_dp_aux,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_aux_ep_driver {
    pub aux_ep): *mut *mut int (probe)(struct dp_aux_ep_device,
    pub aux_ep): *mut *mut void (remove)(struct dp_aux_ep_device,
    pub aux_ep): *mut *mut void (shutdown)(struct dp_aux_ep_device,
    pub driver: device_driver,
}

extern "C" {
    pub fn container_of(_arg: dev, dp_aux_ep_device: struct, _arg: dev) -> return;
}
extern "C" {
    pub fn container_of(_arg: drv, dp_aux_ep_driver: struct, _arg: driver) -> return;
}
extern "C" {
    pub fn of_dp_aux_depopulate_bus(aux: *mut drm_dp_aux);
}
// Deprecated versions of the above functions. To be removed when no callers.
// New API returns -ENODEV for no child case; adapt to old assumption

extern "C" {
    pub fn dp_aux_dp_driver_unregister(aux_ep_drv: *mut dp_aux_ep_driver);
}
