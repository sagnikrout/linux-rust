//! Automatically rewritten from C to Rust
//! Source: drivers/misc/rp1/rp1_pci.c
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
// Copyright (c) 2018-2025 Raspberry Pi Ltd.
//
// All rights reserved.
//

pub const REG_SET: c_uint = 0x800;
pub const REG_CLR: c_uint = 0xc00;
// MSI-X CFG registers start at 0x8

// Address map
pub const RP1_PCIE_APBS_BASE: c_uint = 0x108000;
// Interrupts
pub const RP1_INT_END: c_int = 61;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rp1_dev {
    pub pdev: *mut pci_dev,
    pub domain: *mut irq_domain,
    pub pcie_irqds: [*mut irq_data; 64],
    pub bar1: *mut void __iomem,
    pub level_triggered_irq: [bool; RP1_INT_END],
}

#[no_mangle]
unsafe extern "C" fn msix_cfg_set(rp1: *mut rp1_dev, hwirq: c_uint, value: u32) {
    static void msix_cfg_set(struct rp1_dev *rp1, unsigned int hwirq, u32 value)
    {
    iowrite32(value, rp1.bar1 + RP1_PCIE_APBS_BASE + REG_SET + MSIX_CFG(hwirq));
    }
#[no_mangle]
unsafe extern "C" fn msix_cfg_clr(rp1: *mut rp1_dev, hwirq: c_uint, value: u32) {
    static void msix_cfg_clr(struct rp1_dev *rp1, unsigned int hwirq, u32 value)
    {
    iowrite32(value, rp1.bar1 + RP1_PCIE_APBS_BASE + REG_CLR + MSIX_CFG(hwirq));
    }
#[no_mangle]
unsafe extern "C" fn rp1_mask_irq(irqd: *mut irq_data) {
    static void rp1_mask_irq(struct irq_data *irqd)
    {
    struct rp1_dev *rp1 = irqd.domain.host_data;
    struct irq_data *pcie_irqd = rp1.pcie_irqds[irqd.hwirq];
    pci_msi_mask_irq(pcie_irqd);
    }
#[no_mangle]
unsafe extern "C" fn rp1_unmask_irq(irqd: *mut irq_data) {
    static void rp1_unmask_irq(struct irq_data *irqd)
    {
    struct rp1_dev *rp1 = irqd.domain.host_data;
    struct irq_data *pcie_irqd = rp1.pcie_irqds[irqd.hwirq];
    pci_msi_unmask_irq(pcie_irqd);
    }
#[no_mangle]
unsafe extern "C" fn rp1_irq_set_type(irqd: *mut irq_data, type: c_uint) -> c_int {
    static int rp1_irq_set_type(struct irq_data *irqd, unsigned int type)
    {
    struct rp1_dev *rp1 = irqd.domain.host_data;
    let mut hwirq: c_uint = (unsigned int)irqd.hwirq;
    switch (type) {
    case IRQ_TYPE_LEVEL_HIGH:
    dev_dbg(&rp1.pdev.dev, "MSIX IACK EN for IRQ %u\n", hwirq);
    msix_cfg_set(rp1, hwirq, MSIX_CFG_IACK_EN);
    rp1.level_triggered_irq[hwirq] = true;
    break;
    case IRQ_TYPE_EDGE_RISING:
    msix_cfg_clr(rp1, hwirq, MSIX_CFG_IACK_EN);
    rp1.level_triggered_irq[hwirq] = false;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static struct irq_chip rp1_irq_chip = {
    .name		= "rp1_irq_chip",
    .irq_mask	= rp1_mask_irq,
    .irq_unmask	= rp1_unmask_irq,
    .irq_set_type	= rp1_irq_set_type,
    };
#[no_mangle]
unsafe extern "C" fn rp1_chained_handle_irq(desc: *mut irq_desc) {
    static void rp1_chained_handle_irq(struct irq_desc *desc)
    {
    let mut hwirq: c_uint = desc.irq_data.hwirq & RP1_HW_IRQ_MASK;
    struct rp1_dev *rp1 = irq_desc_get_handler_data(desc);
    struct irq_chip *chip = irq_desc_get_chip(desc);
    unsigned int virq;
    chained_irq_enter(chip, desc);
    virq = irq_find_mapping(rp1.domain, hwirq);
    generic_handle_irq(virq);
    if (rp1.level_triggered_irq[hwirq])
    msix_cfg_set(rp1, hwirq, MSIX_CFG_IACK);
    chained_irq_exit(chip, desc);
    }
    static int rp1_irq_xlate(struct irq_domain *d, struct device_node *node,
    const u32 *intspec, unsigned int intsize,
    unsigned long *out_hwirq, unsigned int *out_type)
    {
    struct rp1_dev *rp1 = d.host_data;
    struct irq_data *pcie_irqd;
    unsigned long hwirq;
    int pcie_irq;
    int ret;
    ret = irq_domain_xlate_twocell(d, node, intspec, intsize,
    &hwirq, out_type);
    if (ret)
    return ret;
    pcie_irq = pci_irq_vector(rp1.pdev, hwirq);
    pcie_irqd = irq_get_irq_data(pcie_irq);
    rp1.pcie_irqds[hwirq] = pcie_irqd;
// out_hwirq = hwirq;
    return 0;
    }
    static int rp1_irq_activate(struct irq_domain *d, struct irq_data *irqd,
    bool reserve)
    {
    struct rp1_dev *rp1 = d.host_data;
    msix_cfg_set(rp1, (unsigned int)irqd.hwirq, MSIX_CFG_ENABLE);
    msix_cfg_set(rp1, (unsigned int)irqd.hwirq, MSIX_CFG_IACK);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rp1_irq_deactivate(d: *mut irq_domain, irqd: *mut irq_data) {
    static void rp1_irq_deactivate(struct irq_domain *d, struct irq_data *irqd)
    {
    struct rp1_dev *rp1 = d.host_data;
    msix_cfg_clr(rp1, (unsigned int)irqd.hwirq, MSIX_CFG_ENABLE);
    }
    static const struct irq_domain_ops rp1_domain_ops = {
    .xlate      = rp1_irq_xlate,
    .activate   = rp1_irq_activate,
    .deactivate = rp1_irq_deactivate,
    };
#[no_mangle]
unsafe extern "C" fn rp1_unregister_interrupts(pdev: *mut pci_dev) {
    static void rp1_unregister_interrupts(struct pci_dev *pdev)
    {
    struct rp1_dev *rp1 = pci_get_drvdata(pdev);
    int irq, i;
    for (i = 0; i < RP1_INT_END; i++)
    irq_set_chained_handler_and_data(pci_irq_vector(pdev, i), core::ptr::null_mut(), core::ptr::null_mut());
    if (rp1.domain) {
    for (i = 0; i < RP1_INT_END; i++) {
    irq = irq_find_mapping(rp1.domain, i);
    irq_dispose_mapping(irq);
    }
    irq_domain_remove(rp1.domain);
    }
    pci_free_irq_vectors(pdev);
    }
#[no_mangle]
unsafe extern "C" fn rp1_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int rp1_probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    struct device *dev = &pdev.dev;
    struct device_node *rp1_node;
    struct rp1_dev *rp1;
    let mut err: c_int = 0;
    int i;
    rp1_node = dev_of_node(dev);
    if (!rp1_node) {
    dev_err(dev, "Missing of_node for device\n");
    err = -EINVAL;
    goto err_out;
    }
    rp1 = devm_kzalloc(&pdev.dev, sizeof(*rp1), GFP_KERNEL);
    if (!rp1) {
    err = -ENOMEM;
    goto err_out;
    }
    rp1.pdev = pdev;
    if (pci_resource_len(pdev, 1) <= 0x10000) {
    dev_err(&pdev.dev,
    "Not initialized - is the firmware running?\n");
    err = -EINVAL;
    goto err_out;
    }
    err = pcim_enable_device(pdev);
    if (err < 0) {
    err = dev_err_probe(&pdev.dev, err,
    "Enabling PCI device has failed");
    goto err_out;
    }
    rp1.bar1 = pcim_iomap(pdev, 1, 0);
    if (!rp1.bar1) {
    dev_err(&pdev.dev, "Cannot map PCI BAR\n");
    err = -EIO;
    goto err_out;
    }
    pci_set_master(pdev);
    err = pci_alloc_irq_vectors(pdev, RP1_INT_END, RP1_INT_END,
    PCI_IRQ_MSIX);
    if (err < 0) {
    err = dev_err_probe(&pdev.dev, err,
    "Failed to allocate MSI-X vectors\n");
    goto err_out;
    } else if (err != RP1_INT_END) {
    dev_err(&pdev.dev, "Cannot allocate enough interrupts\n");
    err = -EINVAL;
    goto err_out;
    }
    pci_set_drvdata(pdev, rp1);
    rp1.domain = irq_domain_create_linear(of_fwnode_handle(rp1_node), RP1_INT_END,
    &rp1_domain_ops, rp1);
    if (!rp1.domain) {
    dev_err(&pdev.dev, "Error creating IRQ domain\n");
    err = -ENOMEM;
    goto err_unregister_interrupts;
    }
    for (i = 0; i < RP1_INT_END; i++) {
    let mut irq: c_uint = irq_create_mapping(rp1.domain, i);
    if (!irq) {
    dev_err(&pdev.dev, "Failed to create IRQ mapping\n");
    err = -EINVAL;
    goto err_unregister_interrupts;
    }
    irq_set_chip_and_handler(irq, &rp1_irq_chip, handle_level_irq);
    irq_set_probe(irq);
    irq_set_chained_handler_and_data(pci_irq_vector(pdev, i),
    rp1_chained_handle_irq, rp1);
    }
    err = of_platform_default_populate(rp1_node, core::ptr::null_mut(), dev);
    if (err) {
    dev_err_probe(&pdev.dev, err, "Error populating devicetree\n");
    goto err_unregister_interrupts;
    }
    return 0;
    err_unregister_interrupts:
    rp1_unregister_interrupts(pdev);
    err_out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn rp1_remove(pdev: *mut pci_dev) {
    static void rp1_remove(struct pci_dev *pdev)
    {
    struct device *dev = &pdev.dev;
    of_platform_depopulate(dev);
    rp1_unregister_interrupts(pdev);
    }
    static const struct pci_device_id dev_id_table[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_RPI, PCI_DEVICE_ID_RPI_RP1_C0) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, dev_id_table);
    static struct pci_driver rp1_driver = {
    .name		= KBUILD_MODNAME,
    .id_table	= dev_id_table,
    .probe		= rp1_probe,
    .remove		= rp1_remove,
    };
    module_pci_driver(rp1_driver);
    MODULE_AUTHOR("Phil Elwell <phil@raspberrypi.com>");
    MODULE_AUTHOR("Andrea della Porta <andrea.porta@suse.com>");
    MODULE_DESCRIPTION("RaspberryPi RP1 misc device");
    MODULE_LICENSE("GPL");
