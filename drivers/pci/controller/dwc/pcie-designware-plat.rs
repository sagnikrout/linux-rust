//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/dwc/pcie-designware-plat.c
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
// PCIe RC driver for Synopsys DesignWare Core
//
// Copyright (C) 2015-2016 Synopsys, Inc. (www.synopsys.com)
//
// Authors: Joao Pinto <Joao.Pinto@synopsys.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_plat_pcie {
    pub pci: *mut dw_pcie,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_plat_pcie_of_data {
    pub mode: enum dw_pcie_device_mode,
}

    static const struct dw_pcie_host_ops dw_plat_pcie_host_ops = {
    };
    static int dw_plat_pcie_ep_raise_irq(struct dw_pcie_ep *ep, u8 func_no,
    unsigned int type, u16 interrupt_num)
    {
    struct dw_pcie *pci = to_dw_pcie_from_ep(ep);
    switch (type) {
    case PCI_IRQ_INTX:
    return dw_pcie_ep_raise_intx_irq(ep, func_no);
    case PCI_IRQ_MSI:
    return dw_pcie_ep_raise_msi_irq(ep, func_no, interrupt_num);
    case PCI_IRQ_MSIX:
    return dw_pcie_ep_raise_msix_irq(ep, func_no, interrupt_num);
    default:
    dev_err(pci.dev, "UNKNOWN IRQ type\n");
    }
    return 0;
    }
    static const struct pci_epc_features dw_plat_pcie_epc_features = {
    DWC_EPC_COMMON_FEATURES,
    .msi_capable = true,
    .msix_capable = true,
    };
    static const struct pci_epc_features*
    dw_plat_pcie_get_features(struct dw_pcie_ep *ep)
    {
    return &dw_plat_pcie_epc_features;
    }
    static const struct dw_pcie_ep_ops pcie_ep_ops = {
    .raise_irq = dw_plat_pcie_ep_raise_irq,
    .get_features = dw_plat_pcie_get_features,
    };
    static int dw_plat_add_pcie_port(struct dw_plat_pcie *dw_plat_pcie,
    struct platform_device *pdev)
    {
    struct dw_pcie *pci = dw_plat_pcie.pci;
    struct dw_pcie_rp *pp = &pci.pp;
    struct device *dev = &pdev.dev;
    int ret;
    pp.irq = platform_get_irq(pdev, 1);
    if (pp.irq < 0)
    return pp.irq;
    pp.num_vectors = MAX_MSI_IRQS;
    pp.ops = &dw_plat_pcie_host_ops;
    ret = dw_pcie_host_init(pp);
    if (ret) {
    dev_err(dev, "Failed to initialize host\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_plat_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int dw_plat_pcie_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct dw_plat_pcie *dw_plat_pcie;
    struct dw_pcie *pci;
    int ret;
    const struct dw_plat_pcie_of_data *data;
    enum dw_pcie_device_mode mode;
    data = of_device_get_match_data(dev);
    if (!data)
    return -EINVAL;
    mode = (enum dw_pcie_device_mode)data.mode;
    dw_plat_pcie = devm_kzalloc(dev, sizeof(*dw_plat_pcie), GFP_KERNEL);
    if (!dw_plat_pcie)
    return -ENOMEM;
    pci = devm_kzalloc(dev, sizeof(*pci), GFP_KERNEL);
    if (!pci)
    return -ENOMEM;
    pci.dev = dev;
    dw_plat_pcie.pci = pci;
    dw_plat_pcie.pci.mode = mode;
    platform_set_drvdata(pdev, dw_plat_pcie);
    switch (dw_plat_pcie.pci.mode) {
    case DW_PCIE_RC_TYPE:
    if (!IS_ENABLED(CONFIG_PCIE_DW_PLAT_HOST))
    return -ENODEV;
    ret = dw_plat_add_pcie_port(dw_plat_pcie, pdev);
    break;
    case DW_PCIE_EP_TYPE:
    if (!IS_ENABLED(CONFIG_PCIE_DW_PLAT_EP))
    return -ENODEV;
    pci.ep.ops = &pcie_ep_ops;
    ret = dw_pcie_ep_init(&pci.ep);
    if (ret)
    return ret;
    ret = dw_pcie_ep_init_registers(&pci.ep);
    if (ret) {
    dev_err(dev, "Failed to initialize DWC endpoint registers\n");
    dw_pcie_ep_deinit(&pci.ep);
    }
    pci_epc_init_notify(pci.ep.epc);
    break;
    default:
    dev_err(dev, "INVALID device type %d\n", dw_plat_pcie.pci.mode);
    ret = -EINVAL;
    break;
    }
    return ret;
    }
    static const struct dw_plat_pcie_of_data dw_plat_pcie_rc_of_data = {
    .mode = DW_PCIE_RC_TYPE,
    };
    static const struct dw_plat_pcie_of_data dw_plat_pcie_ep_of_data = {
    .mode = DW_PCIE_EP_TYPE,
    };
    static const struct of_device_id dw_plat_pcie_of_match[] = {
    {
    .compatible = "snps,dw-pcie",
    .data = &dw_plat_pcie_rc_of_data,
    },
    {
    .compatible = "snps,dw-pcie-ep",
    .data = &dw_plat_pcie_ep_of_data,
    },
    {},
    };
    static struct platform_driver dw_plat_pcie_driver = {
    .driver = {
    .name	= "dw-pcie",
    .of_match_table = dw_plat_pcie_of_match,
    .suppress_bind_attrs = true,
    },
    .probe = dw_plat_pcie_probe,
    };
    builtin_platform_driver(dw_plat_pcie_driver);
