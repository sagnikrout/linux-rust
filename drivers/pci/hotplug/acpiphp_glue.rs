//! Automatically rewritten from C to Rust
//! Source: drivers/pci/hotplug/acpiphp_glue.c
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
// ACPI PCI HotPlug glue functions to ACPI CA subsystem
//
// Copyright (C) 2002,2003 Takayoshi Kochi (t-kochi@bq.jp.nec.com)
// Copyright (C) 2002 Hiroshi Aono (h-aono@ap.jp.nec.com)
// Copyright (C) 2002,2003 NEC Corporation
// Copyright (C) 2003-2005 Matthew Wilcox (willy@infradead.org)
// Copyright (C) 2003-2005 Hewlett Packard
// Copyright (C) 2005 Rajesh Shah (rajesh.shah@intel.com)
// Copyright (C) 2005 Intel Corporation
//
// All rights reserved.
//
// Send feedback to <kristen.c.accardi@intel.com>
//
// Lifetime rules for pci_dev:
// - The one in acpiphp_bridge has its refcount elevated by pci_get_slot()
// when the bridge is scanned and it loses a refcount when the bridge
// is removed.
// - When a P2P bridge is present, we elevate the refcount on the subordinate
// bus. It loses the refcount when the driver unloads.
//

    static LIST_HEAD(bridge_list);
    static DEFINE_MUTEX(bridge_mutex);
    static int acpiphp_hotplug_notify(struct acpi_device *adev, u32 type);
    static void acpiphp_post_dock_fixup(struct acpi_device *adev);
    static void acpiphp_sanitize_bus(struct pci_bus *bus);
    static void hotplug_event(u32 type, struct acpiphp_context *context);
    static void free_bridge(struct kref *kref);
//
// acpiphp_init_context - Create hotplug context and grab a reference to it.
// @adev: ACPI device object to create the context for.
//
// Call under acpi_hp_context_lock.
//
    static struct acpiphp_context *acpiphp_init_context(struct acpi_device *adev)
    {
    struct acpiphp_context *context;
    context = kzalloc_obj(*context);
    if (!context)
    return core::ptr::null_mut();
    context.refcount = 1;
    context.hp.notify = acpiphp_hotplug_notify;
    context.hp.fixup = acpiphp_post_dock_fixup;
    acpi_set_hp_context(adev, &context.hp);
    return context;
    }
//
// acpiphp_get_context - Get hotplug context and grab a reference to it.
// @adev: ACPI device object to get the context for.
//
// Call under acpi_hp_context_lock.
//
    static struct acpiphp_context *acpiphp_get_context(struct acpi_device *adev)
    {
    struct acpiphp_context *context;
    if (!adev.hp)
    return core::ptr::null_mut();
    context = to_acpiphp_context(adev.hp);
    context.refcount++;
    return context;
    }
