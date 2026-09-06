//! Automatically rewritten from C to Rust
//! Source: drivers/xen/xen-pciback/vpci.c
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


// SPDX-License-Identifier: GPL-2.0
//
// PCI Backend - Provides a Virtual PCI bus (with real devices)
// to the frontend
//
// Author: Ryan Wilson <hap9@epoch.ncsc.mil>
//

pub const PCI_SLOT_MAX: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpci_dev_data {
// Access to dev_list must be protected by lock
    pub dev_list: [list_head; PCI_SLOT_MAX],
    pub lock: mutex,
}

    static inline struct list_head *list_first(struct list_head *head)
    {
    return head.next;
    }
    static struct pci_dev *__xen_pcibk_get_pci_dev(struct xen_pcibk_device *pdev,
    unsigned int domain,
    unsigned int bus,
    unsigned int devfn)
    {
    struct pci_dev_entry *entry;
    struct pci_dev *dev = core::ptr::null_mut();
    struct vpci_dev_data *vpci_dev = pdev.pci_dev_data;
    if (domain != 0 || bus != 0)
    return core::ptr::null_mut();
    if (PCI_SLOT(devfn) < PCI_SLOT_MAX) {
    mutex_lock(&vpci_dev.lock);
    list_for_each_entry(entry,
    &vpci_dev.dev_list[PCI_SLOT(devfn)],
    list) {
    if (PCI_FUNC(entry.dev.devfn) == PCI_FUNC(devfn)) {
    dev = entry.dev;
    break;
    }
    }
    mutex_unlock(&vpci_dev.lock);
    }
    return dev;
    }
