//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/hed.c
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
// ACPI Hardware Error Device (PNP0C33) Driver
//
// Copyright (C) 2010, Intel Corp.
// Author: Huang Ying <ying.huang@intel.com>
//
// ACPI Hardware Error Device is used to report some hardware errors
// notified via SCI, mainly the corrected errors.
//

    static const struct acpi_device_id acpi_hed_ids[] = {
    {"PNP0C33", 0},
    {"", 0},
    };
    MODULE_DEVICE_TABLE(acpi, acpi_hed_ids);
    static bool hed_present;
    static BLOCKING_NOTIFIER_HEAD(acpi_hed_notify_list);
#[no_mangle]
pub unsafe extern "C" fn register_acpi_hed_notifier(nb: *mut notifier_block) -> c_int {
    int register_acpi_hed_notifier(struct notifier_block *nb)
    {
    return blocking_notifier_chain_register(&acpi_hed_notify_list, nb);
    }
    EXPORT_SYMBOL_GPL(register_acpi_hed_notifier);
#[no_mangle]
pub unsafe extern "C" fn unregister_acpi_hed_notifier(nb: *mut notifier_block) {
    void unregister_acpi_hed_notifier(struct notifier_block *nb)
    {
    blocking_notifier_chain_unregister(&acpi_hed_notify_list, nb);
    }
    EXPORT_SYMBOL_GPL(unregister_acpi_hed_notifier);
//
// SCI to report hardware error is forwarded to the listeners of HED,
// it is used by HEST Generic Hardware Error Source with notify type
// SCI.
//
#[no_mangle]
unsafe extern "C" fn acpi_hed_notify(handle: acpi_handle, event: u32, data: *mut c_void) {
    static void acpi_hed_notify(acpi_handle handle, u32 event, void *data)
    {
    blocking_notifier_call_chain(&acpi_hed_notify_list, 0, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn acpi_hed_probe(pdev: *mut platform_device) -> c_int {
    static int acpi_hed_probe(struct platform_device *pdev)
    {
    int err;
// Only one hardware error device
    if (hed_present)
    return -EINVAL;
    err = devm_acpi_install_notify_handler(&pdev.dev, ACPI_DEVICE_NOTIFY,
    acpi_hed_notify, core::ptr::null_mut());
    if (err)
    return err;
    hed_present = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_hed_remove(pdev: *mut platform_device) {
    static void acpi_hed_remove(struct platform_device *pdev)
    {
    hed_present = false;
    }
    static struct platform_driver acpi_hed_driver = {
    .probe = acpi_hed_probe,
    .remove = acpi_hed_remove,
    .driver = {
    .name = "acpi-hardware-error-device",
    .acpi_match_table = acpi_hed_ids,
    },
    };
#[no_mangle]
unsafe extern "C" fn acpi_hed_driver_init() -> int __init {
    static int __init acpi_hed_driver_init(void)
    {
    return platform_driver_register(&acpi_hed_driver);
    }
    subsys_initcall(acpi_hed_driver_init);
    MODULE_AUTHOR("Huang Ying");
    MODULE_DESCRIPTION("ACPI Hardware Error Device Driver");
    MODULE_LICENSE("GPL");
