//! Automatically rewritten from C to Rust
//! Source: drivers/pci/hotplug/acpi_pcihp.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Common ACPI functions for hot plug platforms
//
// Copyright (C) 2006 Intel Corporation
//
// All rights reserved.
//
// Send feedback to <kristen.c.accardi@intel.com>
//

    static bool debug_acpi;
// acpi_run_oshp - get control of hotplug from the firmware
//
// @handle - the handle of the hotplug controller.
//
#[no_mangle]
unsafe extern "C" fn acpi_run_oshp(handle: acpi_handle) -> acpi_status {
    static acpi_status acpi_run_oshp(acpi_handle handle)
    {
    acpi_status		status;
    let mut string: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    acpi_get_name(handle, ACPI_FULL_PATHNAME, &string);
// run OSHP
    status = acpi_evaluate_object(handle, METHOD_NAME_OSHP, core::ptr::null_mut(), core::ptr::null_mut());
    if (ACPI_FAILURE(status))
    if (status != AE_NOT_FOUND)
    printk(KERN_ERR "%s:%s OSHP fails=0x%x\n",
    __func__, (char *)string.pointer, status);
    else
    dbg("%s:%s OSHP not found\n",
    __func__, (char *)string.pointer);
    else
    pr_debug("%s:%s OSHP passes\n", __func__,
    (char *)string.pointer);
    kfree(string.pointer);
    return status;
    }
//
// acpi_get_hp_hw_control_from_firmware
// @pdev: the pci_dev of the bridge that has a hotplug controller
//
// Attempt to take hotplug control from firmware.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_get_hp_hw_control_from_firmware(pdev: *mut pci_dev) -> c_int {
    int acpi_get_hp_hw_control_from_firmware(struct pci_dev *pdev)
    {
    const struct pci_host_bridge *host;
    const struct acpi_pci_root *root;
    acpi_status status;
    acpi_handle chandle, handle;
    let mut string: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
//
// If there's no ACPI host bridge (i.e., ACPI support is compiled
// into the kernel but the hardware platform doesn't support ACPI),
// there's nothing to do here.
//
    host = pci_find_host_bridge(pdev.bus);
    root = acpi_pci_find_root(ACPI_HANDLE(&host.dev));
    if (!root)
    return 0;
//
// If _OSC exists, it determines whether we're allowed to manage
// the SHPC.  We executed it while enumerating the host bridge.
//
    if (root.osc_support_set) {
    if (host.native_shpc_hotplug)
    return 0;
    return -ENODEV;
    }
//
// In the absence of _OSC, we're always allowed to manage the SHPC.
// However, if an OSHP method is present, we must execute it so the
// firmware can transfer control to the OS, e.g., direct interrupts
// to the OS instead of to the firmware.
//
// N.B. The PCI Firmware Spec (r3.2, sec 4.8) does not endorse
// searching up the ACPI hierarchy, so the loops below are suspect.
//
    handle = ACPI_HANDLE(&pdev.dev);
    if (!handle) {
//
// This hotplug controller was not listed in the ACPI name
// space at all. Try to get ACPI handle of parent PCI bus.
//
    struct pci_bus *pbus;
    for (pbus = pdev.bus; pbus; pbus = pbus.parent) {
    handle = acpi_pci_get_bridge_handle(pbus);
    if (handle)
    break;
    }
    }
    while (handle) {
    acpi_get_name(handle, ACPI_FULL_PATHNAME, &string);
    pci_info(pdev, "Requesting control of SHPC hotplug via OSHP (%s)\n",
    (char *)string.pointer);
    status = acpi_run_oshp(handle);
    if (ACPI_SUCCESS(status))
    goto got_one;
    if (acpi_is_root_bridge(handle))
    break;
    chandle = handle;
    status = acpi_get_parent(chandle, &handle);
    if (ACPI_FAILURE(status))
    break;
    }
    pci_info(pdev, "Cannot get control of SHPC hotplug\n");
    kfree(string.pointer);
    return -ENODEV;
    got_one:
    pci_info(pdev, "Gained control of SHPC hotplug (%s)\n",
    (char *)string.pointer);
    kfree(string.pointer);
    return 0;
    }
    EXPORT_SYMBOL(acpi_get_hp_hw_control_from_firmware);
#[no_mangle]
unsafe extern "C" fn pcihp_is_ejectable(handle: acpi_handle) -> c_int {
    static int pcihp_is_ejectable(acpi_handle handle)
    {
    acpi_status status;
    unsigned long long removable;
    if (!acpi_has_method(handle, "_ADR"))
    return 0;
    if (acpi_has_method(handle, "_EJ0"))
    return 1;
    status = acpi_evaluate_integer(handle, "_RMV", core::ptr::null_mut(), &removable);
    if (ACPI_SUCCESS(status) && removable)
    return 1;
    return 0;
    }
//
// acpi_pci_check_ejectable - check if handle is ejectable ACPI PCI slot
// @pbus: the PCI bus of the PCI slot corresponding to 'handle'
// @handle: ACPI handle to check
//
// Return 1 if handle is ejectable PCI slot, 0 otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_pci_check_ejectable(pbus: *mut pci_bus, handle: acpi_handle) -> c_int {
    int acpi_pci_check_ejectable(struct pci_bus *pbus, acpi_handle handle)
    {
    acpi_handle bridge_handle, parent_handle;
    bridge_handle = acpi_pci_get_bridge_handle(pbus);
    if (!bridge_handle)
    return 0;
    if ((ACPI_FAILURE(acpi_get_parent(handle, &parent_handle))))
    return 0;
    if (bridge_handle != parent_handle)
    return 0;
    return pcihp_is_ejectable(handle);
    }
    EXPORT_SYMBOL_GPL(acpi_pci_check_ejectable);
    static acpi_status
    check_hotplug(acpi_handle handle, u32 lvl, void *context, void **rv)
    {
    int *found = (int *)context;
    if (pcihp_is_ejectable(handle)) {
// found = 1;
    return AE_CTRL_TERMINATE;
    }
    return AE_OK;
    }
//
// acpi_pci_detect_ejectable - check if the PCI bus has ejectable slots
// @handle: handle of the PCI bus to scan
//
// Returns 1 if the PCI bus has ACPI based ejectable slots, 0 otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_pci_detect_ejectable(handle: acpi_handle) -> c_int {
    int acpi_pci_detect_ejectable(acpi_handle handle)
    {
    let mut found: c_int = 0;
    if (!handle)
    return found;
    acpi_walk_namespace(ACPI_TYPE_DEVICE, handle, 1,
    check_hotplug, core::ptr::null_mut(), (void *)&found, core::ptr::null_mut());
    return found;
    }
    EXPORT_SYMBOL_GPL(acpi_pci_detect_ejectable);
    module_param(debug_acpi, bool, 0644);
    MODULE_PARM_DESC(debug_acpi, "Debugging mode for ACPI enabled or not");
