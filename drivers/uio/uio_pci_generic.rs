//! Automatically rewritten from C to Rust
//! Source: drivers/uio/uio_pci_generic.c
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
// uio_pci_generic - generic UIO driver for PCI 2.3 devices
//
// Copyright (C) 2009 Red Hat, Inc.
// Author: Michael S. Tsirkin <mst@redhat.com>
//
// Since the driver does not declare any device ids, you must allocate
// id and bind the device to the driver yourself.  For example:
//
// # echo "8086 10f5" > /sys/bus/pci/drivers/uio_pci_generic/new_id
// # echo -n 0000:00:19.0 > /sys/bus/pci/drivers/e1000e/unbind
// # echo -n 0000:00:19.0 > /sys/bus/pci/drivers/uio_pci_generic/bind
// # ls -l /sys/bus/pci/devices/0000:00:19.0/driver
// .../0000:00:19.0/driver -> ../../../bus/pci/drivers/uio_pci_generic
//
// Driver won't bind to devices which do not support the Interrupt Disable Bit
// in the command register. All devices compliant to PCI 2.3 (circa 2002) and
// all compliant PCI Express devices should support this bit.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uio_pci_generic_dev {
    pub info: uio_info,
    pub pdev: *mut pci_dev,
}

    static inline struct uio_pci_generic_dev *
    to_uio_pci_generic_dev(struct uio_info *info)
    {
    return container_of(info, struct uio_pci_generic_dev, info);
    }
#[no_mangle]
unsafe extern "C" fn release(info: *mut uio_info, inode: *mut inode) -> c_int {
    static int release(struct uio_info *info, struct inode *inode)
    {
    struct uio_pci_generic_dev *gdev = to_uio_pci_generic_dev(info);
//
// This driver is insecure when used with devices doing DMA, but some
// people (mis)use it with such devices.
// Let's at least make sure DMA isn't left enabled after the userspace
// driver closes the fd.
// Note that there's a non-zero chance doing this will wedge the device
// at least until reset.
//
    pci_clear_master(gdev.pdev);
    return 0;
    }
// Interrupt handler. Read/modify/write the command register to disable
// the interrupt.
#[no_mangle]
unsafe extern "C" fn irqhandler(irq: c_int, info: *mut uio_info) -> irqreturn_t {
    static irqreturn_t irqhandler(int irq, struct uio_info *info)
    {
    struct uio_pci_generic_dev *gdev = to_uio_pci_generic_dev(info);
    if (!pci_check_and_mask_intx(gdev.pdev))
    return IRQ_NONE;
// UIO core will signal the user process.
    return IRQ_HANDLED;
    }
    static int probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    struct uio_pci_generic_dev *gdev;
    struct uio_mem *uiomem;
    int err;
    int i;
    err = pcim_enable_device(pdev);
    if (err) {
    dev_err(&pdev.dev, "%s: pci_enable_device failed: %d\n",
    __func__, err);
    return err;
    }
    if (pdev.irq && !pci_intx_mask_supported(pdev))
    return -ENODEV;
    gdev = devm_kzalloc(&pdev.dev, sizeof(struct uio_pci_generic_dev), GFP_KERNEL);
    if (!gdev)
    return -ENOMEM;
    gdev.info.name = "uio_pci_generic";
    gdev.info.version = DRIVER_VERSION;
    gdev.info.release = release;
    gdev.pdev = pdev;
    if (pdev.irq && (pdev.irq != IRQ_NOTCONNECTED)) {
    gdev.info.irq = pdev.irq;
    gdev.info.irq_flags = IRQF_SHARED;
    gdev.info.handler = irqhandler;
    } else {
    dev_warn(&pdev.dev, "No IRQ assigned to device: "
    "no support for interrupts?\n");
    }
    uiomem = &gdev.info.mem[0];
    for (i = 0; i < MAX_UIO_MAPS; ++i) {
    struct resource *r = &pdev.resource[i];
    if (r.flags != (IORESOURCE_SIZEALIGN | IORESOURCE_MEM))
    continue;
    if (uiomem >= &gdev.info.mem[MAX_UIO_MAPS]) {
    dev_warn(
    &pdev.dev,
    "device has more than " __stringify(
    MAX_UIO_MAPS) " I/O memory resources.\n");
    break;
    }
    uiomem.memtype = UIO_MEM_PHYS;
    uiomem.addr = r.start & PAGE_MASK;
    uiomem.offs = r.start & ~PAGE_MASK;
    uiomem.size =
    (uiomem.offs + resource_size(r) + PAGE_SIZE - 1) &
    PAGE_MASK;
    uiomem.name = r.name;
    ++uiomem;
    }
    while (uiomem < &gdev.info.mem[MAX_UIO_MAPS]) {
    uiomem.size = 0;
    ++uiomem;
    }
    return devm_uio_register_device(&pdev.dev, &gdev.info);
    }
    static struct pci_driver uio_pci_driver = {
    .name = "uio_pci_generic",
    .id_table = core::ptr::null_mut(), /* only dynamic id's */
    .probe = probe,
    };
    module_pci_driver(uio_pci_driver);
    MODULE_VERSION(DRIVER_VERSION);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR(DRIVER_AUTHOR);
    MODULE_DESCRIPTION(DRIVER_DESC);
