//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/dwc/pcie-spear13xx.c
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
// PCIe host controller driver for ST Microelectronics SPEAr13xx SoCs
//
// SPEAr13xx PCIe Glue Layer Source Code
//
// Copyright (C) 2010-2014 ST Microelectronics
// Pratyush Anand <pratyush.anand@gmail.com>
// Mohit Kumar <mohit.kumar.dhaka@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear13xx_pcie {
    pub pci: *mut dw_pcie,
    pub app_base: *mut void __iomem,
    pub phy: *mut phy,
    pub clk: *mut clk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcie_app_reg {
    pub /: *mut *mut u32 app_ctrl_0; / cr0,
    pub /: *mut *mut u32 app_ctrl_1; / cr1,
    pub /: *mut *mut u32 app_status_0; / cr2,
    pub /: *mut *mut u32 app_status_1; / cr3,
    pub /: *mut *mut u32 msg_status; / cr4,
    pub /: *mut *mut u32 msg_payload; / cr5,
    pub /: *mut *mut u32 int_sts; / cr6,
    pub /: *mut *mut u32 int_clr; / cr7,
    pub /: *mut *mut u32 int_mask; / cr8,
    pub /: *mut *mut u32 mst_bmisc; / cr9,
    pub /: *mut *mut u32 phy_ctrl; / cr10,
    pub /: *mut *mut u32 phy_status; / cr11,
    pub /: *mut *mut u32 cxpl_debug_info_0; / cr12,
    pub /: *mut *mut u32 cxpl_debug_info_1; / cr13,
    pub /: *mut *mut u32 ven_msg_ctrl_0; / cr14,
    pub /: *mut *mut u32 ven_msg_ctrl_1; / cr15,
    pub /: *mut *mut u32 ven_msg_data_0; / cr16,
    pub /: *mut *mut u32 ven_msg_data_1; / cr17,
    pub /: *mut *mut u32 ven_msi_0; / cr18,
    pub /: *mut *mut u32 ven_msi_1; / cr19,
    pub /: *mut *mut u32 mst_rmisc; / cr20,
}

// CR0 ID
pub const APP_LTSSM_ENABLE_ID: c_int = 3;

pub const MISCTRL_EN_ID: c_int = 30;
pub const REG_TRANSLATION_ENABLE: c_int = 31;
// CR3 ID

// CR6

