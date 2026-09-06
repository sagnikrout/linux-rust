//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/dwc/pcie-histb.c
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
// PCIe host controller driver for HiSilicon STB SoCs
//
// Copyright (C) 2016-2017 HiSilicon Co., Ltd. http://www.hisilicon.com
//
// Authors: Ruqiang Ju <juruqiang@hisilicon.com>
// Jianguo Sun <sunjianguo1@huawei.com>
//

pub const PCIE_SYS_CTRL0: c_uint = 0x0000;
pub const PCIE_SYS_CTRL1: c_uint = 0x0004;
pub const PCIE_SYS_CTRL7: c_uint = 0x001C;
pub const PCIE_SYS_CTRL13: c_uint = 0x0034;
pub const PCIE_SYS_CTRL15: c_uint = 0x003C;
pub const PCIE_SYS_CTRL16: c_uint = 0x0040;
pub const PCIE_SYS_CTRL17: c_uint = 0x0044;
pub const PCIE_SYS_STAT0: c_uint = 0x0100;
pub const PCIE_SYS_STAT4: c_uint = 0x0110;

pub const PCIE_WM_EP: c_int = 0;

pub const PCIE_LTSSM_STATE_ACTIVE: c_uint = 0x11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct histb_pcie {
    pub pci: *mut dw_pcie,
    pub aux_clk: *mut clk,
    pub pipe_clk: *mut clk,
    pub sys_clk: *mut clk,
    pub bus_clk: *mut clk,
    pub phy: *mut phy,
    pub soft_reset: *mut reset_control,
    pub sys_reset: *mut reset_control,
    pub bus_reset: *mut reset_control,
    pub ctrl: *mut void __iomem,
    pub reset_gpio: *mut gpio_desc,
    pub vpcie: *mut regulator,
}

#[no_mangle]
unsafe extern "C" fn histb_pcie_readl(histb_pcie: *mut histb_pcie, reg: u32) -> u32 {
    static u32 histb_pcie_readl(struct histb_pcie *histb_pcie, u32 reg)
    {
    return readl(histb_pcie.ctrl + reg);
    }
#[no_mangle]
unsafe extern "C" fn histb_pcie_writel(histb_pcie: *mut histb_pcie, reg: u32, val: u32) {
    static void histb_pcie_writel(struct histb_pcie *histb_pcie, u32 reg, u32 val)
    {
    writel(val, histb_pcie.ctrl + reg);
    }
#[no_mangle]
unsafe extern "C" fn histb_pcie_dbi_w_mode(pp: *mut dw_pcie_rp, enable: bool) {
    static void histb_pcie_dbi_w_mode(struct dw_pcie_rp *pp, bool enable)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct histb_pcie *hipcie = to_histb_pcie(pci);
    u32 val;
    val = histb_pcie_readl(hipcie, PCIE_SYS_CTRL0);
    if (enable)
    val |= PCIE_ELBI_SLV_DBI_ENABLE;
    else
    val &= ~PCIE_ELBI_SLV_DBI_ENABLE;
    histb_pcie_writel(hipcie, PCIE_SYS_CTRL0, val);
    }
#[no_mangle]
unsafe extern "C" fn histb_pcie_dbi_r_mode(pp: *mut dw_pcie_rp, enable: bool) {
    static void histb_pcie_dbi_r_mode(struct dw_pcie_rp *pp, bool enable)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct histb_pcie *hipcie = to_histb_pcie(pci);
    u32 val;
    val = histb_pcie_readl(hipcie, PCIE_SYS_CTRL1);
    if (enable)
    val |= PCIE_ELBI_SLV_DBI_ENABLE;
    else
    val &= ~PCIE_ELBI_SLV_DBI_ENABLE;
    histb_pcie_writel(hipcie, PCIE_SYS_CTRL1, val);
    }
    static u32 histb_pcie_read_dbi(struct dw_pcie *pci, void __iomem *base,
    u32 reg, size_t size)
    {
    u32 val;
    histb_pcie_dbi_r_mode(&pci.pp, true);
    dw_pcie_read(base + reg, size, &val);
    histb_pcie_dbi_r_mode(&pci.pp, false);
    return val;
    }
    static void histb_pcie_write_dbi(struct dw_pcie *pci, void __iomem *base,
    u32 reg, size_t size, u32 val)
    {
    histb_pcie_dbi_w_mode(&pci.pp, true);
    dw_pcie_write(base + reg, size, val);
    histb_pcie_dbi_w_mode(&pci.pp, false);
    }
    static int histb_pcie_rd_own_conf(struct pci_bus *bus, unsigned int devfn,
    int where, int size, u32 *val)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(bus.sysdata);
    if (PCI_SLOT(devfn))
    return PCIBIOS_DEVICE_NOT_FOUND;
// val = dw_pcie_read_dbi(pci, where, size);
    return PCIBIOS_SUCCESSFUL;
    }
    static int histb_pcie_wr_own_conf(struct pci_bus *bus, unsigned int devfn,
    int where, int size, u32 val)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(bus.sysdata);
    if (PCI_SLOT(devfn))
    return PCIBIOS_DEVICE_NOT_FOUND;
    dw_pcie_write_dbi(pci, where, size, val);
    return PCIBIOS_SUCCESSFUL;
    }
    static struct pci_ops histb_pci_ops = {
    .read = histb_pcie_rd_own_conf,
    .write = histb_pcie_wr_own_conf,
    };
