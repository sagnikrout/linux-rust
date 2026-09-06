//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/dwc/pcie-qcom-ep.c
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
// Qualcomm PCIe Endpoint controller driver
//
// Copyright (c) 2020, The Linux Foundation. All rights reserved.
// Author: Siddartha Mohanadoss <smohanad@codeaurora.org
//
// Copyright (c) 2021, Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org
//

// PARF registers
pub const PARF_SYS_CTRL: c_uint = 0x00;
pub const PARF_DB_CTRL: c_uint = 0x10;
pub const PARF_PM_CTRL: c_uint = 0x20;
pub const PARF_MHI_CLOCK_RESET_CTRL: c_uint = 0x174;
pub const PARF_MHI_BASE_ADDR_LOWER: c_uint = 0x178;
pub const PARF_MHI_BASE_ADDR_UPPER: c_uint = 0x17c;
pub const PARF_DEBUG_INT_EN: c_uint = 0x190;
pub const PARF_AXI_MSTR_RD_HALT_NO_WRITES: c_uint = 0x1a4;
pub const PARF_AXI_MSTR_WR_ADDR_HALT: c_uint = 0x1a8;
pub const PARF_Q2A_FLUSH: c_uint = 0x1ac;
pub const PARF_LTSSM: c_uint = 0x1b0;
pub const PARF_CFG_BITS: c_uint = 0x210;
pub const PARF_INT_ALL_STATUS: c_uint = 0x224;
pub const PARF_INT_ALL_CLEAR: c_uint = 0x228;
pub const PARF_INT_ALL_MASK: c_uint = 0x22c;
pub const PARF_SLV_ADDR_MSB_CTRL: c_uint = 0x2c0;
pub const PARF_DBI_BASE_ADDR: c_uint = 0x350;
pub const PARF_DBI_BASE_ADDR_HI: c_uint = 0x354;
pub const PARF_SLV_ADDR_SPACE_SIZE: c_uint = 0x358;
pub const PARF_SLV_ADDR_SPACE_SIZE_HI: c_uint = 0x35c;
pub const PARF_NO_SNOOP_OVERRIDE: c_uint = 0x3d4;
pub const PARF_ATU_BASE_ADDR: c_uint = 0x634;
pub const PARF_ATU_BASE_ADDR_HI: c_uint = 0x638;
pub const PARF_SRIS_MODE: c_uint = 0x644;
pub const PARF_DEBUG_CNT_PM_LINKST_IN_L2: c_uint = 0xc04;
pub const PARF_DEBUG_CNT_PM_LINKST_IN_L1: c_uint = 0xc0c;
pub const PARF_DEBUG_CNT_PM_LINKST_IN_L0S: c_uint = 0xc10;
pub const PARF_DEBUG_CNT_AUX_CLK_IN_L1SUB_L1: c_uint = 0xc84;
pub const PARF_DEBUG_CNT_AUX_CLK_IN_L1SUB_L2: c_uint = 0xc88;
pub const PARF_DEVICE_TYPE: c_uint = 0x1000;
pub const PARF_BDF_TO_SID_CFG: c_uint = 0x2c00;
pub const PARF_INT_ALL_5_MASK: c_uint = 0x2dcc;
pub const PARF_INT_ALL_3_MASK: c_uint = 0x2e18;
// PARF_INT_ALL_{STATUS/CLEAR/MASK} register fields

// PARF_BDF_TO_SID_CFG register fields

// PARF_DEBUG_INT_EN register fields

// PARF_NO_SNOOP_OVERRIDE register fields

// PARF_DEVICE_TYPE register fields
pub const PARF_DEVICE_TYPE_EP: c_uint = 0x0;
// PARF_PM_CTRL register fields

// PARF_MHI_CLOCK_RESET_CTRL fields

// PARF_AXI_MSTR_RD_HALT_NO_WRITES register fields

// PARF_AXI_MSTR_WR_ADDR_HALT register fields

// PARF_Q2A_FLUSH register fields

// PARF_SYS_CTRL register fields

// PARF_DB_CTRL register fields

// PARF_CFG_BITS register fields

// PARF_INT_ALL_5_MASK fields

// PARF_INT_ALL_3_MASK fields

// ELBI registers
pub const ELBI_SYS_STTS: c_uint = 0x08;
pub const ELBI_CS2_ENABLE: c_uint = 0xa4;
// DBI registers
pub const DBI_CON_STATUS: c_uint = 0x44;
// DBI register fields

pub const XMLH_LINK_UP: c_uint = 0x400;
pub const CORE_RESET_TIME_US_MIN: c_int = 1000;
pub const CORE_RESET_TIME_US_MAX: c_int = 1005;

    Mbps_to_icc(PCIE_SPEED2MBS_ENC(pcie_get_link_speed(speed)))

    enum qcom_pcie_ep_link_status {
    QCOM_PCIE_EP_LINK_DISABLED,
    QCOM_PCIE_EP_LINK_ENABLED,
    QCOM_PCIE_EP_LINK_UP,
    QCOM_PCIE_EP_LINK_DOWN,
    };
