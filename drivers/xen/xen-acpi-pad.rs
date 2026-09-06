//! Automatically rewritten from C to Rust
//! Source: drivers/xen/xen-acpi-pad.c
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
// xen-acpi-pad.c - Xen pad interface
//
// Copyright (c) 2012, Intel Corporation.
// Author: Liu, Jinsong <jinsong.liu@intel.com>
//

pub const ACPI_PROCESSOR_AGGREGATOR_NOTIFY: c_uint = 0x80;
    static DEFINE_MUTEX(xen_cpu_lock);
#[no_mangle]
unsafe extern "C" fn xen_acpi_pad_idle_cpus(idle_nums: c_uint) -> c_int {
    static int xen_acpi_pad_idle_cpus(unsigned int idle_nums)
    {
    struct xen_platform_op op;
    op.cmd = XENPF_core_parking;
    op.u.core_parking.type = XEN_CORE_PARKING_SET;
    op.u.core_parking.idle_nums = idle_nums;
    return HYPERVISOR_platform_op(&op);
    }
#[no_mangle]
unsafe extern "C" fn xen_acpi_pad_idle_cpus_num() -> c_int {
    static int xen_acpi_pad_idle_cpus_num(void)
    {
    struct xen_platform_op op;
    op.cmd = XENPF_core_parking;
    op.u.core_parking.type = XEN_CORE_PARKING_GET;
#[no_mangle]
pub unsafe extern "C" fn HYPERVISOR_platform_op(_arg: &op) -> return {
    return HYPERVISOR_platform_op(&op)
    ?: op.u.core_parking.idle_nums;
    }
//
// Query firmware how many CPUs should be idle
// return -1 on failure
//
#[no_mangle]
unsafe extern "C" fn acpi_pad_pur(handle: acpi_handle) -> c_int {
    static int acpi_pad_pur(acpi_handle handle)
    {
    let mut buffer: acpi_buffer = {ACPI_ALLOCATE_BUFFER, core::ptr::null_mut()};
    union acpi_object *package;
    let mut num: c_int = -1;
    if (ACPI_FAILURE(acpi_evaluate_object(handle, "_PUR", core::ptr::null_mut(), &buffer)))
    return num;
    if (!buffer.length || !buffer.pointer)
    return num;
    package = buffer.pointer;
    if (package.type == ACPI_TYPE_PACKAGE &&
    package.package.count == 2 &&
    package.package.elements[0].integer.value == 1) /* rev 1 */
    num = package.package.elements[1].integer.value;
    kfree(buffer.pointer);
    return num;
    }
#[no_mangle]
unsafe extern "C" fn acpi_pad_handle_notify(handle: acpi_handle) {
    static void acpi_pad_handle_notify(acpi_handle handle)
    {
    int idle_nums;
    struct acpi_buffer param = {
    .length = 4,
    .pointer = (void *)&idle_nums,
    };
    mutex_lock(&xen_cpu_lock);
    idle_nums = acpi_pad_pur(handle);
    if (idle_nums < 0) {
    mutex_unlock(&xen_cpu_lock);
    return;
    }
    idle_nums = xen_acpi_pad_idle_cpus(idle_nums)
    ?: xen_acpi_pad_idle_cpus_num();
    if (idle_nums >= 0)
    acpi_evaluate_ost(handle, ACPI_PROCESSOR_AGGREGATOR_NOTIFY,
    0, &param);
    mutex_unlock(&xen_cpu_lock);
    }
    static void acpi_pad_notify(acpi_handle handle, u32 event,
    void *data)
    {
    switch (event) {
    case ACPI_PROCESSOR_AGGREGATOR_NOTIFY:
    acpi_pad_handle_notify(handle);
    break;
    default:
    pr_warn("Unsupported event [0x%x]\n", event);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn acpi_pad_probe(pdev: *mut platform_device) -> c_int {
    static int acpi_pad_probe(struct platform_device *pdev)
    {
    struct acpi_device *device;
    acpi_status status;
    device = ACPI_COMPANION(&pdev.dev);
    if (!device)
    return -ENODEV;
    status = acpi_install_notify_handler(device.handle,
    ACPI_DEVICE_NOTIFY, acpi_pad_notify, device);
    if (ACPI_FAILURE(status))
    return -ENODEV;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_pad_remove(pdev: *mut platform_device) {
    static void acpi_pad_remove(struct platform_device *pdev)
    {
    mutex_lock(&xen_cpu_lock);
    xen_acpi_pad_idle_cpus(0);
    mutex_unlock(&xen_cpu_lock);
    acpi_remove_notify_handler(ACPI_HANDLE(&pdev.dev),
    ACPI_DEVICE_NOTIFY, acpi_pad_notify);
    }
    static const struct acpi_device_id pad_device_ids[] = {
    {"ACPI000C", 0},
    {"", 0},
    };
    static struct platform_driver acpi_pad_driver = {
    .probe = acpi_pad_probe,
    .remove = acpi_pad_remove,
    .driver = {
    .name = "acpi_processor_aggregator",
    .acpi_match_table = pad_device_ids,
    },
    };
#[no_mangle]
unsafe extern "C" fn xen_acpi_pad_init() -> int __init {
    static int __init xen_acpi_pad_init(void)
    {
// Only DOM0 is responsible for Xen acpi pad
    if (!xen_initial_domain())
    return -ENODEV;
// Only Xen4.2 or later support Xen acpi pad
    if (!xen_running_on_version_or_later(4, 2))
    return -ENODEV;
    return platform_driver_register(&acpi_pad_driver);
    }
    subsys_initcall(xen_acpi_pad_init);
