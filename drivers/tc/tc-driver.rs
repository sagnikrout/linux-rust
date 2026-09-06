//! Automatically rewritten from C to Rust
//! Source: drivers/tc/tc-driver.c
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


//
// TURBOchannel driver services.
//
// Copyright (c) 2005  James Simmons
// Copyright (c) 2006  Maciej W. Rozycki
//
// Loosely based on drivers/dio/dio-driver.c and
// drivers/pci/pci-driver.c.
//
// This file is subject to the terms and conditions of the GNU
// General Public License.  See the file "COPYING" in the main
// directory of this archive for more details.
//

//
// tc_register_driver - register a new TC driver
// @drv: the driver structure to register
//
// Adds the driver structure to the list of registered drivers
// Returns a negative value on error, otherwise 0.
// If no error occurred, the driver remains registered even if
// no device was claimed during registration.
//
#[no_mangle]
pub unsafe extern "C" fn tc_register_driver(tdrv: *mut tc_driver) -> c_int {
    int tc_register_driver(struct tc_driver *tdrv)
    {
    return driver_register(&tdrv.driver);
    }
    EXPORT_SYMBOL(tc_register_driver);
//
// tc_unregister_driver - unregister a TC driver
// @drv: the driver structure to unregister
//
// Deletes the driver structure from the list of registered TC drivers,
// gives it a chance to clean up by calling its remove() function for
// each device it was responsible for, and marks those devices as
// driverless.
//
#[no_mangle]
pub unsafe extern "C" fn tc_unregister_driver(tdrv: *mut tc_driver) {
    void tc_unregister_driver(struct tc_driver *tdrv)
    {
    driver_unregister(&tdrv.driver);
    }
    EXPORT_SYMBOL(tc_unregister_driver);
//
// tc_match_device - tell if a TC device structure has a matching
// TC device ID structure
// @tdrv: the TC driver to earch for matching TC device ID strings
// @tdev: the TC device structure to match against
//
// Used by a driver to check whether a TC device present in the
// system is in its list of supported devices.  Returns the matching
// tc_device_id structure or %NULL if there is no match.
//
    static const struct tc_device_id *tc_match_device(const struct tc_driver *tdrv,
    struct tc_dev *tdev)
    {
    const struct tc_device_id *id = tdrv.id_table;
    if (id) {
    while (id.name[0] || id.vendor[0]) {
    if (strcmp(tdev.name, id.name) == 0 &&
    strcmp(tdev.vendor, id.vendor) == 0)
    return id;
    id++;
    }
    }
    return core::ptr::null_mut();
    }
//
// tc_bus_match - Tell if a device structure has a matching
// TC device ID structure
// @dev: the device structure to match against
// @drv: the device driver to search for matching TC device ID strings
//
// Used by a driver to check whether a TC device present in the
// system is in its list of supported devices.  Returns 1 if there
// is a match or 0 otherwise.
//
#[no_mangle]
unsafe extern "C" fn tc_bus_match(dev: *mut device, drv: *const device_driver) -> c_int {
    static int tc_bus_match(struct device *dev, const struct device_driver *drv)
    {
    struct tc_dev *tdev = to_tc_dev(dev);
    const struct tc_driver *tdrv = to_tc_driver(drv);
    const struct tc_device_id *id;
    id = tc_match_device(tdrv, tdev);
    if (id)
    return 1;
    return 0;
    }
    const struct bus_type tc_bus_type = {
    .name	= "tc",
    .match	= tc_bus_match,
    };
    EXPORT_SYMBOL(tc_bus_type);
#[no_mangle]
unsafe extern "C" fn tc_driver_init() -> int __init {
    static int __init tc_driver_init(void)
    {
    return bus_register(&tc_bus_type);
    }
    postcore_initcall(tc_driver_init);