//
// struct qcom_pcie_ep_cfg - Per SoC config struct
// @hdma_support: HDMA support on this SoC
// @override_no_snoop: Override NO_SNOOP attribute in TLP to enable cache snooping
// @disable_mhi_ram_parity_check: Disable MHI RAM data parity error check
// @firmware_managed: Set if the controller is firmware managed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pcie_ep_cfg {
    pub hdma_support: bool,
    pub override_no_snoop: bool,
    pub disable_mhi_ram_parity_check: bool,
    pub firmware_managed: bool,
}

//
// struct qcom_pcie_ep - Qualcomm PCIe Endpoint Controller
// @pci: Designware PCIe controller struct
// @parf: Qualcomm PCIe specific PARF register base
// @mmio: MMIO register base
// @perst_map: PERST regmap
// @mmio_res: MMIO region resource
// @core_reset: PCIe Endpoint core reset
// @reset: PERST# GPIO
// @wake: WAKE# GPIO
// @phy: PHY controller block
// @debugfs: PCIe Endpoint Debugfs directory
// @icc_mem: Handle to an interconnect path between PCIe and MEM
// @clks: PCIe clocks
// @num_clks: PCIe clocks count
// @perst_en: Flag for PERST enable
// @perst_sep_en: Flag for PERST separation enable
// @cfg: PCIe EP config struct
// @link_status: PCIe Link status
// @global_irq: Qualcomm PCIe specific Global IRQ
// @perst_irq: PERST# IRQ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pcie_ep {
    pub pci: dw_pcie,
    pub parf: *mut void __iomem,
    pub mmio: *mut void __iomem,
    pub perst_map: *mut regmap,
    pub mmio_res: *mut resource,
    pub core_reset: *mut reset_control,
    pub reset: *mut gpio_desc,
    pub wake: *mut gpio_desc,
    pub phy: *mut phy,
    pub debugfs: *mut dentry,
    pub icc_mem: *mut icc_path,
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub perst_en: u32,
    pub perst_sep_en: u32,
    pub cfg: *const qcom_pcie_ep_cfg,
    pub link_status: enum qcom_pcie_ep_link_status,
    pub global_irq: c_int,
    pub perst_irq: c_int,
}

