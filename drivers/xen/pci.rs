//! Automatically rewritten from C to Rust
//! Source: drivers/xen/pci.c
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
// Copyright (c) 2009, Intel Corporation.
//
// Author: Weidong Han <weidong.han@intel.com>
//

    static int xen_mcfg_late(void);

    let mut pci_seg_supported: static bool __read_mostly = true;
#[no_mangle]
unsafe extern "C" fn xen_add_device(dev: *mut device) -> c_int {
    static int xen_add_device(struct device *dev)
    {
    int r;
    struct pci_dev *pci_dev = to_pci_dev(dev);

    struct pci_dev *physfn = pci_dev.physfn;

    let mut pci_mcfg_reserved: static bool = false;
//
// Reserve MCFG areas in Xen on first invocation due to this being
// potentially called from inside of acpi_init immediately after
// MCFG table has been finally parsed.
//
    if (!pci_mcfg_reserved) {
    xen_mcfg_late();
    pci_mcfg_reserved = true;
    }

    if (pci_domain_nr(pci_dev.bus) >> 16) {
//
// The hypercall interface is limited to 16bit PCI segment
// values, do not attempt to register devices with Xen in
// segments greater or equal than 0x10000.
//
    dev_info(dev,
    "not registering with Xen: invalid PCI segment\n");
    return 0;
    }
    if (pci_seg_supported) {
    DEFINE_RAW_FLEX(struct physdev_pci_device_add, add, optarr, 1);
    add.seg = pci_domain_nr(pci_dev.bus);
    add.bus = pci_dev.bus.number;
    add.devfn = pci_dev.devfn;

    acpi_handle handle;

    if (pci_dev.is_virtfn) {
    add.flags = XEN_PCI_DEV_VIRTFN;
    add.physfn.bus = physfn.bus.number;
    add.physfn.devfn = physfn.devfn;
    } else

    if (pci_ari_enabled(pci_dev.bus) && PCI_SLOT(pci_dev.devfn))
    add.flags = XEN_PCI_DEV_EXTFN;

    handle = ACPI_HANDLE(&pci_dev.dev);

    if (!handle && pci_dev.is_virtfn)
    handle = ACPI_HANDLE(physfn.bus.bridge);

    if (!handle) {
//
// This device was not listed in the ACPI name space at
// all. Try to get acpi handle of parent pci bus.
//
    struct pci_bus *pbus;
    for (pbus = pci_dev.bus; pbus; pbus = pbus.parent) {
    handle = acpi_pci_get_bridge_handle(pbus);
    if (handle)
    break;
    }
    }
    if (handle) {
    acpi_status status;
    do {
    unsigned long long pxm;
    status = acpi_evaluate_integer(handle, "_PXM",
    core::ptr::null_mut(), &pxm);
    if (ACPI_SUCCESS(status)) {
    add.optarr[0] = pxm;
    add.flags |= XEN_PCI_DEV_PXM;
    break;
    }
    status = acpi_get_parent(handle, &handle);
    } while (ACPI_SUCCESS(status));
    }

    r = HYPERVISOR_physdev_op(PHYSDEVOP_pci_device_add, add);
    if (r != -ENOSYS)
    return r;
    pci_seg_supported = false;
    }
    if (pci_domain_nr(pci_dev.bus))
    r = -ENOSYS;

#[no_mangle]
pub unsafe extern "C" fn if(_arg: pci_dev->is_virtfn) -> else {
    struct physdev_manage_pci_ext manage_pci_ext = {
    .bus		= pci_dev.bus.number,
    .devfn		= pci_dev.devfn,
    .is_virtfn 	= 1,
    .physfn.bus	= physfn.bus.number,
    .physfn.devfn	= physfn.devfn,
    };
    r = HYPERVISOR_physdev_op(PHYSDEVOP_manage_pci_add_ext,
    &manage_pci_ext);
    }

#[no_mangle]
pub unsafe extern "C" fn if(PCI_SLOT(pci_dev->devfn): pci_ari_enabled(pci_dev->bus) &&) -> else {
    struct physdev_manage_pci_ext manage_pci_ext = {
    .bus		= pci_dev.bus.number,
    .devfn		= pci_dev.devfn,
    .is_extfn	= 1,
    };
    r = HYPERVISOR_physdev_op(PHYSDEVOP_manage_pci_add_ext,
    &manage_pci_ext);
    } else {
    struct physdev_manage_pci manage_pci = {
    .bus	= pci_dev.bus.number,
    .devfn	= pci_dev.devfn,
    };
    r = HYPERVISOR_physdev_op(PHYSDEVOP_manage_pci_add,
    &manage_pci);
    }
    return r;
    }
#[no_mangle]
unsafe extern "C" fn xen_remove_device(dev: *mut device) -> c_int {
    static int xen_remove_device(struct device *dev)
    {
    int r;
    struct pci_dev *pci_dev = to_pci_dev(dev);
    if (pci_domain_nr(pci_dev.bus) >> 16) {
//
// The hypercall interface is limited to 16bit PCI segment
// values.
//
    dev_info(dev,
    "not unregistering with Xen: invalid PCI segment\n");
    return 0;
    }
    if (pci_seg_supported) {
    struct physdev_pci_device device = {
    .seg = pci_domain_nr(pci_dev.bus),
    .bus = pci_dev.bus.number,
    .devfn = pci_dev.devfn
    };
    r = HYPERVISOR_physdev_op(PHYSDEVOP_pci_device_remove,
    &device);
    } else if (pci_domain_nr(pci_dev.bus))
    r = -ENOSYS;
    else {
    struct physdev_manage_pci manage_pci = {
    .bus = pci_dev.bus.number,
    .devfn = pci_dev.devfn
    };
    r = HYPERVISOR_physdev_op(PHYSDEVOP_manage_pci_remove,
    &manage_pci);
    }
    return r;
    }
#[no_mangle]
pub unsafe extern "C" fn xen_reset_device(dev: *const pci_dev) -> c_int {
    int xen_reset_device(const struct pci_dev *dev)
    {
    struct pci_device_reset device = {
    .dev.seg = pci_domain_nr(dev.bus),
    .dev.bus = dev.bus.number,
    .dev.devfn = dev.devfn,
    .flags = PCI_DEVICE_RESET_FLR,
    };
    if (pci_domain_nr(dev.bus) >> 16) {
//
// The hypercall interface is limited to 16bit PCI segment
// values.
//
    dev_info(&dev.dev,
    "unable to notify Xen of device reset: invalid PCI segment\n");
    return 0;
    }
    return HYPERVISOR_physdev_op(PHYSDEVOP_pci_device_reset, &device);
    }
    EXPORT_SYMBOL_GPL(xen_reset_device);
    static int xen_pci_notifier(struct notifier_block *nb,
    unsigned long action, void *data)
    {
    struct device *dev = data;
    let mut r: c_int = 0;
    switch (action) {
    case BUS_NOTIFY_ADD_DEVICE:
    r = xen_add_device(dev);
    break;
    case BUS_NOTIFY_DEL_DEVICE:
    r = xen_remove_device(dev);
    break;
    default:
    return NOTIFY_DONE;
    }
    if (r)
    dev_err(dev, "Failed to %s - passthrough or MSI/MSI-X might fail!\n",
    action == BUS_NOTIFY_ADD_DEVICE ? "add" :
    (action == BUS_NOTIFY_DEL_DEVICE ? "delete" : "?"));
    return NOTIFY_OK;
    }
    static struct notifier_block device_nb = {
    .notifier_call = xen_pci_notifier,
    };
#[no_mangle]
unsafe extern "C" fn register_xen_pci_notifier() -> int __init {
    static int __init register_xen_pci_notifier(void)
    {
    if (!xen_initial_domain())
    return 0;
    return bus_register_notifier(&pci_bus_type, &device_nb);
    }
    arch_initcall(register_xen_pci_notifier);

#[no_mangle]
unsafe extern "C" fn xen_mcfg_late() -> c_int {
    static int xen_mcfg_late(void)
    {
    struct pci_mmcfg_region *cfg;
    int rc;
    if (!xen_initial_domain())
    return 0;
    if ((pci_probe & PCI_PROBE_MMCONF) == 0)
    return 0;
    if (list_empty(&pci_mmcfg_list))
    return 0;
// Check whether they are in the right area.
    list_for_each_entry(cfg, &pci_mmcfg_list, list) {
    struct physdev_pci_mmcfg_reserved r;
    r.address = cfg.address;
    r.segment = cfg.segment;
    r.start_bus = cfg.start_bus;
    r.end_bus = cfg.end_bus;
    r.flags = XEN_PCI_MMCFG_RESERVED;
    rc = HYPERVISOR_physdev_op(PHYSDEVOP_pci_mmcfg_reserved, &r);
    switch (rc) {
    case 0:
    case -ENOSYS:
    continue;
    default:
    pr_warn("Failed to report MMCONFIG reservation"
    " state for %s to hypervisor"
    " (%d)\n",
    cfg.name, rc);
    }
    }
    return 0;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_device_domain_owner {
    pub domain: domid_t,
    pub dev: *mut pci_dev,
    pub list: list_head,
}

    static DEFINE_SPINLOCK(dev_domain_list_spinlock);
    static LIST_HEAD(dev_domain_list);
    static struct xen_device_domain_owner *find_device(struct pci_dev *dev)
    {
    struct xen_device_domain_owner *owner;
    list_for_each_entry(owner, &dev_domain_list, list) {
    if (owner.dev == dev)
    return owner;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn xen_find_device_domain_owner(dev: *mut pci_dev) -> c_int {
    int xen_find_device_domain_owner(struct pci_dev *dev)
    {
    struct xen_device_domain_owner *owner;
    let mut domain: c_int = -ENODEV;
    spin_lock(&dev_domain_list_spinlock);
    owner = find_device(dev);
    if (owner)
    domain = owner.domain;
    spin_unlock(&dev_domain_list_spinlock);
    return domain;
    }
    EXPORT_SYMBOL_GPL(xen_find_device_domain_owner);
#[no_mangle]
pub unsafe extern "C" fn xen_register_device_domain_owner(dev: *mut pci_dev, domain: u16) -> c_int {
    int xen_register_device_domain_owner(struct pci_dev *dev, uint16_t domain)
    {
    struct xen_device_domain_owner *owner;
    owner = kzalloc_obj(struct xen_device_domain_owner);
    if (!owner)
    return -ENODEV;
    spin_lock(&dev_domain_list_spinlock);
    if (find_device(dev)) {
    spin_unlock(&dev_domain_list_spinlock);
    kfree(owner);
    return -EEXIST;
    }
    owner.domain = domain;
    owner.dev = dev;
    list_add_tail(&owner.list, &dev_domain_list);
    spin_unlock(&dev_domain_list_spinlock);
    return 0;
    }
    EXPORT_SYMBOL_GPL(xen_register_device_domain_owner);
#[no_mangle]
pub unsafe extern "C" fn xen_unregister_device_domain_owner(dev: *mut pci_dev) -> c_int {
    int xen_unregister_device_domain_owner(struct pci_dev *dev)
    {
    struct xen_device_domain_owner *owner;
    spin_lock(&dev_domain_list_spinlock);
    owner = find_device(dev);
    if (!owner) {
    spin_unlock(&dev_domain_list_spinlock);
    return -ENODEV;
    }
    list_del(&owner.list);
    spin_unlock(&dev_domain_list_spinlock);
    kfree(owner);
    return 0;
    }
    EXPORT_SYMBOL_GPL(xen_unregister_device_domain_owner);
