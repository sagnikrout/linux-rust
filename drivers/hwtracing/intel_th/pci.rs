//! Automatically rewritten from C to Rust
//! Source: drivers/hwtracing/intel_th/pci.c
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
// Intel(R) Trace Hub pci driver
//
// Copyright (C) 2014-2015 Intel Corporation.
//

    enum {
    TH_PCI_CONFIG_BAR	= 0,
    TH_PCI_STH_SW_BAR	= 2,
    TH_PCI_RTIT_BAR		= 4,
    };
pub const PCI_REG_NPKDSC: c_uint = 0x80;

#[no_mangle]
unsafe extern "C" fn intel_th_pci_activate(th: *mut intel_th) -> c_int {
    static int intel_th_pci_activate(struct intel_th *th)
    {
    struct pci_dev *pdev = to_pci_dev(th.dev);
    u32 npkdsc;
    int err;
    if (!INTEL_TH_CAP(th, tscu_enable))
    return 0;
    err = pci_read_config_dword(pdev, PCI_REG_NPKDSC, &npkdsc);
    if (!err) {
    npkdsc |= NPKDSC_TSACT;
    err = pci_write_config_dword(pdev, PCI_REG_NPKDSC, npkdsc);
    }
    if (err)
    dev_err(&pdev.dev, "failed to read NPKDSC register\n");
    return err;
    }
#[no_mangle]
unsafe extern "C" fn intel_th_pci_deactivate(th: *mut intel_th) {
    static void intel_th_pci_deactivate(struct intel_th *th)
    {
    struct pci_dev *pdev = to_pci_dev(th.dev);
    u32 npkdsc;
    int err;
    if (!INTEL_TH_CAP(th, tscu_enable))
    return;
    err = pci_read_config_dword(pdev, PCI_REG_NPKDSC, &npkdsc);
    if (!err) {
    npkdsc |= NPKDSC_TSACT;
    err = pci_write_config_dword(pdev, PCI_REG_NPKDSC, npkdsc);
    }
    if (err)
    dev_err(&pdev.dev, "failed to read NPKDSC register\n");
    }
    static int intel_th_pci_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    const struct intel_th_drvdata *drvdata = (void *)id.driver_data;
    struct resource resource[TH_MMIO_END + TH_NVEC_MAX] = {
    [TH_MMIO_CONFIG]	= pdev.resource[TH_PCI_CONFIG_BAR],
    [TH_MMIO_SW]		= pdev.resource[TH_PCI_STH_SW_BAR],
    };
    int err, r = TH_MMIO_SW + 1, i;
    struct intel_th *th;
    err = pcim_enable_device(pdev);
    if (err)
    return err;
    err = pcim_request_all_regions(pdev, DRIVER_NAME);
    if (err)
    return err;
    if (!pcim_iomap(pdev, TH_PCI_CONFIG_BAR, 0))
    return -ENOMEM;
    if (!pcim_iomap(pdev, TH_PCI_STH_SW_BAR, 0))
    return -ENOMEM;
    if (pdev.resource[TH_PCI_RTIT_BAR].start) {
    resource[TH_MMIO_RTIT] = pdev.resource[TH_PCI_RTIT_BAR];
    r++;
    }
    err = pci_alloc_irq_vectors(pdev, 1, 8, PCI_IRQ_ALL_TYPES);
    if (err > 0)
    for (i = 0; i < err; i++, r++) {
    resource[r].flags = IORESOURCE_IRQ;
    resource[r].start = pci_irq_vector(pdev, i);
    }
    th = intel_th_alloc(&pdev.dev, drvdata, resource, r);
    if (IS_ERR(th)) {
    err = PTR_ERR(th);
    goto err_free_irq;
    }
    th.activate   = intel_th_pci_activate;
    th.deactivate = intel_th_pci_deactivate;
    pci_set_master(pdev);
    return 0;
    err_free_irq:
    pci_free_irq_vectors(pdev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn intel_th_pci_remove(pdev: *mut pci_dev) {
    static void intel_th_pci_remove(struct pci_dev *pdev)
    {
    struct intel_th *th = pci_get_drvdata(pdev);
    intel_th_free(th);
    pci_free_irq_vectors(pdev);
    }
    static const struct intel_th_drvdata intel_th_1x_multi_is_broken = {
    .multi_is_broken	= 1,
    };
    static const struct intel_th_drvdata intel_th_2x = {
    .tscu_enable	= 1,
    .has_mintctl	= 1,
    };
    static const struct pci_device_id intel_th_pci_id_table[] = {
    { PCI_DEVICE_DATA(INTEL, NPK_CML,	&intel_th_2x) },   /* Comet Lake */
    { PCI_DEVICE_DATA(INTEL, NPK_CML_PCH,	&intel_th_2x) },   /* Comet Lake PCH */
    { PCI_DEVICE_DATA(INTEL, NPK_GNR,	&intel_th_2x) },   /* Granite Rapids */
    { PCI_DEVICE_DATA(INTEL, NPK_BXT,	core::ptr::null_mut()) },	   /* Broxton */
    { PCI_DEVICE_DATA(INTEL, NPK_CDF,	&intel_th_2x) },   /* Cedar Fork PCH */
    { PCI_DEVICE_DATA(INTEL, NPK_DNV,	core::ptr::null_mut()) },	   /* Denverton */
    { PCI_DEVICE_DATA(INTEL, NPK_BXT_B,	core::ptr::null_mut()) },	   /* Broxton B-step */
    { PCI_DEVICE_DATA(INTEL, NPK_EBG,	&intel_th_2x) },   /* Emmitsburg PCH */
    { PCI_DEVICE_DATA(INTEL, NPK_GLK,	&intel_th_2x) },   /* Gemini Lake */
    { PCI_DEVICE_DATA(INTEL, NPK_GNR_SOC,	&intel_th_2x) },   /* Granite Rapids SOC */
    { PCI_DEVICE_DATA(INTEL, NPK_SPR,	&intel_th_2x) },   /* Sapphire Rapids SOC */
    { PCI_DEVICE_DATA(INTEL, NPK_ICL_PCH,	&intel_th_2x) },   /* Ice Lake PCH */
    { PCI_DEVICE_DATA(INTEL, NPK_TGL_PCH_H,	&intel_th_2x) },   /* Tiger Lake PCH-H */
    { PCI_DEVICE_DATA(INTEL, NPK_EHL_CPU,	&intel_th_2x) },   /* Elkhart Lake CPU */
    { PCI_DEVICE_DATA(INTEL, NPK_ICL_NNPI,	&intel_th_2x) },   /* Ice Lake NNPI */
    { PCI_DEVICE_DATA(INTEL, NPK_ADL_CPU,	&intel_th_2x) },   /* Alder Lake CPU */
    { PCI_DEVICE_DATA(INTEL, NPK_EHL,	&intel_th_2x) },   /* Elkhart Lake */
    { PCI_DEVICE_DATA(INTEL, NPK_RKL,	&intel_th_2x) },   /* Rocket Lake CPU */
    { PCI_DEVICE_DATA(INTEL, NPK_JSL_PCH,	&intel_th_2x) },   /* Jasper Lake PCH */
    { PCI_DEVICE_DATA(INTEL, NPK_JSL_CPU,	&intel_th_2x) },   /* Jasper Lake CPU */
    { PCI_DEVICE_DATA(INTEL, NPK_ADL_P,	&intel_th_2x) },   /* Alder Lake-P */
    { PCI_DEVICE_DATA(INTEL, NPK_ADL_M,	&intel_th_2x) },   /* Alder Lake-M */
    { PCI_DEVICE_DATA(INTEL, NPK_APL,	core::ptr::null_mut()) },	   /* Apollo Lake */
    { PCI_DEVICE_DATA(INTEL, NPK_NVL_PCH,	&intel_th_2x) },   /* Nova Lake-PCH */
    { PCI_DEVICE_DATA(INTEL, NPK_ARL,	&intel_th_2x) },   /* Arrow Lake */
    { PCI_DEVICE_DATA(INTEL, NPK_RPL_S,	&intel_th_2x) },   /* Raptor Lake-S */
    { PCI_DEVICE_DATA(INTEL, NPK_ADL,	&intel_th_2x) },   /* Alder Lake */
    { PCI_DEVICE_DATA(INTEL, NPK_MTL_P,	&intel_th_2x) },   /* Meteor Lake-P */
    { PCI_DEVICE_DATA(INTEL, NPK_MTL_S,	&intel_th_2x) },   /* Meteor Lake-S */
    { PCI_DEVICE_DATA(INTEL, NPK_ICL_CPU,	&intel_th_2x) },   /* Ice Lake CPU */
    { PCI_DEVICE_DATA(INTEL, NPK_TGL_CPU,	&intel_th_2x) },   /* Tiger Lake CPU */
    { PCI_DEVICE_DATA(INTEL, NPK_0,		core::ptr::null_mut()) },
    { PCI_DEVICE_DATA(INTEL, NPK_CNL_LP,	&intel_th_2x) },   /* Cannon Lake LP */
    { PCI_DEVICE_DATA(INTEL, NPK_TGL_PCH,	&intel_th_2x) },   /* Tiger Lake PCH */
    { PCI_DEVICE_DATA(INTEL, NPK_1,		core::ptr::null_mut()) },
    { PCI_DEVICE_DATA(INTEL, NPK_LBG_PCH,	core::ptr::null_mut()) },	   /* Lewisburg PCH */
    { PCI_DEVICE_DATA(INTEL, NPK_LBG_PCH_2,	core::ptr::null_mut()) },	   /* Lewisburg PCH */
    { PCI_DEVICE_DATA(INTEL, NPK_KBL_PCH,	&intel_th_1x_multi_is_broken) },   /* Kaby Lake PCH-H */
    { PCI_DEVICE_DATA(INTEL, NPK_CNL_H,	&intel_th_2x) },   /* Cannon Lake H */
    { PCI_DEVICE_DATA(INTEL, NPK_CML_PCH_V,	&intel_th_1x_multi_is_broken) },   /* Comet Lake PCH-V */
    { PCI_DEVICE_DATA(INTEL, NPK_RPL_S_CPU,	&intel_th_2x) },   /* Raptor Lake-S CPU */
    { PCI_DEVICE_DATA(INTEL, NPK_LNL,	&intel_th_2x) },   /* Lunar Lake */
    { PCI_DEVICE_DATA(INTEL, NPK_MTL_S_CPU,	&intel_th_2x) },   /* Meteor Lake-S CPU */
    { PCI_DEVICE_DATA(INTEL, NPK_NVL_P,	&intel_th_2x) },   /* Nova Lake-P */
    { PCI_DEVICE_DATA(INTEL, NPK_NVL_H,	&intel_th_2x) },   /* Nova Lake-H */
    { PCI_DEVICE_DATA(INTEL, NPK_NVL_S,	&intel_th_2x) },   /* Nova Lake-S */
    { PCI_DEVICE_DATA(INTEL, NPK_PTL_H,	&intel_th_2x) },   /* Panther Lake-H */
    { PCI_DEVICE_DATA(INTEL, NPK_PTL_PU,	&intel_th_2x) },   /* Panther Lake-P/U */
    { }
    };
    MODULE_DEVICE_TABLE(pci, intel_th_pci_id_table);
    static struct pci_driver intel_th_pci_driver = {
    .name		= DRIVER_NAME,
    .id_table	= intel_th_pci_id_table,
    .probe		= intel_th_pci_probe,
    .remove		= intel_th_pci_remove,
    };
    module_pci_driver(intel_th_pci_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Intel(R) Trace Hub PCI controller driver");
    MODULE_AUTHOR("Alexander Shishkin <alexander.shishkin@intel.com>");
