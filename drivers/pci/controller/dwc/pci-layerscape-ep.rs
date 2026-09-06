//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/dwc/pci-layerscape-ep.c
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
// PCIe controller EP driver for Freescale Layerscape SoCs
//
// Copyright (C) 2018 NXP Semiconductor.
//
// Author: Xiaowei Bao <xiaowei.bao@nxp.com>
//

pub const PEX_PF0_CONFIG: c_uint = 0xC0014;

// PEX PFa PCIE PME and message interrupt registers
pub const PEX_PF0_PME_MES_DR: c_uint = 0xC0020;

pub const PEX_PF0_PME_MES_IER: c_uint = 0xC0028;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls_pcie_ep_drvdata {
    pub func_offset: u32,
    pub ops: *const dw_pcie_ep_ops,
    pub dw_pcie_ops: *const dw_pcie_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls_pcie_ep {
    pub pci: *mut dw_pcie,
    pub ls_epc: *mut pci_epc_features,
    pub drvdata: *const ls_pcie_ep_drvdata,
    pub irq: c_int,
    pub lnkcap: u32,
    pub big_endian: bool,
}

#[no_mangle]
unsafe extern "C" fn ls_pcie_pf_lut_readl(pcie: *mut ls_pcie_ep, offset: u32) -> u32 {
    static u32 ls_pcie_pf_lut_readl(struct ls_pcie_ep *pcie, u32 offset)
    {
    struct dw_pcie *pci = pcie.pci;
    if (pcie.big_endian)
    return ioread32be(pci.dbi_base + offset);
    else
    return ioread32(pci.dbi_base + offset);
    }
#[no_mangle]
unsafe extern "C" fn ls_pcie_pf_lut_writel(pcie: *mut ls_pcie_ep, offset: u32, value: u32) {
    static void ls_pcie_pf_lut_writel(struct ls_pcie_ep *pcie, u32 offset, u32 value)
    {
    struct dw_pcie *pci = pcie.pci;
    if (pcie.big_endian)
    iowrite32be(value, pci.dbi_base + offset);
    else
    iowrite32(value, pci.dbi_base + offset);
    }
#[no_mangle]
unsafe extern "C" fn ls_pcie_ep_event_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ls_pcie_ep_event_handler(int irq, void *dev_id)
    {
    struct ls_pcie_ep *pcie = dev_id;
    struct dw_pcie *pci = pcie.pci;
    u32 val, cfg;
    u8 offset;
    val = ls_pcie_pf_lut_readl(pcie, PEX_PF0_PME_MES_DR);
    ls_pcie_pf_lut_writel(pcie, PEX_PF0_PME_MES_DR, val);
    if (!val)
    return IRQ_NONE;
    if (val & PEX_PF0_PME_MES_DR_LUD) {
    offset = dw_pcie_find_capability(pci, PCI_CAP_ID_EXP);
//
// The values of the Maximum Link Width and Supported Link
// Speed from the Link Capabilities Register will be lost
// during link down or hot reset. Restore initial value
// that configured by the Reset Configuration Word (RCW).
//
    dw_pcie_dbi_ro_wr_en(pci);
    dw_pcie_writel_dbi(pci, offset + PCI_EXP_LNKCAP, pcie.lnkcap);
    dw_pcie_dbi_ro_wr_dis(pci);
    cfg = ls_pcie_pf_lut_readl(pcie, PEX_PF0_CONFIG);
    cfg |= PEX_PF0_CFG_READY;
    ls_pcie_pf_lut_writel(pcie, PEX_PF0_CONFIG, cfg);
    dw_pcie_ep_linkup(&pci.ep);
    dev_dbg(pci.dev, "Link up\n");
    } else if (val & PEX_PF0_PME_MES_DR_LDD) {
    dev_dbg(pci.dev, "Link down\n");
    dw_pcie_ep_linkdown(&pci.ep);
    } else if (val & PEX_PF0_PME_MES_DR_HRD) {
    dev_dbg(pci.dev, "Hot reset\n");
    }
    return IRQ_HANDLED;
    }
    static int ls_pcie_ep_interrupt_init(struct ls_pcie_ep *pcie,
    struct platform_device *pdev)
    {
    u32 val;
    int ret;
    pcie.irq = platform_get_irq_byname(pdev, "pme");
    if (pcie.irq < 0)
    return pcie.irq;
    ret = devm_request_irq(&pdev.dev, pcie.irq, ls_pcie_ep_event_handler,
    IRQF_SHARED, pdev.name, pcie);
    if (ret) {
    dev_err(&pdev.dev, "Can't register PCIe IRQ\n");
    return ret;
    }
// Enable interrupts
    val = ls_pcie_pf_lut_readl(pcie, PEX_PF0_PME_MES_IER);
    val |=  PEX_PF0_PME_MES_IER_LDDIE | PEX_PF0_PME_MES_IER_HRDIE |
    PEX_PF0_PME_MES_IER_LUDIE;
    ls_pcie_pf_lut_writel(pcie, PEX_PF0_PME_MES_IER, val);
    return 0;
    }
    static const struct pci_epc_features*
    ls_pcie_ep_get_features(struct dw_pcie_ep *ep)
    {
    struct dw_pcie *pci = to_dw_pcie_from_ep(ep);
    struct ls_pcie_ep *pcie = to_ls_pcie_ep(pci);
    return pcie.ls_epc;
    }
#[no_mangle]
unsafe extern "C" fn ls_pcie_ep_init(ep: *mut dw_pcie_ep) -> c_int {
    static int ls_pcie_ep_init(struct dw_pcie_ep *ep)
    {
    struct dw_pcie *pci = to_dw_pcie_from_ep(ep);
    struct ls_pcie_ep *pcie = to_ls_pcie_ep(pci);
    struct dw_pcie_ep_func *ep_func;
    ep_func = dw_pcie_ep_get_func_from_ep(ep, 0);
    if (!ep_func)
    return -ENODEV;
    pcie.ls_epc.msi_capable = ep_func.msi_cap ? true : false;
    pcie.ls_epc.msix_capable = ep_func.msix_cap ? true : false;
    return 0;
    }
    static int ls_pcie_ep_raise_irq(struct dw_pcie_ep *ep, u8 func_no,
    unsigned int type, u16 interrupt_num)
    {
    struct dw_pcie *pci = to_dw_pcie_from_ep(ep);
    switch (type) {
    case PCI_IRQ_INTX:
    return dw_pcie_ep_raise_intx_irq(ep, func_no);
    case PCI_IRQ_MSI:
    return dw_pcie_ep_raise_msi_irq(ep, func_no, interrupt_num);
    case PCI_IRQ_MSIX:
    return dw_pcie_ep_raise_msix_irq_doorbell(ep, func_no,
    interrupt_num);
    default:
    dev_err(pci.dev, "UNKNOWN IRQ type\n");
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn ls_pcie_ep_get_dbi_offset(ep: *mut dw_pcie_ep, func_no: u8) -> c_uint {
    static unsigned int ls_pcie_ep_get_dbi_offset(struct dw_pcie_ep *ep, u8 func_no)
    {
    struct dw_pcie *pci = to_dw_pcie_from_ep(ep);
    struct ls_pcie_ep *pcie = to_ls_pcie_ep(pci);
    WARN_ON(func_no && !pcie.drvdata.func_offset);
    return pcie.drvdata.func_offset * func_no;
    }
    static const struct dw_pcie_ep_ops ls_pcie_ep_ops = {
    .init = ls_pcie_ep_init,
    .raise_irq = ls_pcie_ep_raise_irq,
    .get_features = ls_pcie_ep_get_features,
    .get_dbi_offset = ls_pcie_ep_get_dbi_offset,
    };
    static const struct ls_pcie_ep_drvdata ls1_ep_drvdata = {
    .ops = &ls_pcie_ep_ops,
    };
    static const struct ls_pcie_ep_drvdata ls2_ep_drvdata = {
    .func_offset = 0x20000,
    .ops = &ls_pcie_ep_ops,
    };
    static const struct ls_pcie_ep_drvdata lx2_ep_drvdata = {
    .func_offset = 0x8000,
    .ops = &ls_pcie_ep_ops,
    };
    static const struct of_device_id ls_pcie_ep_of_match[] = {
    { .compatible = "fsl,ls1028a-pcie-ep", .data = &ls1_ep_drvdata },
    { .compatible = "fsl,ls1046a-pcie-ep", .data = &ls1_ep_drvdata },
    { .compatible = "fsl,ls1088a-pcie-ep", .data = &ls2_ep_drvdata },
    { .compatible = "fsl,ls2088a-pcie-ep", .data = &ls2_ep_drvdata },
    { .compatible = "fsl,lx2160ar2-pcie-ep", .data = &lx2_ep_drvdata },
    { },
    };
#[no_mangle]
unsafe extern "C" fn ls_pcie_ep_probe(pdev: *mut platform_device) -> int __init {
    static int __init ls_pcie_ep_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct dw_pcie *pci;
    struct ls_pcie_ep *pcie;
    struct pci_epc_features *ls_epc;
    struct resource *dbi_base;
    u8 offset;
    int ret;
    pcie = devm_kzalloc(dev, sizeof(*pcie), GFP_KERNEL);
    if (!pcie)
    return -ENOMEM;
    pci = devm_kzalloc(dev, sizeof(*pci), GFP_KERNEL);
    if (!pci)
    return -ENOMEM;
    ls_epc = devm_kzalloc(dev, sizeof(*ls_epc), GFP_KERNEL);
    if (!ls_epc)
    return -ENOMEM;
    pcie.drvdata = of_device_get_match_data(dev);
    pci.dev = dev;
    pci.ops = pcie.drvdata.dw_pcie_ops;
    ls_epc.bar[BAR_2].only_64bit = true;
    ls_epc.bar[BAR_4].only_64bit = true;
    ls_epc.linkup_notifier = true;
    pcie.pci = pci;
    pcie.ls_epc = ls_epc;
    dbi_base = platform_get_resource_byname(pdev, IORESOURCE_MEM, "regs");
    pci.dbi_base = devm_pci_remap_cfg_resource(dev, dbi_base);
    if (IS_ERR(pci.dbi_base))
    return PTR_ERR(pci.dbi_base);
    pci.ep.ops = &ls_pcie_ep_ops;
    pcie.big_endian = of_property_read_bool(dev.of_node, "big-endian");
    dma_set_mask_and_coherent(dev, DMA_BIT_MASK(64));
    platform_set_drvdata(pdev, pcie);
    offset = dw_pcie_find_capability(pci, PCI_CAP_ID_EXP);
    pcie.lnkcap = dw_pcie_readl_dbi(pci, offset + PCI_EXP_LNKCAP);
    ret = dw_pcie_ep_init(&pci.ep);
    if (ret)
    return ret;
    ret = dw_pcie_ep_init_registers(&pci.ep);
    if (ret) {
    dev_err(dev, "Failed to initialize DWC endpoint registers\n");
    dw_pcie_ep_deinit(&pci.ep);
    return ret;
    }
    pci_epc_init_notify(pci.ep.epc);
    return ls_pcie_ep_interrupt_init(pcie, pdev);
    }
    static struct platform_driver ls_pcie_ep_driver = {
    .driver = {
    .name = "layerscape-pcie-ep",
    .of_match_table = ls_pcie_ep_of_match,
    .suppress_bind_attrs = true,
    },
    };
    builtin_platform_driver_probe(ls_pcie_ep_driver, ls_pcie_ep_probe);
