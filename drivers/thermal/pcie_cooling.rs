//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/pcie_cooling.c
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
// PCIe cooling device
//
// Copyright (C) 2023-2024 Intel Corporation
//

#[no_mangle]
unsafe extern "C" fn pcie_cooling_get_max_level(cdev: *mut thermal_cooling_device, state: *mut c_ulong) -> c_int {
    static int pcie_cooling_get_max_level(struct thermal_cooling_device *cdev, unsigned long *state)
    {
    struct pci_dev *port = cdev.devdata;
// cooling state 0 is same as the maximum PCIe speed
// state = port->subordinate->max_bus_speed - PCIE_SPEED_2_5GT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcie_cooling_get_cur_level(cdev: *mut thermal_cooling_device, state: *mut c_ulong) -> c_int {
    static int pcie_cooling_get_cur_level(struct thermal_cooling_device *cdev, unsigned long *state)
    {
    struct pci_dev *port = cdev.devdata;
// cooling state 0 is same as the maximum PCIe speed
// state = cdev->max_state - (port->subordinate->cur_bus_speed - PCIE_SPEED_2_5GT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcie_cooling_set_cur_level(cdev: *mut thermal_cooling_device, state: c_ulong) -> c_int {
    static int pcie_cooling_set_cur_level(struct thermal_cooling_device *cdev, unsigned long state)
    {
    struct pci_dev *port = cdev.devdata;
    enum pci_bus_speed speed;
// cooling state 0 is same as the maximum PCIe speed
    speed = (cdev.max_state - state) + PCIE_SPEED_2_5GT;
    return pcie_set_target_speed(port, speed, true);
    }
    static struct thermal_cooling_device_ops pcie_cooling_ops = {
    .get_max_state = pcie_cooling_get_max_level,
    .get_cur_state = pcie_cooling_get_cur_level,
    .set_cur_state = pcie_cooling_set_cur_level,
    };
    struct thermal_cooling_device *pcie_cooling_device_register(struct pci_dev *port)
    {
    char *name __free(kfree) =
    kasprintf(GFP_KERNEL, COOLING_DEV_TYPE_PREFIX "%s", pci_name(port));
    if (!name)
    return ERR_PTR(-ENOMEM);
    return thermal_cooling_device_register(name, port, &pcie_cooling_ops);
    }
#[no_mangle]
pub unsafe extern "C" fn pcie_cooling_device_unregister(cdev: *mut thermal_cooling_device) {
    void pcie_cooling_device_unregister(struct thermal_cooling_device *cdev)
    {
    thermal_cooling_device_unregister(cdev);
    }
// For bus_speed <-> state arithmetic
    static_assert(PCIE_SPEED_2_5GT + 1 == PCIE_SPEED_5_0GT);
    static_assert(PCIE_SPEED_5_0GT + 1 == PCIE_SPEED_8_0GT);
    static_assert(PCIE_SPEED_8_0GT + 1 == PCIE_SPEED_16_0GT);
    static_assert(PCIE_SPEED_16_0GT + 1 == PCIE_SPEED_32_0GT);
    static_assert(PCIE_SPEED_32_0GT + 1 == PCIE_SPEED_64_0GT);
    MODULE_AUTHOR("Ilpo Järvinen <ilpo.jarvinen@linux.intel.com>");
    MODULE_DESCRIPTION("PCIe cooling driver");
