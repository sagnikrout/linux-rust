//! Automatically rewritten from C to Rust
//! Source: drivers/pci/hotplug/shpchp_core.c
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

// Global variables
    bool shpchp_poll_mode;
    int shpchp_poll_time;

    MODULE_AUTHOR(DRIVER_AUTHOR);
    MODULE_DESCRIPTION(DRIVER_DESC);
    module_param(shpchp_poll_mode, bool, 0644);
    module_param(shpchp_poll_time, int, 0644);
    MODULE_PARM_DESC(shpchp_poll_mode, "Using polling mechanism for hot-plug events or not");
    MODULE_PARM_DESC(shpchp_poll_time, "Polling mechanism frequency, in seconds");

    static int set_attention_status(struct hotplug_slot *slot, u8 value);
    static int enable_slot(struct hotplug_slot *slot);
    static int disable_slot(struct hotplug_slot *slot);
    static int get_power_status(struct hotplug_slot *slot, u8 *value);
    static int get_attention_status(struct hotplug_slot *slot, u8 *value);
    static int get_latch_status(struct hotplug_slot *slot, u8 *value);
    static int get_adapter_status(struct hotplug_slot *slot, u8 *value);
    static const struct hotplug_slot_ops shpchp_hotplug_slot_ops = {
    .set_attention_status =	set_attention_status,
    .enable_slot =		enable_slot,
    .disable_slot =		disable_slot,
    .get_power_status =	get_power_status,
    .get_attention_status =	get_attention_status,
    .get_latch_status =	get_latch_status,
    .get_adapter_status =	get_adapter_status,
    };
