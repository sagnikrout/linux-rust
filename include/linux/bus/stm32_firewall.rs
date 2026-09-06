//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bus/stm32_firewall.h
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

//
// STM32_PERIPHERAL_FIREWALL:		This type of firewall protects peripherals
// STM32_MEMORY_FIREWALL:		This type of firewall protects memories/subsets of memory
// zones
// STM32_NOTYPE_FIREWALL:		Undefined firewall type
//

//
// struct stm32_firewall_controller - Information on firewall controller supplying services
//
// @name:			Name of the firewall controller
// @dev:			Device reference of the firewall controller
// @mmio:			Base address of the firewall controller
// @entry:			List entry of the firewall controller list
// @type:			Type of firewall
// @max_entries:		Number of entries covered by the firewall
// @grant_access:		Callback used to grant access for a device access against a
// firewall controller
// @release_access:		Callback used to release resources taken by a device when access was
// granted
// @grant_memory_range_access:	Callback used to grant access for a device to a given memory region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_firewall_controller {
    pub name: *const c_char,
    pub dev: *mut device,
    pub mmio: *mut void __iomem,
    pub entry: list_head,
    pub type: c_uint,
    pub max_entries: c_uint,
    pub id): *mut *mut *mut int (grant_access)(struct stm32_firewall_controller ctrl, u32,
    pub id): *mut *mut *mut void (release_access)(struct stm32_firewall_controller ctrl, u32,
    pub size): usize,
}

//
// stm32_firewall_controller_register - Register a firewall controller to the STM32 firewall
// framework
// @firewall_controller:	Firewall controller to register
//
// Returns 0 in case of success or -ENODEV if no controller was given.
//
extern "C" {
    pub fn stm32_firewall_controller_register(firewall_controller: *mut stm32_firewall_controller) -> c_int;
}
//
// stm32_firewall_controller_unregister - Unregister a firewall controller from the STM32
// firewall framework
// @firewall_controller:	Firewall controller to unregister
//
extern "C" {
    pub fn stm32_firewall_controller_unregister(firewall_controller: *mut stm32_firewall_controller);
}
//
// stm32_firewall_populate_bus - Populate device tree nodes that have a correct firewall
// configuration. This is used at boot-time only, as a sanity check
// between device tree and firewalls hardware configurations to
// prevent a kernel crash when a device driver is not granted access
//
// @firewall_controller:	Firewall controller which nodes will be populated or not
//
// Returns 0 in case of success or appropriate errno code if error occurred.
//
extern "C" {
    pub fn stm32_firewall_populate_bus(firewall_controller: *mut stm32_firewall_controller) -> c_int;
}
