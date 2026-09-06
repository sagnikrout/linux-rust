//! Automatically rewritten from C to Rust
//! Source: drivers/pci/hotplug/rpaphp_slot.c
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
// RPA Virtual I/O device functions
// Copyright (C) 2004 Linda Xie <lxie@us.ibm.com>
//
// All rights reserved.
//
// Send feedback to <lxie@us.ibm.com>
//

// free up the memory used by a slot
#[no_mangle]
pub unsafe extern "C" fn dealloc_slot_struct(slot: *mut slot) {
    void dealloc_slot_struct(struct slot *slot)
    {
    of_node_put(slot.dn);
    kfree(slot.name);
    kfree(slot);
    }
    struct slot *alloc_slot_struct(struct device_node *dn,
    int drc_index, char *drc_name, int power_domain)
    {
    struct slot *slot;
    slot = kzalloc_obj(struct slot);
    if (!slot)
    goto error_nomem;
    slot.name = kstrdup(drc_name, GFP_KERNEL);
    if (!slot.name)
    goto error_slot;
    slot.dn = of_node_get(dn);
    slot.index = drc_index;
    slot.power_domain = power_domain;
    slot.hotplug_slot.ops = &rpaphp_hotplug_slot_ops;
    return (slot);
    error_slot:
    kfree(slot);
    error_nomem:
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn is_registered(slot: *mut slot) -> c_int {
    static int is_registered(struct slot *slot)
    {
    struct slot *tmp_slot;
    list_for_each_entry(tmp_slot, &rpaphp_slot_head, rpaphp_slot_list) {
    if (!strcmp(tmp_slot.name, slot.name))
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rpaphp_deregister_slot(slot: *mut slot) -> c_int {
    int rpaphp_deregister_slot(struct slot *slot)
    {
    let mut retval: c_int = 0;
    struct hotplug_slot *php_slot = &slot.hotplug_slot;
    dbg("%s - Entry: deregistering slot=%s\n",
    __func__, slot.name);
    list_del(&slot.rpaphp_slot_list);
    pci_hp_deregister(php_slot);
    dealloc_slot_struct(slot);
    dbg("%s - Exit: rc[%d]\n", __func__, retval);
    return retval;
    }
    EXPORT_SYMBOL_GPL(rpaphp_deregister_slot);
#[no_mangle]
pub unsafe extern "C" fn rpaphp_register_slot(slot: *mut slot) -> c_int {
    int rpaphp_register_slot(struct slot *slot)
    {
    struct hotplug_slot *php_slot = &slot.hotplug_slot;
    u32 my_index;
    int retval;
    let mut slotno: c_int = PCI_SLOT_PLACEHOLDER;
    dbg("%s registering slot:path[%pOF] index[%x], name[%s] pdomain[%x] type[%d]\n",
    __func__, slot.dn, slot.index, slot.name,
    slot.power_domain, slot.type);
// should not try to register the same slot twice
    if (is_registered(slot)) {
    err("rpaphp_register_slot: slot[%s] is already registered\n", slot.name);
    return -EAGAIN;
    }
    for_each_child_of_node_scoped(slot.dn, child) {
    retval = of_property_read_u32(child, "ibm,my-drc-index", &my_index);
    if (my_index == slot.index) {
    slotno = PCI_SLOT(PCI_DN(child).devfn);
    break;
    }
    }
    retval = pci_hp_register(php_slot, slot.bus, slotno, slot.name);
    if (retval) {
    err("pci_hp_register failed with error %d\n", retval);
    return retval;
    }
// add slot to our internal list
    list_add(&slot.rpaphp_slot_list, &rpaphp_slot_head);
    info("Slot [%s] registered\n", slot.name);
    return 0;
    }
