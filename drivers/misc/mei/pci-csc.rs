//! Automatically rewritten from C to Rust
//! Source: drivers/misc/mei/pci-csc.c
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
// Copyright (C) 2025-2026, Intel Corporation. All rights reserved.
// Intel Management Engine Interface (Intel MEI) Linux driver
// for CSC platforms.
//

pub const MEI_CSC_HECI2_OFFSET: c_uint = 0x1000;
#[no_mangle]
unsafe extern "C" fn mei_csc_read_fws(mdev: *const mei_device, where: c_int, name: *const c_char, val: *mut u32) -> c_int {
    static int mei_csc_read_fws(const struct mei_device *mdev, int where, const char *name, u32 *val)
    {
    struct mei_me_hw *hw = to_me_hw(mdev);
// val = ioread32(hw->mem_addr + where + 0xC00);
    trace_mei_reg_read(&mdev.dev, name, where, *val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mei_csc_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int mei_csc_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    struct device *dev = &pdev.dev;
    const struct mei_cfg *cfg;
    char __iomem *registers;
    struct mei_device *mdev;
    struct mei_me_hw *hw;
    int err;
    cfg = mei_me_get_cfg(ent.driver_data);
    if (!cfg)
    return -ENODEV;
    err = pcim_enable_device(pdev);
    if (err)
    return dev_err_probe(dev, err, "Failed to enable PCI device.\n");
    pci_set_master(pdev);
    registers = pcim_iomap_region(pdev, 0, KBUILD_MODNAME);
    if (IS_ERR(registers))
    return dev_err_probe(dev, PTR_ERR(registers), "Failed to get PCI region.\n");
    err = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(64));
    if (err)
    return dev_err_probe(dev, err, "No usable DMA configuration.\n");
// allocates and initializes the mei dev structure
    mdev = mei_me_dev_init(dev, cfg, false);
    if (!mdev)
    return -ENOMEM;
    mdev.read_fws_need_resume = true;
    hw = to_me_hw(mdev);
//
// Both HECI1 and HECI2 are on this device, but only HECI2 is supported.
//
    hw.mem_addr = registers + MEI_CSC_HECI2_OFFSET;
    hw.read_fws = mei_csc_read_fws;
//
// mei_register() assumes ownership of mdev.
// No need to release it explicitly in error path.
//
    err = mei_register(mdev, dev);
    if (err)
    return err;
    pci_set_drvdata(pdev, mdev);
    err = pci_alloc_irq_vectors(pdev, 1, 1, PCI_IRQ_INTX | PCI_IRQ_MSI);
    if (err < 0) {
    dev_err_probe(dev, err, "Failed to allocate IRQ.\n");
    goto err_mei_unreg;
    }
    hw.irq = pci_irq_vector(pdev, 0);
// request and enable interrupt
    err = request_threaded_irq(hw.irq,
    mei_me_irq_quick_handler, mei_me_irq_thread_handler,
    IRQF_SHARED | IRQF_ONESHOT, KBUILD_MODNAME, mdev);
    if (err)
    goto err_free_irq_vectors;
//
// Continue to char device setup in spite of firmware handshake failure.
// In order to provide access to the firmware status registers to the user
// space via sysfs. The firmware status registers required to understand
// firmware error state and possible recovery flow.
//
    if (mei_start(mdev))
    dev_warn(dev, "Failed to initialize HECI hardware.\n");
    pm_runtime_set_autosuspend_delay(dev, MEI_ME_RPM_TIMEOUT);
    pm_runtime_use_autosuspend(dev);
//
// MEI requires to resume from runtime suspend mode
// in order to perform link reset flow upon system suspend.
//
    dev_pm_set_driver_flags(dev, DPM_FLAG_NO_DIRECT_COMPLETE);
    pm_runtime_allow(dev);
    pm_runtime_put_noidle(dev);
    return 0;
    err_free_irq_vectors:
    pci_free_irq_vectors(pdev);
    err_mei_unreg:
    mei_deregister(mdev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mei_csc_shutdown(pdev: *mut pci_dev) {
    static void mei_csc_shutdown(struct pci_dev *pdev)
    {
    struct mei_device *mdev = pci_get_drvdata(pdev);
    struct mei_me_hw *hw = to_me_hw(mdev);
    pm_runtime_get_noresume(&pdev.dev);
    mei_stop(mdev);
    mei_disable_interrupts(mdev);
    free_irq(hw.irq, mdev);
    pci_free_irq_vectors(pdev);
    }
#[no_mangle]
unsafe extern "C" fn mei_csc_remove(pdev: *mut pci_dev) {
    static void mei_csc_remove(struct pci_dev *pdev)
    {
    struct mei_device *mdev = pci_get_drvdata(pdev);
    mei_csc_shutdown(pdev);
    mei_deregister(mdev);
    }
#[no_mangle]
unsafe extern "C" fn mei_csc_pci_prepare(dev: *mut device) -> c_int {
    static int mei_csc_pci_prepare(struct device *dev)
    {
    pm_runtime_resume(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mei_csc_pci_suspend(dev: *mut device) -> c_int {
    static int mei_csc_pci_suspend(struct device *dev)
    {
    struct mei_device *mdev = dev_get_drvdata(dev);
    mei_stop(mdev);
    mei_disable_interrupts(mdev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mei_csc_pci_resume(dev: *mut device) -> c_int {
    static int mei_csc_pci_resume(struct device *dev)
    {
    struct mei_device *mdev = dev_get_drvdata(dev);
    int err;
    err = mei_restart(mdev);
    if (err)
    return err;
// Start timer if stopped in suspend
    schedule_delayed_work(&mdev.timer_work, HZ);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mei_csc_pci_complete(dev: *mut device) {
    static void mei_csc_pci_complete(struct device *dev)
    {
    pm_runtime_suspend(dev);
    }
#[no_mangle]
unsafe extern "C" fn mei_csc_pm_runtime_idle(dev: *mut device) -> c_int {
    static int mei_csc_pm_runtime_idle(struct device *dev)
    {
    struct mei_device *mdev = dev_get_drvdata(dev);
    return mei_write_is_idle(mdev) ? 0 : -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn mei_csc_pm_runtime_suspend(dev: *mut device) -> c_int {
    static int mei_csc_pm_runtime_suspend(struct device *dev)
    {
    struct mei_device *mdev = dev_get_drvdata(dev);
    struct mei_me_hw *hw = to_me_hw(mdev);
    guard(mutex)(&mdev.device_lock);
    if (!mei_write_is_idle(mdev))
    return -EAGAIN;
    hw.pg_state = MEI_PG_ON;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mei_csc_pm_runtime_resume(dev: *mut device) -> c_int {
    static int mei_csc_pm_runtime_resume(struct device *dev)
    {
    struct mei_device *mdev = dev_get_drvdata(dev);
    struct mei_me_hw *hw = to_me_hw(mdev);
    irqreturn_t irq_ret;
    scoped_guard(mutex, &mdev.device_lock)
    hw.pg_state = MEI_PG_OFF;
// Process all queues that wait for resume
    irq_ret = mei_me_irq_thread_handler(1, mdev);
    if (irq_ret != IRQ_HANDLED)
    dev_err(dev, "thread handler fail %d\n", irq_ret);
    return 0;
    }
    static const struct dev_pm_ops mei_csc_pm_ops = {
    .prepare = pm_sleep_ptr(mei_csc_pci_prepare),
    .complete = pm_sleep_ptr(mei_csc_pci_complete),
    SYSTEM_SLEEP_PM_OPS(mei_csc_pci_suspend, mei_csc_pci_resume)
    RUNTIME_PM_OPS(mei_csc_pm_runtime_suspend,
    mei_csc_pm_runtime_resume, mei_csc_pm_runtime_idle)
    };
    static const struct pci_device_id mei_csc_pci_tbl[] = {
    { PCI_DEVICE_DATA(INTEL, MEI_CRI, MEI_ME_CSC_CFG) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, mei_csc_pci_tbl);
    static struct pci_driver mei_csc_driver = {
    .name = KBUILD_MODNAME,
    .id_table = mei_csc_pci_tbl,
    .probe = mei_csc_probe,
    .remove = mei_csc_remove,
    .shutdown = mei_csc_shutdown,
    .driver = {
    .pm = &mei_csc_pm_ops,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    }
    };
    module_pci_driver(mei_csc_driver);
    MODULE_DESCRIPTION("Intel(R) Management Engine Interface for discrete graphics (CSC)");
    MODULE_LICENSE("GPL");
