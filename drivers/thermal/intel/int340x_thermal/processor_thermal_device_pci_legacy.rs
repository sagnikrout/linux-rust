//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/intel/int340x_thermal/processor_thermal_device_pci_legacy.c
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
// B0D4 processor thermal device
// Copyright (c) 2020, Intel Corporation.
//

#[no_mangle]
unsafe extern "C" fn proc_thermal_pci_msi_irq(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t proc_thermal_pci_msi_irq(int irq, void *devid)
    {
    struct proc_thermal_device *proc_priv;
    struct pci_dev *pdev = devid;
    proc_priv = pci_get_drvdata(pdev);
    intel_soc_dts_iosf_interrupt_handler(proc_priv.soc_dts);
    return IRQ_HANDLED;
    }
    static int proc_thermal_pci_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    struct proc_thermal_device *proc_priv;
    int ret;
    ret = pcim_enable_device(pdev);
    if (ret < 0) {
    dev_err(&pdev.dev, "error: could not enable device\n");
    return ret;
    }
    proc_priv = devm_kzalloc(&pdev.dev, sizeof(*proc_priv), GFP_KERNEL);
    if (!proc_priv)
    return -ENOMEM;
    ret = proc_thermal_add(&pdev.dev, proc_priv);
    if (ret)
    return ret;
    pci_set_drvdata(pdev, proc_priv);
    if (pdev.device == PCI_DEVICE_ID_INTEL_BSW_THERMAL) {
//
// Enumerate additional DTS sensors available via IOSF.
// But we are not treating as a failure condition, if
// there are no aux DTSs enabled or fails. This driver
// already exposes sensors, which can be accessed via
// ACPI/MSR. So we don't want to fail for auxiliary DTSs.
//
    proc_priv.soc_dts = intel_soc_dts_iosf_init(
    INTEL_SOC_DTS_INTERRUPT_MSI, false, 0);
    if (!IS_ERR(proc_priv.soc_dts) && pdev.irq) {
    ret = pci_enable_msi(pdev);
    if (!ret) {
    ret = request_threaded_irq(pdev.irq, core::ptr::null_mut(),
    proc_thermal_pci_msi_irq,
    IRQF_ONESHOT, "proc_thermal",
    pdev);
    if (ret) {
    intel_soc_dts_iosf_exit(
    proc_priv.soc_dts);
    pci_disable_msi(pdev);
    proc_priv.soc_dts = core::ptr::null_mut();
    }
    }
    } else
    dev_err(&pdev.dev, "No auxiliary DTSs enabled\n");
    } else {
    }
    ret = proc_thermal_mmio_add(pdev, proc_priv, id.driver_data);
    if (ret) {
    proc_thermal_remove(proc_priv);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn proc_thermal_pci_remove(pdev: *mut pci_dev) {
    static void proc_thermal_pci_remove(struct pci_dev *pdev)
    {
    struct proc_thermal_device *proc_priv = pci_get_drvdata(pdev);
    if (proc_priv.soc_dts) {
    intel_soc_dts_iosf_exit(proc_priv.soc_dts);
    if (pdev.irq) {
    free_irq(pdev.irq, pdev);
    pci_disable_msi(pdev);
    }
    }
    proc_thermal_mmio_remove(pdev, proc_priv);
    proc_thermal_remove(proc_priv);
    }

#[no_mangle]
unsafe extern "C" fn proc_thermal_pci_suspend(dev: *mut device) -> c_int {
    static int proc_thermal_pci_suspend(struct device *dev)
    {
    return proc_thermal_suspend(dev);
    }
#[no_mangle]
unsafe extern "C" fn proc_thermal_pci_resume(dev: *mut device) -> c_int {
    static int proc_thermal_pci_resume(struct device *dev)
    {
    return proc_thermal_resume(dev);
    }

    static SIMPLE_DEV_PM_OPS(proc_thermal_pci_pm, proc_thermal_pci_suspend,
    proc_thermal_pci_resume);
    static const struct pci_device_id proc_thermal_pci_ids[] = {
    { PCI_DEVICE_DATA(INTEL, BDW_THERMAL, 0) },
    { PCI_DEVICE_DATA(INTEL, BSW_THERMAL, 0) },
    { PCI_DEVICE_DATA(INTEL, BXT0_THERMAL, 0) },
    { PCI_DEVICE_DATA(INTEL, BXT1_THERMAL, 0) },
    { PCI_DEVICE_DATA(INTEL, BXTX_THERMAL, 0) },
    { PCI_DEVICE_DATA(INTEL, BXTP_THERMAL, 0) },
    { PCI_DEVICE_DATA(INTEL, CNL_THERMAL, 0) },
    { PCI_DEVICE_DATA(INTEL, CFL_THERMAL, 0) },
    { PCI_DEVICE_DATA(INTEL, GLK_THERMAL, 0) },
    { PCI_DEVICE_DATA(INTEL, HSB_THERMAL, 0) },
    { PCI_DEVICE_DATA(INTEL, ICL_THERMAL, PROC_THERMAL_FEATURE_RAPL) },
    { PCI_DEVICE_DATA(INTEL, JSL_THERMAL, 0) },
    { PCI_DEVICE_DATA(INTEL, SKL_THERMAL, PROC_THERMAL_FEATURE_RAPL) },
    { PCI_DEVICE_DATA(INTEL, TGL_THERMAL, PROC_THERMAL_FEATURE_RAPL |
    PROC_THERMAL_FEATURE_FIVR | PROC_THERMAL_FEATURE_WT_REQ) },
    { },
    };
    MODULE_DEVICE_TABLE(pci, proc_thermal_pci_ids);
    static struct pci_driver proc_thermal_pci_driver = {
    .name		= DRV_NAME,
    .probe		= proc_thermal_pci_probe,
    .remove		= proc_thermal_pci_remove,
    .id_table	= proc_thermal_pci_ids,
    .driver.pm	= &proc_thermal_pci_pm,
    };
    module_pci_driver(proc_thermal_pci_driver);
    MODULE_AUTHOR("Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>");
    MODULE_DESCRIPTION("Processor Thermal Reporting Device Driver");
    MODULE_LICENSE("GPL v2");
