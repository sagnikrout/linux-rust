//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bus/stm32_firewall_device.h
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
// Copyright (C) 2023, STMicroelectronics - All Rights Reserved
//

pub const STM32_FIREWALL_MAX_EXTRA_ARGS: c_int = 5;
// Opaque reference to stm32_firewall_controller
//
// struct stm32_firewall - Information on a device's firewall. Each device can have more than one
// firewall.
//
// @firewall_ctrl:		Pointer referencing a firewall controller of the device. It is
// opaque so a device cannot manipulate the controller's ops or access
// the controller's data
// @extra_args:			Extra arguments that are implementation dependent
// @entry:			Name of the firewall entry
// @extra_args_size:		Number of extra arguments
// @firewall_id:		Firewall ID associated the device for this firewall controller
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_firewall {
    pub firewall_ctrl: *mut stm32_firewall_controller,
    pub extra_args: [u32; STM32_FIREWALL_MAX_EXTRA_ARGS],
    pub entry: *const c_char,
    pub extra_args_size: usize,
    pub firewall_id: u32,
}

//
// stm32_firewall_get_firewall - Get the firewall(s) associated to given device.
// The firewall controller reference is always the first argument
// of each of the access-controller property entries.
// The firewall ID is always the second argument of each of the
// access-controller  property entries.
// If there's no argument linked to the phandle, then the firewall ID
// field is set to U32_MAX, which is an invalid ID.
//
// @np:				Device node to parse
// @firewall:			Array of firewall references
// @nb_firewall:		Number of firewall references to get. Must be at least 1.
//
// Returns 0 on success, -ENODEV if there's no match with a firewall controller or appropriate errno
// code if error occurred.
//
// stm32_firewall_grant_access - Request firewall access rights and grant access.
//
// @firewall:			Firewall reference containing the ID to check against its firewall
// controller
//
// Returns 0 if access is granted, -EACCES if access is denied, -ENODEV if firewall is null or
// appropriate errno code if error occurred
//
extern "C" {
    pub fn stm32_firewall_grant_access(firewall: *mut stm32_firewall) -> c_int;
}
//
// stm32_firewall_release_access - Release access granted from a call to
// stm32_firewall_grant_access().
//
// @firewall:			Firewall reference containing the ID to check against its firewall
// controller
//
extern "C" {
    pub fn stm32_firewall_release_access(firewall: *mut stm32_firewall);
}
//
// stm32_firewall_grant_access_by_id - Request firewall access rights of a given device
// based on a specific firewall ID
//
// Warnings:
// There is no way to ensure that the given ID will correspond to the firewall referenced in the
// device node if the ID did not come from stm32_firewall_get_firewall(). In that case, this
// function must be used with caution.
// This function should be used for subsystem resources that do not have the same firewall ID
// as their parent.
// U32_MAX is an invalid ID.
//
// @firewall:			Firewall reference containing the firewall controller
// @subsystem_id:		Firewall ID of the subsystem resource
//
// Returns 0 if access is granted, -EACCES if access is denied, -ENODEV if firewall is null or
// appropriate errno code if error occurred
//
extern "C" {
    pub fn stm32_firewall_grant_access_by_id(firewall: *mut stm32_firewall, subsystem_id: u32) -> c_int;
}
//
// stm32_firewall_release_access_by_id - Release access granted from a call to
// stm32_firewall_grant_access_by_id().
//
// Warnings:
// There is no way to ensure that the given ID will correspond to the firewall referenced in the
// device node if the ID did not come from stm32_firewall_get_firewall(). In that case, this
// function must be used with caution.
// This function should be used for subsystem resources that do not have the same firewall ID
// as their parent.
// U32_MAX is an invalid ID.
//
// @firewall:			Firewall reference containing the firewall controller
// @subsystem_id:		Firewall ID of the subsystem resource
//
extern "C" {
    pub fn stm32_firewall_release_access_by_id(firewall: *mut stm32_firewall, subsystem_id: u32);
}
//
// stm32_firewall_get_grant_all_access - Allocate and get all the firewall(s) associated to given
// device. Then, try to grant access rights for each element.
// This function is basically a helper function that wraps
// both stm32_firewall_get_firewall() and
// stm32_firewall_grant_access() on all firewall references of
// a device along with the allocation of the array.
// Realease access using stm32_firewall_release_access* APIs
// when done.
//
// @dev:			Device performing the checks
// @firewall:			Pointer to the array of firewall references to be allocated
// @nb_firewall:		Number of allocated elements in @firewall
//
// Returns 0 on success, or appropriate errno code if error occurred.
//