#[no_mangle]
pub unsafe extern "C" fn match_slot(l: *mut pci_dev, r: *mut pci_dev) -> c_int {
    static inline int match_slot(struct pci_dev *l, struct pci_dev *r)
    {
    if (pci_domain_nr(l.bus) == pci_domain_nr(r.bus)
    && l.bus == r.bus && PCI_SLOT(l.devfn) == PCI_SLOT(r.devfn))
    return 1;
    return 0;
    }
    static int __xen_pcibk_add_pci_dev(struct xen_pcibk_device *pdev,
    struct pci_dev *dev, int devid,
    publish_pci_dev_cb publish_cb)
    {
    let mut err: c_int = 0, slot, func = PCI_FUNC(dev.devfn);
    struct pci_dev_entry *t, *dev_entry;
    struct vpci_dev_data *vpci_dev = pdev.pci_dev_data;
    if ((dev.class >> 24) == PCI_BASE_CLASS_BRIDGE) {
    err = -EFAULT;
    xenbus_dev_fatal(pdev.xdev, err,
    "Can't export bridges on the virtual PCI bus");
    goto out;
    }
    dev_entry = kmalloc_obj(*dev_entry);
    if (!dev_entry) {
    err = -ENOMEM;
    xenbus_dev_fatal(pdev.xdev, err,
    "Error adding entry to virtual PCI bus");
    goto out;
    }
    dev_entry.dev = dev;
    mutex_lock(&vpci_dev.lock);
//
// Keep multi-function devices together on the virtual PCI bus, except
// that we want to keep virtual functions at func 0 on their own. They
// aren't multi-function devices and hence their presence at func 0
// may cause guests to not scan the other functions.
//
    if (!dev.is_virtfn || func) {
    for (slot = 0; slot < PCI_SLOT_MAX; slot++) {
    if (list_empty(&vpci_dev.dev_list[slot]))
    continue;
    t = list_entry(list_first(&vpci_dev.dev_list[slot]),
    struct pci_dev_entry, list);
    if (t.dev.is_virtfn && !PCI_FUNC(t.dev.devfn))
    continue;
    if (match_slot(dev, t.dev)) {
    dev_info(&dev.dev, "vpci: assign to virtual slot %d func %d\n",
    slot, func);
    list_add_tail(&dev_entry.list,
    &vpci_dev.dev_list[slot]);
    goto unlock;
    }
    }
    }
// Assign to a new slot on the virtual PCI bus
    for (slot = 0; slot < PCI_SLOT_MAX; slot++) {
    if (list_empty(&vpci_dev.dev_list[slot])) {
    dev_info(&dev.dev, "vpci: assign to virtual slot %d\n",
    slot);
    list_add_tail(&dev_entry.list,
    &vpci_dev.dev_list[slot]);
    goto unlock;
    }
    }
    err = -ENOMEM;
    xenbus_dev_fatal(pdev.xdev, err,
    "No more space on root virtual PCI bus");
    unlock:
    mutex_unlock(&vpci_dev.lock);
// Publish this device.
    if (!err)
    err = publish_cb(pdev, 0, 0, PCI_DEVFN(slot, func), devid);
    else
    kfree(dev_entry);
    out:
    return err;
    }
    static void __xen_pcibk_release_pci_dev(struct xen_pcibk_device *pdev,
    struct pci_dev *dev, bool lock)
    {
    int slot;
    struct vpci_dev_data *vpci_dev = pdev.pci_dev_data;
    struct pci_dev *found_dev = core::ptr::null_mut();
    mutex_lock(&vpci_dev.lock);
    for (slot = 0; slot < PCI_SLOT_MAX; slot++) {
    struct pci_dev_entry *e;
    list_for_each_entry(e, &vpci_dev.dev_list[slot], list) {
    if (e.dev == dev) {
    list_del(&e.list);
    found_dev = e.dev;
    kfree(e);
    goto out;
    }
    }
    }
    out:
    mutex_unlock(&vpci_dev.lock);
    if (found_dev) {
    if (lock)
    device_lock(&found_dev.dev);
    pcistub_put_pci_dev(found_dev);
    if (lock)
    device_unlock(&found_dev.dev);
    }
    }
#[no_mangle]
unsafe extern "C" fn __xen_pcibk_init_devices(pdev: *mut xen_pcibk_device) -> c_int {
    static int __xen_pcibk_init_devices(struct xen_pcibk_device *pdev)
    {
    int slot;
    struct vpci_dev_data *vpci_dev;
    vpci_dev = kmalloc_obj(*vpci_dev);
    if (!vpci_dev)
    return -ENOMEM;
    mutex_init(&vpci_dev.lock);
    for (slot = 0; slot < PCI_SLOT_MAX; slot++)
    INIT_LIST_HEAD(&vpci_dev.dev_list[slot]);
    pdev.pci_dev_data = vpci_dev;
    return 0;
    }
    static int __xen_pcibk_publish_pci_roots(struct xen_pcibk_device *pdev,
    publish_pci_root_cb publish_cb)
    {
// The Virtual PCI bus has only one root
    return publish_cb(pdev, 0, 0);
    }
#[no_mangle]
unsafe extern "C" fn __xen_pcibk_release_devices(pdev: *mut xen_pcibk_device) {
    static void __xen_pcibk_release_devices(struct xen_pcibk_device *pdev)
    {
    int slot;
    struct vpci_dev_data *vpci_dev = pdev.pci_dev_data;
    for (slot = 0; slot < PCI_SLOT_MAX; slot++) {
    struct pci_dev_entry *e, *tmp;
    list_for_each_entry_safe(e, tmp, &vpci_dev.dev_list[slot],
    list) {
    struct pci_dev *dev = e.dev;
    list_del(&e.list);
    device_lock(&dev.dev);
    pcistub_put_pci_dev(dev);
    device_unlock(&dev.dev);
    kfree(e);
    }
    }
    kfree(vpci_dev);
    pdev.pci_dev_data = core::ptr::null_mut();
    }
    static int __xen_pcibk_get_pcifront_dev(struct pci_dev *pcidev,
    struct xen_pcibk_device *pdev,
    unsigned int *domain, unsigned int *bus,
    unsigned int *devfn)
    {
    struct pci_dev_entry *entry;
    struct vpci_dev_data *vpci_dev = pdev.pci_dev_data;
    let mut found: c_int = 0, slot;
    mutex_lock(&vpci_dev.lock);
    for (slot = 0; slot < PCI_SLOT_MAX; slot++) {
    list_for_each_entry(entry,
    &vpci_dev.dev_list[slot],
    list) {
    if (entry.dev == pcidev) {
    found = 1;
// domain = 0;
// bus = 0;
// devfn = PCI_DEVFN(slot,
    PCI_FUNC(pcidev.devfn));
    }
    }
    }
    mutex_unlock(&vpci_dev.lock);
    return found;
    }
    const struct xen_pcibk_backend xen_pcibk_vpci_backend = {
    .name		= "vpci",
    .init		= __xen_pcibk_init_devices,
    .free		= __xen_pcibk_release_devices,
    .find		= __xen_pcibk_get_pcifront_dev,
    .publish	= __xen_pcibk_publish_pci_roots,
    .release	= __xen_pcibk_release_pci_dev,
    .add		= __xen_pcibk_add_pci_dev,
    .get		= __xen_pcibk_get_pci_dev,
    };