#[no_mangle]
unsafe extern "C" fn init_slots(ctrl: *mut controller) -> c_int {
    static int init_slots(struct controller *ctrl)
    {
    struct slot *slot;
    struct hotplug_slot *hotplug_slot;
    char name[SLOT_NAME_SIZE];
    int retval;
    int i;
    for (i = 0; i < ctrl.num_slots; i++) {
    slot = kzalloc_obj(*slot);
    if (!slot) {
    retval = -ENOMEM;
    goto error;
    }
    hotplug_slot = &slot.hotplug_slot;
    slot.hp_slot = i;
    slot.ctrl = ctrl;
    slot.bus = ctrl.pci_dev.subordinate.number;
    slot.device = ctrl.slot_device_offset + i;
    slot.number = ctrl.first_slot + (ctrl.slot_num_inc * i);
    slot.wq = alloc_workqueue("shpchp-%d", WQ_PERCPU, 0,
    slot.number);
    if (!slot.wq) {
    retval = -ENOMEM;
    goto error_slot;
    }
    mutex_init(&slot.lock);
    INIT_DELAYED_WORK(&slot.work, shpchp_queue_pushbutton_work);
// register this slot with the hotplug pci core
    snprintf(name, SLOT_NAME_SIZE, "%d", slot.number);
    hotplug_slot.ops = &shpchp_hotplug_slot_ops;
    ctrl_dbg(ctrl, "Registering domain:bus:dev=%04x:%02x:%02x hp_slot=%x sun=%x slot_device_offset=%x\n",
    pci_domain_nr(ctrl.pci_dev.subordinate),
    slot.bus, slot.device, slot.hp_slot, slot.number,
    ctrl.slot_device_offset);
    retval = pci_hp_register(hotplug_slot,
    ctrl.pci_dev.subordinate, slot.device, name);
    if (retval) {
    ctrl_err(ctrl, "pci_hp_register failed with error %d\n",
    retval);
    goto error_slotwq;
    }
    get_power_status(hotplug_slot, &slot.pwr_save);
    get_attention_status(hotplug_slot, &slot.attention_save);
    get_latch_status(hotplug_slot, &slot.latch_save);
    get_adapter_status(hotplug_slot, &slot.presence_save);
    list_add(&slot.slot_list, &ctrl.slot_list);
    }
    return 0;
    error_slotwq:
    destroy_workqueue(slot.wq);
    error_slot:
    kfree(slot);
    error:
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn cleanup_slots(ctrl: *mut controller) {
    void cleanup_slots(struct controller *ctrl)
    {
    struct slot *slot, *next;
    list_for_each_entry_safe(slot, next, &ctrl.slot_list, slot_list) {
    list_del(&slot.slot_list);
    cancel_delayed_work(&slot.work);
    destroy_workqueue(slot.wq);
    pci_hp_deregister(&slot.hotplug_slot);
    kfree(slot);
    }
    }
//
// set_attention_status - Turns the Amber LED for a slot on, off or blink
//
#[no_mangle]
unsafe extern "C" fn set_attention_status(hotplug_slot: *mut hotplug_slot, status: u8) -> c_int {
    static int set_attention_status(struct hotplug_slot *hotplug_slot, u8 status)
    {
    struct slot *slot = get_slot(hotplug_slot);
    ctrl_dbg(slot.ctrl, "%s: physical_slot = %s\n",
    __func__, slot_name(slot));
    slot.attention_save = status;
    shpchp_set_attention_status(slot, status);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn enable_slot(hotplug_slot: *mut hotplug_slot) -> c_int {
    static int enable_slot(struct hotplug_slot *hotplug_slot)
    {
    struct slot *slot = get_slot(hotplug_slot);
    ctrl_dbg(slot.ctrl, "%s: physical_slot = %s\n",
    __func__, slot_name(slot));
    return shpchp_sysfs_enable_slot(slot);
    }
#[no_mangle]
unsafe extern "C" fn disable_slot(hotplug_slot: *mut hotplug_slot) -> c_int {
    static int disable_slot(struct hotplug_slot *hotplug_slot)
    {
    struct slot *slot = get_slot(hotplug_slot);
    ctrl_dbg(slot.ctrl, "%s: physical_slot = %s\n",
    __func__, slot_name(slot));
    return shpchp_sysfs_disable_slot(slot);
    }
#[no_mangle]
unsafe extern "C" fn get_power_status(hotplug_slot: *mut hotplug_slot, value: *mut u8) -> c_int {
    static int get_power_status(struct hotplug_slot *hotplug_slot, u8 *value)
    {
    struct slot *slot = get_slot(hotplug_slot);
    int retval;
    ctrl_dbg(slot.ctrl, "%s: physical_slot = %s\n",
    __func__, slot_name(slot));
    retval = shpchp_get_power_status(slot, value);
    if (retval < 0)
// value = slot->pwr_save;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_attention_status(hotplug_slot: *mut hotplug_slot, value: *mut u8) -> c_int {
    static int get_attention_status(struct hotplug_slot *hotplug_slot, u8 *value)
    {
    struct slot *slot = get_slot(hotplug_slot);
    int retval;
    ctrl_dbg(slot.ctrl, "%s: physical_slot = %s\n",
    __func__, slot_name(slot));
    retval = shpchp_get_attention_status(slot, value);
    if (retval < 0)
// value = slot->attention_save;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_latch_status(hotplug_slot: *mut hotplug_slot, value: *mut u8) -> c_int {
    static int get_latch_status(struct hotplug_slot *hotplug_slot, u8 *value)
    {
    struct slot *slot = get_slot(hotplug_slot);
    int retval;
    ctrl_dbg(slot.ctrl, "%s: physical_slot = %s\n",
    __func__, slot_name(slot));
    retval = shpchp_get_latch_status(slot, value);
    if (retval < 0)
// value = slot->latch_save;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_adapter_status(hotplug_slot: *mut hotplug_slot, value: *mut u8) -> c_int {
    static int get_adapter_status(struct hotplug_slot *hotplug_slot, u8 *value)
    {
    struct slot *slot = get_slot(hotplug_slot);
    int retval;
    ctrl_dbg(slot.ctrl, "%s: physical_slot = %s\n",
    __func__, slot_name(slot));
    retval = shpchp_get_adapter_status(slot, value);
    if (retval < 0)
// value = slot->presence_save;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn shpc_capable(bridge: *mut pci_dev) -> bool {
    static bool shpc_capable(struct pci_dev *bridge)
    {
//
// It is assumed that AMD GOLAM chips support SHPC but they do not
// have SHPC capability.
//
    if (bridge.vendor == PCI_VENDOR_ID_AMD &&
    bridge.device == PCI_DEVICE_ID_AMD_GOLAM_7450)
    return true;
    if (pci_find_capability(bridge, PCI_CAP_ID_SHPC))
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn shpc_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int shpc_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    int rc;
    struct controller *ctrl;
    if (!shpc_capable(pdev))
    return -ENODEV;
    if (acpi_get_hp_hw_control_from_firmware(pdev))
    return -ENODEV;
    ctrl = kzalloc_obj(*ctrl);
    if (!ctrl)
    goto err_out_none;
    INIT_LIST_HEAD(&ctrl.slot_list);
    rc = shpc_init(ctrl, pdev);
    if (rc) {
    ctrl_dbg(ctrl, "Controller initialization failed\n");
    goto err_out_free_ctrl;
    }
    pci_set_drvdata(pdev, ctrl);
// Setup the slot information structures
    rc = init_slots(ctrl);
    if (rc) {
    ctrl_err(ctrl, "Slot initialization failed\n");
    goto err_out_release_ctlr;
    }
    rc = shpchp_create_ctrl_files(ctrl);
    if (rc)
    goto err_cleanup_slots;
    pdev.shpc_managed = 1;
    return 0;
    err_cleanup_slots:
    cleanup_slots(ctrl);
    err_out_release_ctlr:
    shpchp_release_ctlr(ctrl);
    err_out_free_ctrl:
    kfree(ctrl);
    err_out_none:
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn shpc_remove(dev: *mut pci_dev) {
    static void shpc_remove(struct pci_dev *dev)
    {
    struct controller *ctrl = pci_get_drvdata(dev);
    dev.shpc_managed = 0;
    shpchp_remove_ctrl_files(ctrl);
    shpchp_release_ctlr(ctrl);
    kfree(ctrl);
    }
    static const struct pci_device_id shpcd_pci_tbl[] = {
    {PCI_DEVICE_CLASS(PCI_CLASS_BRIDGE_PCI_NORMAL, ~0)},
    { /* end: all zeroes */ }
    };
    MODULE_DEVICE_TABLE(pci, shpcd_pci_tbl);
    static struct pci_driver shpc_driver = {
    .name =		SHPC_MODULE_NAME,
    .id_table =	shpcd_pci_tbl,
    .probe =	shpc_probe,
    .remove =	shpc_remove,
    };
#[no_mangle]
unsafe extern "C" fn shpcd_init() -> int __init {
    static int __init shpcd_init(void)
    {
    return pci_register_driver(&shpc_driver);
    }
#[no_mangle]
unsafe extern "C" fn shpcd_cleanup() -> void __exit {
    static void __exit shpcd_cleanup(void)
    {
    pci_unregister_driver(&shpc_driver);
    }
    module_init(shpcd_init);
    module_exit(shpcd_cleanup);