#[no_mangle]
unsafe extern "C" fn histb_pcie_link_up(pci: *mut dw_pcie) -> bool {
    static bool histb_pcie_link_up(struct dw_pcie *pci)
    {
    struct histb_pcie *hipcie = to_histb_pcie(pci);
    u32 regval;
    u32 status;
    regval = histb_pcie_readl(hipcie, PCIE_SYS_STAT0);
    status = histb_pcie_readl(hipcie, PCIE_SYS_STAT4);
    status &= PCIE_LTSSM_STATE_MASK;
    return ((regval & PCIE_XMLH_LINK_UP) && (regval & PCIE_RDLH_LINK_UP) &&
    (status == PCIE_LTSSM_STATE_ACTIVE));
    }
#[no_mangle]
unsafe extern "C" fn histb_pcie_start_link(pci: *mut dw_pcie) -> c_int {
    static int histb_pcie_start_link(struct dw_pcie *pci)
    {
    struct histb_pcie *hipcie = to_histb_pcie(pci);
    u32 regval;
// assert LTSSM enable
    regval = histb_pcie_readl(hipcie, PCIE_SYS_CTRL7);
    regval |= PCIE_APP_LTSSM_ENABLE;
    histb_pcie_writel(hipcie, PCIE_SYS_CTRL7, regval);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn histb_pcie_host_init(pp: *mut dw_pcie_rp) -> c_int {
    static int histb_pcie_host_init(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct histb_pcie *hipcie = to_histb_pcie(pci);
    u32 regval;
    pp.bridge.ops = &histb_pci_ops;
// PCIe RC work mode
    regval = histb_pcie_readl(hipcie, PCIE_SYS_CTRL0);
    regval &= ~PCIE_DEVICE_TYPE_MASK;
    regval |= PCIE_WM_RC;
    histb_pcie_writel(hipcie, PCIE_SYS_CTRL0, regval);
    return 0;
    }
    static const struct dw_pcie_host_ops histb_pcie_host_ops = {
    .init = histb_pcie_host_init,
    };
#[no_mangle]
unsafe extern "C" fn histb_pcie_host_disable(hipcie: *mut histb_pcie) {
    static void histb_pcie_host_disable(struct histb_pcie *hipcie)
    {
    reset_control_assert(hipcie.soft_reset);
    reset_control_assert(hipcie.sys_reset);
    reset_control_assert(hipcie.bus_reset);
    clk_disable_unprepare(hipcie.aux_clk);
    clk_disable_unprepare(hipcie.pipe_clk);
    clk_disable_unprepare(hipcie.sys_clk);
    clk_disable_unprepare(hipcie.bus_clk);
    if (hipcie.reset_gpio)
    gpiod_set_value_cansleep(hipcie.reset_gpio, 1);
    if (hipcie.vpcie)
    regulator_disable(hipcie.vpcie);
    }
#[no_mangle]
unsafe extern "C" fn histb_pcie_host_enable(pp: *mut dw_pcie_rp) -> c_int {
    static int histb_pcie_host_enable(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct histb_pcie *hipcie = to_histb_pcie(pci);
    struct device *dev = pci.dev;
    int ret;
// power on PCIe device if have
    if (hipcie.vpcie) {
    ret = regulator_enable(hipcie.vpcie);
    if (ret) {
    dev_err(dev, "failed to enable regulator: %d\n", ret);
    return ret;
    }
    }
    if (hipcie.reset_gpio)
    gpiod_set_value_cansleep(hipcie.reset_gpio, 0);
    ret = clk_prepare_enable(hipcie.bus_clk);
    if (ret) {
    dev_err(dev, "cannot prepare/enable bus clk\n");
    goto err_bus_clk;
    }
    ret = clk_prepare_enable(hipcie.sys_clk);
    if (ret) {
    dev_err(dev, "cannot prepare/enable sys clk\n");
    goto err_sys_clk;
    }
    ret = clk_prepare_enable(hipcie.pipe_clk);
    if (ret) {
    dev_err(dev, "cannot prepare/enable pipe clk\n");
    goto err_pipe_clk;
    }
    ret = clk_prepare_enable(hipcie.aux_clk);
    if (ret) {
    dev_err(dev, "cannot prepare/enable aux clk\n");
    goto err_aux_clk;
    }
    reset_control_assert(hipcie.soft_reset);
    reset_control_deassert(hipcie.soft_reset);
    reset_control_assert(hipcie.sys_reset);
    reset_control_deassert(hipcie.sys_reset);
    reset_control_assert(hipcie.bus_reset);
    reset_control_deassert(hipcie.bus_reset);
    return 0;
    err_aux_clk:
    clk_disable_unprepare(hipcie.pipe_clk);
    err_pipe_clk:
    clk_disable_unprepare(hipcie.sys_clk);
    err_sys_clk:
    clk_disable_unprepare(hipcie.bus_clk);
    err_bus_clk:
    if (hipcie.vpcie)
    regulator_disable(hipcie.vpcie);
    return ret;
    }
    static const struct dw_pcie_ops dw_pcie_ops = {
    .read_dbi = histb_pcie_read_dbi,
    .write_dbi = histb_pcie_write_dbi,
    .link_up = histb_pcie_link_up,
    .start_link = histb_pcie_start_link,
    };
#[no_mangle]
unsafe extern "C" fn histb_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int histb_pcie_probe(struct platform_device *pdev)
    {
    struct histb_pcie *hipcie;
    struct dw_pcie *pci;
    struct dw_pcie_rp *pp;
    struct device *dev = &pdev.dev;
    int ret;
    hipcie = devm_kzalloc(dev, sizeof(*hipcie), GFP_KERNEL);
    if (!hipcie)
    return -ENOMEM;
    pci = devm_kzalloc(dev, sizeof(*pci), GFP_KERNEL);
    if (!pci)
    return -ENOMEM;
    hipcie.pci = pci;
    pp = &pci.pp;
    pci.dev = dev;
    pci.ops = &dw_pcie_ops;
    hipcie.ctrl = devm_platform_ioremap_resource_byname(pdev, "control");
    if (IS_ERR(hipcie.ctrl)) {
    dev_err(dev, "cannot get control reg base\n");
    return PTR_ERR(hipcie.ctrl);
    }
    pci.dbi_base = devm_platform_ioremap_resource_byname(pdev, "rc-dbi");
    if (IS_ERR(pci.dbi_base)) {
    dev_err(dev, "cannot get rc-dbi base\n");
    return PTR_ERR(pci.dbi_base);
    }
    hipcie.vpcie = devm_regulator_get_optional(dev, "vpcie");
    if (IS_ERR(hipcie.vpcie)) {
    if (PTR_ERR(hipcie.vpcie) != -ENODEV)
    return PTR_ERR(hipcie.vpcie);
    hipcie.vpcie = core::ptr::null_mut();
    }
    hipcie.reset_gpio = devm_gpiod_get_optional(dev, "reset",
    GPIOD_OUT_HIGH);
    ret = PTR_ERR_OR_ZERO(hipcie.reset_gpio);
    if (ret) {
    dev_err(dev, "unable to request reset gpio: %d\n", ret);
    return ret;
    }
    ret = gpiod_set_consumer_name(hipcie.reset_gpio,
    "PCIe device power control");
    if (ret) {
    dev_err(dev, "unable to set reset gpio name: %d\n", ret);
    return ret;
    }
    hipcie.aux_clk = devm_clk_get(dev, "aux");
    if (IS_ERR(hipcie.aux_clk)) {
    dev_err(dev, "Failed to get PCIe aux clk\n");
    return PTR_ERR(hipcie.aux_clk);
    }
    hipcie.pipe_clk = devm_clk_get(dev, "pipe");
    if (IS_ERR(hipcie.pipe_clk)) {
    dev_err(dev, "Failed to get PCIe pipe clk\n");
    return PTR_ERR(hipcie.pipe_clk);
    }
    hipcie.sys_clk = devm_clk_get(dev, "sys");
    if (IS_ERR(hipcie.sys_clk)) {
    dev_err(dev, "Failed to get PCIEe sys clk\n");
    return PTR_ERR(hipcie.sys_clk);
    }
    hipcie.bus_clk = devm_clk_get(dev, "bus");
    if (IS_ERR(hipcie.bus_clk)) {
    dev_err(dev, "Failed to get PCIe bus clk\n");
    return PTR_ERR(hipcie.bus_clk);
    }
    hipcie.soft_reset = devm_reset_control_get(dev, "soft");
    if (IS_ERR(hipcie.soft_reset)) {
    dev_err(dev, "couldn't get soft reset\n");
    return PTR_ERR(hipcie.soft_reset);
    }
    hipcie.sys_reset = devm_reset_control_get(dev, "sys");
    if (IS_ERR(hipcie.sys_reset)) {
    dev_err(dev, "couldn't get sys reset\n");
    return PTR_ERR(hipcie.sys_reset);
    }
    hipcie.bus_reset = devm_reset_control_get(dev, "bus");
    if (IS_ERR(hipcie.bus_reset)) {
    dev_err(dev, "couldn't get bus reset\n");
    return PTR_ERR(hipcie.bus_reset);
    }
    hipcie.phy = devm_phy_get(dev, "phy");
    if (IS_ERR(hipcie.phy)) {
    dev_info(dev, "no pcie-phy found\n");
    hipcie.phy = core::ptr::null_mut();
// fall through here!
// if no pcie-phy found, phy init
// should be done under boot!
//
    } else {
    phy_init(hipcie.phy);
    }
    pp.ops = &histb_pcie_host_ops;
    platform_set_drvdata(pdev, hipcie);
    ret = histb_pcie_host_enable(pp);
    if (ret) {
    dev_err(dev, "failed to enable host\n");
    goto err_exit_phy;
    }
    ret = dw_pcie_host_init(pp);
    if (ret) {
    dev_err(dev, "failed to initialize host\n");
    goto err_exit_phy;
    }
    return 0;
    err_exit_phy:
    phy_exit(hipcie.phy);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn histb_pcie_remove(pdev: *mut platform_device) {
    static void histb_pcie_remove(struct platform_device *pdev)
    {
    struct histb_pcie *hipcie = platform_get_drvdata(pdev);
    histb_pcie_host_disable(hipcie);
    phy_exit(hipcie.phy);
    }
    static const struct of_device_id histb_pcie_of_match[] = {
    { .compatible = "hisilicon,hi3798cv200-pcie", },
    {},
    };
    MODULE_DEVICE_TABLE(of, histb_pcie_of_match);
    static struct platform_driver histb_pcie_platform_driver = {
    .probe	= histb_pcie_probe,
    .remove = histb_pcie_remove,
    .driver = {
    .name = "histb-pcie",
    .of_match_table = histb_pcie_of_match,
    },
    };
    module_platform_driver(histb_pcie_platform_driver);
    MODULE_DESCRIPTION("HiSilicon STB PCIe host controller driver");
