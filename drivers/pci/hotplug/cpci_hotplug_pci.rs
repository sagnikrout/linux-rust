//! Automatically rewritten from C to Rust
//! Source: drivers/pci/hotplug/cpci_hotplug_pci.c
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
// CompactPCI Hot Plug Driver PCI functions
//
// Copyright (C) 2002,2005 by SOMA Networks, Inc.
//
// All rights reserved.
//
// Send feedback to <scottm@somanetworks.com>
//

    do {							\
    if (cpci_debug)					\
    printk(KERN_DEBUG "%s: " format "\n",	\
    MY_NAME, ## arg);		\
    } while (0)

#[no_mangle]
pub unsafe extern "C" fn cpci_get_attention_status(slot: *mut slot) -> u8 {
    u8 cpci_get_attention_status(struct slot *slot)
    {
    int hs_cap;
    u16 hs_csr;
    hs_cap = pci_bus_find_capability(slot.bus,
    slot.devfn,
    PCI_CAP_ID_CHSWP);
    if (!hs_cap)
    return 0;
    if (pci_bus_read_config_word(slot.bus,
    slot.devfn,
    hs_cap + 2,
    &hs_csr))
    return 0;
    return hs_csr & 0x0008 ? 1 : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cpci_set_attention_status(slot: *mut slot, status: c_int) -> c_int {
    int cpci_set_attention_status(struct slot *slot, int status)
    {
    int hs_cap;
    u16 hs_csr;
    hs_cap = pci_bus_find_capability(slot.bus,
    slot.devfn,
    PCI_CAP_ID_CHSWP);
    if (!hs_cap)
    return 0;
    if (pci_bus_read_config_word(slot.bus,
    slot.devfn,
    hs_cap + 2,
    &hs_csr))
    return 0;
    if (status)
    hs_csr |= HS_CSR_LOO;
    else
    hs_csr &= ~HS_CSR_LOO;
    if (pci_bus_write_config_word(slot.bus,
    slot.devfn,
    hs_cap + 2,
    hs_csr))
    return 0;
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn cpci_get_hs_csr(slot: *mut slot) -> u16 {
    u16 cpci_get_hs_csr(struct slot *slot)
    {
    int hs_cap;
    u16 hs_csr;
    hs_cap = pci_bus_find_capability(slot.bus,
    slot.devfn,
    PCI_CAP_ID_CHSWP);
    if (!hs_cap)
    return 0xFFFF;
    if (pci_bus_read_config_word(slot.bus,
    slot.devfn,
    hs_cap + 2,
    &hs_csr))
    return 0xFFFF;
    return hs_csr;
    }
#[no_mangle]
pub unsafe extern "C" fn cpci_check_and_clear_ins(slot: *mut slot) -> c_int {
    int cpci_check_and_clear_ins(struct slot *slot)
    {
    int hs_cap;
    u16 hs_csr;
    let mut ins: c_int = 0;
    hs_cap = pci_bus_find_capability(slot.bus,
    slot.devfn,
    PCI_CAP_ID_CHSWP);
    if (!hs_cap)
    return 0;
    if (pci_bus_read_config_word(slot.bus,
    slot.devfn,
    hs_cap + 2,
    &hs_csr))
    return 0;
    if (hs_csr & HS_CSR_INS) {
// Clear INS (by setting it)
    if (pci_bus_write_config_word(slot.bus,
    slot.devfn,
    hs_cap + 2,
    hs_csr))
    ins = 0;
    else
    ins = 1;
    }
    return ins;
    }
#[no_mangle]
pub unsafe extern "C" fn cpci_check_ext(slot: *mut slot) -> c_int {
    int cpci_check_ext(struct slot *slot)
    {
    int hs_cap;
    u16 hs_csr;
    let mut ext: c_int = 0;
    hs_cap = pci_bus_find_capability(slot.bus,
    slot.devfn,
    PCI_CAP_ID_CHSWP);
    if (!hs_cap)
    return 0;
    if (pci_bus_read_config_word(slot.bus,
    slot.devfn,
    hs_cap + 2,
    &hs_csr))
    return 0;
    if (hs_csr & HS_CSR_EXT)
    ext = 1;
    return ext;
    }
#[no_mangle]
pub unsafe extern "C" fn cpci_clear_ext(slot: *mut slot) -> c_int {
    int cpci_clear_ext(struct slot *slot)
    {
    int hs_cap;
    u16 hs_csr;
    hs_cap = pci_bus_find_capability(slot.bus,
    slot.devfn,
    PCI_CAP_ID_CHSWP);
    if (!hs_cap)
    return -ENODEV;
    if (pci_bus_read_config_word(slot.bus,
    slot.devfn,
    hs_cap + 2,
    &hs_csr))
    return -ENODEV;
    if (hs_csr & HS_CSR_EXT) {
// Clear EXT (by setting it)
    if (pci_bus_write_config_word(slot.bus,
    slot.devfn,
    hs_cap + 2,
    hs_csr))
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cpci_led_on(slot: *mut slot) -> c_int {
    int cpci_led_on(struct slot *slot)
    {
    int hs_cap;
    u16 hs_csr;
    hs_cap = pci_bus_find_capability(slot.bus,
    slot.devfn,
    PCI_CAP_ID_CHSWP);
    if (!hs_cap)
    return -ENODEV;
    if (pci_bus_read_config_word(slot.bus,
    slot.devfn,
    hs_cap + 2,
    &hs_csr))
    return -ENODEV;
    if ((hs_csr & HS_CSR_LOO) != HS_CSR_LOO) {
    hs_csr |= HS_CSR_LOO;
    if (pci_bus_write_config_word(slot.bus,
    slot.devfn,
    hs_cap + 2,
    hs_csr)) {
    err("Could not set LOO for slot %s", slot_name(slot));
    return -ENODEV;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cpci_led_off(slot: *mut slot) -> c_int {
    int cpci_led_off(struct slot *slot)
    {
    int hs_cap;
    u16 hs_csr;
    hs_cap = pci_bus_find_capability(slot.bus,
    slot.devfn,
    PCI_CAP_ID_CHSWP);
    if (!hs_cap)
    return -ENODEV;
    if (pci_bus_read_config_word(slot.bus,
    slot.devfn,
    hs_cap + 2,
    &hs_csr))
    return -ENODEV;
    if (hs_csr & HS_CSR_LOO) {
    hs_csr &= ~HS_CSR_LOO;
    if (pci_bus_write_config_word(slot.bus,
    slot.devfn,
    hs_cap + 2,
    hs_csr)) {
    err("Could not clear LOO for slot %s", slot_name(slot));
    return -ENODEV;
    }
    }
    return 0;
    }
//
// Device configuration functions
//
#[no_mangle]
pub unsafe extern "C" fn cpci_configure_slot(slot: *mut slot) -> c_int {
    int cpci_configure_slot(struct slot *slot)
    {
    struct pci_dev *dev;
    struct pci_bus *parent;
    let mut ret: c_int = 0;
    dbg("%s - enter", __func__);
    pci_lock_rescan_remove();
    if (slot.dev == core::ptr::null_mut()) {
    dbg("pci_dev null, finding %02x:%02x:%x",
    slot.bus.number, PCI_SLOT(slot.devfn), PCI_FUNC(slot.devfn));
    slot.dev = pci_get_slot(slot.bus, slot.devfn);
    }
// Still NULL? Well then scan for it!
    if (slot.dev == core::ptr::null_mut()) {
    int n;
    dbg("pci_dev still null");
//
// This will generate pci_dev structures for all functions, but
// we will only call this case when lookup fails.
//
    n = pci_scan_slot(slot.bus, slot.devfn);
    dbg("%s: pci_scan_slot returned %d", __func__, n);
    slot.dev = pci_get_slot(slot.bus, slot.devfn);
    if (slot.dev == core::ptr::null_mut()) {
    err("Could not find PCI device for slot %02x", slot.number);
    ret = -ENODEV;
    goto out;
    }
    }
    parent = slot.dev.bus;
    for_each_pci_bridge(dev, parent) {
    if (PCI_SLOT(dev.devfn) == PCI_SLOT(slot.devfn))
    pci_hp_add_bridge(dev);
    }
    pci_assign_unassigned_bridge_resources(parent.self);
    pci_bus_add_devices(parent);
    out:
    pci_unlock_rescan_remove();
    dbg("%s - exit", __func__);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn cpci_unconfigure_slot(slot: *mut slot) -> c_int {
    int cpci_unconfigure_slot(struct slot *slot)
    {
    struct pci_dev *dev, *temp;
    dbg("%s - enter", __func__);
    if (!slot.dev) {
    err("No device for slot %02x\n", slot.number);
    return -ENODEV;
    }
    pci_lock_rescan_remove();
    list_for_each_entry_safe(dev, temp, &slot.bus.devices, bus_list) {
    if (PCI_SLOT(dev.devfn) != PCI_SLOT(slot.devfn))
    continue;
    pci_dev_get(dev);
    pci_stop_and_remove_bus_device(dev);
    pci_dev_put(dev);
    }
    pci_dev_put(slot.dev);
    slot.dev = core::ptr::null_mut();
    pci_unlock_rescan_remove();
    dbg("%s - exit", __func__);
    return 0;
    }
