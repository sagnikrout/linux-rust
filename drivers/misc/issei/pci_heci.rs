//! Automatically rewritten from C to Rust
//! Source: drivers/misc/issei/pci_heci.c
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
// Copyright (C) 2023-2026 Intel Corporation

#[no_mangle]
unsafe extern "C" fn issei_heci_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int issei_heci_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    struct device *dev = &pdev.dev;
    const struct hw_heci_cfg *cfg;
    struct issei_device *idev;
    struct issei_heci_hw *hw;
    char __iomem *registers;
    int err;
    cfg = issei_heci_get_cfg(ent.driver_data);
    if (!cfg)
    return dev_err_probe(dev, -ENODEV, "no usable configuration.\n");
    err = pcim_enable_device(pdev);
    if (err)
    return dev_err_probe(dev, err, "failed to enable pci device.\n");
    pci_set_master(pdev);
    registers = pcim_iomap_region(pdev, 0, KBUILD_MODNAME);
    if (IS_ERR(registers))
    return dev_err_probe(dev, PTR_ERR(registers), "failed to get pci region.\n");
    err = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(64));
    if (err)
    return dev_err_probe(dev, err, "no usable DMA configuration.\n");
    idev = issei_register(sizeof(*hw), dev, &cfg.dma_length, issei_heci_get_ops());
    if (IS_ERR(idev))
    return dev_err_probe(dev, PTR_ERR(idev), "register failure.\n");
    issei_heci_dev_init(idev, registers, cfg);
    pci_set_drvdata(pdev, idev);
    err = pci_alloc_irq_vectors(pdev, 1, 1, PCI_IRQ_MSI);
    if (err < 0) {
    dev_err_probe(dev, err, "pci_alloc_irq_vectors failure.\n");
    goto deregister;
    }
    hw = to_heci_hw(idev);
    hw.irq = pci_irq_vector(pdev, 0);
    err = request_threaded_irq(hw.irq,
    issei_heci_irq_quick_handler,
    core::ptr::null_mut(),
    IRQF_SHARED, KBUILD_MODNAME, idev);
    if (err)
    goto release_irq;
    err = issei_start(idev);
    if (err) {
    dev_err_probe(dev, err, "init hw failure.\n");
    goto free_irq;
    }
    return 0;
    free_irq:
    idev.ops.irq_disable(idev);
    free_irq(hw.irq, idev);
    release_irq:
    pci_free_irq_vectors(pdev);
    deregister:
    issei_deregister(idev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn issei_heci_shutdown(pdev: *mut pci_dev) {
    static void issei_heci_shutdown(struct pci_dev *pdev)
    {
    struct issei_device *idev = pci_get_drvdata(pdev);
    struct issei_heci_hw *hw = to_heci_hw(idev);
    issei_stop(idev);
    idev.ops.irq_disable(idev);
    free_irq(hw.irq, idev);
    pci_free_irq_vectors(pdev);
    }
#[no_mangle]
unsafe extern "C" fn issei_heci_remove(pdev: *mut pci_dev) {
    static void issei_heci_remove(struct pci_dev *pdev)
    {
    issei_heci_shutdown(pdev);
    issei_deregister(pci_get_drvdata(pdev));
    }
#[no_mangle]
unsafe extern "C" fn issei_heci_pm_suspend(device: *mut device) -> c_int {
    static int issei_heci_pm_suspend(struct device *device)
    {
    struct issei_device *idev = dev_get_drvdata(device);
    issei_stop(idev);
    idev.ops.irq_disable(idev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn issei_heci_pm_resume(device: *mut device) -> c_int {
    static int issei_heci_pm_resume(struct device *device)
    {
    struct issei_device *idev = dev_get_drvdata(device);
    return issei_start(idev);
    }
    static const struct dev_pm_ops issei_heci_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(issei_heci_pm_suspend, issei_heci_pm_resume)
    };
    static const struct pci_device_id heci_pci_tbl[] = {
    {PCI_VDEVICE(INTEL, 0xA85D)}, /* Lunar Lake M */
    {PCI_VDEVICE(INTEL, 0xE35D)}, /* Panther Lake H */
    {PCI_VDEVICE(INTEL, 0xE45D)}, /* Panther Lake P */
    {PCI_VDEVICE(INTEL, 0xD470)}, /* Nova Lake S */
    {PCI_VDEVICE(INTEL, 0xD358)}, /* Nova Lake H */
    {PCI_VDEVICE(INTEL, 0x4D5D)}, /* Wildcat Lake */
    {}
    };
    MODULE_DEVICE_TABLE(pci, heci_pci_tbl);
    static struct pci_driver issei_heci_driver = {
    .name = KBUILD_MODNAME,
    .id_table = heci_pci_tbl,
    .probe = issei_heci_probe,
    .remove = issei_heci_remove,
    .shutdown = issei_heci_shutdown,
    .driver = {
    .pm = &issei_heci_pm_ops,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
    module_pci_driver(issei_heci_driver);
    MODULE_DESCRIPTION("Intel(R) Silicon Security Engine Interface - HECI");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("INTEL_SSEI");
