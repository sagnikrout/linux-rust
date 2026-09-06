//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/dwc/pcie-hisi.c
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
// PCIe host controller driver for HiSilicon SoCs
//
// Copyright (C) 2015 HiSilicon Co., Ltd. http://www.hisilicon.com
//
// Authors: Zhou Wang <wangzhou1@hisilicon.com>
// Dacai Zhu <zhudacai@hisilicon.com>
// Gabriele Paoloni <gabriele.paoloni@huawei.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_pcie {
    pub reg_base: *mut void __iomem,
}

    static int hisi_pcie_rd_conf(struct pci_bus *bus, u32 devfn, int where,
    int size, u32 *val)
    {
    struct pci_config_window *cfg = bus.sysdata;
    let mut dev: c_int = PCI_SLOT(devfn);
    if (bus.number == cfg.busr.start) {
// access only one slot on each root port
    if (dev > 0)
    return PCIBIOS_DEVICE_NOT_FOUND;
    else
    return pci_generic_config_read32(bus, devfn, where,
    size, val);
    }
    return pci_generic_config_read(bus, devfn, where, size, val);
    }
    static int hisi_pcie_wr_conf(struct pci_bus *bus, u32 devfn,
    int where, int size, u32 val)
    {
    struct pci_config_window *cfg = bus.sysdata;
    let mut dev: c_int = PCI_SLOT(devfn);
    if (bus.number == cfg.busr.start) {
// access only one slot on each root port
    if (dev > 0)
    return PCIBIOS_DEVICE_NOT_FOUND;
    else
    return pci_generic_config_write32(bus, devfn, where,
    size, val);
    }
    return pci_generic_config_write(bus, devfn, where, size, val);
    }
    static void __iomem *hisi_pcie_map_bus(struct pci_bus *bus, unsigned int devfn,
    int where)
    {
    struct pci_config_window *cfg = bus.sysdata;
    struct hisi_pcie *pcie = cfg.priv;
    if (bus.number == cfg.busr.start)
    return pcie.reg_base + where;
    else
    return pci_ecam_map_bus(bus, devfn, where);
    }

#[no_mangle]
unsafe extern "C" fn hisi_pcie_init(cfg: *mut pci_config_window) -> c_int {
    static int hisi_pcie_init(struct pci_config_window *cfg)
    {
    struct device *dev = cfg.parent;
    struct hisi_pcie *pcie;
    struct acpi_device *adev = to_acpi_device(dev);
    struct acpi_pci_root *root = acpi_driver_data(adev);
    struct resource *res;
    int ret;
    pcie = devm_kzalloc(dev, sizeof(*pcie), GFP_KERNEL);
    if (!pcie)
    return -ENOMEM;
//
// Retrieve RC base and size from a HISI0081 device with _UID
// matching our segment.
//
    res = devm_kzalloc(dev, sizeof(*res), GFP_KERNEL);
    if (!res)
    return -ENOMEM;
    ret = acpi_get_rc_resources(dev, "HISI0081", root.segment, res);
    if (ret) {
    dev_err(dev, "can't get rc base address\n");
    return -ENOMEM;
    }
    pcie.reg_base = devm_pci_remap_cfgspace(dev, res.start, resource_size(res));
    if (!pcie.reg_base)
    return -ENOMEM;
    cfg.priv = pcie;
    return 0;
    }
    const struct pci_ecam_ops hisi_pcie_ops = {
    .init         =  hisi_pcie_init,
    .pci_ops      = {
    .map_bus    = hisi_pcie_map_bus,
    .read       = hisi_pcie_rd_conf,
    .write      = hisi_pcie_wr_conf,
    }
    };

#[no_mangle]
unsafe extern "C" fn hisi_pcie_platform_init(cfg: *mut pci_config_window) -> c_int {
    static int hisi_pcie_platform_init(struct pci_config_window *cfg)
    {
    struct device *dev = cfg.parent;
    struct hisi_pcie *pcie;
    struct platform_device *pdev = to_platform_device(dev);
    struct resource *res;
    pcie = devm_kzalloc(dev, sizeof(*pcie), GFP_KERNEL);
    if (!pcie)
    return -ENOMEM;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 1);
    if (!res) {
    dev_err(dev, "missing \"reg[1]\"property\n");
    return -EINVAL;
    }
    pcie.reg_base = devm_pci_remap_cfgspace(dev, res.start, resource_size(res));
    if (!pcie.reg_base)
    return -ENOMEM;
    cfg.priv = pcie;
    return 0;
    }
    static const struct pci_ecam_ops hisi_pcie_platform_ops = {
    .init         =  hisi_pcie_platform_init,
    .pci_ops      = {
    .map_bus    = hisi_pcie_map_bus,
    .read       = hisi_pcie_rd_conf,
    .write      = hisi_pcie_wr_conf,
    }
    };
    static const struct of_device_id hisi_pcie_almost_ecam_of_match[] = {
    {
    .compatible =  "hisilicon,hip06-pcie-ecam",
    .data	    =  &hisi_pcie_platform_ops,
    },
    {
    .compatible =  "hisilicon,hip07-pcie-ecam",
    .data       =  &hisi_pcie_platform_ops,
    },
    {},
    };
    static struct platform_driver hisi_pcie_almost_ecam_driver = {
    .probe  = pci_host_common_probe,
    .driver = {
    .name = "hisi-pcie-almost-ecam",
    .of_match_table = hisi_pcie_almost_ecam_of_match,
    .suppress_bind_attrs = true,
    },
    };
    builtin_platform_driver(hisi_pcie_almost_ecam_driver);

