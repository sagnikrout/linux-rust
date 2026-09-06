//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/framer/framer-provider.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Generic framer profider header file
//
// Copyright 2023 CS GROUP France
//
// Author: Herve Codina <herve.codina@bootlin.com>
//

//
// struct framer_ops - set of function pointers for performing framer operations
// @init: operation to be performed for initializing the framer
// @exit: operation to be performed while exiting
// @power_on: powering on the framer
// @power_off: powering off the framer
// @flags: OR-ed flags (FRAMER_FLAG_*) to ask for core functionality
// - @FRAMER_FLAG_POLL_STATUS:
// Ask the core to perform a polling to get the framer status and
// notify consumers on change.
// The framer should call @framer_notify_status_change() when it
// detects a status change. This is usually done using interrupts.
// If the framer cannot detect this change, it can ask the core for
// a status polling. The core will call @get_status() periodically
// and, on change detected, it will notify the consumer.
// the @get_status()
// @owner: the module owner containing the ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct framer_ops {
    pub framer): *mut *mut int (init)(struct framer,
    pub framer): *mut *mut void (exit)(struct framer,
    pub framer): *mut *mut int (power_on)(struct framer,
    pub framer): *mut *mut int (power_off)(struct framer,
//
// @get_status:
//
// Optional.
//
// Used to get the framer status. framer_init() must have
// been called on the framer.
//
// Returns: 0 if successful, an negative error code otherwise
//
    pub status): *mut *mut *mut int (get_status)(struct framer framer, struct framer_status,
//
// @set_config:
//
// Optional.
//
// Used to set the framer configuration. framer_init() must have
// been called on the framer.
//
// Returns: 0 if successful, an negative error code otherwise
//
    pub config): *const *const *const int (set_config)(struct framer framer, struct framer_config,
//
// @get_config:
//
// Optional.
//
// Used to get the framer configuration. framer_init() must have
// been called on the framer.
//
// Returns: 0 if successful, an negative error code otherwise
//
    pub config): *mut *mut *mut int (get_config)(struct framer framer, struct framer_config,
    pub flags: u32,
    pub owner: *mut module,
}

//
// struct framer_provider - represents the framer provider
// @dev: framer provider device
// @owner: the module owner having of_xlate
// @list: to maintain a linked list of framer providers
// @of_xlate: function pointer to obtain framer instance from framer pointer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct framer_provider {
    pub dev: *mut device,
    pub owner: *mut module,
    pub list: list_head,
    pub args): *const of_phandle_args,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: &framer->dev) -> return;
}

// Create and destroy a framer
extern "C" {
    pub fn framer_destroy(framer: *mut framer);
}
// devm version
extern "C" {
    pub fn framer_provider_of_unregister(framer_provider: *mut framer_provider);
}
extern "C" {
    pub fn framer_notify_status_change(framer: *mut framer);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
// devm version
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}

