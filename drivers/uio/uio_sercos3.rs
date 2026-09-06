//! Automatically rewritten from C to Rust
//! Source: drivers/uio/uio_sercos3.c
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
// sercos3: UIO driver for the Automata Sercos III PCI card
    Copyright (C) 2008 Linutronix GmbH
    Author: John Ogness <john.ogness@linutronix.de>
    This is a straight-forward UIO driver, where interrupts are disabled
    by the interrupt handler and re-enabled via a write to the UIO device
    by the userspace-part.
    The only part that may seem odd is the use of a logical OR when
    storing and restoring enabled interrupts. This is done because the
    userspace-part could directly modify the Interrupt Enable Register
    at any time. To reduce possible conflicts, the kernel driver uses
    a logical OR to make more controlled changes (rather than blindly
    overwriting previous values).
    Race conditions exist if the userspace-part directly modifies the
    Interrupt Enable Register while in operation. The consequences are
    that certain interrupts would fail to be enabled or disabled. For
    this reason, the userspace-part should only directly modify the
    Interrupt Enable Register at the beginning (to get things going).
    The userspace-part can safely disable interrupts at any time using
    a write to the UIO device.
//

// ID's for SERCOS III PCI card (PLX 9030)
pub const SERCOS_SUB_VENDOR_ID: c_uint = 0x1971;
pub const SERCOS_SUB_SYSID_3530: c_uint = 0x3530;
pub const SERCOS_SUB_SYSID_3535: c_uint = 0x3535;
pub const SERCOS_SUB_SYSID_3780: c_uint = 0x3780;
// Interrupt Enable Register
pub const IER0_OFFSET: c_uint = 0x08;
// Interrupt Status Register
pub const ISR0_OFFSET: c_uint = 0x18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sercos3_priv {
    pub ier0_cache: u32,
    pub ier0_cache_lock: spinlock_t,
}

// this function assumes ier0_cache_lock is locked!
    static void sercos3_disable_interrupts(struct uio_info *info,
    struct sercos3_priv *priv)
    {
    void __iomem *ier0 = info.mem[3].internal_addr + IER0_OFFSET;
// add enabled interrupts to cache
    priv.ier0_cache |= ioread32(ier0);
// disable interrupts
    iowrite32(0, ier0);
    }
// this function assumes ier0_cache_lock is locked!
    static void sercos3_enable_interrupts(struct uio_info *info,
    struct sercos3_priv *priv)
    {
    void __iomem *ier0 = info.mem[3].internal_addr + IER0_OFFSET;
// restore previously enabled interrupts
    iowrite32(ioread32(ier0) | priv.ier0_cache, ier0);
    priv.ier0_cache = 0;
    }