#[no_mangle]
unsafe extern "C" fn spear13xx_pcie_start_link(pci: *mut dw_pcie) -> c_int {
    static int spear13xx_pcie_start_link(struct dw_pcie *pci)
    {
    struct spear13xx_pcie *spear13xx_pcie = to_spear13xx_pcie(pci);
    struct pcie_app_reg __iomem *app_reg = spear13xx_pcie.app_base;
// enable ltssm
    writel(DEVICE_TYPE_RC | (1 << MISCTRL_EN_ID)
    | (1 << APP_LTSSM_ENABLE_ID)
    | ((u32)1 << REG_TRANSLATION_ENABLE),
    &app_reg.app_ctrl_0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spear13xx_pcie_irq_handler(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t spear13xx_pcie_irq_handler(int irq, void *arg)
    {
    struct spear13xx_pcie *spear13xx_pcie = arg;
    struct pcie_app_reg __iomem *app_reg = spear13xx_pcie.app_base;
    struct dw_pcie *pci = spear13xx_pcie.pci;
    struct dw_pcie_rp *pp = &pci.pp;
    unsigned int status;
    status = readl(&app_reg.int_sts);
    if (status & MSI_CTRL_INT) {
    BUG_ON(!IS_ENABLED(CONFIG_PCI_MSI));
    dw_handle_msi_irq(pp);
    }
    writel(status, &app_reg.int_clr);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn spear13xx_pcie_enable_interrupts(spear13xx_pcie: *mut spear13xx_pcie) {
    static void spear13xx_pcie_enable_interrupts(struct spear13xx_pcie *spear13xx_pcie)
    {
    struct pcie_app_reg __iomem *app_reg = spear13xx_pcie.app_base;
// Enable MSI interrupt
    if (IS_ENABLED(CONFIG_PCI_MSI))
    writel(readl(&app_reg.int_mask) |
    MSI_CTRL_INT, &app_reg.int_mask);
    }
#[no_mangle]
unsafe extern "C" fn spear13xx_pcie_link_up(pci: *mut dw_pcie) -> bool {
    static bool spear13xx_pcie_link_up(struct dw_pcie *pci)
    {
    struct spear13xx_pcie *spear13xx_pcie = to_spear13xx_pcie(pci);
    struct pcie_app_reg __iomem *app_reg = spear13xx_pcie.app_base;
    return readl(&app_reg.app_status_1) & XMLH_LINK_UP;
    }
#[no_mangle]
unsafe extern "C" fn spear13xx_pcie_host_init(pp: *mut dw_pcie_rp) -> c_int {
    static int spear13xx_pcie_host_init(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct spear13xx_pcie *spear13xx_pcie = to_spear13xx_pcie(pci);
    let mut exp_cap_off: u32 = dw_pcie_find_capability(pci, PCI_CAP_ID_EXP);
    u32 val;
    spear13xx_pcie.app_base = pci.dbi_base + 0x2000;
//
// this controller support only 128 bytes read size, however its
// default value in capability register is 512 bytes. So force
// it to 128 here.
//
    val = dw_pcie_readw_dbi(pci, exp_cap_off + PCI_EXP_DEVCTL);
    val &= ~PCI_EXP_DEVCTL_READRQ;
    dw_pcie_writew_dbi(pci, exp_cap_off + PCI_EXP_DEVCTL, val);
    dw_pcie_writew_dbi(pci, PCI_VENDOR_ID, 0x104A);
    dw_pcie_writew_dbi(pci, PCI_DEVICE_ID, 0xCD80);
    spear13xx_pcie_enable_interrupts(spear13xx_pcie);
    return 0;
    }
    static const struct dw_pcie_host_ops spear13xx_pcie_host_ops = {
    .init = spear13xx_pcie_host_init,
    };
    static int spear13xx_add_pcie_port(struct spear13xx_pcie *spear13xx_pcie,
    struct platform_device *pdev)
    {
    struct dw_pcie *pci = spear13xx_pcie.pci;
    struct dw_pcie_rp *pp = &pci.pp;
    struct device *dev = &pdev.dev;
    int ret;
    pp.irq = platform_get_irq(pdev, 0);
    if (pp.irq < 0)
    return pp.irq;
    ret = devm_request_irq(dev, pp.irq, spear13xx_pcie_irq_handler,
    IRQF_SHARED | IRQF_NO_THREAD,
    "spear1340-pcie", spear13xx_pcie);
    if (ret) {
    dev_err(dev, "failed to request irq %d\n", pp.irq);
    return ret;
    }
    pp.ops = &spear13xx_pcie_host_ops;
    pp.msi_irq[0] = -ENODEV;
    ret = dw_pcie_host_init(pp);
    if (ret) {
    dev_err(dev, "failed to initialize host\n");
    return ret;
    }
    return 0;
    }
    static const struct dw_pcie_ops dw_pcie_ops = {
    .link_up = spear13xx_pcie_link_up,
    .start_link = spear13xx_pcie_start_link,
    };
#[no_mangle]
unsafe extern "C" fn spear13xx_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int spear13xx_pcie_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct dw_pcie *pci;
    struct spear13xx_pcie *spear13xx_pcie;
    struct device_node *np = dev.of_node;
    int ret;
    spear13xx_pcie = devm_kzalloc(dev, sizeof(*spear13xx_pcie), GFP_KERNEL);
    if (!spear13xx_pcie)
    return -ENOMEM;
    pci = devm_kzalloc(dev, sizeof(*pci), GFP_KERNEL);
    if (!pci)
    return -ENOMEM;
    pci.dev = dev;
    pci.ops = &dw_pcie_ops;
    spear13xx_pcie.pci = pci;
    spear13xx_pcie.phy = devm_phy_get(dev, "pcie-phy");
    if (IS_ERR(spear13xx_pcie.phy)) {
    ret = PTR_ERR(spear13xx_pcie.phy);
    if (ret == -EPROBE_DEFER)
    dev_info(dev, "probe deferred\n");
    else
    dev_err(dev, "couldn't get pcie-phy\n");
    return ret;
    }
    phy_init(spear13xx_pcie.phy);
    spear13xx_pcie.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(spear13xx_pcie.clk)) {
    dev_err(dev, "couldn't get clk for pcie\n");
    return PTR_ERR(spear13xx_pcie.clk);
    }
    ret = clk_prepare_enable(spear13xx_pcie.clk);
    if (ret) {
    dev_err(dev, "couldn't enable clk for pcie\n");
    return ret;
    }
    if (of_property_read_bool(np, "st,pcie-is-gen1"))
    pci.max_link_speed = 1;
    platform_set_drvdata(pdev, spear13xx_pcie);
    ret = spear13xx_add_pcie_port(spear13xx_pcie, pdev);
    if (ret < 0)
    goto fail_clk;
    return 0;
    fail_clk:
    clk_disable_unprepare(spear13xx_pcie.clk);
    return ret;
    }
    static const struct of_device_id spear13xx_pcie_of_match[] = {
    { .compatible = "st,spear1340-pcie", },
    {},
    };
    static struct platform_driver spear13xx_pcie_driver = {
    .probe		= spear13xx_pcie_probe,
    .driver = {
    .name	= "spear-pcie",
    .of_match_table = spear13xx_pcie_of_match,
    .suppress_bind_attrs = true,
    },
    };
    builtin_platform_driver(spear13xx_pcie_driver);
