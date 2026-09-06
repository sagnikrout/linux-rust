//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/cadence/pci-sky1.c
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
// PCIe controller driver for CIX's sky1 SoCs
//
// Copyright 2025 Cix Technology Group Co., Ltd.
// Author: Hans Zhang <hans.zhang@cixtech.com>
//

pub const PCI_VENDOR_ID_CIX: c_uint = 0x1f6c;
pub const PCI_DEVICE_ID_CIX_SKY1: c_uint = 0x0001;

pub const SKY1_IP_REG_BANK: c_uint = 0x1000;
pub const SKY1_IP_CFG_CTRL_REG_BANK: c_uint = 0x4c00;
pub const SKY1_IP_AXI_MASTER_COMMON: c_uint = 0xf000;
pub const SKY1_AXI_SLAVE: c_uint = 0x9000;
pub const SKY1_AXI_MASTER: c_uint = 0xb000;
pub const SKY1_AXI_HLS_REGISTERS: c_uint = 0xc000;
pub const SKY1_AXI_RAS_REGISTERS: c_uint = 0xe000;
pub const SKY1_DTI_REGISTERS: c_uint = 0xd000;
pub const IP_REG_I_DBG_STS_0: c_uint = 0x420;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sky1_pcie {
    pub cdns_pcie: *mut cdns_pcie,
    pub cdns_pcie_rc: *mut cdns_pcie_rc,
    pub cfg_res: *mut resource,
    pub msg_res: *mut resource,
    pub cfg: *mut pci_config_window,
    pub strap_base: *mut void __iomem,
    pub status_base: *mut void __iomem,
    pub reg_base: *mut void __iomem,
    pub cfg_base: *mut void __iomem,
    pub msg_base: *mut void __iomem,
}

    static int sky1_pcie_resource_get(struct platform_device *pdev,
    struct sky1_pcie *pcie)
    {
    struct device *dev = &pdev.dev;
    struct resource *res;
    void __iomem *base;
    base = devm_platform_ioremap_resource_byname(pdev, "reg");
    if (IS_ERR(base))
    return dev_err_probe(dev, PTR_ERR(base),
    "unable to find \"reg\" registers\n");
    pcie.reg_base = base;
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "cfg");
    if (!res)
    return dev_err_probe(dev, -ENODEV, "unable to get \"cfg\" resource\n");
    pcie.cfg_res = res;
    base = devm_platform_ioremap_resource_byname(pdev, "rcsu_strap");
    if (IS_ERR(base))
    return dev_err_probe(dev, PTR_ERR(base),
    "unable to find \"rcsu_strap\" registers\n");
    pcie.strap_base = base;
    base = devm_platform_ioremap_resource_byname(pdev, "rcsu_status");
    if (IS_ERR(base))
    return dev_err_probe(dev, PTR_ERR(base),
    "unable to find \"rcsu_status\" registers\n");
    pcie.status_base = base;
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "msg");
    if (!res)
    return dev_err_probe(dev, -ENODEV, "unable to get \"msg\" resource\n");
    pcie.msg_res = res;
    pcie.msg_base = devm_ioremap_resource(dev, res);
    if (IS_ERR(pcie.msg_base)) {
    return dev_err_probe(dev, PTR_ERR(pcie.msg_base),
    "unable to ioremap msg resource\n");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sky1_pcie_start_link(cdns_pcie: *mut cdns_pcie) -> c_int {
    static int sky1_pcie_start_link(struct cdns_pcie *cdns_pcie)
    {
    struct sky1_pcie *pcie = dev_get_drvdata(cdns_pcie.dev);
    u32 val;
    val = readl(pcie.strap_base + STRAP_REG(1));
    val |= LINK_TRAINING_ENABLE;
    writel(val, pcie.strap_base + STRAP_REG(1));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sky1_pcie_stop_link(cdns_pcie: *mut cdns_pcie) {
    static void sky1_pcie_stop_link(struct cdns_pcie *cdns_pcie)
    {
    struct sky1_pcie *pcie = dev_get_drvdata(cdns_pcie.dev);
    u32 val;
    val = readl(pcie.strap_base + STRAP_REG(1));
    val &= ~LINK_TRAINING_ENABLE;
    writel(val, pcie.strap_base + STRAP_REG(1));
    }
#[no_mangle]
unsafe extern "C" fn sky1_pcie_link_up(cdns_pcie: *mut cdns_pcie) -> bool {
    static bool sky1_pcie_link_up(struct cdns_pcie *cdns_pcie)
    {
    u32 val;
    val = cdns_pcie_hpa_readl(cdns_pcie, REG_BANK_IP_REG,
    IP_REG_I_DBG_STS_0);
    return val & LINK_COMPLETE;
    }
    static const struct cdns_pcie_ops sky1_pcie_ops = {
    .start_link = sky1_pcie_start_link,
    .stop_link = sky1_pcie_stop_link,
    .link_up = sky1_pcie_link_up,
    };
#[no_mangle]
unsafe extern "C" fn sky1_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int sky1_pcie_probe(struct platform_device *pdev)
    {
    struct cdns_plat_pcie_of_data *reg_off;
    struct device *dev = &pdev.dev;
    struct pci_host_bridge *bridge;
    struct cdns_pcie *cdns_pcie;
    struct resource_entry *bus;
    struct cdns_pcie_rc *rc;
    struct sky1_pcie *pcie;
    int ret;
    pcie = devm_kzalloc(dev, sizeof(*pcie), GFP_KERNEL);
    if (!pcie)
    return -ENOMEM;
    bridge = devm_pci_alloc_host_bridge(dev, sizeof(*rc));
    if (!bridge)
    return -ENOMEM;
    ret = sky1_pcie_resource_get(pdev, pcie);
    if (ret < 0)
    return ret;
    bus = resource_list_first_type(&bridge.windows, IORESOURCE_BUS);
    if (!bus)
    return -ENODEV;
    pcie.cfg = pci_ecam_create(dev, pcie.cfg_res, bus.res,
    &pci_generic_ecam_ops);
    if (IS_ERR(pcie.cfg))
    return PTR_ERR(pcie.cfg);
    bridge.ops = (struct pci_ops *)&pci_generic_ecam_ops.pci_ops;
    rc = pci_host_bridge_priv(bridge);
    rc.ecam_supported = 1;
    rc.cfg_base = pcie.cfg.win;
    rc.cfg_res = &pcie.cfg.res;
    cdns_pcie = &rc.pcie;
    cdns_pcie.dev = dev;
    cdns_pcie.ops = &sky1_pcie_ops;
    cdns_pcie.reg_base = pcie.reg_base;
    cdns_pcie.msg_res = pcie.msg_res;
    cdns_pcie.is_rc = true;
    cdns_pcie.is_hpa = true;
    reg_off = devm_kzalloc(dev, sizeof(*reg_off), GFP_KERNEL);
    if (!reg_off) {
    pci_ecam_free(pcie.cfg);
    return -ENOMEM;
    }
    reg_off.ip_reg_bank_offset = SKY1_IP_REG_BANK;
    reg_off.ip_cfg_ctrl_reg_offset = SKY1_IP_CFG_CTRL_REG_BANK;
    reg_off.axi_mstr_common_offset = SKY1_IP_AXI_MASTER_COMMON;
    reg_off.axi_slave_offset = SKY1_AXI_SLAVE;
    reg_off.axi_master_offset = SKY1_AXI_MASTER;
    reg_off.axi_hls_offset = SKY1_AXI_HLS_REGISTERS;
    reg_off.axi_ras_offset = SKY1_AXI_RAS_REGISTERS;
    reg_off.axi_dti_offset = SKY1_DTI_REGISTERS;
    cdns_pcie.cdns_pcie_reg_offsets = reg_off;
    pcie.cdns_pcie = cdns_pcie;
    pcie.cdns_pcie_rc = rc;
    pcie.cfg_base = rc.cfg_base;
    bridge.sysdata = pcie.cfg;
    rc.vendor_id = PCI_VENDOR_ID_CIX;
    rc.device_id = PCI_DEVICE_ID_CIX_SKY1;
    rc.no_inbound_map = 1;
    dev_set_drvdata(dev, pcie);
    ret = cdns_pcie_hpa_host_setup(rc);
    if (ret < 0) {
    pci_ecam_free(pcie.cfg);
    return ret;
    }
    return 0;
    }
    static const struct of_device_id of_sky1_pcie_match[] = {
    { .compatible = "cix,sky1-pcie-host", },
    {},
    };
    MODULE_DEVICE_TABLE(of, of_sky1_pcie_match);
#[no_mangle]
unsafe extern "C" fn sky1_pcie_remove(pdev: *mut platform_device) {
    static void sky1_pcie_remove(struct platform_device *pdev)
    {
    struct sky1_pcie *pcie = platform_get_drvdata(pdev);
    struct cdns_pcie_rc *rc;
    rc = container_of(pcie.cdns_pcie, struct cdns_pcie_rc, pcie);
    cdns_pcie_hpa_host_disable(rc);
    pci_ecam_free(pcie.cfg);
    }
    static struct platform_driver sky1_pcie_driver = {
    .probe  = sky1_pcie_probe,
    .remove = sky1_pcie_remove,
    .driver = {
    .name = "sky1-pcie",
    .of_match_table = of_sky1_pcie_match,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
    module_platform_driver(sky1_pcie_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("PCIe controller driver for CIX's sky1 SoCs");
    MODULE_AUTHOR("Hans Zhang <hans.zhang@cixtech.com>");
