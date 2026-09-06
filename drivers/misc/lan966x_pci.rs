//! Automatically rewritten from C to Rust
//! Source: drivers/misc/lan966x_pci.c
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
// Microchip LAN966x PCI driver
//
// Copyright (c) 2024 Microchip Technology Inc. and its subsidiaries.
//
// Authors:
// Clément Léger <clement.leger@bootlin.com>
// Hervé Codina <herve.codina@bootlin.com>
//

// Embedded dtbo symbols created by cmd_wrap_S_dtb in scripts/Makefile.lib
    extern char __dtbo_lan966x_pci_begin[];
    extern char __dtbo_lan966x_pci_end[];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_dev_intr_ctrl {
    pub pci_dev: *mut pci_dev,
    pub irq_domain: *mut irq_domain,
    pub irq: c_int,
}

#[no_mangle]
unsafe extern "C" fn pci_dev_irq_domain_map(d: *mut irq_domain, virq: c_uint, hw: irq_hw_number_t) -> c_int {
    static int pci_dev_irq_domain_map(struct irq_domain *d, unsigned int virq, irq_hw_number_t hw)
    {
    irq_set_chip_and_handler(virq, &dummy_irq_chip, handle_simple_irq);
    return 0;
    }
    static const struct irq_domain_ops pci_dev_irq_domain_ops = {
    .map = pci_dev_irq_domain_map,
    .xlate = irq_domain_xlate_onecell,
    };
#[no_mangle]
unsafe extern "C" fn pci_dev_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pci_dev_irq_handler(int irq, void *data)
    {
    struct pci_dev_intr_ctrl *intr_ctrl = data;
    int ret;
    ret = generic_handle_domain_irq(intr_ctrl.irq_domain, 0);
    return ret ? IRQ_NONE : IRQ_HANDLED;
    }
    static struct pci_dev_intr_ctrl *pci_dev_create_intr_ctrl(struct pci_dev *pdev)
    {
    struct pci_dev_intr_ctrl *intr_ctrl __free(kfree) = core::ptr::null_mut();
    struct fwnode_handle *fwnode;
    int ret;
    fwnode = dev_fwnode(&pdev.dev);
    if (!fwnode)
    return ERR_PTR(-ENODEV);
    intr_ctrl = kmalloc_obj(*intr_ctrl);
    if (!intr_ctrl)
    return ERR_PTR(-ENOMEM);
    intr_ctrl.pci_dev = pdev;
    intr_ctrl.irq_domain = irq_domain_create_linear(fwnode, 1, &pci_dev_irq_domain_ops,
    intr_ctrl);
    if (!intr_ctrl.irq_domain) {
    pci_err(pdev, "Failed to create irqdomain\n");
    return ERR_PTR(-ENOMEM);
    }
    ret = pci_alloc_irq_vectors(pdev, 1, 1, PCI_IRQ_INTX);
    if (ret < 0) {
    pci_err(pdev, "Unable alloc irq vector (%d)\n", ret);
    goto err_remove_domain;
    }
    intr_ctrl.irq = pci_irq_vector(pdev, 0);
    ret = request_irq(intr_ctrl.irq, pci_dev_irq_handler, IRQF_SHARED,
    pci_name(pdev), intr_ctrl);
    if (ret) {
    pci_err(pdev, "Unable to request irq %d (%d)\n", intr_ctrl.irq, ret);
    goto err_free_irq_vector;
    }
    return_ptr(intr_ctrl);
    err_free_irq_vector:
    pci_free_irq_vectors(pdev);
    err_remove_domain:
    irq_domain_remove(intr_ctrl.irq_domain);
    return ERR_PTR(ret);
    }
#[no_mangle]
unsafe extern "C" fn pci_dev_remove_intr_ctrl(intr_ctrl: *mut pci_dev_intr_ctrl) {
    static void pci_dev_remove_intr_ctrl(struct pci_dev_intr_ctrl *intr_ctrl)
    {
    free_irq(intr_ctrl.irq, intr_ctrl);
    pci_free_irq_vectors(intr_ctrl.pci_dev);
    irq_dispose_mapping(irq_find_mapping(intr_ctrl.irq_domain, 0));
    irq_domain_remove(intr_ctrl.irq_domain);
    kfree(intr_ctrl);
    }
#[no_mangle]
unsafe extern "C" fn devm_pci_dev_remove_intr_ctrl(intr_ctrl: *mut c_void) {
    static void devm_pci_dev_remove_intr_ctrl(void *intr_ctrl)
    {
    pci_dev_remove_intr_ctrl(intr_ctrl);
    }
#[no_mangle]
unsafe extern "C" fn devm_pci_dev_create_intr_ctrl(pdev: *mut pci_dev) -> c_int {
    static int devm_pci_dev_create_intr_ctrl(struct pci_dev *pdev)
    {
    struct pci_dev_intr_ctrl *intr_ctrl;
    intr_ctrl = pci_dev_create_intr_ctrl(pdev);
    if (IS_ERR(intr_ctrl))
    return PTR_ERR(intr_ctrl);
    return devm_add_action_or_reset(&pdev.dev, devm_pci_dev_remove_intr_ctrl, intr_ctrl);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan966x_pci {
    pub dev: *mut device,
    pub ovcs_id: c_int,
}

#[no_mangle]
unsafe extern "C" fn lan966x_pci_load_overlay(data: *mut lan966x_pci) -> c_int {
    static int lan966x_pci_load_overlay(struct lan966x_pci *data)
    {
    let mut dtbo_size: u32 = __dtbo_lan966x_pci_end - __dtbo_lan966x_pci_begin;
    void *dtbo_start = __dtbo_lan966x_pci_begin;
    return of_overlay_fdt_apply(dtbo_start, dtbo_size, &data.ovcs_id, dev_of_node(data.dev));
    }
#[no_mangle]
unsafe extern "C" fn lan966x_pci_unload_overlay(data: *mut lan966x_pci) {
    static void lan966x_pci_unload_overlay(struct lan966x_pci *data)
    {
    of_overlay_remove(&data.ovcs_id);
    }
#[no_mangle]
unsafe extern "C" fn lan966x_pci_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int lan966x_pci_probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    struct device *dev = &pdev.dev;
    struct lan966x_pci *data;
    int ret;
//
// On ACPI system, fwnode can point to the ACPI node.
// This driver needs an of_node to be used as the device-tree overlay
// target. This of_node should be set by the PCI core if it succeeds in
// creating it (CONFIG_PCI_DYNAMIC_OF_NODES feature).
// Check here for the validity of this of_node.
//
    if (!dev_of_node(dev))
    return dev_err_probe(dev, -EINVAL, "Missing of_node for device\n");
// Need to be done before devm_pci_dev_create_intr_ctrl.
// It allocates an IRQ and so pdev->irq is updated.
//
    ret = pcim_enable_device(pdev);
    if (ret)
    return ret;
    ret = devm_pci_dev_create_intr_ctrl(pdev);
    if (ret)
    return ret;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    pci_set_drvdata(pdev, data);
    data.dev = dev;
    ret = lan966x_pci_load_overlay(data);
    if (ret)
    return ret;
    pci_set_master(pdev);
    ret = of_platform_default_populate(dev_of_node(dev), core::ptr::null_mut(), dev);
    if (ret)
    goto err_unload_overlay;
    return 0;
    err_unload_overlay:
    of_platform_depopulate(dev);
    lan966x_pci_unload_overlay(data);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lan966x_pci_remove(pdev: *mut pci_dev) {
    static void lan966x_pci_remove(struct pci_dev *pdev)
    {
    struct lan966x_pci *data = pci_get_drvdata(pdev);
    of_platform_depopulate(data.dev);
    lan966x_pci_unload_overlay(data);
    }
    static struct pci_device_id lan966x_pci_ids[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_EFAR, 0x9660) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, lan966x_pci_ids);
    static struct pci_driver lan966x_pci_driver = {
    .name = "mchp_lan966x_pci",
    .id_table = lan966x_pci_ids,
    .probe = lan966x_pci_probe,
    .remove = lan966x_pci_remove,
    };
    module_pci_driver(lan966x_pci_driver);
    MODULE_AUTHOR("Herve Codina <herve.codina@bootlin.com>");
    MODULE_DESCRIPTION("Microchip LAN966x PCI driver");
    MODULE_LICENSE("GPL");