#[no_mangle]
unsafe extern "C" fn sercos3_handler(irq: c_int, info: *mut uio_info) -> irqreturn_t {
    static irqreturn_t sercos3_handler(int irq, struct uio_info *info)
    {
    struct sercos3_priv *priv = info.priv;
    void __iomem *isr0 = info.mem[3].internal_addr + ISR0_OFFSET;
    void __iomem *ier0 = info.mem[3].internal_addr + IER0_OFFSET;
    if (!(ioread32(isr0) & ioread32(ier0)))
    return IRQ_NONE;
    spin_lock(&priv.ier0_cache_lock);
    sercos3_disable_interrupts(info, priv);
    spin_unlock(&priv.ier0_cache_lock);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn sercos3_irqcontrol(info: *mut uio_info, irq_on: i32) -> c_int {
    static int sercos3_irqcontrol(struct uio_info *info, s32 irq_on)
    {
    struct sercos3_priv *priv = info.priv;
    spin_lock_irq(&priv.ier0_cache_lock);
    if (irq_on)
    sercos3_enable_interrupts(info, priv);
    else
    sercos3_disable_interrupts(info, priv);
    spin_unlock_irq(&priv.ier0_cache_lock);
    return 0;
    }
    static int sercos3_setup_iomem(struct pci_dev *dev, struct uio_info *info,
    int n, int pci_bar)
    {
    info.mem[n].addr = pci_resource_start(dev, pci_bar);
    if (!info.mem[n].addr)
    return -1;
    info.mem[n].internal_addr = ioremap(pci_resource_start(dev, pci_bar),
    pci_resource_len(dev, pci_bar));
    if (!info.mem[n].internal_addr)
    return -1;
    info.mem[n].size = pci_resource_len(dev, pci_bar);
    info.mem[n].memtype = UIO_MEM_PHYS;
    return 0;
    }
    static int sercos3_pci_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    struct uio_info *info;
    struct sercos3_priv *priv;
    int i;
    info = devm_kzalloc(&dev.dev, sizeof(struct uio_info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    priv = devm_kzalloc(&dev.dev, sizeof(struct sercos3_priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    if (pci_enable_device(dev))
    return -ENODEV;
    if (pci_request_regions(dev, "sercos3"))
    goto out_disable;
// we only need PCI BAR's 0, 2, 3, 4, 5
    if (sercos3_setup_iomem(dev, info, 0, 0))
    goto out_unmap;
    if (sercos3_setup_iomem(dev, info, 1, 2))
    goto out_unmap;
    if (sercos3_setup_iomem(dev, info, 2, 3))
    goto out_unmap;
    if (sercos3_setup_iomem(dev, info, 3, 4))
    goto out_unmap;
    if (sercos3_setup_iomem(dev, info, 4, 5))
    goto out_unmap;
    spin_lock_init(&priv.ier0_cache_lock);
    info.priv = priv;
    info.name = "Sercos_III_PCI";
    info.version = "0.0.1";
    info.irq = dev.irq;
    info.irq_flags = IRQF_SHARED;
    info.handler = sercos3_handler;
    info.irqcontrol = sercos3_irqcontrol;
    pci_set_drvdata(dev, info);
    if (uio_register_device(&dev.dev, info))
    goto out_unmap;
    return 0;
    out_unmap:
    for (i = 0; i < 5; i++) {
    if (info.mem[i].internal_addr)
    iounmap(info.mem[i].internal_addr);
    }
    pci_release_regions(dev);
    out_disable:
    pci_disable_device(dev);
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn sercos3_pci_remove(dev: *mut pci_dev) {
    static void sercos3_pci_remove(struct pci_dev *dev)
    {
    struct uio_info *info = pci_get_drvdata(dev);
    int i;
    uio_unregister_device(info);
    pci_release_regions(dev);
    pci_disable_device(dev);
    for (i = 0; i < 5; i++) {
    if (info.mem[i].internal_addr)
    iounmap(info.mem[i].internal_addr);
    }
    }
    static const struct pci_device_id sercos3_pci_ids[] = {
    {
    .vendor =       PCI_VENDOR_ID_PLX,
    .device =       PCI_DEVICE_ID_PLX_9030,
    .subvendor =    SERCOS_SUB_VENDOR_ID,
    .subdevice =    SERCOS_SUB_SYSID_3530,
    },
    {
    .vendor =       PCI_VENDOR_ID_PLX,
    .device =       PCI_DEVICE_ID_PLX_9030,
    .subvendor =    SERCOS_SUB_VENDOR_ID,
    .subdevice =    SERCOS_SUB_SYSID_3535,
    },
    {
    .vendor =       PCI_VENDOR_ID_PLX,
    .device =       PCI_DEVICE_ID_PLX_9030,
    .subvendor =    SERCOS_SUB_VENDOR_ID,
    .subdevice =    SERCOS_SUB_SYSID_3780,
    },
    { 0, }
    };
    MODULE_DEVICE_TABLE(pci, sercos3_pci_ids);
    static struct pci_driver sercos3_pci_driver = {
    .name = "sercos3",
    .id_table = sercos3_pci_ids,
    .probe = sercos3_pci_probe,
    .remove = sercos3_pci_remove,
    };
    module_pci_driver(sercos3_pci_driver);
    MODULE_DESCRIPTION("UIO driver for the Automata Sercos III PCI card");
    MODULE_AUTHOR("John Ogness <john.ogness@linutronix.de>");
    MODULE_LICENSE("GPL v2");
