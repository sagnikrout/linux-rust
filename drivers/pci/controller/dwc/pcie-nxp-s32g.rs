//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/dwc/pcie-nxp-s32g.c
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
// PCIe host controller driver for NXP S32G SoCs
//
// Copyright 2019-2025 NXP
//

// PCIe controller Sub-System
// PCIe controller 0 General Control 1
pub const PCIE_S32G_PE0_GEN_CTRL_1: c_uint = 0x50;

// PCIe controller 0 General Control 3
pub const PCIE_S32G_PE0_GEN_CTRL_3: c_uint = 0x58;

// PCIe Controller 0  Interrupt Status
pub const PCIE_S32G_PE0_INT_STS: c_uint = 0xE8;

// Boundary between peripheral space and physical memory space
pub const S32G_MEMORY_BOUNDARY_ADDR: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s32g_pcie_port {
    pub list: list_head,
    pub phy: *mut phy,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s32g_pcie {
    pub pci: dw_pcie,
    pub ctrl_base: *mut void __iomem,
    pub ports: list_head,
}

    container_of(x, struct s32g_pcie, pci)
#[no_mangle]
unsafe extern "C" fn s32g_pcie_writel_ctrl(s32g_pp: *mut s32g_pcie, reg: u32, val: u32) {
    static void s32g_pcie_writel_ctrl(struct s32g_pcie *s32g_pp, u32 reg, u32 val)
    {
    writel(val, s32g_pp.ctrl_base + reg);
    }
#[no_mangle]
unsafe extern "C" fn s32g_pcie_readl_ctrl(s32g_pp: *mut s32g_pcie, reg: u32) -> u32 {
    static u32 s32g_pcie_readl_ctrl(struct s32g_pcie *s32g_pp, u32 reg)
    {
    return readl(s32g_pp.ctrl_base + reg);
    }
#[no_mangle]
unsafe extern "C" fn s32g_pcie_enable_ltssm(s32g_pp: *mut s32g_pcie) {
    static void s32g_pcie_enable_ltssm(struct s32g_pcie *s32g_pp)
    {
    u32 reg;
    reg = s32g_pcie_readl_ctrl(s32g_pp, PCIE_S32G_PE0_GEN_CTRL_3);
    reg |= LTSSM_EN;
    s32g_pcie_writel_ctrl(s32g_pp, PCIE_S32G_PE0_GEN_CTRL_3, reg);
    }
#[no_mangle]
unsafe extern "C" fn s32g_pcie_disable_ltssm(s32g_pp: *mut s32g_pcie) {
    static void s32g_pcie_disable_ltssm(struct s32g_pcie *s32g_pp)
    {
    u32 reg;
    reg = s32g_pcie_readl_ctrl(s32g_pp, PCIE_S32G_PE0_GEN_CTRL_3);
    reg &= ~LTSSM_EN;
    s32g_pcie_writel_ctrl(s32g_pp, PCIE_S32G_PE0_GEN_CTRL_3, reg);
    }
#[no_mangle]
unsafe extern "C" fn s32g_pcie_start_link(pci: *mut dw_pcie) -> c_int {
    static int s32g_pcie_start_link(struct dw_pcie *pci)
    {
    struct s32g_pcie *s32g_pp = to_s32g_from_dw_pcie(pci);
    s32g_pcie_enable_ltssm(s32g_pp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s32g_pcie_stop_link(pci: *mut dw_pcie) {
    static void s32g_pcie_stop_link(struct dw_pcie *pci)
    {
    struct s32g_pcie *s32g_pp = to_s32g_from_dw_pcie(pci);
    s32g_pcie_disable_ltssm(s32g_pp);
    }
    static struct dw_pcie_ops s32g_pcie_ops = {
    .start_link = s32g_pcie_start_link,
    .stop_link = s32g_pcie_stop_link,
    };
// Configure the AMBA AXI Coherency Extensions (ACE) interface
#[no_mangle]
unsafe extern "C" fn s32g_pcie_reset_mstr_ace(pci: *mut dw_pcie) {
    static void s32g_pcie_reset_mstr_ace(struct dw_pcie *pci)
    {
    let mut ddr_base_low: u32 = lower_32_bits(S32G_MEMORY_BOUNDARY_ADDR);
    let mut ddr_base_high: u32 = upper_32_bits(S32G_MEMORY_BOUNDARY_ADDR);
    dw_pcie_dbi_ro_wr_en(pci);
    dw_pcie_writel_dbi(pci, COHERENCY_CONTROL_3_OFF, 0x0);
//
// Ncore is a cache-coherent interconnect module that enables the
// integration of heterogeneous coherent and non-coherent agents in
// the chip. Ncore transactions to peripheral should be non-coherent
// or it might drop them.
//
// One example where this is needed are PCIe MSIs, which use NoSnoop=0
// and might end up routed to Ncore. PCIe coherent traffic (e.g. MSIs)
// that targets peripheral space will be dropped by Ncore because
// peripherals on S32G are not coherent as slaves. We add a hard
// boundary in the PCIe controller coherency control registers to
// separate physical memory space from peripheral space.
//
// Define the start of DDR as seen by Linux as this boundary between
// "memory" and "peripherals", with peripherals being below.
//
    dw_pcie_writel_dbi(pci, COHERENCY_CONTROL_1_OFF,
    (ddr_base_low & CFG_MEMTYPE_BOUNDARY_LOW_ADDR_MASK));
    dw_pcie_writel_dbi(pci, COHERENCY_CONTROL_2_OFF, ddr_base_high);
    dw_pcie_dbi_ro_wr_dis(pci);
    }
#[no_mangle]
unsafe extern "C" fn s32g_init_pcie_controller(pp: *mut dw_pcie_rp) -> c_int {
    static int s32g_init_pcie_controller(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct s32g_pcie *s32g_pp = to_s32g_from_dw_pcie(pci);
    u32 val;
// Set RP mode
    val = s32g_pcie_readl_ctrl(s32g_pp, PCIE_S32G_PE0_GEN_CTRL_1);
    FIELD_MODIFY(DEVICE_TYPE_MASK, &val, PCI_EXP_TYPE_ROOT_PORT);
// Use default CRNS
    val &= ~SRIS_MODE;
    s32g_pcie_writel_ctrl(s32g_pp, PCIE_S32G_PE0_GEN_CTRL_1, val);
//
// Make sure we use the coherency defaults (just in case the settings
// have been changed from their reset values)
//
    s32g_pcie_reset_mstr_ace(pci);
    dw_pcie_dbi_ro_wr_en(pci);
    val = dw_pcie_readl_dbi(pci, PCIE_PORT_FORCE);
    val |= PORT_FORCE_DO_DESKEW_FOR_SRIS;
    dw_pcie_writel_dbi(pci, PCIE_PORT_FORCE, val);
    val = dw_pcie_readl_dbi(pci, GEN3_RELATED_OFF);
    val |= GEN3_RELATED_OFF_EQ_PHASE_2_3;
    dw_pcie_writel_dbi(pci, GEN3_RELATED_OFF, val);
    dw_pcie_dbi_ro_wr_dis(pci);
    return 0;
    }
    static const struct dw_pcie_host_ops s32g_pcie_host_ops = {
    .init = s32g_init_pcie_controller,
    };
#[no_mangle]
unsafe extern "C" fn s32g_init_pcie_phy(s32g_pp: *mut s32g_pcie) -> c_int {
    static int s32g_init_pcie_phy(struct s32g_pcie *s32g_pp)
    {
    struct dw_pcie *pci = &s32g_pp.pci;
    struct device *dev = pci.dev;
    struct s32g_pcie_port *port, *tmp;
    int ret;
    list_for_each_entry(port, &s32g_pp.ports, list) {
    ret = phy_init(port.phy);
    if (ret) {
    dev_err(dev, "Failed to init serdes PHY\n");
    goto err_phy_revert;
    }
    ret = phy_set_mode_ext(port.phy, PHY_MODE_PCIE, 0);
    if (ret) {
    dev_err(dev, "Failed to set mode on serdes PHY\n");
    goto err_phy_exit;
    }
    ret = phy_power_on(port.phy);
    if (ret) {
    dev_err(dev, "Failed to power on serdes PHY\n");
    goto err_phy_exit;
    }
    }
    return 0;
    err_phy_exit:
    phy_exit(port.phy);
    err_phy_revert:
    list_for_each_entry_continue_reverse(port, &s32g_pp.ports, list) {
    phy_power_off(port.phy);
    phy_exit(port.phy);
    }
    list_for_each_entry_safe(port, tmp, &s32g_pp.ports, list)
    list_del(&port.list);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn s32g_deinit_pcie_phy(s32g_pp: *mut s32g_pcie) {
    static void s32g_deinit_pcie_phy(struct s32g_pcie *s32g_pp)
    {
    struct s32g_pcie_port *port, *tmp;
    list_for_each_entry_safe(port, tmp, &s32g_pp.ports, list) {
    phy_power_off(port.phy);
    phy_exit(port.phy);
    list_del(&port.list);
    }
    }
#[no_mangle]
unsafe extern "C" fn s32g_pcie_init(dev: *mut device, s32g_pp: *mut s32g_pcie) -> c_int {
    static int s32g_pcie_init(struct device *dev, struct s32g_pcie *s32g_pp)
    {
    s32g_pcie_disable_ltssm(s32g_pp);
    return s32g_init_pcie_phy(s32g_pp);
    }
#[no_mangle]
unsafe extern "C" fn s32g_pcie_deinit(s32g_pp: *mut s32g_pcie) {
    static void s32g_pcie_deinit(struct s32g_pcie *s32g_pp)
    {
    s32g_pcie_disable_ltssm(s32g_pp);
    s32g_deinit_pcie_phy(s32g_pp);
    }
#[no_mangle]
unsafe extern "C" fn s32g_pcie_parse_port(s32g_pp: *mut s32g_pcie, node: *mut device_node) -> c_int {
    static int s32g_pcie_parse_port(struct s32g_pcie *s32g_pp, struct device_node *node)
    {
    struct device *dev = s32g_pp.pci.dev;
    struct s32g_pcie_port *port;
    int num_lanes;
    port = devm_kzalloc(dev, sizeof(*port), GFP_KERNEL);
    if (!port)
    return -ENOMEM;
    port.phy = devm_of_phy_get(dev, node, core::ptr::null_mut());
    if (IS_ERR(port.phy))
    return dev_err_probe(dev, PTR_ERR(port.phy),
    "Failed to get serdes PHY\n");
    INIT_LIST_HEAD(&port.list);
    list_add_tail(&port.list, &s32g_pp.ports);
//
// The DWC core initialization code cannot yet parse the num-lanes
// attribute in the Root Port node. The S32G only supports one Root
// Port for now so its driver can parse the node and set the num_lanes
// field of struct dwc_pcie before calling dw_pcie_host_init().
//
    if (!of_property_read_u32(node, "num-lanes", &num_lanes))
    s32g_pp.pci.num_lanes = num_lanes;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s32g_pcie_parse_ports(dev: *mut device, s32g_pp: *mut s32g_pcie) -> c_int {
    static int s32g_pcie_parse_ports(struct device *dev, struct s32g_pcie *s32g_pp)
    {
    struct s32g_pcie_port *port, *tmp;
    let mut ret: c_int = -ENOENT;
    for_each_available_child_of_node_scoped(dev.of_node, of_port) {
    if (!of_node_is_type(of_port, "pci"))
    continue;
    ret = s32g_pcie_parse_port(s32g_pp, of_port);
    if (ret)
    break;
    }
    if (ret)
    list_for_each_entry_safe(port, tmp, &s32g_pp.ports, list)
    list_del(&port.list);
    return ret;
    }
    static int s32g_pcie_get_resources(struct platform_device *pdev,
    struct s32g_pcie *s32g_pp)
    {
    struct dw_pcie *pci = &s32g_pp.pci;
    struct device *dev = &pdev.dev;
    int ret;
    pci.dev = dev;
    pci.ops = &s32g_pcie_ops;
    s32g_pp.ctrl_base = devm_platform_ioremap_resource_byname(pdev, "ctrl");
    if (IS_ERR(s32g_pp.ctrl_base))
    return PTR_ERR(s32g_pp.ctrl_base);
    INIT_LIST_HEAD(&s32g_pp.ports);
    ret = s32g_pcie_parse_ports(dev, s32g_pp);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to parse Root Port: %d\n", ret);
    platform_set_drvdata(pdev, s32g_pp);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s32g_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int s32g_pcie_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct s32g_pcie *s32g_pp;
    struct dw_pcie_rp *pp;
    int ret;
    s32g_pp = devm_kzalloc(dev, sizeof(*s32g_pp), GFP_KERNEL);
    if (!s32g_pp)
    return -ENOMEM;
    ret = s32g_pcie_get_resources(pdev, s32g_pp);
    if (ret)
    return ret;
    pm_runtime_no_callbacks(dev);
    devm_pm_runtime_enable(dev);
    ret = pm_runtime_get_sync(dev);
    if (ret < 0)
    goto err_pm_runtime_put;
    ret = s32g_pcie_init(dev, s32g_pp);
    if (ret)
    goto err_pm_runtime_put;
    pp = &s32g_pp.pci.pp;
    pp.ops = &s32g_pcie_host_ops;
    pp.use_atu_msg = true;
    ret = dw_pcie_host_init(pp);
    if (ret)
    goto err_pcie_deinit;
    return 0;
    err_pcie_deinit:
    s32g_pcie_deinit(s32g_pp);
    err_pm_runtime_put:
    pm_runtime_put(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn s32g_pcie_suspend_noirq(dev: *mut device) -> c_int {
    static int s32g_pcie_suspend_noirq(struct device *dev)
    {
    struct s32g_pcie *s32g_pp = dev_get_drvdata(dev);
    struct dw_pcie *pci = &s32g_pp.pci;
    return dw_pcie_suspend_noirq(pci);
    }
#[no_mangle]
unsafe extern "C" fn s32g_pcie_resume_noirq(dev: *mut device) -> c_int {
    static int s32g_pcie_resume_noirq(struct device *dev)
    {
    struct s32g_pcie *s32g_pp = dev_get_drvdata(dev);
    struct dw_pcie *pci = &s32g_pp.pci;
    return dw_pcie_resume_noirq(pci);
    }
    static const struct dev_pm_ops s32g_pcie_pm_ops = {
    NOIRQ_SYSTEM_SLEEP_PM_OPS(s32g_pcie_suspend_noirq,
    s32g_pcie_resume_noirq)
    };
    static const struct of_device_id s32g_pcie_of_match[] = {
    { .compatible = "nxp,s32g2-pcie" },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, s32g_pcie_of_match);
    static struct platform_driver s32g_pcie_driver = {
    .driver = {
    .name	= "s32g-pcie",
    .of_match_table = s32g_pcie_of_match,
    .suppress_bind_attrs = true,
    .pm = pm_sleep_ptr(&s32g_pcie_pm_ops),
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe = s32g_pcie_probe,
    };
    builtin_platform_driver(s32g_pcie_driver);
    MODULE_AUTHOR("Ionut Vicovan <Ionut.Vicovan@nxp.com>");
    MODULE_DESCRIPTION("NXP S32G PCIe Host controller driver");
    MODULE_LICENSE("GPL");
