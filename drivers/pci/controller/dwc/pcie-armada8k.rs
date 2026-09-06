//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/dwc/pcie-armada8k.c
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
// PCIe host controller driver for Marvell Armada-8K SoCs
//
// Armada-8K PCIe Glue Layer Source Code
//
// Copyright (C) 2016 Marvell Technology Group Ltd.
//
// Author: Yehuda Yitshak <yehuday@marvell.com>
// Author: Shadi Ammouri <shadi@marvell.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct armada8k_pcie {
    pub pci: *mut dw_pcie,
    pub clk: *mut clk,
    pub clk_reg: *mut clk,
    pub phy: [*mut phy; ARMADA8K_PCIE_MAX_LANES],
    pub phy_count: c_uint,
}

pub const PCIE_VENDOR_REGS_OFFSET: c_uint = 0x8000;

pub const PCIE_DEVICE_TYPE_SHIFT: c_int = 4;
pub const PCIE_DEVICE_TYPE_MASK: c_uint = 0xF;
pub const PCIE_DEVICE_TYPE_RC: c_uint = 0x4 /* Root complex */;

//
// AR/AW Cache defaults: Normal memory, Write-Back, Read / Write
// allocate
//
pub const ARCACHE_DEFAULT_VALUE: c_uint = 0x3511;
pub const AWCACHE_DEFAULT_VALUE: c_uint = 0x5311;
pub const DOMAIN_OUTER_SHAREABLE: c_uint = 0x2;
pub const AX_USER_DOMAIN_MASK: c_uint = 0x3;
pub const AX_USER_DOMAIN_SHIFT: c_int = 4;

