//! Automatically rewritten from C to Rust
//! Source: drivers/pci/hotplug/s390_pci_hpc.c
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
// PCI Hot Plug Controller Driver for System z
//
// Copyright 2012 IBM Corp.
//
// Author(s):
// Jan Glauber <jang@linux.vnet.ibm.com>
//

pub const SLOT_NAME_SIZE: c_int = 10;
#[no_mangle]
unsafe extern "C" fn enable_slot(hotplug_slot: *mut hotplug_slot) -> c_int {
    static int enable_slot(struct hotplug_slot *hotplug_slot)
    {
    struct zpci_dev *zdev = container_of(hotplug_slot, struct zpci_dev,
    hotplug_slot);
    int rc;
    mutex_lock(&zdev.state_lock);
    if (zdev.state != ZPCI_FN_STATE_STANDBY) {
    rc = -EIO;
    goto out;
    }
    rc = sclp_pci_configure(zdev.fid);
    zpci_dbg(3, "conf fid:%x, rc:%d\n", zdev.fid, rc);
    if (rc)
    goto out;
    zdev.state = ZPCI_FN_STATE_CONFIGURED;
    rc = zpci_scan_configured_device(zdev, zdev.fh);
    out:
    mutex_unlock(&zdev.state_lock);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn disable_slot(hotplug_slot: *mut hotplug_slot) -> c_int {
    static int disable_slot(struct hotplug_slot *hotplug_slot)
    {
    struct zpci_dev *zdev = container_of(hotplug_slot, struct zpci_dev,
    hotplug_slot);
    struct pci_dev *pdev = core::ptr::null_mut();
    int rc;
    mutex_lock(&zdev.state_lock);
    if (zdev.state != ZPCI_FN_STATE_CONFIGURED) {
    rc = -EIO;
    goto out;
    }
    pdev = pci_get_slot(zdev.zbus.bus, zdev.devfn);
    if (pdev && pci_num_vf(pdev)) {
    rc = -EBUSY;
    goto out;
    }
    rc = zpci_deconfigure_device(zdev);
    out:
    if (pdev)
    pci_dev_put(pdev);
    mutex_unlock(&zdev.state_lock);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn reset_slot(hotplug_slot: *mut hotplug_slot, probe: bool) -> c_int {
    static int reset_slot(struct hotplug_slot *hotplug_slot, bool probe)
    {
    struct zpci_dev *zdev = container_of(hotplug_slot, struct zpci_dev,
    hotplug_slot);
    let mut rc: c_int = -EIO;
//
// If we can't get the zdev->state_lock the device state is
// currently undergoing a transition and we bail out - just
// the same as if the device's state is not configured at all.
//
    if (!mutex_trylock(&zdev.state_lock))
    return rc;
// We can reset only if the function is configured
    if (zdev.state != ZPCI_FN_STATE_CONFIGURED)
    goto out;
    if (probe) {
    rc = 0;
    goto out;
    }
    rc = zpci_hot_reset_device(zdev);
    out:
    mutex_unlock(&zdev.state_lock);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn get_power_status(hotplug_slot: *mut hotplug_slot, value: *mut u8) -> c_int {
    static int get_power_status(struct hotplug_slot *hotplug_slot, u8 *value)
    {
    struct zpci_dev *zdev = container_of(hotplug_slot, struct zpci_dev,
    hotplug_slot);
// value = zpci_is_device_configured(zdev) ? 1 : 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_adapter_status(hotplug_slot: *mut hotplug_slot, value: *mut u8) -> c_int {
    static int get_adapter_status(struct hotplug_slot *hotplug_slot, u8 *value)
    {
// if the slot exists it always contains a function
// value = 1;
    return 0;
    }
    static const struct hotplug_slot_ops s390_hotplug_slot_ops = {
    .enable_slot =		enable_slot,
    .disable_slot =		disable_slot,
    .reset_slot =		reset_slot,
    .get_power_status =	get_power_status,
    .get_adapter_status =	get_adapter_status,
    };
#[no_mangle]
pub unsafe extern "C" fn zpci_init_slot(zdev: *mut zpci_dev) -> c_int {
    int zpci_init_slot(struct zpci_dev *zdev)
    {
    char name[SLOT_NAME_SIZE];
    struct zpci_bus *zbus = zdev.zbus;
    zdev.hotplug_slot.ops = &s390_hotplug_slot_ops;
    snprintf(name, SLOT_NAME_SIZE, "%08x", zdev.fid);
    return pci_hp_register(&zdev.hotplug_slot, zbus.bus,
    zdev.devfn, name);
    }
#[no_mangle]
pub unsafe extern "C" fn zpci_exit_slot(zdev: *mut zpci_dev) {
    void zpci_exit_slot(struct zpci_dev *zdev)
    {
    pci_hp_deregister(&zdev.hotplug_slot);
    }