//
// acpiphp_put_context - Drop a reference to ACPI hotplug context.
// @context: ACPI hotplug context to drop a reference to.
//
// The context object is removed if there are no more references to it.
//
// Call under acpi_hp_context_lock.
//
#[no_mangle]
unsafe extern "C" fn acpiphp_put_context(context: *mut acpiphp_context) {
    static void acpiphp_put_context(struct acpiphp_context *context)
    {
    if (--context.refcount)
    return;
    WARN_ON(context.bridge);
    context.hp.self.hp = core::ptr::null_mut();
    kfree(context);
    }
#[no_mangle]
pub unsafe extern "C" fn get_bridge(bridge: *mut acpiphp_bridge) {
    static inline void get_bridge(struct acpiphp_bridge *bridge)
    {
    kref_get(&bridge.ref);
    }
#[no_mangle]
pub unsafe extern "C" fn put_bridge(bridge: *mut acpiphp_bridge) {
    static inline void put_bridge(struct acpiphp_bridge *bridge)
    {
    kref_put(&bridge.ref, free_bridge);
    }
    static struct acpiphp_context *acpiphp_grab_context(struct acpi_device *adev)
    {
    struct acpiphp_context *context;
    acpi_lock_hp_context();
    context = acpiphp_get_context(adev);
    if (!context)
    goto unlock;
    if (context.func.parent.is_going_away) {
    acpiphp_put_context(context);
    context = core::ptr::null_mut();
    goto unlock;
    }
    get_bridge(context.func.parent);
    acpiphp_put_context(context);
    unlock:
    acpi_unlock_hp_context();
    return context;
    }
#[no_mangle]
unsafe extern "C" fn acpiphp_let_context_go(context: *mut acpiphp_context) {
    static void acpiphp_let_context_go(struct acpiphp_context *context)
    {
    put_bridge(context.func.parent);
    }
#[no_mangle]
unsafe extern "C" fn free_bridge(kref: *mut kref) {
    static void free_bridge(struct kref *kref)
    {
    struct acpiphp_context *context;
    struct acpiphp_bridge *bridge;
    struct acpiphp_slot *slot, *next;
    struct acpiphp_func *func, *tmp;
    acpi_lock_hp_context();
    bridge = container_of(kref, struct acpiphp_bridge, ref);
    list_for_each_entry_safe(slot, next, &bridge.slots, node) {
    list_for_each_entry_safe(func, tmp, &slot.funcs, sibling)
    acpiphp_put_context(func_to_context(func));
    kfree(slot);
    }
    context = bridge.context;
// Root bridges will not have hotplug context.
    if (context) {
// Release the reference taken by acpiphp_enumerate_slots().
    put_bridge(context.func.parent);
    context.bridge = core::ptr::null_mut();
    acpiphp_put_context(context);
    }
    put_device(&bridge.pci_bus.dev);
    pci_dev_put(bridge.pci_dev);
    kfree(bridge);
    acpi_unlock_hp_context();
    }
//
// acpiphp_post_dock_fixup - Post-dock fixups for PCI devices.
// @adev: ACPI device object corresponding to a PCI device.
//
// TBD - figure out a way to only call fixups for systems that require them.
//
#[no_mangle]
unsafe extern "C" fn acpiphp_post_dock_fixup(adev: *mut acpi_device) {
    static void acpiphp_post_dock_fixup(struct acpi_device *adev)
    {
    struct acpiphp_context *context = acpiphp_grab_context(adev);
    struct pci_bus *bus;
    u32 buses;
    if (!context)
    return;
    bus = context.func.slot.bus;
    if (!bus.self)
    goto out;
// fixup bad _DCK function that rewrites
// secondary bridge on slot
//
    pci_read_config_dword(bus.self, PCI_PRIMARY_BUS, &buses);
    if (((buses >> 8) & 0xff) != bus.busn_res.start) {
    buses = (buses & 0xff000000)
    | ((unsigned int)(bus.primary)     <<  0)
    | ((unsigned int)(bus.busn_res.start)   <<  8)
    | ((unsigned int)(bus.busn_res.end) << 16);
    pci_write_config_dword(bus.self, PCI_PRIMARY_BUS, buses);
    }
    out:
    acpiphp_let_context_go(context);
    }
//
// acpiphp_add_context - Add ACPIPHP context to an ACPI device object.
// @handle: ACPI handle of the object to add a context to.
// @lvl: Not used.
// @data: The object's parent ACPIPHP bridge.
// @rv: Not used.
//
    static acpi_status acpiphp_add_context(acpi_handle handle, u32 lvl, void *data,
    void **rv)
    {
    struct acpi_device *adev = acpi_fetch_acpi_dev(handle);
    struct acpiphp_bridge *bridge = data;
    struct acpiphp_context *context;
    struct acpiphp_slot *slot;
    struct acpiphp_func *newfunc;
    let mut status: acpi_status = AE_OK;
    unsigned long long adr;
    int device, function;
    struct pci_bus *pbus = bridge.pci_bus;
    struct pci_dev *pdev = bridge.pci_dev;
    u32 val;
    if (!adev)
    return AE_OK;
    status = acpi_evaluate_integer(handle, "_ADR", core::ptr::null_mut(), &adr);
    if (ACPI_FAILURE(status)) {
    if (status != AE_NOT_FOUND)
    acpi_handle_warn(handle,
    "can't evaluate _ADR (%#x)\n", status);
    return AE_OK;
    }
    device = (adr >> 16) & 0xffff;
    function = adr & 0xffff;
    acpi_lock_hp_context();
    context = acpiphp_init_context(adev);
    if (!context) {
    acpi_unlock_hp_context();
    acpi_handle_err(handle, "No hotplug context\n");
    return AE_NOT_EXIST;
    }
    newfunc = &context.func;
    newfunc.function = function;
    newfunc.parent = bridge;
    acpi_unlock_hp_context();
//
// If this is a dock device, its _EJ0 should be executed by the dock
// notify handler after calling _DCK.
//
    if (!is_dock_device(adev) && acpi_has_method(handle, "_EJ0"))
    newfunc.flags = FUNC_HAS_EJ0;
    if (acpi_has_method(handle, "_STA"))
    newfunc.flags |= FUNC_HAS_STA;
// search for objects that share the same slot
    list_for_each_entry(slot, &bridge.slots, node)
    if (slot.device == device)
    goto slot_found;
    slot = kzalloc_obj(struct acpiphp_slot);
    if (!slot) {
    acpi_lock_hp_context();
    acpiphp_put_context(context);
    acpi_unlock_hp_context();
    return AE_NO_MEMORY;
    }
    slot.bus = bridge.pci_bus;
    slot.device = device;
    INIT_LIST_HEAD(&slot.funcs);
    list_add_tail(&slot.node, &bridge.slots);
//
// Expose slots to user space for functions that have _EJ0 or _RMV or
// are located in dock stations.  Do not expose them for devices handled
// by the native PCIe hotplug (PCIeHP) or standard PCI hotplug
// (SHPCHP), because that code is supposed to expose slots to user
// space in those cases.
//
    if ((acpi_pci_check_ejectable(pbus, handle) || is_dock_device(adev))
    && !(pdev && hotplug_is_native(pdev))) {
    unsigned long long sun;
    int retval;
    bridge.nr_slots++;
    status = acpi_evaluate_integer(handle, "_SUN", core::ptr::null_mut(), &sun);
    if (ACPI_FAILURE(status))
    sun = bridge.nr_slots;
    pr_debug("found ACPI PCI Hotplug slot %llu at PCI %04x:%02x:%02x\n",
    sun, pci_domain_nr(pbus), pbus.number, device);
    retval = acpiphp_register_hotplug_slot(slot, sun);
    if (retval) {
    slot.slot = core::ptr::null_mut();
    bridge.nr_slots--;
    if (retval == -EBUSY)
    pr_warn("Slot %llu already registered by another hotplug driver\n", sun);
    else
    pr_warn("acpiphp_register_hotplug_slot failed (err code = 0x%x)\n", retval);
    }
// Even if the slot registration fails, we can still use it.
    }
    slot_found:
    newfunc.slot = slot;
    list_add_tail(&newfunc.sibling, &slot.funcs);
    if (pci_bus_read_dev_vendor_id(pbus, PCI_DEVFN(device, function),
    &val, 60*1000))
    slot.flags |= SLOT_ENABLED;
    return AE_OK;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_bridge(bridge: *mut acpiphp_bridge) {
    static void cleanup_bridge(struct acpiphp_bridge *bridge)
    {
    struct acpiphp_slot *slot;
    struct acpiphp_func *func;
    list_for_each_entry(slot, &bridge.slots, node) {
    list_for_each_entry(func, &slot.funcs, sibling) {
    struct acpi_device *adev = func_to_acpi_device(func);
    acpi_lock_hp_context();
    adev.hp.notify = core::ptr::null_mut();
    adev.hp.fixup = core::ptr::null_mut();
    acpi_unlock_hp_context();
    }
    slot.flags |= SLOT_IS_GOING_AWAY;
    if (slot.slot)
    acpiphp_unregister_hotplug_slot(slot);
    }
    mutex_lock(&bridge_mutex);
    list_del(&bridge.list);
    mutex_unlock(&bridge_mutex);
    acpi_lock_hp_context();
    bridge.is_going_away = true;
    acpi_unlock_hp_context();
    }
//
// acpiphp_max_busnr - return the highest reserved bus number under the given bus.
// @bus: bus to start search with
//
#[no_mangle]
unsafe extern "C" fn acpiphp_max_busnr(bus: *mut pci_bus) -> c_uchar {
    static unsigned char acpiphp_max_busnr(struct pci_bus *bus)
    {
    struct pci_bus *tmp;
    unsigned char max, n;
//
// pci_bus_max_busnr will return the highest
// reserved busnr for all these children.
// that is equivalent to the bus->subordinate
// value.  We don't want to use the parent's
// bus->subordinate value because it could have
// padding in it.
//
    max = bus.busn_res.start;
    list_for_each_entry(tmp, &bus.children, node) {
    n = pci_bus_max_busnr(tmp);
    if (n > max)
    max = n;
    }
    return max;
    }
#[no_mangle]
unsafe extern "C" fn acpiphp_set_acpi_region(slot: *mut acpiphp_slot) {
    static void acpiphp_set_acpi_region(struct acpiphp_slot *slot)
    {
    struct acpiphp_func *func;
    list_for_each_entry(func, &slot.funcs, sibling) {
// _REG is optional, we don't care about if there is failure
    acpi_evaluate_reg(func_to_handle(func),
    ACPI_ADR_SPACE_PCI_CONFIG,
    ACPI_REG_CONNECT);
    }
    }
#[no_mangle]
unsafe extern "C" fn check_hotplug_bridge(slot: *mut acpiphp_slot, dev: *mut pci_dev) {
    static void check_hotplug_bridge(struct acpiphp_slot *slot, struct pci_dev *dev)
    {
    struct acpiphp_func *func;
// quirk, or pcie could set it already
    if (dev.is_hotplug_bridge)
    return;
//
// In the PCIe case, only Root Ports and Downstream Ports are capable of
// accommodating hotplug devices, so avoid marking Upstream Ports as
// "hotplug bridges".
//
    if (pci_is_pcie(dev) && pci_pcie_type(dev) == PCI_EXP_TYPE_UPSTREAM)
    return;
    list_for_each_entry(func, &slot.funcs, sibling) {
    if (PCI_FUNC(dev.devfn) == func.function) {
    dev.is_hotplug_bridge = 1;
    break;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn acpiphp_rescan_slot(slot: *mut acpiphp_slot) -> c_int {
    static int acpiphp_rescan_slot(struct acpiphp_slot *slot)
    {
    struct acpiphp_func *func;
    list_for_each_entry(func, &slot.funcs, sibling) {
    struct acpi_device *adev = func_to_acpi_device(func);
    acpi_bus_scan(adev.handle);
    if (acpi_device_enumerated(adev))
    acpi_device_set_power(adev, ACPI_STATE_D0);
    }
    return pci_scan_slot(slot.bus, PCI_DEVFN(slot.device, 0));
    }
#[no_mangle]
unsafe extern "C" fn acpiphp_native_scan_bridge(bridge: *mut pci_dev) {
    static void acpiphp_native_scan_bridge(struct pci_dev *bridge)
    {
    struct pci_bus *bus = bridge.subordinate;
    struct pci_dev *dev;
    int max;
    if (!bus)
    return;
    max = bus.busn_res.start;
// Scan already configured non-hotplug bridges
    for_each_pci_bridge(dev, bus) {
    if (!hotplug_is_native(dev))
    max = pci_scan_bridge(bus, dev, max, 0);
    }
// Scan non-hotplug bridges that need to be reconfigured
    for_each_pci_bridge(dev, bus) {
    if (hotplug_is_native(dev))
    continue;
    max = pci_scan_bridge(bus, dev, max, 1);
    if (dev.subordinate) {
    pcibios_resource_survey_bus(dev.subordinate);
    pci_bus_size_bridges(dev.subordinate);
    pci_bus_assign_resources(dev.subordinate);
    }
    }
    }
//
// enable_slot - enable, configure a slot
// @slot: slot to be enabled
// @bridge: true if enable is for the whole bridge (not a single slot)
//
// This function should be called per *physical slot*,
// not per each slot object in ACPI namespace.
//
#[no_mangle]
unsafe extern "C" fn enable_slot(slot: *mut acpiphp_slot, bridge: bool) {
    static void enable_slot(struct acpiphp_slot *slot, bool bridge)
    {
    struct pci_dev *dev;
    struct pci_bus *bus = slot.bus;
    struct acpiphp_func *func;
    if (bridge && bus.self && hotplug_is_native(bus.self)) {
//
// If native hotplug is used, it will take care of hotplug
// slot management and resource allocation for hotplug
// bridges. However, ACPI hotplug may still be used for
// non-hotplug bridges to bring in additional devices such
// as a Thunderbolt host controller.
//
    for_each_pci_bridge(dev, bus) {
    if (PCI_SLOT(dev.devfn) == slot.device)
    acpiphp_native_scan_bridge(dev);
    }
    } else {
    LIST_HEAD(add_list);
    int max, pass;
    acpiphp_rescan_slot(slot);
    max = acpiphp_max_busnr(bus);
    for (pass = 0; pass < 2; pass++) {
    for_each_pci_bridge(dev, bus) {
    if (PCI_SLOT(dev.devfn) != slot.device)
    continue;
    max = pci_scan_bridge(bus, dev, max, pass);
    if (pass && dev.subordinate) {
    check_hotplug_bridge(slot, dev);
    pcibios_resource_survey_bus(dev.subordinate);
    __pci_bus_size_bridges(dev.subordinate,
    &add_list);
    }
    }
    }
    __pci_bus_assign_resources(bus, &add_list, core::ptr::null_mut());
    }
    acpiphp_sanitize_bus(bus);
    pcie_bus_configure_settings(bus);
    acpiphp_set_acpi_region(slot);
    list_for_each_entry(dev, &bus.devices, bus_list) {
// Assume that newly added devices are powered on already.
    if (!pci_dev_is_added(dev))
    dev.current_state = PCI_D0;
    }
    pci_bus_add_devices(bus);
    slot.flags |= SLOT_ENABLED;
    list_for_each_entry(func, &slot.funcs, sibling) {
    dev = pci_get_slot(bus, PCI_DEVFN(slot.device,
    func.function));
    if (!dev) {
// Do not set SLOT_ENABLED flag if some funcs
    are not added. */
    slot.flags &= ~SLOT_ENABLED;
    continue;
    }
    pci_dev_put(dev);
    }
    }
//
// disable_slot - disable a slot
// @slot: ACPI PHP slot
//
#[no_mangle]
unsafe extern "C" fn disable_slot(slot: *mut acpiphp_slot) {
    static void disable_slot(struct acpiphp_slot *slot)
    {
    struct pci_bus *bus = slot.bus;
    struct pci_dev *dev, *prev;
    struct acpiphp_func *func;
//
// enable_slot() enumerates all functions in this device via
// pci_scan_slot(), whether they have associated ACPI hotplug
// methods (_EJ0, etc.) or not.  Therefore, we remove all functions
// here.
//
    list_for_each_entry_safe_reverse(dev, prev, &bus.devices, bus_list)
    if (PCI_SLOT(dev.devfn) == slot.device)
    pci_stop_and_remove_bus_device(dev);
    list_for_each_entry(func, &slot.funcs, sibling)
    acpi_bus_trim(func_to_acpi_device(func));
    slot.flags &= ~SLOT_ENABLED;
    }
#[no_mangle]
unsafe extern "C" fn slot_no_hotplug(slot: *mut acpiphp_slot) -> bool {
    static bool slot_no_hotplug(struct acpiphp_slot *slot)
    {
    struct pci_bus *bus = slot.bus;
    struct pci_dev *dev;
    list_for_each_entry(dev, &bus.devices, bus_list) {
    if (PCI_SLOT(dev.devfn) == slot.device && dev.ignore_hotplug)
    return true;
    }
    return false;
    }
//
// get_slot_status - get ACPI slot status
// @slot: ACPI PHP slot
//
// If a slot has _STA for each function and if any one of them
// returned non-zero status, return it.
//
// If a slot doesn't have _STA and if any one of its functions'
// configuration space is configured, return 0x0f as a _STA.
//
// Otherwise return 0.
//
#[no_mangle]
unsafe extern "C" fn get_slot_status(slot: *mut acpiphp_slot) -> c_uint {
    static unsigned int get_slot_status(struct acpiphp_slot *slot)
    {
    let mut sta: c_ulonglong = 0;
    struct acpiphp_func *func;
    u32 dvid;
    list_for_each_entry(func, &slot.funcs, sibling) {
    if (func.flags & FUNC_HAS_STA) {
    acpi_status status;
    status = acpi_evaluate_integer(func_to_handle(func),
    "_STA", core::ptr::null_mut(), &sta);
    if (ACPI_SUCCESS(status) && sta)
    break;
    } else {
    if (pci_bus_read_dev_vendor_id(slot.bus,
    PCI_DEVFN(slot.device, func.function),
    &dvid, 0)) {
    sta = ACPI_STA_ALL;
    break;
    }
    }
    }
    if (!sta) {
//
// Check for the slot itself since it may be that the
// ACPI slot is a device below PCIe upstream port so in
// that case it may not even be reachable yet.
//
    if (pci_bus_read_dev_vendor_id(slot.bus,
    PCI_DEVFN(slot.device, 0), &dvid, 0)) {
    sta = ACPI_STA_ALL;
    }
    }
    return (unsigned int)sta;
    }
#[no_mangle]
pub unsafe extern "C" fn device_status_valid(sta: c_uint) -> bool {
    static inline bool device_status_valid(unsigned int sta)
    {
//
// ACPI spec says that _STA may return bit 0 clear with bit 3 set
// if the device is valid but does not require a device driver to be
// loaded (Section 6.3.7 of ACPI 5.0A).
//
    let mut mask: c_uint = ACPI_STA_DEVICE_ENABLED | ACPI_STA_DEVICE_FUNCTIONING;
    return (sta & mask) == mask;
    }
//
// trim_stale_devices - remove PCI devices that are not responding.
// @dev: PCI device to start walking the hierarchy from.
//
#[no_mangle]
unsafe extern "C" fn trim_stale_devices(dev: *mut pci_dev) {
    static void trim_stale_devices(struct pci_dev *dev)
    {
    struct acpi_device *adev = ACPI_COMPANION(&dev.dev);
    struct pci_bus *bus = dev.subordinate;
    let mut alive: bool = dev.ignore_hotplug;
    if (adev) {
    acpi_status status;
    unsigned long long sta;
    status = acpi_evaluate_integer(adev.handle, "_STA", core::ptr::null_mut(), &sta);
    alive = alive || (ACPI_SUCCESS(status) && device_status_valid(sta));
    }
    if (!alive)
    alive = pci_device_is_present(dev);
    if (!alive) {
    pci_dev_set_disconnected(dev, core::ptr::null_mut());
    if (pci_has_subordinate(dev))
    pci_walk_bus(dev.subordinate, pci_dev_set_disconnected,
    core::ptr::null_mut());
    pci_stop_and_remove_bus_device(dev);
    if (adev)
    acpi_bus_trim(adev);
    } else if (bus) {
    struct pci_dev *child, *tmp;
// The device is a bridge. so check the bus below it.
    pm_runtime_get_sync(&dev.dev);
    list_for_each_entry_safe_reverse(child, tmp, &bus.devices, bus_list)
    trim_stale_devices(child);
    pm_runtime_put(&dev.dev);
    }
    }
//
// acpiphp_check_bridge - re-enumerate devices
// @bridge: where to begin re-enumeration
//
// Iterate over all slots under this bridge and make sure that if a
// card is present they are enabled, and if not they are disabled.
//
#[no_mangle]
unsafe extern "C" fn acpiphp_check_bridge(bridge: *mut acpiphp_bridge) {
    static void acpiphp_check_bridge(struct acpiphp_bridge *bridge)
    {
    struct acpiphp_slot *slot;
// Bail out if the bridge is going away.
    if (bridge.is_going_away)
    return;
    if (bridge.pci_dev)
    pm_runtime_get_sync(&bridge.pci_dev.dev);
    list_for_each_entry(slot, &bridge.slots, node) {
    struct pci_bus *bus = slot.bus;
    struct pci_dev *dev, *tmp;
    if (slot_no_hotplug(slot)) {
    ; /* do nothing */
    } else if (device_status_valid(get_slot_status(slot))) {
// remove stale devices if any
    list_for_each_entry_safe_reverse(dev, tmp,
    &bus.devices, bus_list)
    if (PCI_SLOT(dev.devfn) == slot.device)
    trim_stale_devices(dev);
// configure all functions
    enable_slot(slot, true);
    } else {
    disable_slot(slot);
    }
    }
    if (bridge.pci_dev)
    pm_runtime_put(&bridge.pci_dev.dev);
    }
//
// Remove devices for which we could not assign resources, call
// arch specific code to fix-up the bus
//
#[no_mangle]
unsafe extern "C" fn acpiphp_sanitize_bus(bus: *mut pci_bus) {
    static void acpiphp_sanitize_bus(struct pci_bus *bus)
    {
    struct pci_dev *dev, *tmp;
    int i;
    let mut type_mask: c_ulong = IORESOURCE_IO | IORESOURCE_MEM;
    list_for_each_entry_safe_reverse(dev, tmp, &bus.devices, bus_list) {
    for (i = 0; i < PCI_BRIDGE_RESOURCES; i++) {
    struct resource *res = &dev.resource[i];
    if ((res.flags & type_mask) && !res.start &&
    res.end) {
// Could not assign a required resources
// for this device, remove it
    pci_stop_and_remove_bus_device(dev);
    break;
    }
    }
    }
    }
//
// ACPI event handlers
//
#[no_mangle]
pub unsafe extern "C" fn acpiphp_check_host_bridge(adev: *mut acpi_device) {
    void acpiphp_check_host_bridge(struct acpi_device *adev)
    {
    struct acpiphp_bridge *bridge = core::ptr::null_mut();
    acpi_lock_hp_context();
    if (adev.hp) {
    bridge = to_acpiphp_root_context(adev.hp).root_bridge;
    if (bridge)
    get_bridge(bridge);
    }
    acpi_unlock_hp_context();
    if (bridge) {
    pci_lock_rescan_remove();
    acpiphp_check_bridge(bridge);
    pci_unlock_rescan_remove();
    put_bridge(bridge);
    }
    }
    static int acpiphp_disable_and_eject_slot(struct acpiphp_slot *slot);
#[no_mangle]
unsafe extern "C" fn hotplug_event(type: u32, context: *mut acpiphp_context) {
    static void hotplug_event(u32 type, struct acpiphp_context *context)
    {
    let mut handle: acpi_handle = context.hp.self.handle;
    struct acpiphp_func *func = &context.func;
    struct acpiphp_slot *slot = func.slot;
    struct acpiphp_bridge *bridge;
    acpi_lock_hp_context();
    bridge = context.bridge;
    if (bridge)
    get_bridge(bridge);
    acpi_unlock_hp_context();
    pci_lock_rescan_remove();
    switch (type) {
    case ACPI_NOTIFY_BUS_CHECK:
// bus re-enumerate
    acpi_handle_debug(handle, "Bus check in %s()\n", __func__);
    if (bridge)
    acpiphp_check_bridge(bridge);
#[no_mangle]
pub unsafe extern "C" fn if(SLOT_IS_GOING_AWAY): !(slot->flags &) -> else {
    else if (!(slot.flags & SLOT_IS_GOING_AWAY))
    enable_slot(slot, false);
    break;
    case ACPI_NOTIFY_DEVICE_CHECK:
// device check
    acpi_handle_debug(handle, "Device check in %s()\n", __func__);
    if (bridge) {
    acpiphp_check_bridge(bridge);
    } else if (!(slot.flags & SLOT_IS_GOING_AWAY)) {
//
// Check if anything has changed in the slot and rescan
// from the parent if that's the case.
//
    if (acpiphp_rescan_slot(slot))
    acpiphp_check_bridge(func.parent);
    }
    break;
    case ACPI_NOTIFY_EJECT_REQUEST:
// request device eject
    acpi_handle_debug(handle, "Eject request in %s()\n", __func__);
    acpiphp_disable_and_eject_slot(slot);
    break;
    }
    pci_unlock_rescan_remove();
    if (bridge)
    put_bridge(bridge);
    }
#[no_mangle]
unsafe extern "C" fn acpiphp_hotplug_notify(adev: *mut acpi_device, type: u32) -> c_int {
    static int acpiphp_hotplug_notify(struct acpi_device *adev, u32 type)
    {
    struct acpiphp_context *context;
    context = acpiphp_grab_context(adev);
    if (!context)
    return -ENODATA;
    hotplug_event(type, context);
    acpiphp_let_context_go(context);
    return 0;
    }
//
// acpiphp_enumerate_slots - Enumerate PCI slots for a given bus.
// @bus: PCI bus to enumerate the slots for.
//
// A "slot" is an object associated with a PCI device number.  All functions
// (PCI devices) with the same bus and device number belong to the same slot.
//
#[no_mangle]
pub unsafe extern "C" fn acpiphp_enumerate_slots(bus: *mut pci_bus) {
    void acpiphp_enumerate_slots(struct pci_bus *bus)
    {
    struct acpiphp_bridge *bridge;
    struct acpi_device *adev;
    acpi_handle handle;
    acpi_status status;
    if (acpiphp_disabled)
    return;
    adev = ACPI_COMPANION(bus.bridge);
    if (!adev)
    return;
    handle = adev.handle;
    bridge = kzalloc_obj(struct acpiphp_bridge);
    if (!bridge)
    return;
    INIT_LIST_HEAD(&bridge.slots);
    kref_init(&bridge.ref);
    bridge.pci_dev = pci_dev_get(bus.self);
    bridge.pci_bus = bus;
//
// Grab a ref to the subordinate PCI bus in case the bus is
// removed via PCI core logical hotplug. The ref pins the bus
// (which we access during module unload).
//
    get_device(&bus.dev);
    acpi_lock_hp_context();
    if (pci_is_root_bus(bridge.pci_bus)) {
    struct acpiphp_root_context *root_context;
    root_context = kzalloc_obj(*root_context);
    if (!root_context)
    goto err;
    root_context.root_bridge = bridge;
    acpi_set_hp_context(adev, &root_context.hp);
    } else {
    struct acpiphp_context *context;
//
// This bridge should have been registered as a hotplug function
// under its parent, so the context should be there, unless the
// parent is going to be handled by pciehp, in which case this
// bridge is not interesting to us either.
//
    context = acpiphp_get_context(adev);
    if (!context)
    goto err;
    bridge.context = context;
    context.bridge = bridge;
// Get a reference to the parent bridge.
    get_bridge(context.func.parent);
    }
    acpi_unlock_hp_context();
// Must be added to the list prior to calling acpiphp_add_context().
    mutex_lock(&bridge_mutex);
    list_add(&bridge.list, &bridge_list);
    mutex_unlock(&bridge_mutex);
// register all slot objects under this bridge
    status = acpi_walk_namespace(ACPI_TYPE_DEVICE, handle, 1,
    acpiphp_add_context, core::ptr::null_mut(), bridge, core::ptr::null_mut());
    if (ACPI_FAILURE(status)) {
    acpi_handle_err(handle, "failed to register slots\n");
    cleanup_bridge(bridge);
    put_bridge(bridge);
    }
    return;
    err:
    acpi_unlock_hp_context();
    put_device(&bus.dev);
    pci_dev_put(bridge.pci_dev);
    kfree(bridge);
    }
#[no_mangle]
unsafe extern "C" fn acpiphp_drop_bridge(bridge: *mut acpiphp_bridge) {
    static void acpiphp_drop_bridge(struct acpiphp_bridge *bridge)
    {
    if (pci_is_root_bus(bridge.pci_bus)) {
    struct acpiphp_root_context *root_context;
    struct acpi_device *adev;
    acpi_lock_hp_context();
    adev = ACPI_COMPANION(bridge.pci_bus.bridge);
    root_context = to_acpiphp_root_context(adev.hp);
    adev.hp = core::ptr::null_mut();
    acpi_unlock_hp_context();
    kfree(root_context);
    }
    cleanup_bridge(bridge);
    put_bridge(bridge);
    }
//
// acpiphp_remove_slots - Remove slot objects associated with a given bus.
// @bus: PCI bus to remove the slot objects for.
//
#[no_mangle]
pub unsafe extern "C" fn acpiphp_remove_slots(bus: *mut pci_bus) {
    void acpiphp_remove_slots(struct pci_bus *bus)
    {
    struct acpiphp_bridge *bridge;
    if (acpiphp_disabled)
    return;
    mutex_lock(&bridge_mutex);
    list_for_each_entry(bridge, &bridge_list, list)
    if (bridge.pci_bus == bus) {
    mutex_unlock(&bridge_mutex);
    acpiphp_drop_bridge(bridge);
    return;
    }
    mutex_unlock(&bridge_mutex);
    }
//
// acpiphp_enable_slot - power on slot
// @slot: ACPI PHP slot
//
#[no_mangle]
pub unsafe extern "C" fn acpiphp_enable_slot(slot: *mut acpiphp_slot) -> c_int {
    int acpiphp_enable_slot(struct acpiphp_slot *slot)
    {
    pci_lock_rescan_remove();
    if (slot.flags & SLOT_IS_GOING_AWAY) {
    pci_unlock_rescan_remove();
    return -ENODEV;
    }
// configure all functions
    if (!(slot.flags & SLOT_ENABLED))
    enable_slot(slot, false);
    pci_unlock_rescan_remove();
    return 0;
    }
//
// acpiphp_disable_and_eject_slot - power off and eject slot
// @slot: ACPI PHP slot
//
#[no_mangle]
unsafe extern "C" fn acpiphp_disable_and_eject_slot(slot: *mut acpiphp_slot) -> c_int {
    static int acpiphp_disable_and_eject_slot(struct acpiphp_slot *slot)
    {
    struct acpiphp_func *func;
    if (slot.flags & SLOT_IS_GOING_AWAY)
    return -ENODEV;
// unconfigure all functions
    disable_slot(slot);
    list_for_each_entry(func, &slot.funcs, sibling)
    if (func.flags & FUNC_HAS_EJ0) {
    let mut handle: acpi_handle = func_to_handle(func);
    if (ACPI_FAILURE(acpi_evaluate_ej0(handle)))
    acpi_handle_err(handle, "_EJ0 failed\n");
    break;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn acpiphp_disable_slot(slot: *mut acpiphp_slot) -> c_int {
    int acpiphp_disable_slot(struct acpiphp_slot *slot)
    {
    int ret;
//
// Acquire acpi_scan_lock to ensure that the execution of _EJ0 in
// acpiphp_disable_and_eject_slot() will be synchronized properly.
//
    acpi_scan_lock_acquire();
    pci_lock_rescan_remove();
    ret = acpiphp_disable_and_eject_slot(slot);
    pci_unlock_rescan_remove();
    acpi_scan_lock_release();
    return ret;
    }
//
// slot enabled:  1
// slot disabled: 0
//
#[no_mangle]
pub unsafe extern "C" fn acpiphp_get_power_status(slot: *mut acpiphp_slot) -> u8 {
    u8 acpiphp_get_power_status(struct acpiphp_slot *slot)
    {
    return (slot.flags & SLOT_ENABLED);
    }
//
// latch   open:  1
// latch closed:  0
//
#[no_mangle]
pub unsafe extern "C" fn acpiphp_get_latch_status(slot: *mut acpiphp_slot) -> u8 {
    u8 acpiphp_get_latch_status(struct acpiphp_slot *slot)
    {
    return !(get_slot_status(slot) & ACPI_STA_DEVICE_UI);
    }
//
// adapter presence : 1
// absence : 0
//
#[no_mangle]
pub unsafe extern "C" fn acpiphp_get_adapter_status(slot: *mut acpiphp_slot) -> u8 {
    u8 acpiphp_get_adapter_status(struct acpiphp_slot *slot)
    {
    return !!get_slot_status(slot);
    }