#[no_mangle]
unsafe extern "C" fn armada8k_pcie_disable_phys(pcie: *mut armada8k_pcie) {
    static void armada8k_pcie_disable_phys(struct armada8k_pcie *pcie)
    {
    int i;
    for (i = 0; i < ARMADA8K_PCIE_MAX_LANES; i++) {
    phy_power_off(pcie.phy[i]);
    phy_exit(pcie.phy[i]);
    }
    }
#[no_mangle]
unsafe extern "C" fn armada8k_pcie_enable_phys(pcie: *mut armada8k_pcie) -> c_int {
    static int armada8k_pcie_enable_phys(struct armada8k_pcie *pcie)
    {
    int ret;
    int i;
    for (i = 0; i < ARMADA8K_PCIE_MAX_LANES; i++) {
    ret = phy_init(pcie.phy[i]);
    if (ret)
    return ret;
    ret = phy_set_mode_ext(pcie.phy[i], PHY_MODE_PCIE,
    pcie.phy_count);
    if (ret) {
    phy_exit(pcie.phy[i]);
    return ret;
    }
    ret = phy_power_on(pcie.phy[i]);
    if (ret) {
    phy_exit(pcie.phy[i]);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn armada8k_pcie_setup_phys(pcie: *mut armada8k_pcie) -> c_int {
    static int armada8k_pcie_setup_phys(struct armada8k_pcie *pcie)
    {
    struct dw_pcie *pci = pcie.pci;
    struct device *dev = pci.dev;
    struct device_node *node = dev.of_node;
    let mut ret: c_int = 0;
    int i;
    for (i = 0; i < ARMADA8K_PCIE_MAX_LANES; i++) {
    pcie.phy[i] = devm_of_phy_get_by_index(dev, node, i);
    if (IS_ERR(pcie.phy[i])) {
    if (PTR_ERR(pcie.phy[i]) != -ENODEV)
    return PTR_ERR(pcie.phy[i]);
    pcie.phy[i] = core::ptr::null_mut();
    continue;
    }
    pcie.phy_count++;
    }
// Old bindings miss the PHY handle, so just warn if there is no PHY
    if (!pcie.phy_count)
    dev_warn(dev, "No available PHY\n");
    ret = armada8k_pcie_enable_phys(pcie);
    if (ret)
    dev_err(dev, "Failed to initialize PHY(s) (%d)\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn armada8k_pcie_link_up(pci: *mut dw_pcie) -> bool {
    static bool armada8k_pcie_link_up(struct dw_pcie *pci)
    {
    u32 reg;
    let mut mask: u32 = PCIE_GLB_STS_RDLH_LINK_UP | PCIE_GLB_STS_PHY_LINK_UP;
    reg = dw_pcie_readl_dbi(pci, PCIE_GLOBAL_STATUS_REG);
    if ((reg & mask) == mask)
    return true;
    dev_dbg(pci.dev, "No link detected (Global-Status: 0x%08x).\n", reg);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn armada8k_pcie_start_link(pci: *mut dw_pcie) -> c_int {
    static int armada8k_pcie_start_link(struct dw_pcie *pci)
    {
    u32 reg;
// Start LTSSM
    reg = dw_pcie_readl_dbi(pci, PCIE_GLOBAL_CONTROL_REG);
    reg |= PCIE_APP_LTSSM_EN;
    dw_pcie_writel_dbi(pci, PCIE_GLOBAL_CONTROL_REG, reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn armada8k_pcie_host_init(pp: *mut dw_pcie_rp) -> c_int {
    static int armada8k_pcie_host_init(struct dw_pcie_rp *pp)
    {
    u32 reg;
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    if (!dw_pcie_link_up(pci)) {
// Disable LTSSM state machine to enable configuration
    reg = dw_pcie_readl_dbi(pci, PCIE_GLOBAL_CONTROL_REG);
    reg &= ~(PCIE_APP_LTSSM_EN);
    dw_pcie_writel_dbi(pci, PCIE_GLOBAL_CONTROL_REG, reg);
    }
// Set the device to root complex mode
    reg = dw_pcie_readl_dbi(pci, PCIE_GLOBAL_CONTROL_REG);
    reg &= ~(PCIE_DEVICE_TYPE_MASK << PCIE_DEVICE_TYPE_SHIFT);
    reg |= PCIE_DEVICE_TYPE_RC << PCIE_DEVICE_TYPE_SHIFT;
    dw_pcie_writel_dbi(pci, PCIE_GLOBAL_CONTROL_REG, reg);
// Set the PCIe master AxCache attributes
    dw_pcie_writel_dbi(pci, PCIE_ARCACHE_TRC_REG, ARCACHE_DEFAULT_VALUE);
    dw_pcie_writel_dbi(pci, PCIE_AWCACHE_TRC_REG, AWCACHE_DEFAULT_VALUE);
// Set the PCIe master AxDomain attributes
    reg = dw_pcie_readl_dbi(pci, PCIE_ARUSER_REG);
    reg &= ~(AX_USER_DOMAIN_MASK << AX_USER_DOMAIN_SHIFT);
    reg |= DOMAIN_OUTER_SHAREABLE << AX_USER_DOMAIN_SHIFT;
    dw_pcie_writel_dbi(pci, PCIE_ARUSER_REG, reg);
    reg = dw_pcie_readl_dbi(pci, PCIE_AWUSER_REG);
    reg &= ~(AX_USER_DOMAIN_MASK << AX_USER_DOMAIN_SHIFT);
    reg |= DOMAIN_OUTER_SHAREABLE << AX_USER_DOMAIN_SHIFT;
    dw_pcie_writel_dbi(pci, PCIE_AWUSER_REG, reg);
// Enable INT A-D interrupts
    reg = dw_pcie_readl_dbi(pci, PCIE_GLOBAL_INT_MASK1_REG);
    reg |= PCIE_INT_A_ASSERT_MASK | PCIE_INT_B_ASSERT_MASK |
    PCIE_INT_C_ASSERT_MASK | PCIE_INT_D_ASSERT_MASK;
    dw_pcie_writel_dbi(pci, PCIE_GLOBAL_INT_MASK1_REG, reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn armada8k_pcie_irq_handler(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t armada8k_pcie_irq_handler(int irq, void *arg)
    {
    struct armada8k_pcie *pcie = arg;
    struct dw_pcie *pci = pcie.pci;
    u32 val;
//
// Interrupts are directly handled by the device driver of the
// PCI device. However, they are also latched into the PCIe
// controller, so we simply discard them.
//
    val = dw_pcie_readl_dbi(pci, PCIE_GLOBAL_INT_CAUSE1_REG);
    dw_pcie_writel_dbi(pci, PCIE_GLOBAL_INT_CAUSE1_REG, val);
    return IRQ_HANDLED;
    }
    static const struct dw_pcie_host_ops armada8k_pcie_host_ops = {
    .init = armada8k_pcie_host_init,
    };
    static int armada8k_add_pcie_port(struct armada8k_pcie *pcie,
    struct platform_device *pdev)
    {
    struct dw_pcie *pci = pcie.pci;
    struct dw_pcie_rp *pp = &pci.pp;
    struct device *dev = &pdev.dev;
    int ret;
    pp.ops = &armada8k_pcie_host_ops;
    pp.irq = platform_get_irq(pdev, 0);
    if (pp.irq < 0)
    return pp.irq;
    ret = devm_request_irq(dev, pp.irq, armada8k_pcie_irq_handler,
    IRQF_SHARED, "armada8k-pcie", pcie);
    if (ret) {
    dev_err(dev, "failed to request irq %d\n", pp.irq);
    return ret;
    }
    ret = dw_pcie_host_init(pp);
    if (ret) {
    dev_err(dev, "failed to initialize host: %d\n", ret);
    return ret;
    }
    return 0;
    }
    static const struct dw_pcie_ops dw_pcie_ops = {
    .link_up = armada8k_pcie_link_up,
    .start_link = armada8k_pcie_start_link,
    };
#[no_mangle]
unsafe extern "C" fn armada8k_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int armada8k_pcie_probe(struct platform_device *pdev)
    {
    struct dw_pcie *pci;
    struct armada8k_pcie *pcie;
    struct device *dev = &pdev.dev;
    struct resource *base;
    int ret;
    pcie = devm_kzalloc(dev, sizeof(*pcie), GFP_KERNEL);
    if (!pcie)
    return -ENOMEM;
    pci = devm_kzalloc(dev, sizeof(*pci), GFP_KERNEL);
    if (!pci)
    return -ENOMEM;
    pci.dev = dev;
    pci.ops = &dw_pcie_ops;
    pcie.pci = pci;
    pcie.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(pcie.clk))
    return PTR_ERR(pcie.clk);
    ret = clk_prepare_enable(pcie.clk);
    if (ret)
    return ret;
    pcie.clk_reg = devm_clk_get(dev, "reg");
    if (pcie.clk_reg == ERR_PTR(-EPROBE_DEFER)) {
    ret = -EPROBE_DEFER;
    goto fail;
    }
    if (!IS_ERR(pcie.clk_reg)) {
    ret = clk_prepare_enable(pcie.clk_reg);
    if (ret)
    goto fail_clkreg;
    }
// Get the dw-pcie unit configuration/control registers base.
    base = platform_get_resource_byname(pdev, IORESOURCE_MEM, "ctrl");
    pci.dbi_base = devm_pci_remap_cfg_resource(dev, base);
    if (IS_ERR(pci.dbi_base)) {
    ret = PTR_ERR(pci.dbi_base);
    goto fail_clkreg;
    }
    ret = armada8k_pcie_setup_phys(pcie);
    if (ret)
    goto fail_clkreg;
    platform_set_drvdata(pdev, pcie);
    ret = armada8k_add_pcie_port(pcie, pdev);
    if (ret)
    goto disable_phy;
    return 0;
    disable_phy:
    armada8k_pcie_disable_phys(pcie);
    fail_clkreg:
    clk_disable_unprepare(pcie.clk_reg);
    fail:
    clk_disable_unprepare(pcie.clk);
    return ret;
    }
    static const struct of_device_id armada8k_pcie_of_match[] = {
    { .compatible = "marvell,armada8k-pcie", },
    {},
    };
    static struct platform_driver armada8k_pcie_driver = {
    .probe		= armada8k_pcie_probe,
    .driver = {
    .name	= "armada8k-pcie",
    .of_match_table = armada8k_pcie_of_match,
    .suppress_bind_attrs = true,
    },
    };
    builtin_platform_driver(armada8k_pcie_driver);