#[no_mangle]
unsafe extern "C" fn qcom_pcie_ep_core_reset(pcie_ep: *mut qcom_pcie_ep) -> c_int {
    static int qcom_pcie_ep_core_reset(struct qcom_pcie_ep *pcie_ep)
    {
    struct dw_pcie *pci = &pcie_ep.pci;
    struct device *dev = pci.dev;
    int ret;
    ret = reset_control_assert(pcie_ep.core_reset);
    if (ret) {
    dev_err(dev, "Cannot assert core reset\n");
    return ret;
    }
    usleep_range(CORE_RESET_TIME_US_MIN, CORE_RESET_TIME_US_MAX);
    ret = reset_control_deassert(pcie_ep.core_reset);
    if (ret) {
    dev_err(dev, "Cannot de-assert core reset\n");
    return ret;
    }
    usleep_range(CORE_RESET_TIME_US_MIN, CORE_RESET_TIME_US_MAX);
    return 0;
    }
//
// Delatch PERST_EN and PERST_SEPARATION_ENABLE with TCSR to avoid
// device reset during host reboot and hibernation. The driver is
// expected to handle this situation.
//
#[no_mangle]
unsafe extern "C" fn qcom_pcie_ep_configure_tcsr(pcie_ep: *mut qcom_pcie_ep) {
    static void qcom_pcie_ep_configure_tcsr(struct qcom_pcie_ep *pcie_ep)
    {
    if (pcie_ep.perst_map) {
    regmap_write(pcie_ep.perst_map, pcie_ep.perst_en, 0);
    regmap_write(pcie_ep.perst_map, pcie_ep.perst_sep_en, 0);
    }
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_dw_link_up(pci: *mut dw_pcie) -> bool {
    static bool qcom_pcie_dw_link_up(struct dw_pcie *pci)
    {
    u32 reg;
    reg = readl_relaxed(pci.elbi_base + ELBI_SYS_STTS);
    return reg & XMLH_LINK_UP;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_dw_start_link(pci: *mut dw_pcie) -> c_int {
    static int qcom_pcie_dw_start_link(struct dw_pcie *pci)
    {
    struct qcom_pcie_ep *pcie_ep = to_pcie_ep(pci);
    enable_irq(pcie_ep.perst_irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_dw_stop_link(pci: *mut dw_pcie) {
    static void qcom_pcie_dw_stop_link(struct dw_pcie *pci)
    {
    struct qcom_pcie_ep *pcie_ep = to_pcie_ep(pci);
    disable_irq(pcie_ep.perst_irq);
    }
    static void qcom_pcie_dw_write_dbi2(struct dw_pcie *pci, void __iomem *base,
    u32 reg, size_t size, u32 val)
    {
    int ret;
    writel(1, pci.elbi_base + ELBI_CS2_ENABLE);
    ret = dw_pcie_write(pci.dbi_base2 + reg, size, val);
    if (ret)
    dev_err(pci.dev, "Failed to write DBI2 register (0x%x): %d\n", reg, ret);
    writel(0, pci.elbi_base + ELBI_CS2_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_ep_icc_update(pcie_ep: *mut qcom_pcie_ep) {
    static void qcom_pcie_ep_icc_update(struct qcom_pcie_ep *pcie_ep)
    {
    struct dw_pcie *pci = &pcie_ep.pci;
    u32 offset, status;
    int speed, width;
    int ret;
    if (!pcie_ep.icc_mem)
    return;
    offset = dw_pcie_find_capability(pci, PCI_CAP_ID_EXP);
    status = readw(pci.dbi_base + offset + PCI_EXP_LNKSTA);
    speed = FIELD_GET(PCI_EXP_LNKSTA_CLS, status);
    width = FIELD_GET(PCI_EXP_LNKSTA_NLW, status);
    ret = icc_set_bw(pcie_ep.icc_mem, 0, width * QCOM_PCIE_LINK_SPEED_TO_BW(speed));
    if (ret)
    dev_err(pci.dev, "failed to set interconnect bandwidth: %d\n",
    ret);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_enable_resources(pcie_ep: *mut qcom_pcie_ep) -> c_int {
    static int qcom_pcie_enable_resources(struct qcom_pcie_ep *pcie_ep)
    {
    struct dw_pcie *pci = &pcie_ep.pci;
    int ret;
    ret = clk_bulk_prepare_enable(pcie_ep.num_clks, pcie_ep.clks);
    if (ret)
    return ret;
    ret = qcom_pcie_ep_core_reset(pcie_ep);
    if (ret)
    goto err_disable_clk;
    ret = phy_init(pcie_ep.phy);
    if (ret)
    goto err_disable_clk;
    ret = phy_set_mode_ext(pcie_ep.phy, PHY_MODE_PCIE, PHY_MODE_PCIE_EP);
    if (ret)
    goto err_phy_exit;
    ret = phy_power_on(pcie_ep.phy);
    if (ret)
    goto err_phy_exit;
//
// Some Qualcomm platforms require interconnect bandwidth constraints
// to be set before enabling interconnect clocks.
//
// Set an initial peak bandwidth corresponding to single-lane Gen 1
// for the pcie-mem path.
//
    ret = icc_set_bw(pcie_ep.icc_mem, 0, QCOM_PCIE_LINK_SPEED_TO_BW(1));
    if (ret) {
    dev_err(pci.dev, "failed to set interconnect bandwidth: %d\n",
    ret);
    goto err_phy_off;
    }
    return 0;
    err_phy_off:
    phy_power_off(pcie_ep.phy);
    err_phy_exit:
    phy_exit(pcie_ep.phy);
    err_disable_clk:
    clk_bulk_disable_unprepare(pcie_ep.num_clks, pcie_ep.clks);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_disable_resources(pcie_ep: *mut qcom_pcie_ep) {
    static void qcom_pcie_disable_resources(struct qcom_pcie_ep *pcie_ep)
    {
    struct device *dev = pcie_ep.pci.dev;
    pm_runtime_put(dev);
// Skip resource disablement if controller is firmware-managed
    if (pcie_ep.cfg && pcie_ep.cfg.firmware_managed)
    return;
    icc_set_bw(pcie_ep.icc_mem, 0, 0);
    phy_power_off(pcie_ep.phy);
    phy_exit(pcie_ep.phy);
    clk_bulk_disable_unprepare(pcie_ep.num_clks, pcie_ep.clks);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_perst_deassert(pci: *mut dw_pcie) -> c_int {
    static int qcom_pcie_perst_deassert(struct dw_pcie *pci)
    {
    struct qcom_pcie_ep *pcie_ep = to_pcie_ep(pci);
    struct device *dev = pci.dev;
    u32 val, offset;
    int ret;
    ret = pm_runtime_resume_and_get(dev);
    if (ret < 0) {
    dev_err(dev, "Failed to enable device: %d\n", ret);
    return ret;
    }
// Skip resource enablement if controller is firmware-managed
    if (pcie_ep.cfg && pcie_ep.cfg.firmware_managed)
    goto skip_resources_enable;
    ret = qcom_pcie_enable_resources(pcie_ep);
    if (ret) {
    dev_err(dev, "Failed to enable resources: %d\n", ret);
    pm_runtime_put(dev);
    return ret;
    }
    skip_resources_enable:
// Perform cleanup that requires refclk
    pci_epc_deinit_notify(pci.ep.epc);
    dw_pcie_ep_cleanup(&pci.ep);
// Assert WAKE# to RC to indicate device is ready
    gpiod_set_value_cansleep(pcie_ep.wake, 1);
    usleep_range(WAKE_DELAY_US, WAKE_DELAY_US + 500);
    gpiod_set_value_cansleep(pcie_ep.wake, 0);
    qcom_pcie_ep_configure_tcsr(pcie_ep);
// Disable BDF to SID mapping
    val = readl_relaxed(pcie_ep.parf + PARF_BDF_TO_SID_CFG);
    val |= PARF_BDF_TO_SID_BYPASS;
    writel_relaxed(val, pcie_ep.parf + PARF_BDF_TO_SID_CFG);
// Enable debug IRQ
    val = readl_relaxed(pcie_ep.parf + PARF_DEBUG_INT_EN);
    val |= PARF_DEBUG_INT_RADM_PM_TURNOFF |
    PARF_DEBUG_INT_CFG_BUS_MASTER_EN |
    PARF_DEBUG_INT_PM_DSTATE_CHANGE;
    writel_relaxed(val, pcie_ep.parf + PARF_DEBUG_INT_EN);
// Configure PCIe to endpoint mode
    writel_relaxed(PARF_DEVICE_TYPE_EP, pcie_ep.parf + PARF_DEVICE_TYPE);
// Allow entering L1 state
    val = readl_relaxed(pcie_ep.parf + PARF_PM_CTRL);
    val &= ~PARF_PM_CTRL_REQ_NOT_ENTR_L1;
    writel_relaxed(val, pcie_ep.parf + PARF_PM_CTRL);
// Read halts write
    val = readl_relaxed(pcie_ep.parf + PARF_AXI_MSTR_RD_HALT_NO_WRITES);
    val &= ~PARF_AXI_MSTR_RD_HALT_NO_WRITE_EN;
    writel_relaxed(val, pcie_ep.parf + PARF_AXI_MSTR_RD_HALT_NO_WRITES);
// Write after write halt
    val = readl_relaxed(pcie_ep.parf + PARF_AXI_MSTR_WR_ADDR_HALT);
    val |= PARF_AXI_MSTR_WR_ADDR_HALT_EN;
    writel_relaxed(val, pcie_ep.parf + PARF_AXI_MSTR_WR_ADDR_HALT);
// Q2A flush disable
    val = readl_relaxed(pcie_ep.parf + PARF_Q2A_FLUSH);
    val &= ~PARF_Q2A_FLUSH_EN;
    writel_relaxed(val, pcie_ep.parf + PARF_Q2A_FLUSH);
//
// Disable Master AXI clock during idle.  Do not allow DBI access
// to take the core out of L1.  Disable core clock gating that
// gates PIPE clock from propagating to core clock.  Report to the
// host that Vaux is present.
//
    val = readl_relaxed(pcie_ep.parf + PARF_SYS_CTRL);
    val &= ~PARF_SYS_CTRL_MSTR_ACLK_CGC_DIS;
    val |= PARF_SYS_CTRL_SLV_DBI_WAKE_DISABLE |
    PARF_SYS_CTRL_CORE_CLK_CGC_DIS |
    PARF_SYS_CTRL_AUX_PWR_DET;
    writel_relaxed(val, pcie_ep.parf + PARF_SYS_CTRL);
// Disable the debouncers
    val = readl_relaxed(pcie_ep.parf + PARF_DB_CTRL);
    val |= PARF_DB_CTRL_INSR_DBNCR_BLOCK | PARF_DB_CTRL_RMVL_DBNCR_BLOCK |
    PARF_DB_CTRL_DBI_WKP_BLOCK | PARF_DB_CTRL_SLV_WKP_BLOCK |
    PARF_DB_CTRL_MST_WKP_BLOCK;
    writel_relaxed(val, pcie_ep.parf + PARF_DB_CTRL);
// Request to exit from L1SS for MSI and LTR MSG
    val = readl_relaxed(pcie_ep.parf + PARF_CFG_BITS);
    val |= PARF_CFG_BITS_REQ_EXIT_L1SS_MSI_LTR_EN;
    writel_relaxed(val, pcie_ep.parf + PARF_CFG_BITS);
    dw_pcie_dbi_ro_wr_en(pci);
// Set the L0s Exit Latency to 2us-4us = 0x6
    offset = dw_pcie_find_capability(pci, PCI_CAP_ID_EXP);
    val = dw_pcie_readl_dbi(pci, offset + PCI_EXP_LNKCAP);
    FIELD_MODIFY(PCI_EXP_LNKCAP_L0SEL, &val, 0x6);
    dw_pcie_writel_dbi(pci, offset + PCI_EXP_LNKCAP, val);
// Set the L1 Exit Latency to be 32us-64 us = 0x6
    offset = dw_pcie_find_capability(pci, PCI_CAP_ID_EXP);
    val = dw_pcie_readl_dbi(pci, offset + PCI_EXP_LNKCAP);
    FIELD_MODIFY(PCI_EXP_LNKCAP_L1EL, &val, 0x6);
    dw_pcie_writel_dbi(pci, offset + PCI_EXP_LNKCAP, val);
    dw_pcie_dbi_ro_wr_dis(pci);
    writel_relaxed(0, pcie_ep.parf + PARF_INT_ALL_MASK);
    val = PARF_INT_ALL_LINK_DOWN | PARF_INT_ALL_BME |
    PARF_INT_ALL_PM_TURNOFF | PARF_INT_ALL_DSTATE_CHANGE |
    PARF_INT_ALL_LINK_UP | PARF_INT_ALL_EDMA;
    writel_relaxed(val, pcie_ep.parf + PARF_INT_ALL_MASK);
    if (pcie_ep.cfg && pcie_ep.cfg.disable_mhi_ram_parity_check) {
    val = readl_relaxed(pcie_ep.parf + PARF_INT_ALL_5_MASK);
    val &= ~PARF_INT_ALL_5_MHI_RAM_DATA_PARITY_ERR;
    writel_relaxed(val, pcie_ep.parf + PARF_INT_ALL_5_MASK);
    }
    val = readl_relaxed(pcie_ep.parf + PARF_INT_ALL_3_MASK);
    val &= ~PARF_INT_ALL_3_PTM_UPDATING;
    writel_relaxed(val, pcie_ep.parf + PARF_INT_ALL_3_MASK);
    ret = dw_pcie_ep_init_registers(&pcie_ep.pci.ep);
    if (ret) {
    dev_err(dev, "Failed to complete initialization: %d\n", ret);
    goto err_disable_resources;
    }
    qcom_pcie_common_set_equalization(pci);
    if (pcie_get_link_speed(pci.max_link_speed) == PCIE_SPEED_16_0GT)
    qcom_pcie_common_set_16gt_lane_margining(pci);
//
// The physical address of the MMIO region which is exposed as the BAR
// should be written to MHI BASE registers.
//
    writel_relaxed(pcie_ep.mmio_res.start,
    pcie_ep.parf + PARF_MHI_BASE_ADDR_LOWER);
    writel_relaxed(0, pcie_ep.parf + PARF_MHI_BASE_ADDR_UPPER);
// Gate Master AXI clock to MHI bus during L1SS
    val = readl_relaxed(pcie_ep.parf + PARF_MHI_CLOCK_RESET_CTRL);
    val &= ~PARF_MSTR_AXI_CLK_EN;
    writel_relaxed(val, pcie_ep.parf + PARF_MHI_CLOCK_RESET_CTRL);
    pci_epc_init_notify(pcie_ep.pci.ep.epc);
// Enable LTSSM
    val = readl_relaxed(pcie_ep.parf + PARF_LTSSM);
    val |= BIT(8);
    writel_relaxed(val, pcie_ep.parf + PARF_LTSSM);
    if (pcie_ep.cfg && pcie_ep.cfg.override_no_snoop)
    writel_relaxed(WR_NO_SNOOP_OVERRIDE_EN | RD_NO_SNOOP_OVERRIDE_EN,
    pcie_ep.parf + PARF_NO_SNOOP_OVERRIDE);
    return 0;
    err_disable_resources:
    qcom_pcie_disable_resources(pcie_ep);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_perst_assert(pci: *mut dw_pcie) {
    static void qcom_pcie_perst_assert(struct dw_pcie *pci)
    {
    struct qcom_pcie_ep *pcie_ep = to_pcie_ep(pci);
    qcom_pcie_disable_resources(pcie_ep);
    pcie_ep.link_status = QCOM_PCIE_EP_LINK_DISABLED;
    }
// Common DWC controller ops
    static const struct dw_pcie_ops pci_ops = {
    .link_up = qcom_pcie_dw_link_up,
    .start_link = qcom_pcie_dw_start_link,
    .stop_link = qcom_pcie_dw_stop_link,
    .write_dbi2 = qcom_pcie_dw_write_dbi2,
    };
    static int qcom_pcie_ep_get_io_resources(struct platform_device *pdev,
    struct qcom_pcie_ep *pcie_ep)
    {
    struct device *dev = &pdev.dev;
    struct dw_pcie *pci = &pcie_ep.pci;
    struct device_node *syscon;
    struct resource *res;
    int ret;
    pcie_ep.parf = devm_platform_ioremap_resource_byname(pdev, "parf");
    if (IS_ERR(pcie_ep.parf))
    return PTR_ERR(pcie_ep.parf);
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "dbi");
    pci.dbi_base = devm_pci_remap_cfg_resource(dev, res);
    if (IS_ERR(pci.dbi_base))
    return PTR_ERR(pci.dbi_base);
    pci.dbi_base2 = pci.dbi_base;
    pcie_ep.mmio_res = platform_get_resource_byname(pdev, IORESOURCE_MEM,
    "mmio");
    if (!pcie_ep.mmio_res) {
    dev_err(dev, "Failed to get mmio resource\n");
    return -EINVAL;
    }
    pcie_ep.mmio = devm_pci_remap_cfg_resource(dev, pcie_ep.mmio_res);
    if (IS_ERR(pcie_ep.mmio))
    return PTR_ERR(pcie_ep.mmio);
    syscon = of_parse_phandle(dev.of_node, "qcom,perst-regs", 0);
    if (!syscon) {
    dev_dbg(dev, "PERST separation not available\n");
    return 0;
    }
    pcie_ep.perst_map = syscon_node_to_regmap(syscon);
    of_node_put(syscon);
    if (IS_ERR(pcie_ep.perst_map))
    return PTR_ERR(pcie_ep.perst_map);
    ret = of_property_read_u32_index(dev.of_node, "qcom,perst-regs",
    1, &pcie_ep.perst_en);
    if (ret < 0) {
    dev_err(dev, "No Perst Enable offset in syscon\n");
    return ret;
    }
    ret = of_property_read_u32_index(dev.of_node, "qcom,perst-regs",
    2, &pcie_ep.perst_sep_en);
    if (ret < 0) {
    dev_err(dev, "No Perst Separation Enable offset in syscon\n");
    return ret;
    }
    return 0;
    }
    static int qcom_pcie_ep_get_resources(struct platform_device *pdev,
    struct qcom_pcie_ep *pcie_ep)
    {
    struct device *dev = &pdev.dev;
    int ret;
    ret = qcom_pcie_ep_get_io_resources(pdev, pcie_ep);
    if (ret) {
    dev_err(dev, "Failed to get io resources %d\n", ret);
    return ret;
    }
    pcie_ep.reset = devm_gpiod_get(dev, "reset", GPIOD_IN);
    if (IS_ERR(pcie_ep.reset))
    return PTR_ERR(pcie_ep.reset);
    pcie_ep.wake = devm_gpiod_get_optional(dev, "wake", GPIOD_OUT_LOW);
    if (IS_ERR(pcie_ep.wake))
    return PTR_ERR(pcie_ep.wake);
    if (pcie_ep.cfg && pcie_ep.cfg.firmware_managed)
    return 0;
    pcie_ep.num_clks = devm_clk_bulk_get_all(dev, &pcie_ep.clks);
    if (pcie_ep.num_clks < 0) {
    dev_err(dev, "Failed to get clocks\n");
    return pcie_ep.num_clks;
    }
    pcie_ep.core_reset = devm_reset_control_get_exclusive(dev, "core");
    if (IS_ERR(pcie_ep.core_reset))
    return PTR_ERR(pcie_ep.core_reset);
    pcie_ep.phy = devm_phy_optional_get(dev, "pciephy");
    if (IS_ERR(pcie_ep.phy))
    ret = PTR_ERR(pcie_ep.phy);
    pcie_ep.icc_mem = devm_of_icc_get(dev, "pcie-mem");
    if (IS_ERR(pcie_ep.icc_mem))
    ret = PTR_ERR(pcie_ep.icc_mem);
    return ret;
    }
// TODO: Notify clients about PCIe state change
#[no_mangle]
unsafe extern "C" fn qcom_pcie_ep_global_irq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t qcom_pcie_ep_global_irq_thread(int irq, void *data)
    {
    struct qcom_pcie_ep *pcie_ep = data;
    struct dw_pcie *pci = &pcie_ep.pci;
    struct device *dev = pci.dev;
    let mut status: u32 = readl_relaxed(pcie_ep.parf + PARF_INT_ALL_STATUS);
    u32 dstate, val;
    writel_relaxed(status, pcie_ep.parf + PARF_INT_ALL_CLEAR);
    if (FIELD_GET(PARF_INT_ALL_LINK_DOWN, status)) {
    dev_dbg(dev, "Received Linkdown event\n");
    pcie_ep.link_status = QCOM_PCIE_EP_LINK_DOWN;
    dw_pcie_ep_linkdown(&pci.ep);
    } else if (FIELD_GET(PARF_INT_ALL_BME, status)) {
    dev_dbg(dev, "Received Bus Master Enable event\n");
    pcie_ep.link_status = QCOM_PCIE_EP_LINK_ENABLED;
    qcom_pcie_ep_icc_update(pcie_ep);
    pci_epc_bus_master_enable_notify(pci.ep.epc);
    } else if (FIELD_GET(PARF_INT_ALL_PM_TURNOFF, status)) {
    dev_dbg(dev, "Received PM Turn-off event! Entering L23\n");
    val = readl_relaxed(pcie_ep.parf + PARF_PM_CTRL);
    val |= PARF_PM_CTRL_READY_ENTR_L23;
    writel_relaxed(val, pcie_ep.parf + PARF_PM_CTRL);
    } else if (FIELD_GET(PARF_INT_ALL_DSTATE_CHANGE, status)) {
    dstate = dw_pcie_readl_dbi(pci, DBI_CON_STATUS) &
    DBI_CON_STATUS_POWER_STATE_MASK;
    dev_dbg(dev, "Received D%d state event\n", dstate);
    if (dstate == 3) {
    val = readl_relaxed(pcie_ep.parf + PARF_PM_CTRL);
    val |= PARF_PM_CTRL_REQ_EXIT_L1;
    writel_relaxed(val, pcie_ep.parf + PARF_PM_CTRL);
    }
    } else if (FIELD_GET(PARF_INT_ALL_LINK_UP, status)) {
    dev_dbg(dev, "Received Linkup event. Enumeration complete!\n");
    dw_pcie_ep_linkup(&pci.ep);
    pcie_ep.link_status = QCOM_PCIE_EP_LINK_UP;
    } else {
    dev_WARN_ONCE(dev, 1, "Received unknown event. INT_STATUS: 0x%08x\n",
    status);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_ep_perst_irq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t qcom_pcie_ep_perst_irq_thread(int irq, void *data)
    {
    struct qcom_pcie_ep *pcie_ep = data;
    struct dw_pcie *pci = &pcie_ep.pci;
    struct device *dev = pci.dev;
    u32 perst;
    perst = gpiod_get_value(pcie_ep.reset);
    if (perst) {
    dev_dbg(dev, "PERST asserted by host. Shutting down the PCIe link!\n");
    qcom_pcie_perst_assert(pci);
    } else {
    dev_dbg(dev, "PERST de-asserted by host. Starting link training!\n");
    qcom_pcie_perst_deassert(pci);
    }
    irq_set_irq_type(gpiod_to_irq(pcie_ep.reset),
    (perst ? IRQF_TRIGGER_HIGH : IRQF_TRIGGER_LOW));
    return IRQ_HANDLED;
    }
    static int qcom_pcie_ep_enable_irq_resources(struct platform_device *pdev,
    struct qcom_pcie_ep *pcie_ep)
    {
    struct device *dev = pcie_ep.pci.dev;
    char *name;
    int ret;
    name = devm_kasprintf(dev, GFP_KERNEL, "qcom_pcie_ep_global_irq%d",
    pcie_ep.pci.ep.epc.domain_nr);
    if (!name)
    return -ENOMEM;
    pcie_ep.global_irq = platform_get_irq_byname(pdev, "global");
    if (pcie_ep.global_irq < 0)
    return pcie_ep.global_irq;
    ret = devm_request_threaded_irq(&pdev.dev, pcie_ep.global_irq, core::ptr::null_mut(),
    qcom_pcie_ep_global_irq_thread,
    IRQF_ONESHOT,
    name, pcie_ep);
    if (ret) {
    dev_err(&pdev.dev, "Failed to request Global IRQ\n");
    return ret;
    }
    name = devm_kasprintf(dev, GFP_KERNEL, "qcom_pcie_ep_perst_irq%d",
    pcie_ep.pci.ep.epc.domain_nr);
    if (!name)
    return -ENOMEM;
    pcie_ep.perst_irq = gpiod_to_irq(pcie_ep.reset);
    irq_set_status_flags(pcie_ep.perst_irq, IRQ_NOAUTOEN);
    ret = devm_request_threaded_irq(&pdev.dev, pcie_ep.perst_irq, core::ptr::null_mut(),
    qcom_pcie_ep_perst_irq_thread,
    IRQF_TRIGGER_HIGH | IRQF_ONESHOT,
    name, pcie_ep);
    if (ret) {
    dev_err(&pdev.dev, "Failed to request PERST IRQ\n");
    disable_irq(pcie_ep.global_irq);
    return ret;
    }
    return 0;
    }
    static int qcom_pcie_ep_raise_irq(struct dw_pcie_ep *ep, u8 func_no,
    unsigned int type, u16 interrupt_num)
    {
    struct dw_pcie *pci = to_dw_pcie_from_ep(ep);
    switch (type) {
    case PCI_IRQ_INTX:
    return dw_pcie_ep_raise_intx_irq(ep, func_no);
    case PCI_IRQ_MSI:
    return dw_pcie_ep_raise_msi_irq(ep, func_no, interrupt_num);
    default:
    dev_err(pci.dev, "Unknown IRQ type\n");
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_ep_link_transition_count(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int qcom_pcie_ep_link_transition_count(struct seq_file *s, void *data)
    {
    struct qcom_pcie_ep *pcie_ep = (struct qcom_pcie_ep *)
    dev_get_drvdata(s.private);
    seq_printf(s, "L0s transition count: %u\n",
    readl_relaxed(pcie_ep.mmio + PARF_DEBUG_CNT_PM_LINKST_IN_L0S));
    seq_printf(s, "L1 transition count: %u\n",
    readl_relaxed(pcie_ep.mmio + PARF_DEBUG_CNT_PM_LINKST_IN_L1));
    seq_printf(s, "L1.1 transition count: %u\n",
    readl_relaxed(pcie_ep.mmio + PARF_DEBUG_CNT_AUX_CLK_IN_L1SUB_L1));
    seq_printf(s, "L1.2 transition count: %u\n",
    readl_relaxed(pcie_ep.mmio + PARF_DEBUG_CNT_AUX_CLK_IN_L1SUB_L2));
    seq_printf(s, "L2 transition count: %u\n",
    readl_relaxed(pcie_ep.mmio + PARF_DEBUG_CNT_PM_LINKST_IN_L2));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_ep_init_debugfs(pcie_ep: *mut qcom_pcie_ep) {
    static void qcom_pcie_ep_init_debugfs(struct qcom_pcie_ep *pcie_ep)
    {
    struct dw_pcie *pci = &pcie_ep.pci;
    debugfs_create_devm_seqfile(pci.dev, "link_transition_count", pcie_ep.debugfs,
    qcom_pcie_ep_link_transition_count);
    }
    static const struct pci_epc_features qcom_pcie_epc_features = {
    DWC_EPC_COMMON_FEATURES,
    .linkup_notifier = true,
    .msi_capable = true,
    .align = SZ_4K,
    .bar[BAR_0] = { .only_64bit = true, },
    .bar[BAR_2] = { .only_64bit = true, },
    };
    static const struct pci_epc_features *
    qcom_pcie_epc_get_features(struct dw_pcie_ep *pci_ep)
    {
    return &qcom_pcie_epc_features;
    }
    static const struct dw_pcie_ep_ops pci_ep_ops = {
    .raise_irq = qcom_pcie_ep_raise_irq,
    .get_features = qcom_pcie_epc_get_features,
    };
#[no_mangle]
unsafe extern "C" fn qcom_pcie_ep_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_pcie_ep_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct qcom_pcie_ep *pcie_ep;
    char *name;
    int ret;
    pcie_ep = devm_kzalloc(dev, sizeof(*pcie_ep), GFP_KERNEL);
    if (!pcie_ep)
    return -ENOMEM;
    pcie_ep.pci.dev = dev;
    pcie_ep.pci.ops = &pci_ops;
    pcie_ep.pci.ep.ops = &pci_ep_ops;
    pcie_ep.cfg = of_device_get_match_data(dev);
    if (pcie_ep.cfg && pcie_ep.cfg.hdma_support) {
    pcie_ep.pci.edma.ll_wr_cnt = 8;
    pcie_ep.pci.edma.ll_rd_cnt = 8;
    pcie_ep.pci.edma.mf = EDMA_MF_HDMA_NATIVE;
    }
    platform_set_drvdata(pdev, pcie_ep);
    pm_runtime_get_noresume(dev);
    pm_runtime_set_active(dev);
    ret = devm_pm_runtime_enable(dev);
    if (ret)
    return ret;
    ret = qcom_pcie_ep_get_resources(pdev, pcie_ep);
    if (ret)
    return ret;
    ret = dw_pcie_ep_init(&pcie_ep.pci.ep);
    if (ret) {
    dev_err(dev, "Failed to initialize endpoint: %d\n", ret);
    return ret;
    }
    ret = qcom_pcie_ep_enable_irq_resources(pdev, pcie_ep);
    if (ret)
    goto err_ep_deinit;
    name = devm_kasprintf(dev, GFP_KERNEL, "%pOFP", dev.of_node);
    if (!name) {
    ret = -ENOMEM;
    goto err_disable_irqs;
    }
    ret = pm_runtime_put_sync(dev);
    if (ret < 0) {
    dev_err(dev, "Failed to suspend device: %d\n", ret);
    goto err_disable_irqs;
    }
    pcie_ep.debugfs = debugfs_create_dir(name, core::ptr::null_mut());
    qcom_pcie_ep_init_debugfs(pcie_ep);
    return 0;
    err_disable_irqs:
    disable_irq(pcie_ep.global_irq);
    disable_irq(pcie_ep.perst_irq);
    err_ep_deinit:
    dw_pcie_ep_deinit(&pcie_ep.pci.ep);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_ep_remove(pdev: *mut platform_device) {
    static void qcom_pcie_ep_remove(struct platform_device *pdev)
    {
    struct qcom_pcie_ep *pcie_ep = platform_get_drvdata(pdev);
    disable_irq(pcie_ep.global_irq);
    disable_irq(pcie_ep.perst_irq);
    debugfs_remove_recursive(pcie_ep.debugfs);
    if (pcie_ep.link_status == QCOM_PCIE_EP_LINK_DISABLED)
    return;
    qcom_pcie_disable_resources(pcie_ep);
    }
    static const struct qcom_pcie_ep_cfg cfg_1_34_0 = {
    .hdma_support = true,
    .override_no_snoop = true,
    .disable_mhi_ram_parity_check = true,
    };
    static const struct qcom_pcie_ep_cfg cfg_1_34_0_fw_managed = {
    .hdma_support = true,
    .override_no_snoop = true,
    .disable_mhi_ram_parity_check = true,
    .firmware_managed = true,
    };
    static const struct of_device_id qcom_pcie_ep_match[] = {
    { .compatible = "qcom,sa8255p-pcie-ep", .data = &cfg_1_34_0_fw_managed},
    { .compatible = "qcom,sa8775p-pcie-ep", .data = &cfg_1_34_0},
    { .compatible = "qcom,sdx55-pcie-ep", },
    { .compatible = "qcom,sm8450-pcie-ep", },
    { .compatible = "qcom,sar2130p-pcie-ep", },
    { }
    };
    MODULE_DEVICE_TABLE(of, qcom_pcie_ep_match);
    static struct platform_driver qcom_pcie_ep_driver = {
    .probe	= qcom_pcie_ep_probe,
    .remove = qcom_pcie_ep_remove,
    .driver	= {
    .name = "qcom-pcie-ep",
    .of_match_table	= qcom_pcie_ep_match,
    },
    };
    builtin_platform_driver(qcom_pcie_ep_driver);
    MODULE_AUTHOR("Siddartha Mohanadoss <smohanad@codeaurora.org>");
    MODULE_AUTHOR("Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>");
    MODULE_DESCRIPTION("Qualcomm PCIe Endpoint controller driver");
    MODULE_LICENSE("GPL v2");
