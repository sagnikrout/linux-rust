//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/pci_slot.c
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
// pci_slot.c - ACPI PCI Slot Driver
//
// The code here is heavily leveraged from the acpiphp module.
// Thanks to Matthew Wilcox <matthew@wil.cx> for much guidance.
// Thanks to Kenji Kaneshige <kaneshige.kenji@jp.fujitsu.com> for code
// review and fixes.
//
// Copyright (C) 2007-2008 Hewlett-Packard Development Company, L.P.
// Alex Chiang <achiang@hp.com>
//
// Copyright (C) 2013 Huawei Tech. Co., Ltd.
// Jiang Liu <jiang.liu@huawei.com>
//

    static int check_sta_before_sun;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pci_slot {
    pub /: *mut *mut *mut pci_slot pci_slot; / corresponding pci_slot,
    pub /: *mut *mut list_head list; / node in the list of slots,
}

    static LIST_HEAD(slot_list);
    static DEFINE_MUTEX(slot_list_lock);
    static int
    check_slot(acpi_handle handle, unsigned long long *sun)
    {
    let mut device: c_int = -1;
    unsigned long long sta;
    acpi_status status;
    u64 adr;
    let mut buffer: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    acpi_get_name(handle, ACPI_FULL_PATHNAME, &buffer);
    pr_debug("Checking slot on path: %s\n", (char *)buffer.pointer);
    if (check_sta_before_sun) {
// If SxFy doesn't have _STA, we just assume it's there
    status = acpi_evaluate_integer(handle, "_STA", core::ptr::null_mut(), &sta);
    if (ACPI_SUCCESS(status) && !(sta & ACPI_STA_DEVICE_PRESENT))
    goto out;
    }
    if (acpi_get_local_u64_address(handle, &adr)) {
    pr_debug("_ADR returned with failure on %s\n",
    (char *)buffer.pointer);
    goto out;
    }
// No _SUN == not a slot == bail
    status = acpi_evaluate_integer(handle, "_SUN", core::ptr::null_mut(), sun);
    if (ACPI_FAILURE(status)) {
    pr_debug("_SUN returned %d on %s\n",
    status, (char *)buffer.pointer);
    goto out;
    }
    device = (adr >> 16) & 0xffff;
    out:
    kfree(buffer.pointer);
    return device;
    }
//
// Check whether handle has an associated slot and create PCI slot if it has.
//
    static acpi_status
    register_slot(acpi_handle handle, u32 lvl, void *context, void **rv)
    {
    int device;
    unsigned long long sun;
    char name[SLOT_NAME_SIZE];
    struct acpi_pci_slot *slot;
    struct pci_slot *pci_slot;
    struct pci_bus *pci_bus = context;
    device = check_slot(handle, &sun);
    if (device < 0)
    return AE_OK;
//
// There may be multiple PCI functions associated with the same slot.
// Check whether PCI slot has already been created for this PCI device.
//
    list_for_each_entry(slot, &slot_list, list) {
    pci_slot = slot.pci_slot;
    if (pci_slot.bus == pci_bus && pci_slot.number == device)
    return AE_OK;
    }
    slot = kmalloc_obj(*slot);
    if (!slot)
    return AE_OK;
    snprintf(name, sizeof(name), "%llu", sun);
    pci_slot = pci_create_slot(pci_bus, device, name, core::ptr::null_mut());
    if (IS_ERR(pci_slot)) {
    pr_err("pci_create_slot returned %pe\n", pci_slot);
    kfree(slot);
    return AE_OK;
    }
    slot.pci_slot = pci_slot;
    list_add(&slot.list, &slot_list);
    get_device(&pci_bus.dev);
    pr_debug("%p, pci_bus: %x, device: %d, name: %s\n",
    pci_slot, pci_bus.number, device, name);
    return AE_OK;
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_pci_slot_enumerate(bus: *mut pci_bus) {
    void acpi_pci_slot_enumerate(struct pci_bus *bus)
    {
    let mut handle: acpi_handle = ACPI_HANDLE(bus.bridge);
    if (handle) {
    mutex_lock(&slot_list_lock);
    acpi_walk_namespace(ACPI_TYPE_DEVICE, handle, 1,
    register_slot, core::ptr::null_mut(), bus, core::ptr::null_mut());
    mutex_unlock(&slot_list_lock);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_pci_slot_remove(bus: *mut pci_bus) {
    void acpi_pci_slot_remove(struct pci_bus *bus)
    {
    struct acpi_pci_slot *slot, *tmp;
    mutex_lock(&slot_list_lock);
    list_for_each_entry_safe(slot, tmp, &slot_list, list) {
    if (slot.pci_slot.bus == bus) {
    list_del(&slot.list);
    pci_destroy_slot(slot.pci_slot);
    put_device(&bus.dev);
    kfree(slot);
    }
    }
    mutex_unlock(&slot_list_lock);
    }
#[no_mangle]
unsafe extern "C" fn do_sta_before_sun(d: *const dmi_system_id) -> c_int {
    static int do_sta_before_sun(const struct dmi_system_id *d)
    {
    pr_info("%s detected: will evaluate _STA before calling _SUN\n",
    d.ident);
    check_sta_before_sun = 1;
    return 0;
    }
    static const struct dmi_system_id acpi_pci_slot_dmi_table[] __initconst = {
//
// Fujitsu Primequest machines will return 1023 to indicate an
// error if the _SUN method is evaluated on SxFy objects that
// are not present (as indicated by _STA), so for those machines,
// we want to check _STA before evaluating _SUN.
//
    {
    .callback = do_sta_before_sun,
    .ident = "Fujitsu PRIMEQUEST",
    .matches = {
    DMI_MATCH(DMI_BIOS_VENDOR, "FUJITSU LIMITED"),
    DMI_MATCH(DMI_BIOS_VERSION, "PRIMEQUEST"),
    },
    },
    {}
    };
#[no_mangle]
pub unsafe extern "C" fn acpi_pci_slot_init() -> void __init {
    void __init acpi_pci_slot_init(void)
    {
    dmi_check_system(acpi_pci_slot_dmi_table);
    }
