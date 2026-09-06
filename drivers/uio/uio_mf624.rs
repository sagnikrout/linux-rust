//! Automatically rewritten from C to Rust
//! Source: drivers/uio/uio_mf624.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// UIO driver for Humusoft MF624 DAQ card.
// Copyright (C) 2011 Rostislav Lisovy <lisovy@gmail.com>,
// Czech Technical University in Prague
//

pub const PCI_VENDOR_ID_HUMUSOFT: c_uint = 0x186c;
pub const PCI_DEVICE_ID_MF624: c_uint = 0x0624;
pub const PCI_SUBVENDOR_ID_HUMUSOFT: c_uint = 0x186c;
pub const PCI_SUBDEVICE_DEVICE: c_uint = 0x0624;
// BAR0 Interrupt control/status register
pub const INTCSR: c_uint = 0x4C;

    enum mf624_interrupt_source {ADC, CTR4, ALL};
    static void mf624_disable_interrupt(enum mf624_interrupt_source source,
    struct uio_info *info)
    {
    void __iomem *INTCSR_reg = info.mem[0].internal_addr + INTCSR;
    switch (source) {
    case ADC:
    iowrite32(ioread32(INTCSR_reg)
    & ~(INTCSR_ADINT_ENABLE | INTCSR_PCIINT_ENABLE),
    INTCSR_reg);
    break;
    case CTR4:
    iowrite32(ioread32(INTCSR_reg)
    & ~(INTCSR_CTR4INT_ENABLE | INTCSR_PCIINT_ENABLE),
    INTCSR_reg);
    break;
    case ALL:
    default:
    iowrite32(ioread32(INTCSR_reg)
    & ~(INTCSR_ADINT_ENABLE | INTCSR_CTR4INT_ENABLE
    | INTCSR_PCIINT_ENABLE),
    INTCSR_reg);
    break;
    }
    }
    static void mf624_enable_interrupt(enum mf624_interrupt_source source,
    struct uio_info *info)
    {
    void __iomem *INTCSR_reg = info.mem[0].internal_addr + INTCSR;
    switch (source) {
    case ADC:
    iowrite32(ioread32(INTCSR_reg)
    | INTCSR_ADINT_ENABLE | INTCSR_PCIINT_ENABLE,
    INTCSR_reg);
    break;
    case CTR4:
    iowrite32(ioread32(INTCSR_reg)
    | INTCSR_CTR4INT_ENABLE | INTCSR_PCIINT_ENABLE,
    INTCSR_reg);
    break;
    case ALL:
    default:
    iowrite32(ioread32(INTCSR_reg)
    | INTCSR_ADINT_ENABLE | INTCSR_CTR4INT_ENABLE
    | INTCSR_PCIINT_ENABLE,
    INTCSR_reg);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn mf624_irq_handler(irq: c_int, info: *mut uio_info) -> irqreturn_t {
    static irqreturn_t mf624_irq_handler(int irq, struct uio_info *info)
    {
    void __iomem *INTCSR_reg = info.mem[0].internal_addr + INTCSR;
    if ((ioread32(INTCSR_reg) & INTCSR_ADINT_ENABLE)
    && (ioread32(INTCSR_reg) & INTCSR_ADINT_STATUS)) {
    mf624_disable_interrupt(ADC, info);
    return IRQ_HANDLED;
    }
    if ((ioread32(INTCSR_reg) & INTCSR_CTR4INT_ENABLE)
    && (ioread32(INTCSR_reg) & INTCSR_CTR4INT_STATUS)) {
    mf624_disable_interrupt(CTR4, info);
    return IRQ_HANDLED;
    }
    return IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn mf624_irqcontrol(info: *mut uio_info, irq_on: i32) -> c_int {
    static int mf624_irqcontrol(struct uio_info *info, s32 irq_on)
    {
    if (irq_on == 0)
    mf624_disable_interrupt(ALL, info);
#[no_mangle]
pub unsafe extern "C" fn if(1: irq_on ==) -> else {
    else if (irq_on == 1)
    mf624_enable_interrupt(ALL, info);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mf624_setup_mem(dev: *mut pci_dev, bar: c_int, mem: *mut uio_mem, name: *const c_char) -> c_int {
    static int mf624_setup_mem(struct pci_dev *dev, int bar, struct uio_mem *mem, const char *name)
    {
    let mut start: resource_size_t = pci_resource_start(dev, bar);
    let mut len: resource_size_t = pci_resource_len(dev, bar);
    mem.name = name;
    mem.addr = start & PAGE_MASK;
    mem.offs = start & ~PAGE_MASK;
    if (!mem.addr)
    return -ENODEV;
    mem.size = ((start & ~PAGE_MASK) + len + PAGE_SIZE - 1) & PAGE_MASK;
    mem.memtype = UIO_MEM_PHYS;
    mem.internal_addr = pci_ioremap_bar(dev, bar);
    if (!mem.internal_addr)
    return -ENODEV;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mf624_pci_probe(dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int mf624_pci_probe(struct pci_dev *dev, const struct pci_device_id *id)
    {
    struct uio_info *info;
    info = devm_kzalloc(&dev.dev, sizeof(struct uio_info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    if (pci_enable_device(dev))
    return -ENODEV;
    if (pci_request_regions(dev, "mf624"))
    goto out_disable;
    info.name = "mf624";
    info.version = "0.0.1";
// Note: Datasheet says device uses BAR0, BAR1, BAR2 -- do not trust it
// BAR0
    if (mf624_setup_mem(dev, 0, &info.mem[0], "PCI chipset, interrupts, status "
    "bits, special functions"))
    goto out_release;
// BAR2
    if (mf624_setup_mem(dev, 2, &info.mem[1], "ADC, DAC, DIO"))
    goto out_unmap0;
// BAR4
    if (mf624_setup_mem(dev, 4, &info.mem[2], "Counter/timer chip"))
    goto out_unmap1;
    info.irq = dev.irq;
    info.irq_flags = IRQF_SHARED;
    info.handler = mf624_irq_handler;
    info.irqcontrol = mf624_irqcontrol;
    if (uio_register_device(&dev.dev, info))
    goto out_unmap2;
    pci_set_drvdata(dev, info);
    return 0;
    out_unmap2:
    iounmap(info.mem[2].internal_addr);
    out_unmap1:
    iounmap(info.mem[1].internal_addr);
    out_unmap0:
    iounmap(info.mem[0].internal_addr);
    out_release:
    pci_release_regions(dev);
    out_disable:
    pci_disable_device(dev);
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn mf624_pci_remove(dev: *mut pci_dev) {
    static void mf624_pci_remove(struct pci_dev *dev)
    {
    struct uio_info *info = pci_get_drvdata(dev);
    mf624_disable_interrupt(ALL, info);
    uio_unregister_device(info);
    pci_release_regions(dev);
    pci_disable_device(dev);
    iounmap(info.mem[0].internal_addr);
    iounmap(info.mem[1].internal_addr);
    iounmap(info.mem[2].internal_addr);
    }
    static const struct pci_device_id mf624_pci_id[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_HUMUSOFT, PCI_DEVICE_ID_MF624) },
    { 0, }
    };
    static struct pci_driver mf624_pci_driver = {
    .name = "mf624",
    .id_table = mf624_pci_id,
    .probe = mf624_pci_probe,
    .remove = mf624_pci_remove,
    };
    MODULE_DEVICE_TABLE(pci, mf624_pci_id);
    module_pci_driver(mf624_pci_driver);
    MODULE_DESCRIPTION("UIO driver for Humusoft MF624 DAQ card");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Rostislav Lisovy <lisovy@gmail.com>");
