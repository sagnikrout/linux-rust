//! Automatically rewritten from C to Rust
//! Source: drivers/pci/pci-stub.c
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
// Simple stub driver to reserve a PCI device
//
// Copyright (C) 2008 Red Hat, Inc.
// Author:
// Chris Wright
//
// Usage is simple, allocate a new id to the stub driver and bind the
// device to it.  For example:
//
// # echo "8086 10f5" > /sys/bus/pci/drivers/pci-stub/new_id
// # echo -n 0000:00:19.0 > /sys/bus/pci/drivers/e1000e/unbind
// # echo -n 0000:00:19.0 > /sys/bus/pci/drivers/pci-stub/bind
// # ls -l /sys/bus/pci/devices/0000:00:19.0/driver
// .../0000:00:19.0/driver -> ../../../bus/pci/drivers/pci-stub
//

    static char ids[1024] __initdata;
    module_param_string(ids, ids, sizeof(ids), 0);
    MODULE_PARM_DESC(ids, "Initial PCI IDs to add to the stub driver, format is "
    "\"vendor:device[:subvendor[:subdevice[:class[:class_mask]]]]\""
    " and multiple comma separated entries can be specified");
#[no_mangle]
unsafe extern "C" fn pci_stub_probe(dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int pci_stub_probe(struct pci_dev *dev, const struct pci_device_id *id)
    {
    pci_info(dev, "claimed by stub\n");
    return 0;
    }
    static struct pci_driver stub_driver = {
    .name		= "pci-stub",
    .id_table	= core::ptr::null_mut(),	/* only dynamic id's */
    .probe		= pci_stub_probe,
    .driver_managed_dma = true,
    };
#[no_mangle]
unsafe extern "C" fn pci_stub_init() -> int __init {
    static int __init pci_stub_init(void)
    {
    char *p, *id;
    int rc;
    rc = pci_register_driver(&stub_driver);
    if (rc)
    return rc;
// no ids passed actually
    if (ids[0] == '\0')
    return 0;
// add ids specified in the module parameter
    p = ids;
    while ((id = strsep(&p, ","))) {
    unsigned int vendor, device, subvendor = PCI_ANY_ID,
    subdevice = PCI_ANY_ID, class = 0, class_mask = 0;
    int fields;
    if (!strlen(id))
    continue;
    fields = sscanf(id, "%x:%x:%x:%x:%x:%x",
    &vendor, &device, &subvendor, &subdevice,
    &class, &class_mask);
    if (fields < 2) {
    pr_warn("pci-stub: invalid ID string \"%s\"\n", id);
    continue;
    }
    pr_info("pci-stub: add %04X:%04X sub=%04X:%04X cls=%08X/%08X\n",
    vendor, device, subvendor, subdevice, class, class_mask);
    rc = pci_add_dynid(&stub_driver, vendor, device,
    subvendor, subdevice, class, class_mask, 0);
    if (rc)
    pr_warn("pci-stub: failed to add dynamic ID (%d)\n",
    rc);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_stub_exit() -> void __exit {
    static void __exit pci_stub_exit(void)
    {
    pci_unregister_driver(&stub_driver);
    }
    module_init(pci_stub_init);
    module_exit(pci_stub_exit);
    MODULE_DESCRIPTION("VM device assignment stub driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Chris Wright <chrisw@sous-sol.org>");
