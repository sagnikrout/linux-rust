//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dpll/dpll_core.h
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates
// Copyright (c) 2023 Intel and affiliates
//

//
// struct dpll_device - stores DPLL device internal data
// @id:			unique id number for device given by dpll subsystem
// @device_idx:		id given by dev driver
// @clock_id:		unique identifier (clock_id) of a dpll
// @module:		module of creator
// @type:		type of a dpll
// @pin_refs:		stores pins registered within a dpll
// @refcount:		refcount
// @refcnt_tracker:	ref_tracker directory for debugging reference leaks
// @registration_list:	list of registered ops and priv data of dpll owners
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpll_device {
    pub id: u32,
    pub device_idx: u32,
    pub clock_id: u64,
    pub module: *mut module,
    pub type: dpll_type,
    pub pin_refs: xarray,
    pub refcount: refcount_t,
    pub refcnt_tracker: ref_tracker_dir,
    pub registration_list: list_head,
}

//
// struct dpll_pin - structure for a dpll pin
// @id:			unique id number for pin given by dpll subsystem
// @pin_idx:		index of a pin given by dev driver
// @clock_id:		clock_id of creator
// @module:		module of creator
// @module_name:	module name of creator
// @fwnode:		optional reference to firmware node
// @dpll_refs:		hold referencees to dplls pin was registered with
// @parent_refs:	hold references to parent pins pin was registered with
// @ref_sync_pins:	hold references to pins for Reference SYNC feature
// @prop:		pin properties copied from the registerer
// @refcount:		refcount
// @refcnt_tracker:	ref_tracker directory for debugging reference leaks
// @rcu:		rcu_head for kfree_rcu()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpll_pin {
    pub id: u32,
    pub pin_idx: u32,
    pub clock_id: u64,
    pub module: *mut module,
    pub module_name: [c_char; MODULE_NAME_LEN],
    pub fwnode: *mut fwnode_handle,
    pub dpll_refs: xarray,
    pub parent_refs: xarray,
    pub ref_sync_pins: xarray,
    pub prop: dpll_pin_properties,
    pub refcount: refcount_t,
    pub refcnt_tracker: ref_tracker_dir,
    pub rcu: rcu_head,
}

//
// struct dpll_pin_ref - structure for referencing either dpll or pins
// @dpll:		pointer to a dpll
// @pin:		pointer to a pin
// @registration_list:	list of ops and priv data registered with the ref
// @refcount:		refcount
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpll_pin_ref {
    pub dpll: *mut dpll_device,
    pub pin: *mut dpll_pin,
}

extern "C" {
    pub fn dpll_device_notify(dpll: *mut dpll_device, action: c_ulong);
}
