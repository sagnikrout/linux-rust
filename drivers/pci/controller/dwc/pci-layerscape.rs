//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/dwc/pci-layerscape.c
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
// PCIe host controller driver for Freescale Layerscape SoCs
//
// Copyright (C) 2014 Freescale Semiconductor.
// Copyright 2021 NXP
//
// Author: Minghuan Lian <Minghuan.Lian@freescale.com>
//

// PEX Internal Configuration Registers
pub const PCIE_STRFMR1: c_uint = 0x71c /* Symbol Timer & Filter Mask Register1 */;
pub const PCIE_ABSERR: c_uint = 0x8d0 /* Bridge Slave Error Response Register */;
pub const PCIE_ABSERR_SETTING: c_uint = 0x9401 /* Forward error of non-posted request */;
// PF Message Command Register
pub const LS_PCIE_PF_MCR: c_uint = 0x2c;

// LS1021A PEXn PM Write Control Register

pub const SCFG_PEXSFTRSTCR: c_uint = 0x190;

// LS1043A PEX PME control register
pub const SCFG_PEXPMECR: c_uint = 0x144;

// LS1043A PEX LUT debug register
pub const LS_PCIE_LDBG: c_uint = 0x7fc;

pub const PCIE_IATU_NUM: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls_pcie_drvdata {
    pub pf_lut_off: u32,
    pub ops: *const dw_pcie_host_ops,
    pub pp): *mut *mut int (exit_from_l2)(struct dw_pcie_rp,
    pub scfg_support: bool,
    pub pm_support: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls_pcie {
    pub pci: *mut dw_pcie,
    pub drvdata: *const ls_pcie_drvdata,
    pub pf_lut_base: *mut void __iomem,
    pub scfg: *mut regmap,
    pub index: c_int,
    pub big_endian: bool,
}

#[no_mangle]
unsafe extern "C" fn ls_pcie_is_bridge(pcie: *mut ls_pcie) -> bool {
    static bool ls_pcie_is_bridge(struct ls_pcie *pcie)
    {
    struct dw_pcie *pci = pcie.pci;
    u32 header_type;
    header_type = ioread8(pci.dbi_base + PCI_HEADER_TYPE);
    header_type &= PCI_HEADER_TYPE_MASK;
    let mut header_type: return = = PCI_HEADER_TYPE_BRIDGE;
    }
// Clear multi-function bit
#[no_mangle]
unsafe extern "C" fn ls_pcie_clear_multifunction(pcie: *mut ls_pcie) {
    static void ls_pcie_clear_multifunction(struct ls_pcie *pcie)
    {
    struct dw_pcie *pci = pcie.pci;
    iowrite8(PCI_HEADER_TYPE_BRIDGE, pci.dbi_base + PCI_HEADER_TYPE);
    }
// Drop MSG TLP except for Vendor MSG
#[no_mangle]
unsafe extern "C" fn ls_pcie_drop_msg_tlp(pcie: *mut ls_pcie) {
    static void ls_pcie_drop_msg_tlp(struct ls_pcie *pcie)
    {
    u32 val;
    struct dw_pcie *pci = pcie.pci;
    val = ioread32(pci.dbi_base + PCIE_STRFMR1);
    val &= 0xDFFFFFFF;
    iowrite32(val, pci.dbi_base + PCIE_STRFMR1);
    }
// Forward error response of outbound non-posted requests
#[no_mangle]
unsafe extern "C" fn ls_pcie_fix_error_response(pcie: *mut ls_pcie) {
    static void ls_pcie_fix_error_response(struct ls_pcie *pcie)
    {
    struct dw_pcie *pci = pcie.pci;
    iowrite32(PCIE_ABSERR_SETTING, pci.dbi_base + PCIE_ABSERR);
    }
#[no_mangle]
unsafe extern "C" fn ls_pcie_pf_lut_readl(pcie: *mut ls_pcie, off: u32) -> u32 {
    static u32 ls_pcie_pf_lut_readl(struct ls_pcie *pcie, u32 off)
    {
    if (pcie.big_endian)
    return ioread32be(pcie.pf_lut_base + off);
    return ioread32(pcie.pf_lut_base + off);
    }
#[no_mangle]
unsafe extern "C" fn ls_pcie_pf_lut_writel(pcie: *mut ls_pcie, off: u32, val: u32) {
    static void ls_pcie_pf_lut_writel(struct ls_pcie *pcie, u32 off, u32 val)
    {
    if (pcie.big_endian)
    iowrite32be(val, pcie.pf_lut_base + off);
    else
    iowrite32(val, pcie.pf_lut_base + off);
    }
#[no_mangle]
unsafe extern "C" fn ls_pcie_send_turnoff_msg(pp: *mut dw_pcie_rp) {
    static void ls_pcie_send_turnoff_msg(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct ls_pcie *pcie = to_ls_pcie(pci);
    u32 val;
    int ret;
    val = ls_pcie_pf_lut_readl(pcie, LS_PCIE_PF_MCR);
    val |= PF_MCR_PTOMR;
    ls_pcie_pf_lut_writel(pcie, LS_PCIE_PF_MCR, val);
    ret = readx_poll_timeout(ls_pcie_pf_lut_readl_addr, LS_PCIE_PF_MCR,
    val, !(val & PF_MCR_PTOMR),
    PCIE_PME_TO_L2_TIMEOUT_US/10,
    PCIE_PME_TO_L2_TIMEOUT_US);
    if (ret)
    dev_err(pcie.pci.dev, "PME_Turn_off timeout\n");
    }
#[no_mangle]
unsafe extern "C" fn ls_pcie_exit_from_l2(pp: *mut dw_pcie_rp) -> c_int {
    static int ls_pcie_exit_from_l2(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct ls_pcie *pcie = to_ls_pcie(pci);
    u32 val;
    int ret;
//
// Set PF_MCR_EXL2S bit in LS_PCIE_PF_MCR register for the link
// to exit L2 state.
//
    val = ls_pcie_pf_lut_readl(pcie, LS_PCIE_PF_MCR);
    val |= PF_MCR_EXL2S;
    ls_pcie_pf_lut_writel(pcie, LS_PCIE_PF_MCR, val);
//
// L2 exit timeout of 10ms is not defined in the specifications,
// it was chosen based on empirical observations.
//
    ret = readx_poll_timeout(ls_pcie_pf_lut_readl_addr, LS_PCIE_PF_MCR,
    val, !(val & PF_MCR_EXL2S),
    1000,
    10000);
    if (ret)
    dev_err(pcie.pci.dev, "L2 exit timeout\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ls_pcie_host_init(pp: *mut dw_pcie_rp) -> c_int {
    static int ls_pcie_host_init(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct ls_pcie *pcie = to_ls_pcie(pci);
    ls_pcie_fix_error_response(pcie);
    dw_pcie_dbi_ro_wr_en(pci);
    ls_pcie_clear_multifunction(pcie);
    dw_pcie_dbi_ro_wr_dis(pci);
    ls_pcie_drop_msg_tlp(pcie);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn scfg_pcie_send_turnoff_msg(scfg: *mut regmap, reg: u32, mask: u32) {
    static void scfg_pcie_send_turnoff_msg(struct regmap *scfg, u32 reg, u32 mask)
    {
// Send PME_Turn_Off message
    regmap_write_bits(scfg, reg, mask, mask);
//
// There is no specific register to check for PME_To_Ack from endpoint.
// So on the safe side, wait for PCIE_PME_TO_L2_TIMEOUT_US.
//
    mdelay(PCIE_PME_TO_L2_TIMEOUT_US/1000);
//
// Layerscape hardware reference manual recommends clearing the PMXMTTURNOFF bit
// to complete the PME_Turn_Off handshake.
//
    regmap_write_bits(scfg, reg, mask, 0);
    }
#[no_mangle]
unsafe extern "C" fn ls1021a_pcie_send_turnoff_msg(pp: *mut dw_pcie_rp) {
    static void ls1021a_pcie_send_turnoff_msg(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct ls_pcie *pcie = to_ls_pcie(pci);
    scfg_pcie_send_turnoff_msg(pcie.scfg, SCFG_PEXPMWRCR(pcie.index), PMXMTTURNOFF);
    }
#[no_mangle]
unsafe extern "C" fn scfg_pcie_exit_from_l2(scfg: *mut regmap, reg: u32, mask: u32) -> c_int {
    static int scfg_pcie_exit_from_l2(struct regmap *scfg, u32 reg, u32 mask)
    {
// Reset the PEX wrapper to bring the link out of L2
    regmap_write_bits(scfg, reg, mask, mask);
    regmap_write_bits(scfg, reg, mask, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls1021a_pcie_exit_from_l2(pp: *mut dw_pcie_rp) -> c_int {
    static int ls1021a_pcie_exit_from_l2(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct ls_pcie *pcie = to_ls_pcie(pci);
    return scfg_pcie_exit_from_l2(pcie.scfg, SCFG_PEXSFTRSTCR, PEXSR(pcie.index));
    }
#[no_mangle]
unsafe extern "C" fn ls1043a_pcie_send_turnoff_msg(pp: *mut dw_pcie_rp) {
    static void ls1043a_pcie_send_turnoff_msg(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct ls_pcie *pcie = to_ls_pcie(pci);
    scfg_pcie_send_turnoff_msg(pcie.scfg, SCFG_PEXPMECR, PEXPME(pcie.index));
    }
#[no_mangle]
unsafe extern "C" fn ls1043a_pcie_exit_from_l2(pp: *mut dw_pcie_rp) -> c_int {
    static int ls1043a_pcie_exit_from_l2(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct ls_pcie *pcie = to_ls_pcie(pci);
    u32 val;
//
// Reset the PEX wrapper to bring the link out of L2.
// LDBG_WE: allows the user to have write access to the PEXDBG[SR] for both setting and
// clearing the soft reset on the PEX module.
// LDBG_SR: When SR is set to 1, the PEX module enters soft reset.
//
    val = ls_pcie_pf_lut_readl(pcie, LS_PCIE_LDBG);
    val |= LDBG_WE;
    ls_pcie_pf_lut_writel(pcie, LS_PCIE_LDBG, val);
    val = ls_pcie_pf_lut_readl(pcie, LS_PCIE_LDBG);
    val |= LDBG_SR;
    ls_pcie_pf_lut_writel(pcie, LS_PCIE_LDBG, val);
    val = ls_pcie_pf_lut_readl(pcie, LS_PCIE_LDBG);
    val &= ~LDBG_SR;
    ls_pcie_pf_lut_writel(pcie, LS_PCIE_LDBG, val);
    val = ls_pcie_pf_lut_readl(pcie, LS_PCIE_LDBG);
    val &= ~LDBG_WE;
    ls_pcie_pf_lut_writel(pcie, LS_PCIE_LDBG, val);
    return 0;
    }
    static const struct dw_pcie_host_ops ls_pcie_host_ops = {
    .init = ls_pcie_host_init,
    .pme_turn_off = ls_pcie_send_turnoff_msg,
    };
    static const struct dw_pcie_host_ops ls1021a_pcie_host_ops = {
    .init = ls_pcie_host_init,
    .pme_turn_off = ls1021a_pcie_send_turnoff_msg,
    };
    static const struct ls_pcie_drvdata ls1021a_drvdata = {
    .pm_support = true,
    .scfg_support = true,
    .ops = &ls1021a_pcie_host_ops,
    .exit_from_l2 = ls1021a_pcie_exit_from_l2,
    };
    static const struct dw_pcie_host_ops ls1043a_pcie_host_ops = {
    .init = ls_pcie_host_init,
    .pme_turn_off = ls1043a_pcie_send_turnoff_msg,
    };
    static const struct ls_pcie_drvdata ls1043a_drvdata = {
    .pf_lut_off = 0x10000,
    .pm_support = true,
    .scfg_support = true,
    .ops = &ls1043a_pcie_host_ops,
    .exit_from_l2 = ls1043a_pcie_exit_from_l2,
    };
    static const struct ls_pcie_drvdata layerscape_drvdata = {
    .pf_lut_off = 0xc0000,
    .pm_support = true,
    .ops = &ls_pcie_host_ops,
    .exit_from_l2 = ls_pcie_exit_from_l2,
    };
    static const struct of_device_id ls_pcie_of_match[] = {
    { .compatible = "fsl,ls1012a-pcie", .data = &layerscape_drvdata },
    { .compatible = "fsl,ls1021a-pcie", .data = &ls1021a_drvdata },
    { .compatible = "fsl,ls1028a-pcie", .data = &layerscape_drvdata },
    { .compatible = "fsl,ls1043a-pcie", .data = &ls1043a_drvdata },
    { .compatible = "fsl,ls1046a-pcie", .data = &layerscape_drvdata },
    { .compatible = "fsl,ls2080a-pcie", .data = &layerscape_drvdata },
    { .compatible = "fsl,ls2085a-pcie", .data = &layerscape_drvdata },
    { .compatible = "fsl,ls2088a-pcie", .data = &layerscape_drvdata },
    { .compatible = "fsl,ls1088a-pcie", .data = &layerscape_drvdata },
    { },
    };
#[no_mangle]
unsafe extern "C" fn ls_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int ls_pcie_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct dw_pcie *pci;
    struct ls_pcie *pcie;
    struct resource *dbi_base;
    u32 index[2];
    pcie = devm_kzalloc(dev, sizeof(*pcie), GFP_KERNEL);
    if (!pcie)
    return -ENOMEM;
    pci = devm_kzalloc(dev, sizeof(*pci), GFP_KERNEL);
    if (!pci)
    return -ENOMEM;
    pcie.drvdata = of_device_get_match_data(dev);
    pci.dev = dev;
    pcie.pci = pci;
    pci.pp.ops = pcie.drvdata.ops;
    dbi_base = platform_get_resource_byname(pdev, IORESOURCE_MEM, "regs");
    pci.dbi_base = devm_pci_remap_cfg_resource(dev, dbi_base);
    if (IS_ERR(pci.dbi_base))
    return PTR_ERR(pci.dbi_base);
    pcie.big_endian = of_property_read_bool(dev.of_node, "big-endian");
    pcie.pf_lut_base = pci.dbi_base + pcie.drvdata.pf_lut_off;
    if (pcie.drvdata.scfg_support) {
    pcie.scfg =
    syscon_regmap_lookup_by_phandle_args(dev.of_node,
    "fsl,pcie-scfg", 1,
    index);
    if (IS_ERR(pcie.scfg)) {
    dev_err(dev, "No syscfg phandle specified\n");
    return PTR_ERR(pcie.scfg);
    }
    pcie.index = index[1];
    }
    if (!ls_pcie_is_bridge(pcie))
    return -ENODEV;
    platform_set_drvdata(pdev, pcie);
    return dw_pcie_host_init(&pci.pp);
    }
#[no_mangle]
unsafe extern "C" fn ls_pcie_suspend_noirq(dev: *mut device) -> c_int {
    static int ls_pcie_suspend_noirq(struct device *dev)
    {
    struct ls_pcie *pcie = dev_get_drvdata(dev);
    if (!pcie.drvdata.pm_support)
    return 0;
    return dw_pcie_suspend_noirq(pcie.pci);
    }
#[no_mangle]
unsafe extern "C" fn ls_pcie_resume_noirq(dev: *mut device) -> c_int {
    static int ls_pcie_resume_noirq(struct device *dev)
    {
    struct ls_pcie *pcie = dev_get_drvdata(dev);
    int ret;
    if (!pcie.drvdata.pm_support)
    return 0;
    ret = pcie.drvdata.exit_from_l2(&pcie.pci.pp);
    if (ret)
    return ret;
    return dw_pcie_resume_noirq(pcie.pci);
    }
    static const struct dev_pm_ops ls_pcie_pm_ops = {
    NOIRQ_SYSTEM_SLEEP_PM_OPS(ls_pcie_suspend_noirq, ls_pcie_resume_noirq)
    };
#[no_mangle]
unsafe extern "C" fn ls_pcie_remove(pdev: *mut platform_device) {
    static void ls_pcie_remove(struct platform_device *pdev)
    {
    struct ls_pcie *pcie = platform_get_drvdata(pdev);
    dw_pcie_host_deinit(&pcie.pci.pp);
    }
    static struct platform_driver ls_pcie_driver = {
    .probe = ls_pcie_probe,
    .remove = ls_pcie_remove,
    .driver = {
    .name = "layerscape-pcie",
    .of_match_table = ls_pcie_of_match,
    .suppress_bind_attrs = true,
    .pm = &ls_pcie_pm_ops,
    },
    };
    module_platform_driver(ls_pcie_driver);
    MODULE_AUTHOR("Minghuan Lian <Minghuan.Lian@freescale.com>");
    MODULE_DESCRIPTION("Layerscape PCIe host controller driver");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(of, ls_pcie_of_match);
