//! Automatically rewritten from C to Rust
//! Source: drivers/pci/hotplug/shpchp_pci.c
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
// Standard Hot Plug Controller Driver
//
// Copyright (C) 1995,2001 Compaq Computer Corporation
// Copyright (C) 2001 Greg Kroah-Hartman (greg@kroah.com)
// Copyright (C) 2001 IBM Corp.
// Copyright (C) 2003-2004 Intel Corporation
//
// All rights reserved.
//
// Send feedback to <greg@kroah.com>, <kristen.c.accardi@intel.com>
//

#[no_mangle]
pub unsafe extern "C" fn shpchp_configure_device(p_slot: *mut slot) -> c_int {
    int shpchp_configure_device(struct slot *p_slot)
    {
    struct pci_dev *dev;
    struct controller *ctrl = p_slot.ctrl;
    struct pci_dev *bridge = ctrl.pci_dev;
    struct pci_bus *parent = bridge.subordinate;
    int num, ret = 0;
    pci_lock_rescan_remove();
    dev = pci_get_slot(parent, PCI_DEVFN(p_slot.device, 0));
    if (dev) {
    ctrl_err(ctrl, "Device %s already exists at %04x:%02x:%02x, cannot hot-add\n",
    pci_name(dev), pci_domain_nr(parent),
    p_slot.bus, p_slot.device);
    pci_dev_put(dev);
    ret = -EINVAL;
    goto out;
    }
    num = pci_scan_slot(parent, PCI_DEVFN(p_slot.device, 0));
    if (num == 0) {
    ctrl_err(ctrl, "No new device found\n");
    ret = -ENODEV;
    goto out;
    }
    for_each_pci_bridge(dev, parent) {
    if (PCI_SLOT(dev.devfn) == p_slot.device)
    pci_hp_add_bridge(dev);
    }
    pci_assign_unassigned_bridge_resources(bridge);
    pcie_bus_configure_settings(parent);
    pci_bus_add_devices(parent);
    out:
    pci_unlock_rescan_remove();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn shpchp_unconfigure_device(p_slot: *mut slot) {
    void shpchp_unconfigure_device(struct slot *p_slot)
    {
    struct pci_bus *parent = p_slot.ctrl.pci_dev.subordinate;
    struct pci_dev *dev, *temp;
    struct controller *ctrl = p_slot.ctrl;
    ctrl_dbg(ctrl, "%s: domain:bus:dev = %04x:%02x:%02x\n",
    __func__, pci_domain_nr(parent), p_slot.bus, p_slot.device);
    pci_lock_rescan_remove();
    list_for_each_entry_safe(dev, temp, &parent.devices, bus_list) {
    if (PCI_SLOT(dev.devfn) != p_slot.device)
    continue;
    pci_dev_get(dev);
    pci_stop_and_remove_bus_device(dev);
    pci_dev_put(dev);
    }
    pci_unlock_rescan_remove();
    }
