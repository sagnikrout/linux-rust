//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/mobiveil/pcie-layerscape-gen4.c
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
// PCIe Gen4 host controller driver for NXP Layerscape SoCs
//
// Copyright 2019-2020 NXP
//
// Author: Zhiqiang Hou <Zhiqiang.Hou@nxp.com>
//

// LUT and PF control registers
pub const PCIE_LUT_OFF: c_uint = 0x80000;
pub const PCIE_PF_OFF: c_uint = 0xc0000;
pub const PCIE_PF_INT_STAT: c_uint = 0x18;

pub const PCIE_PF_DBG: c_uint = 0x7fc;
pub const PF_DBG_LTSSM_MASK: c_uint = 0x3f;
pub const PF_DBG_LTSSM_L0: c_uint = 0x2d /* L0 state */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls_g4_pcie {
    pub pci: mobiveil_pcie,
    pub dwork: delayed_work,
    pub irq: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn ls_g4_pcie_pf_readl(pcie: *mut ls_g4_pcie, off: u32) -> u32 {
    static inline u32 ls_g4_pcie_pf_readl(struct ls_g4_pcie *pcie, u32 off)
    {
    return ioread32(pcie.pci.csr_axi_slave_base + PCIE_PF_OFF + off);
    }
    static inline void ls_g4_pcie_pf_writel(struct ls_g4_pcie *pcie,
    u32 off, u32 val)
    {
    iowrite32(val, pcie.pci.csr_axi_slave_base + PCIE_PF_OFF + off);
    }
#[no_mangle]
unsafe extern "C" fn ls_g4_pcie_link_up(pci: *mut mobiveil_pcie) -> bool {
    static bool ls_g4_pcie_link_up(struct mobiveil_pcie *pci)
    {
    struct ls_g4_pcie *pcie = to_ls_g4_pcie(pci);
    u32 state;
    state = ls_g4_pcie_pf_readl(pcie, PCIE_PF_DBG);
    return (state & PF_DBG_LTSSM_MASK) == PF_DBG_LTSSM_L0;
    }
#[no_mangle]
unsafe extern "C" fn ls_g4_pcie_disable_interrupt(pcie: *mut ls_g4_pcie) {
    static void ls_g4_pcie_disable_interrupt(struct ls_g4_pcie *pcie)
    {
    struct mobiveil_pcie *mv_pci = &pcie.pci;
    mobiveil_csr_writel(mv_pci, 0, PAB_INTP_AMBA_MISC_ENB);
    }
#[no_mangle]
unsafe extern "C" fn ls_g4_pcie_enable_interrupt(pcie: *mut ls_g4_pcie) {
    static void ls_g4_pcie_enable_interrupt(struct ls_g4_pcie *pcie)
    {
    struct mobiveil_pcie *mv_pci = &pcie.pci;
    u32 val;
// Clear the interrupt status
    mobiveil_csr_writel(mv_pci, 0xffffffff, PAB_INTP_AMBA_MISC_STAT);
    val = PAB_INTP_INTX_MASK | PAB_INTP_MSI | PAB_INTP_RESET |
    PAB_INTP_PCIE_UE | PAB_INTP_IE_PMREDI | PAB_INTP_IE_EC;
    mobiveil_csr_writel(mv_pci, val, PAB_INTP_AMBA_MISC_ENB);
    }
#[no_mangle]
unsafe extern "C" fn ls_g4_pcie_reinit_hw(pcie: *mut ls_g4_pcie) -> c_int {
    static int ls_g4_pcie_reinit_hw(struct ls_g4_pcie *pcie)
    {
    struct mobiveil_pcie *mv_pci = &pcie.pci;
    struct device *dev = &mv_pci.pdev.dev;
    u32 val, act_stat;
    let mut to: c_int = 100;
// Poll for pab_csb_reset to set and PAB activity to clear
    do {
    usleep_range(10, 15);
    val = ls_g4_pcie_pf_readl(pcie, PCIE_PF_INT_STAT);
    act_stat = mobiveil_csr_readl(mv_pci, PAB_ACTIVITY_STAT);
    } while (((val & PF_INT_STAT_PABRST) == 0 || act_stat) && to--);
    if (to < 0) {
    dev_err(dev, "Poll PABRST&PABACT timeout\n");
    return -EIO;
    }
// clear PEX_RESET bit in PEX_PF0_DBG register
    val = ls_g4_pcie_pf_readl(pcie, PCIE_PF_DBG);
    val |= PF_DBG_WE;
    ls_g4_pcie_pf_writel(pcie, PCIE_PF_DBG, val);
    val = ls_g4_pcie_pf_readl(pcie, PCIE_PF_DBG);
    val |= PF_DBG_PABR;
    ls_g4_pcie_pf_writel(pcie, PCIE_PF_DBG, val);
    val = ls_g4_pcie_pf_readl(pcie, PCIE_PF_DBG);
    val &= ~PF_DBG_WE;
    ls_g4_pcie_pf_writel(pcie, PCIE_PF_DBG, val);
    mobiveil_host_init(mv_pci, true);
    to = 100;
    while (!ls_g4_pcie_link_up(mv_pci) && to--)
    usleep_range(200, 250);
    if (to < 0) {
    dev_err(dev, "PCIe link training timeout\n");
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls_g4_pcie_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ls_g4_pcie_isr(int irq, void *dev_id)
    {
    struct ls_g4_pcie *pcie = (struct ls_g4_pcie *)dev_id;
    struct mobiveil_pcie *mv_pci = &pcie.pci;
    u32 val;
    val = mobiveil_csr_readl(mv_pci, PAB_INTP_AMBA_MISC_STAT);
    if (!val)
    return IRQ_NONE;
    if (val & PAB_INTP_RESET) {
    ls_g4_pcie_disable_interrupt(pcie);
    schedule_delayed_work(&pcie.dwork, msecs_to_jiffies(1));
    }
    mobiveil_csr_writel(mv_pci, val, PAB_INTP_AMBA_MISC_STAT);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ls_g4_pcie_interrupt_init(mv_pci: *mut mobiveil_pcie) -> c_int {
    static int ls_g4_pcie_interrupt_init(struct mobiveil_pcie *mv_pci)
    {
    struct ls_g4_pcie *pcie = to_ls_g4_pcie(mv_pci);
    struct platform_device *pdev = mv_pci.pdev;
    struct device *dev = &pdev.dev;
    int ret;
    pcie.irq = platform_get_irq_byname(pdev, "intr");
    if (pcie.irq < 0)
    return pcie.irq;
    ret = devm_request_irq(dev, pcie.irq, ls_g4_pcie_isr,
    IRQF_SHARED, pdev.name, pcie);
    if (ret) {
    dev_err(dev, "Can't register PCIe IRQ, errno = %d\n", ret);
    return  ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls_g4_pcie_reset(work: *mut work_struct) {
    static void ls_g4_pcie_reset(struct work_struct *work)
    {
    struct delayed_work *dwork = to_delayed_work(work);
    struct ls_g4_pcie *pcie = container_of(dwork, struct ls_g4_pcie, dwork);
    struct mobiveil_pcie *mv_pci = &pcie.pci;
    u16 ctrl;
    ctrl = mobiveil_csr_readw(mv_pci, PCI_BRIDGE_CONTROL);
    ctrl &= ~PCI_BRIDGE_CTL_BUS_RESET;
    mobiveil_csr_writew(mv_pci, ctrl, PCI_BRIDGE_CONTROL);
    if (!ls_g4_pcie_reinit_hw(pcie))
    return;
    ls_g4_pcie_enable_interrupt(pcie);
    }
    static const struct mobiveil_rp_ops ls_g4_pcie_rp_ops = {
    .interrupt_init = ls_g4_pcie_interrupt_init,
    };
    static const struct mobiveil_pab_ops ls_g4_pcie_pab_ops = {
    .link_up = ls_g4_pcie_link_up,
    };
#[no_mangle]
unsafe extern "C" fn ls_g4_pcie_probe(pdev: *mut platform_device) -> int __init {
    static int __init ls_g4_pcie_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct pci_host_bridge *bridge;
    struct mobiveil_pcie *mv_pci;
    struct ls_g4_pcie *pcie;
    struct device_node *np = dev.of_node;
    int ret;
    if (!of_parse_phandle(np, "msi-parent", 0)) {
    dev_err(dev, "Failed to find msi-parent\n");
    return -EINVAL;
    }
    bridge = devm_pci_alloc_host_bridge(dev, sizeof(*pcie));
    if (!bridge)
    return -ENOMEM;
    pcie = pci_host_bridge_priv(bridge);
    mv_pci = &pcie.pci;
    mv_pci.pdev = pdev;
    mv_pci.ops = &ls_g4_pcie_pab_ops;
    mv_pci.rp.ops = &ls_g4_pcie_rp_ops;
    mv_pci.rp.bridge = bridge;
    platform_set_drvdata(pdev, pcie);
    INIT_DELAYED_WORK(&pcie.dwork, ls_g4_pcie_reset);
    ret = mobiveil_pcie_host_probe(mv_pci);
    if (ret) {
    dev_err(dev, "Fail to probe\n");
    return  ret;
    }
    ls_g4_pcie_enable_interrupt(pcie);
    return 0;
    }
    static const struct of_device_id ls_g4_pcie_of_match[] = {
    { .compatible = "fsl,lx2160a-pcie", },
    { },
    };
    static struct platform_driver ls_g4_pcie_driver = {
    .driver = {
    .name = "layerscape-pcie-gen4",
    .of_match_table = ls_g4_pcie_of_match,
    .suppress_bind_attrs = true,
    },
    };
    builtin_platform_driver_probe(ls_g4_pcie_driver, ls_g4_pcie_probe);
