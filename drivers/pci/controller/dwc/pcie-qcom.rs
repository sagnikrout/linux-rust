//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/dwc/pcie-qcom.c
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
// Qualcomm PCIe root complex driver
//
// Copyright (c) 2014-2015, The Linux Foundation. All rights reserved.
// Copyright 2015 Linaro Limited.
//
// Author: Stanimir Varbanov <svarbanov@mm-sol.com>
//

// PARF registers
pub const PARF_SYS_CTRL: c_uint = 0x00;
pub const PARF_PM_CTRL: c_uint = 0x20;
pub const PARF_PCS_DEEMPH: c_uint = 0x34;
pub const PARF_PCS_SWING: c_uint = 0x38;
pub const PARF_PHY_CTRL: c_uint = 0x40;
pub const PARF_PHY_REFCLK: c_uint = 0x4c;
pub const PARF_CONFIG_BITS: c_uint = 0x50;
pub const PARF_DBI_BASE_ADDR: c_uint = 0x168;
pub const PARF_SLV_ADDR_SPACE_SIZE: c_uint = 0x16c;
pub const PARF_MHI_CLOCK_RESET_CTRL: c_uint = 0x174;
pub const PARF_AXI_MSTR_WR_ADDR_HALT: c_uint = 0x178;
pub const PARF_AXI_MSTR_WR_ADDR_HALT_V2: c_uint = 0x1a8;
pub const PARF_Q2A_FLUSH: c_uint = 0x1ac;
pub const PARF_LTSSM: c_uint = 0x1b0;
pub const PARF_INT_ALL_STATUS: c_uint = 0x224;
pub const PARF_INT_ALL_CLEAR: c_uint = 0x228;
pub const PARF_INT_ALL_MASK: c_uint = 0x22c;
pub const PARF_STATUS: c_uint = 0x230;
pub const PARF_SID_OFFSET: c_uint = 0x234;
pub const PARF_BDF_TRANSLATE_CFG: c_uint = 0x24c;
pub const PARF_DBI_BASE_ADDR_V2: c_uint = 0x350;
pub const PARF_DBI_BASE_ADDR_V2_HI: c_uint = 0x354;
pub const PARF_SLV_ADDR_SPACE_SIZE_V2: c_uint = 0x358;
pub const PARF_SLV_ADDR_SPACE_SIZE_V2_HI: c_uint = 0x35c;
pub const PARF_NO_SNOOP_OVERRIDE: c_uint = 0x3d4;
pub const PARF_ATU_BASE_ADDR: c_uint = 0x634;
pub const PARF_ATU_BASE_ADDR_HI: c_uint = 0x638;
pub const PARF_DEVICE_TYPE: c_uint = 0x1000;
pub const PARF_BDF_TO_SID_TABLE_N: c_uint = 0x2000;
pub const PARF_BDF_TO_SID_CFG: c_uint = 0x2c00;
// ELBI registers
pub const ELBI_SYS_CTRL: c_uint = 0x04;
pub const ELBI_SYS_STTS: c_uint = 0x08;
// DBI registers
pub const AXI_MSTR_RESP_COMP_CTRL0: c_uint = 0x818;
pub const AXI_MSTR_RESP_COMP_CTRL1: c_uint = 0x81c;
// MHI registers
pub const PARF_DEBUG_CNT_PM_LINKST_IN_L2: c_uint = 0xc04;
pub const PARF_DEBUG_CNT_PM_LINKST_IN_L1: c_uint = 0xc0c;
pub const PARF_DEBUG_CNT_PM_LINKST_IN_L0S: c_uint = 0xc10;
pub const PARF_DEBUG_CNT_AUX_CLK_IN_L1SUB_L1: c_uint = 0xc84;
pub const PARF_DEBUG_CNT_AUX_CLK_IN_L1SUB_L2: c_uint = 0xc88;
// PARF_SYS_CTRL register fields

// PARF_PM_CTRL register fields

// PARF_PCS_DEEMPH register fields

// PARF_PCS_SWING register fields

// PARF_PHY_CTRL register fields

// PARF_PHY_REFCLK register fields

// PARF_CONFIG_BITS register fields

// PARF_SLV_ADDR_SPACE_SIZE register value
pub const SLV_ADDR_SPACE_SZ: c_uint = 0x80000000;
// PARF_MHI_CLOCK_RESET_CTRL register fields

// PARF_AXI_MSTR_WR_ADDR_HALT register fields

// PARF_LTSSM register fields

// PARF_INT_ALL_{STATUS/CLEAR/MASK} register fields
pub const INT_ALL_LINK_DOWN: c_int = 1;

// PARF_NO_SNOOP_OVERRIDE register fields

// PARF_DEVICE_TYPE register fields
pub const DEVICE_TYPE_RC: c_uint = 0x4;
// PARF_BDF_TO_SID_CFG fields

// PARF_STATUS fields

// ELBI_SYS_CTRL register fields

// ELBI_SYS_STTS register fields

// AXI_MSTR_RESP_COMP_CTRL0 register fields
pub const CFG_REMOTE_RD_REQ_BRIDGE_SIZE_2K: c_uint = 0x4;
pub const CFG_REMOTE_RD_REQ_BRIDGE_SIZE_4K: c_uint = 0x5;
// AXI_MSTR_RESP_COMP_CTRL1 register fields

// PCI_EXP_SLTCAP register fields

    PCI_EXP_SLTCAP_PCP | \
    PCI_EXP_SLTCAP_MRLSP | \
    PCI_EXP_SLTCAP_AIP | \
    PCI_EXP_SLTCAP_PIP | \
    PCI_EXP_SLTCAP_HPS | \
    PCI_EXP_SLTCAP_EIP | \
    PCIE_CAP_SLOT_POWER_LIMIT_VAL | \
    PCIE_CAP_SLOT_POWER_LIMIT_SCALE)
pub const PERST_DELAY_US: c_int = 1000;
pub const FLUSH_TIMEOUT_US: c_int = 100;

    Mbps_to_icc(PCIE_SPEED2MBS_ENC(pcie_get_link_speed(speed)))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pcie_resources_1_0_0 {
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub core: *mut reset_control,
    pub vdda: *mut regulator,
}

pub const QCOM_PCIE_2_1_0_MAX_RESETS: c_int = 6;
pub const QCOM_PCIE_2_1_0_MAX_SUPPLY: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pcie_resources_2_1_0 {
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub resets: [reset_control_bulk_data; QCOM_PCIE_2_1_0_MAX_RESETS],
    pub num_resets: c_int,
    pub supplies: [regulator_bulk_data; QCOM_PCIE_2_1_0_MAX_SUPPLY],
}

pub const QCOM_PCIE_2_3_2_MAX_SUPPLY: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pcie_resources_2_3_2 {
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub supplies: [regulator_bulk_data; QCOM_PCIE_2_3_2_MAX_SUPPLY],
}

pub const QCOM_PCIE_2_3_3_MAX_RESETS: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pcie_resources_2_3_3 {
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub rst: [reset_control_bulk_data; QCOM_PCIE_2_3_3_MAX_RESETS],
}

pub const QCOM_PCIE_2_4_0_MAX_RESETS: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pcie_resources_2_4_0 {
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub resets: [reset_control_bulk_data; QCOM_PCIE_2_4_0_MAX_RESETS],
    pub num_resets: c_int,
}

pub const QCOM_PCIE_2_7_0_MAX_SUPPLIES: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pcie_resources_2_7_0 {
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub supplies: [regulator_bulk_data; QCOM_PCIE_2_7_0_MAX_SUPPLIES],
    pub rst: *mut reset_control,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pcie_resources_2_9_0 {
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub rst: *mut reset_control,
}

    union qcom_pcie_resources {
    struct qcom_pcie_resources_1_0_0 v1_0_0;
    struct qcom_pcie_resources_2_1_0 v2_1_0;
    struct qcom_pcie_resources_2_3_2 v2_3_2;
    struct qcom_pcie_resources_2_3_3 v2_3_3;
    struct qcom_pcie_resources_2_4_0 v2_4_0;
    struct qcom_pcie_resources_2_7_0 v2_7_0;
    struct qcom_pcie_resources_2_9_0 v2_9_0;
    };
    struct qcom_pcie;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pcie_ops {
    pub pcie): *mut *mut int (get_resources)(struct qcom_pcie,
    pub pcie): *mut *mut int (init)(struct qcom_pcie,
    pub pcie): *mut *mut int (post_init)(struct qcom_pcie,
    pub pcie): *mut *mut void (host_post_init)(struct qcom_pcie,
    pub pcie): *mut *mut void (deinit)(struct qcom_pcie,
    pub pcie): *mut *mut void (ltssm_enable)(struct qcom_pcie,
    pub pcie): *mut *mut int (config_sid)(struct qcom_pcie,
    pub pcie): *mut *mut enum dw_pcie_ltssm (get_ltssm)(struct qcom_pcie,
}

//
// struct qcom_pcie_cfg - Per SoC config struct
// @ops: qcom PCIe ops structure
// @override_no_snoop: Override NO_SNOOP attribute in TLP to enable cache
// snooping
// @firmware_managed: Set if the Root Complex is firmware managed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pcie_cfg {
    pub ops: *const qcom_pcie_ops,
    pub override_no_snoop: bool,
    pub firmware_managed: bool,
    pub no_l0s: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pcie_perst {
    pub list: list_head,
    pub desc: *mut gpio_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pcie_port {
    pub list: list_head,
    pub phy: *mut phy,
    pub l1ss_t_power_on: u32,
    pub perst: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pcie {
    pub pci: *mut dw_pcie,
    pub /: *mut *mut *mut void __iomem parf; / DT parf,
    pub mhi: *mut void __iomem,
    pub res: union qcom_pcie_resources,
    pub icc_mem: *mut icc_path,
    pub icc_cpu: *mut icc_path,
    pub cfg: *const qcom_pcie_cfg,
    pub debugfs: *mut dentry,
    pub ports: list_head,
    pub reset: *mut gpio_desc,
    pub global_irq: c_int,
    pub use_pm_opp: bool,
}

    static int qcom_pcie_reset_root_port(struct pci_host_bridge *bridge,
    struct pci_dev *pdev);
#[no_mangle]
unsafe extern "C" fn __qcom_pcie_perst_assert(pcie: *mut qcom_pcie, assert: bool) {
    static void __qcom_pcie_perst_assert(struct qcom_pcie *pcie, bool assert)
    {
    struct qcom_pcie_perst *perst;
    struct qcom_pcie_port *port;
    let mut val: c_int = assert ? 1 : 0;
    list_for_each_entry(port, &pcie.ports, list) {
    list_for_each_entry(perst, &port.perst, list)
    gpiod_set_value_cansleep(perst.desc, val);
    }
    usleep_range(PERST_DELAY_US, PERST_DELAY_US + 500);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_perst_assert(pcie: *mut qcom_pcie) {
    static void qcom_pcie_perst_assert(struct qcom_pcie *pcie)
    {
    __qcom_pcie_perst_assert(pcie, true);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_perst_deassert(pcie: *mut qcom_pcie) {
    static void qcom_pcie_perst_deassert(struct qcom_pcie *pcie)
    {
// Ensure that PERST# has been asserted for at least 100 ms
    msleep(PCIE_T_PVPERL_MS);
    __qcom_pcie_perst_assert(pcie, false);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_start_link(pci: *mut dw_pcie) -> c_int {
    static int qcom_pcie_start_link(struct dw_pcie *pci)
    {
    struct qcom_pcie *pcie = to_qcom_pcie(pci);
    qcom_pcie_common_set_equalization(pci);
    if (pcie_get_link_speed(pci.max_link_speed) == PCIE_SPEED_16_0GT)
    qcom_pcie_common_set_16gt_lane_margining(pci);
// Enable Link Training state machine
    if (pcie.cfg.ops.ltssm_enable)
    pcie.cfg.ops.ltssm_enable(pcie);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_clear_aspm_l0s(pci: *mut dw_pcie) {
    static void qcom_pcie_clear_aspm_l0s(struct dw_pcie *pci)
    {
    struct qcom_pcie *pcie = to_qcom_pcie(pci);
    u16 offset;
    u32 val;
    if (!pcie.cfg.no_l0s)
    return;
    offset = dw_pcie_find_capability(pci, PCI_CAP_ID_EXP);
    dw_pcie_dbi_ro_wr_en(pci);
    val = readl(pci.dbi_base + offset + PCI_EXP_LNKCAP);
    val &= ~PCI_EXP_LNKCAP_ASPM_L0S;
    writel(val, pci.dbi_base + offset + PCI_EXP_LNKCAP);
    dw_pcie_dbi_ro_wr_dis(pci);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_set_slot_cap(pci: *mut dw_pcie) {
    static void qcom_pcie_set_slot_cap(struct dw_pcie *pci)
    {
    let mut offset: u16 = dw_pcie_find_capability(pci, PCI_CAP_ID_EXP);
    u32 val;
    dw_pcie_dbi_ro_wr_en(pci);
//
// Qcom PCIe Root Ports do not support generating command completion
// notifications for the Hot-Plug commands. So set the NCCS field to
// avoid waiting for the completions.
//
    val = readl(pci.dbi_base + offset + PCI_EXP_SLTCAP);
    val |= PCI_EXP_SLTCAP_NCCS;
//
// Qcom PCIe Root Ports do not support Attention Button, so clear
// Attention Button Present in Slot Capabilities.
//
    val &= ~PCI_EXP_SLTCAP_ABP;
    writel(val, pci.dbi_base + offset + PCI_EXP_SLTCAP);
    dw_pcie_dbi_ro_wr_dis(pci);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_configure_dbi_base(pcie: *mut qcom_pcie) {
    static void qcom_pcie_configure_dbi_base(struct qcom_pcie *pcie)
    {
    struct dw_pcie *pci = pcie.pci;
    if (pci.dbi_phys_addr) {
//
// PARF_DBI_BASE_ADDR register is in CPU domain and require to
// be programmed with CPU physical address.
//
    writel(lower_32_bits(pci.dbi_phys_addr), pcie.parf +
    PARF_DBI_BASE_ADDR);
    writel(SLV_ADDR_SPACE_SZ, pcie.parf +
    PARF_SLV_ADDR_SPACE_SIZE);
    }
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_configure_dbi_atu_base(pcie: *mut qcom_pcie) {
    static void qcom_pcie_configure_dbi_atu_base(struct qcom_pcie *pcie)
    {
    struct dw_pcie *pci = pcie.pci;
    if (pci.dbi_phys_addr) {
//
// PARF_DBI_BASE_ADDR_V2 and PARF_ATU_BASE_ADDR registers are
// in CPU domain and require to be programmed with CPU
// physical addresses.
//
    writel(lower_32_bits(pci.dbi_phys_addr), pcie.parf +
    PARF_DBI_BASE_ADDR_V2);
    writel(upper_32_bits(pci.dbi_phys_addr), pcie.parf +
    PARF_DBI_BASE_ADDR_V2_HI);
    if (pci.atu_phys_addr) {
    writel(lower_32_bits(pci.atu_phys_addr), pcie.parf +
    PARF_ATU_BASE_ADDR);
    writel(upper_32_bits(pci.atu_phys_addr), pcie.parf +
    PARF_ATU_BASE_ADDR_HI);
    }
    writel(0x0, pcie.parf + PARF_SLV_ADDR_SPACE_SIZE_V2);
    writel(SLV_ADDR_SPACE_SZ, pcie.parf +
    PARF_SLV_ADDR_SPACE_SIZE_V2_HI);
    }
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_2_1_0_ltssm_enable(pcie: *mut qcom_pcie) {
    static void qcom_pcie_2_1_0_ltssm_enable(struct qcom_pcie *pcie)
    {
    struct dw_pcie *pci = pcie.pci;
    u32 val;
    if (!pci.elbi_base) {
    dev_err(pci.dev, "ELBI is not present\n");
    return;
    }
// enable link training
    val = readl(pci.elbi_base + ELBI_SYS_CTRL);
    val |= ELBI_SYS_CTRL_LT_ENABLE;
    writel(val, pci.elbi_base + ELBI_SYS_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_2_1_0_get_ltssm(pcie: *mut qcom_pcie) -> enum dw_pcie_ltssm {
    static enum dw_pcie_ltssm qcom_pcie_2_1_0_get_ltssm(struct qcom_pcie *pcie)
    {
    struct dw_pcie *pci = pcie.pci;
    u32 val;
    val = readl(pci.elbi_base + ELBI_SYS_STTS);
    return (enum dw_pcie_ltssm)FIELD_GET(ELBI_SYS_STTS_LTSSM_STATE_MASK, val);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_get_resources_2_1_0(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_get_resources_2_1_0(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_2_1_0 *res = &pcie.res.v2_1_0;
    struct dw_pcie *pci = pcie.pci;
    struct device *dev = pci.dev;
    let mut is_apq: bool = of_device_is_compatible(dev.of_node, "qcom,pcie-apq8064");
    int ret;
    res.supplies[0].supply = "vdda";
    res.supplies[1].supply = "vdda_phy";
    res.supplies[2].supply = "vdda_refclk";
    ret = devm_regulator_bulk_get(dev, ARRAY_SIZE(res.supplies),
    res.supplies);
    if (ret)
    return ret;
    res.num_clks = devm_clk_bulk_get_all(dev, &res.clks);
    if (res.num_clks < 0) {
    dev_err(dev, "Failed to get clocks\n");
    return res.num_clks;
    }
    res.resets[0].id = "pci";
    res.resets[1].id = "axi";
    res.resets[2].id = "ahb";
    res.resets[3].id = "por";
    res.resets[4].id = "phy";
    res.resets[5].id = "ext";
// ext is optional on APQ8016
    res.num_resets = is_apq ? 5 : 6;
    ret = devm_reset_control_bulk_get_exclusive(dev, res.num_resets, res.resets);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_deinit_2_1_0(pcie: *mut qcom_pcie) {
    static void qcom_pcie_deinit_2_1_0(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_2_1_0 *res = &pcie.res.v2_1_0;
    clk_bulk_disable_unprepare(res.num_clks, res.clks);
    reset_control_bulk_assert(res.num_resets, res.resets);
    writel(1, pcie.parf + PARF_PHY_CTRL);
    regulator_bulk_disable(ARRAY_SIZE(res.supplies), res.supplies);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_init_2_1_0(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_init_2_1_0(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_2_1_0 *res = &pcie.res.v2_1_0;
    struct dw_pcie *pci = pcie.pci;
    struct device *dev = pci.dev;
    int ret;
// reset the PCIe interface as uboot can leave it undefined state
    ret = reset_control_bulk_assert(res.num_resets, res.resets);
    if (ret < 0) {
    dev_err(dev, "cannot assert resets\n");
    return ret;
    }
    ret = regulator_bulk_enable(ARRAY_SIZE(res.supplies), res.supplies);
    if (ret < 0) {
    dev_err(dev, "cannot enable regulators\n");
    return ret;
    }
    ret = reset_control_bulk_deassert(res.num_resets, res.resets);
    if (ret < 0) {
    dev_err(dev, "cannot deassert resets\n");
    regulator_bulk_disable(ARRAY_SIZE(res.supplies), res.supplies);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_post_init_2_1_0(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_post_init_2_1_0(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_2_1_0 *res = &pcie.res.v2_1_0;
    struct dw_pcie *pci = pcie.pci;
    struct device *dev = pci.dev;
    struct device_node *node = dev.of_node;
    u32 val;
    int ret;
// Force PHY out of lowest power state
    val = readl(pcie.parf + PARF_PHY_CTRL);
    val &= ~PHY_TEST_PWR_DOWN;
    writel(val, pcie.parf + PARF_PHY_CTRL);
    ret = clk_bulk_prepare_enable(res.num_clks, res.clks);
    if (ret)
    return ret;
    if (of_device_is_compatible(node, "qcom,pcie-ipq8064") ||
    of_device_is_compatible(node, "qcom,pcie-ipq8064-v2")) {
    writel(PCS_DEEMPH_TX_DEEMPH_GEN1(24) |
    PCS_DEEMPH_TX_DEEMPH_GEN2_3_5DB(24) |
    PCS_DEEMPH_TX_DEEMPH_GEN2_6DB(34),
    pcie.parf + PARF_PCS_DEEMPH);
    writel(PCS_SWING_TX_SWING_FULL(120) |
    PCS_SWING_TX_SWING_LOW(120),
    pcie.parf + PARF_PCS_SWING);
    writel(PHY_RX0_EQ(4), pcie.parf + PARF_CONFIG_BITS);
    }
    if (of_device_is_compatible(node, "qcom,pcie-ipq8064")) {
// set TX termination offset
    val = readl(pcie.parf + PARF_PHY_CTRL);
    val &= ~PHY_CTRL_PHY_TX0_TERM_OFFSET_MASK;
    val |= PHY_CTRL_PHY_TX0_TERM_OFFSET(7);
    writel(val, pcie.parf + PARF_PHY_CTRL);
    }
// enable external reference clock
    val = readl(pcie.parf + PARF_PHY_REFCLK);
// USE_PAD is required only for ipq806x
    if (!of_device_is_compatible(node, "qcom,pcie-apq8064"))
    val &= ~PHY_REFCLK_USE_PAD;
    val |= PHY_REFCLK_SSP_EN;
    writel(val, pcie.parf + PARF_PHY_REFCLK);
// wait for clock acquisition
    usleep_range(1000, 1500);
// Set the Max TLP size to 2K, instead of using default of 4K
    writel(CFG_REMOTE_RD_REQ_BRIDGE_SIZE_2K,
    pci.dbi_base + AXI_MSTR_RESP_COMP_CTRL0);
    writel(CFG_BRIDGE_SB_INIT,
    pci.dbi_base + AXI_MSTR_RESP_COMP_CTRL1);
    qcom_pcie_set_slot_cap(pcie.pci);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_get_resources_1_0_0(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_get_resources_1_0_0(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_1_0_0 *res = &pcie.res.v1_0_0;
    struct dw_pcie *pci = pcie.pci;
    struct device *dev = pci.dev;
    res.vdda = devm_regulator_get(dev, "vdda");
    if (IS_ERR(res.vdda))
    return PTR_ERR(res.vdda);
    res.num_clks = devm_clk_bulk_get_all(dev, &res.clks);
    if (res.num_clks < 0) {
    dev_err(dev, "Failed to get clocks\n");
    return res.num_clks;
    }
    res.core = devm_reset_control_get_exclusive(dev, "core");
    return PTR_ERR_OR_ZERO(res.core);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_deinit_1_0_0(pcie: *mut qcom_pcie) {
    static void qcom_pcie_deinit_1_0_0(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_1_0_0 *res = &pcie.res.v1_0_0;
    reset_control_assert(res.core);
    clk_bulk_disable_unprepare(res.num_clks, res.clks);
    regulator_disable(res.vdda);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_init_1_0_0(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_init_1_0_0(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_1_0_0 *res = &pcie.res.v1_0_0;
    struct dw_pcie *pci = pcie.pci;
    struct device *dev = pci.dev;
    int ret;
    ret = reset_control_deassert(res.core);
    if (ret) {
    dev_err(dev, "cannot deassert core reset\n");
    return ret;
    }
    ret = clk_bulk_prepare_enable(res.num_clks, res.clks);
    if (ret) {
    dev_err(dev, "cannot prepare/enable clocks\n");
    goto err_assert_reset;
    }
    ret = regulator_enable(res.vdda);
    if (ret) {
    dev_err(dev, "cannot enable vdda regulator\n");
    goto err_disable_clks;
    }
    return 0;
    err_disable_clks:
    clk_bulk_disable_unprepare(res.num_clks, res.clks);
    err_assert_reset:
    reset_control_assert(res.core);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_post_init_1_0_0(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_post_init_1_0_0(struct qcom_pcie *pcie)
    {
    qcom_pcie_configure_dbi_base(pcie);
    if (IS_ENABLED(CONFIG_PCI_MSI)) {
    let mut val: u32 = readl(pcie.parf + PARF_AXI_MSTR_WR_ADDR_HALT);
    val |= EN;
    writel(val, pcie.parf + PARF_AXI_MSTR_WR_ADDR_HALT);
    }
    qcom_pcie_set_slot_cap(pcie.pci);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_2_3_2_ltssm_enable(pcie: *mut qcom_pcie) {
    static void qcom_pcie_2_3_2_ltssm_enable(struct qcom_pcie *pcie)
    {
    u32 val;
// enable link training
    val = readl(pcie.parf + PARF_LTSSM);
    val |= LTSSM_EN;
    writel(val, pcie.parf + PARF_LTSSM);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_get_resources_2_3_2(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_get_resources_2_3_2(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_2_3_2 *res = &pcie.res.v2_3_2;
    struct dw_pcie *pci = pcie.pci;
    struct device *dev = pci.dev;
    int ret;
    res.supplies[0].supply = "vdda";
    res.supplies[1].supply = "vddpe-3v3";
    ret = devm_regulator_bulk_get(dev, ARRAY_SIZE(res.supplies),
    res.supplies);
    if (ret)
    return ret;
    res.num_clks = devm_clk_bulk_get_all(dev, &res.clks);
    if (res.num_clks < 0) {
    dev_err(dev, "Failed to get clocks\n");
    return res.num_clks;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_deinit_2_3_2(pcie: *mut qcom_pcie) {
    static void qcom_pcie_deinit_2_3_2(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_2_3_2 *res = &pcie.res.v2_3_2;
    u32 val;
// Force PHY to lowest power state
    val = readl(pcie.parf + PARF_PHY_CTRL);
    val |= PHY_TEST_PWR_DOWN;
    writel(val, pcie.parf + PARF_PHY_CTRL);
    clk_bulk_disable_unprepare(res.num_clks, res.clks);
    regulator_bulk_disable(ARRAY_SIZE(res.supplies), res.supplies);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_init_2_3_2(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_init_2_3_2(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_2_3_2 *res = &pcie.res.v2_3_2;
    struct dw_pcie *pci = pcie.pci;
    struct device *dev = pci.dev;
    int ret;
    ret = regulator_bulk_enable(ARRAY_SIZE(res.supplies), res.supplies);
    if (ret < 0) {
    dev_err(dev, "cannot enable regulators\n");
    return ret;
    }
    ret = clk_bulk_prepare_enable(res.num_clks, res.clks);
    if (ret) {
    dev_err(dev, "cannot prepare/enable clocks\n");
    regulator_bulk_disable(ARRAY_SIZE(res.supplies), res.supplies);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_post_init_2_3_2(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_post_init_2_3_2(struct qcom_pcie *pcie)
    {
    u32 val;
// Force PHY out of lowest power state
    val = readl(pcie.parf + PARF_PHY_CTRL);
    val &= ~PHY_TEST_PWR_DOWN;
    writel(val, pcie.parf + PARF_PHY_CTRL);
    qcom_pcie_configure_dbi_base(pcie);
// MAC PHY_POWERDOWN MUX DISABLE
    val = readl(pcie.parf + PARF_SYS_CTRL);
    val &= ~MAC_PHY_POWERDOWN_IN_P2_D_MUX_EN;
    writel(val, pcie.parf + PARF_SYS_CTRL);
    val = readl(pcie.parf + PARF_MHI_CLOCK_RESET_CTRL);
    val |= BYPASS;
    writel(val, pcie.parf + PARF_MHI_CLOCK_RESET_CTRL);
    val = readl(pcie.parf + PARF_AXI_MSTR_WR_ADDR_HALT_V2);
    val |= EN;
    writel(val, pcie.parf + PARF_AXI_MSTR_WR_ADDR_HALT_V2);
    qcom_pcie_set_slot_cap(pcie.pci);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_get_resources_2_4_0(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_get_resources_2_4_0(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_2_4_0 *res = &pcie.res.v2_4_0;
    struct dw_pcie *pci = pcie.pci;
    struct device *dev = pci.dev;
    let mut is_ipq: bool = of_device_is_compatible(dev.of_node, "qcom,pcie-ipq4019");
    int ret;
    res.num_clks = devm_clk_bulk_get_all(dev, &res.clks);
    if (res.num_clks < 0) {
    dev_err(dev, "Failed to get clocks\n");
    return res.num_clks;
    }
    res.resets[0].id = "axi_m";
    res.resets[1].id = "axi_s";
    res.resets[2].id = "axi_m_sticky";
    res.resets[3].id = "pipe_sticky";
    res.resets[4].id = "pwr";
    res.resets[5].id = "ahb";
    res.resets[6].id = "pipe";
    res.resets[7].id = "axi_m_vmid";
    res.resets[8].id = "axi_s_xpu";
    res.resets[9].id = "parf";
    res.resets[10].id = "phy";
    res.resets[11].id = "phy_ahb";
    res.num_resets = is_ipq ? 12 : 6;
    ret = devm_reset_control_bulk_get_exclusive(dev, res.num_resets, res.resets);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_deinit_2_4_0(pcie: *mut qcom_pcie) {
    static void qcom_pcie_deinit_2_4_0(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_2_4_0 *res = &pcie.res.v2_4_0;
    u32 val;
// Force PHY to lowest power state
    val = readl(pcie.parf + PARF_PHY_CTRL);
    val |= PHY_TEST_PWR_DOWN;
    writel(val, pcie.parf + PARF_PHY_CTRL);
    reset_control_bulk_assert(res.num_resets, res.resets);
    clk_bulk_disable_unprepare(res.num_clks, res.clks);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_init_2_4_0(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_init_2_4_0(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_2_4_0 *res = &pcie.res.v2_4_0;
    struct dw_pcie *pci = pcie.pci;
    struct device *dev = pci.dev;
    int ret;
    ret = reset_control_bulk_assert(res.num_resets, res.resets);
    if (ret < 0) {
    dev_err(dev, "cannot assert resets\n");
    return ret;
    }
    usleep_range(10000, 12000);
    ret = reset_control_bulk_deassert(res.num_resets, res.resets);
    if (ret < 0) {
    dev_err(dev, "cannot deassert resets\n");
    return ret;
    }
    usleep_range(10000, 12000);
    ret = clk_bulk_prepare_enable(res.num_clks, res.clks);
    if (ret) {
    reset_control_bulk_assert(res.num_resets, res.resets);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_get_resources_2_3_3(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_get_resources_2_3_3(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_2_3_3 *res = &pcie.res.v2_3_3;
    struct dw_pcie *pci = pcie.pci;
    struct device *dev = pci.dev;
    int ret;
    res.num_clks = devm_clk_bulk_get_all(dev, &res.clks);
    if (res.num_clks < 0) {
    dev_err(dev, "Failed to get clocks\n");
    return res.num_clks;
    }
    res.rst[0].id = "axi_m";
    res.rst[1].id = "axi_s";
    res.rst[2].id = "pipe";
    res.rst[3].id = "axi_m_sticky";
    res.rst[4].id = "sticky";
    res.rst[5].id = "ahb";
    res.rst[6].id = "sleep";
    ret = devm_reset_control_bulk_get_exclusive(dev, ARRAY_SIZE(res.rst), res.rst);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_deinit_2_3_3(pcie: *mut qcom_pcie) {
    static void qcom_pcie_deinit_2_3_3(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_2_3_3 *res = &pcie.res.v2_3_3;
    u32 val;
// Force PHY to lowest power state
    val = readl(pcie.parf + PARF_PHY_CTRL);
    val |= PHY_TEST_PWR_DOWN;
    writel(val, pcie.parf + PARF_PHY_CTRL);
    clk_bulk_disable_unprepare(res.num_clks, res.clks);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_init_2_3_3(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_init_2_3_3(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_2_3_3 *res = &pcie.res.v2_3_3;
    struct dw_pcie *pci = pcie.pci;
    struct device *dev = pci.dev;
    int ret;
    ret = reset_control_bulk_assert(ARRAY_SIZE(res.rst), res.rst);
    if (ret < 0) {
    dev_err(dev, "cannot assert resets\n");
    return ret;
    }
    usleep_range(2000, 2500);
    ret = reset_control_bulk_deassert(ARRAY_SIZE(res.rst), res.rst);
    if (ret < 0) {
    dev_err(dev, "cannot deassert resets\n");
    return ret;
    }
//
// Don't have a way to see if the reset has completed.
// Wait for some time.
//
    usleep_range(2000, 2500);
    ret = clk_bulk_prepare_enable(res.num_clks, res.clks);
    if (ret) {
    dev_err(dev, "cannot prepare/enable clocks\n");
    goto err_assert_resets;
    }
    return 0;
    err_assert_resets:
//
// Not checking for failure, will anyway return
// the original failure in 'ret'.
//
    reset_control_bulk_assert(ARRAY_SIZE(res.rst), res.rst);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_post_init_2_3_3(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_post_init_2_3_3(struct qcom_pcie *pcie)
    {
    struct dw_pcie *pci = pcie.pci;
    let mut offset: u16 = dw_pcie_find_capability(pci, PCI_CAP_ID_EXP);
    u32 val;
// Force PHY out of lowest power state
    val = readl(pcie.parf + PARF_PHY_CTRL);
    val &= ~PHY_TEST_PWR_DOWN;
    writel(val, pcie.parf + PARF_PHY_CTRL);
    qcom_pcie_configure_dbi_atu_base(pcie);
    writel(MST_WAKEUP_EN | SLV_WAKEUP_EN | MSTR_ACLK_CGC_DIS
    | SLV_ACLK_CGC_DIS | CORE_CLK_CGC_DIS |
    AUX_PWR_DET | L23_CLK_RMV_DIS | L1_CLK_RMV_DIS,
    pcie.parf + PARF_SYS_CTRL);
    writel(0, pcie.parf + PARF_Q2A_FLUSH);
    writel(PCI_COMMAND_MASTER, pci.dbi_base + PCI_COMMAND);
    dw_pcie_dbi_ro_wr_en(pci);
    writel(PCIE_CAP_SLOT_VAL, pci.dbi_base + offset + PCI_EXP_SLTCAP);
    val = readl(pci.dbi_base + offset + PCI_EXP_LNKCAP);
    val &= ~PCI_EXP_LNKCAP_ASPMS;
    writel(val, pci.dbi_base + offset + PCI_EXP_LNKCAP);
    writel(PCI_EXP_DEVCTL2_COMP_TMOUT_DIS, pci.dbi_base + offset +
    PCI_EXP_DEVCTL2);
    dw_pcie_dbi_ro_wr_dis(pci);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_get_resources_2_7_0(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_get_resources_2_7_0(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_2_7_0 *res = &pcie.res.v2_7_0;
    struct dw_pcie *pci = pcie.pci;
    struct device *dev = pci.dev;
    int ret;
    res.rst = devm_reset_control_array_get_exclusive(dev);
    if (IS_ERR(res.rst))
    return PTR_ERR(res.rst);
    res.supplies[0].supply = "vdda";
    res.supplies[1].supply = "vddpe-3v3";
    ret = devm_regulator_bulk_get(dev, ARRAY_SIZE(res.supplies),
    res.supplies);
    if (ret)
    return ret;
    res.num_clks = devm_clk_bulk_get_all(dev, &res.clks);
    if (res.num_clks < 0) {
    dev_err(dev, "Failed to get clocks\n");
    return res.num_clks;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_init_2_7_0(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_init_2_7_0(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_2_7_0 *res = &pcie.res.v2_7_0;
    struct dw_pcie *pci = pcie.pci;
    struct device *dev = pci.dev;
    u32 val;
    int ret;
    ret = regulator_bulk_enable(ARRAY_SIZE(res.supplies), res.supplies);
    if (ret < 0) {
    dev_err(dev, "cannot enable regulators\n");
    return ret;
    }
    ret = clk_bulk_prepare_enable(res.num_clks, res.clks);
    if (ret < 0)
    goto err_disable_regulators;
    ret = reset_control_assert(res.rst);
    if (ret) {
    dev_err(dev, "reset assert failed (%d)\n", ret);
    goto err_disable_clocks;
    }
    usleep_range(1000, 1500);
    ret = reset_control_deassert(res.rst);
    if (ret) {
    dev_err(dev, "reset deassert failed (%d)\n", ret);
    goto err_disable_clocks;
    }
// Wait for reset to complete, required on SM8450
    usleep_range(1000, 1500);
// configure PCIe to RC mode
    writel(DEVICE_TYPE_RC, pcie.parf + PARF_DEVICE_TYPE);
// Force PHY out of lowest power state
    val = readl(pcie.parf + PARF_PHY_CTRL);
    val &= ~PHY_TEST_PWR_DOWN;
    writel(val, pcie.parf + PARF_PHY_CTRL);
    qcom_pcie_configure_dbi_atu_base(pcie);
// MAC PHY_POWERDOWN MUX DISABLE
    val = readl(pcie.parf + PARF_SYS_CTRL);
    val &= ~MAC_PHY_POWERDOWN_IN_P2_D_MUX_EN;
    writel(val, pcie.parf + PARF_SYS_CTRL);
    val = readl(pcie.parf + PARF_MHI_CLOCK_RESET_CTRL);
    val |= BYPASS;
    writel(val, pcie.parf + PARF_MHI_CLOCK_RESET_CTRL);
// Enable L1 and L1SS
    val = readl(pcie.parf + PARF_PM_CTRL);
    val &= ~REQ_NOT_ENTR_L1;
    writel(val, pcie.parf + PARF_PM_CTRL);
    pci.l1ss_support = true;
    val = readl(pcie.parf + PARF_AXI_MSTR_WR_ADDR_HALT_V2);
    val |= EN;
    writel(val, pcie.parf + PARF_AXI_MSTR_WR_ADDR_HALT_V2);
    return 0;
    err_disable_clocks:
    clk_bulk_disable_unprepare(res.num_clks, res.clks);
    err_disable_regulators:
    regulator_bulk_disable(ARRAY_SIZE(res.supplies), res.supplies);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_post_init_2_7_0(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_post_init_2_7_0(struct qcom_pcie *pcie)
    {
    const struct qcom_pcie_cfg *pcie_cfg = pcie.cfg;
    if (pcie_cfg.override_no_snoop)
    writel(WR_NO_SNOOP_OVERRIDE_EN | RD_NO_SNOOP_OVERRIDE_EN,
    pcie.parf + PARF_NO_SNOOP_OVERRIDE);
    qcom_pcie_set_slot_cap(pcie.pci);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_enable_aspm(pdev: *mut pci_dev, userdata: *mut c_void) -> c_int {
    static int qcom_pcie_enable_aspm(struct pci_dev *pdev, void *userdata)
    {
//
// Downstream devices need to be in D0 state before enabling PCI PM
// substates.
//
    pci_set_power_state_locked(pdev, PCI_D0);
    pci_enable_link_state_locked(pdev, PCIE_LINK_STATE_ALL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_host_post_init_2_7_0(pcie: *mut qcom_pcie) {
    static void qcom_pcie_host_post_init_2_7_0(struct qcom_pcie *pcie)
    {
    struct dw_pcie_rp *pp = &pcie.pci.pp;
    pci_walk_bus(pp.bridge.bus, qcom_pcie_enable_aspm, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_deinit_2_7_0(pcie: *mut qcom_pcie) {
    static void qcom_pcie_deinit_2_7_0(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_2_7_0 *res = &pcie.res.v2_7_0;
    u32 val;
// Force PHY to lowest power state
    val = readl(pcie.parf + PARF_PHY_CTRL);
    val |= PHY_TEST_PWR_DOWN;
    writel(val, pcie.parf + PARF_PHY_CTRL);
    clk_bulk_disable_unprepare(res.num_clks, res.clks);
    regulator_bulk_disable(ARRAY_SIZE(res.supplies), res.supplies);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_config_sid_1_9_0(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_config_sid_1_9_0(struct qcom_pcie *pcie)
    {
// iommu map structure
    struct {
    u32 bdf;
    u32 phandle;
    u32 smmu_sid;
    u32 smmu_sid_len;
    } *map;
    void __iomem *bdf_to_sid_base = pcie.parf + PARF_BDF_TO_SID_TABLE_N;
    struct device *dev = pcie.pci.dev;
    u8 qcom_pcie_crc8_table[CRC8_TABLE_SIZE];
    int i, nr_map, size = 0;
    u32 smmu_sid_base;
    u32 val;
    of_get_property(dev.of_node, "iommu-map", &size);
    if (!size)
    return 0;
// Enable BDF to SID translation by disabling bypass mode (default)
    val = readl(pcie.parf + PARF_BDF_TO_SID_CFG);
    val &= ~BDF_TO_SID_BYPASS;
    writel(val, pcie.parf + PARF_BDF_TO_SID_CFG);
    map = kzalloc(size, GFP_KERNEL);
    if (!map)
    return -ENOMEM;
    of_property_read_u32_array(dev.of_node, "iommu-map", (u32 *)map,
    size / sizeof(u32));
    nr_map = size / (sizeof(*map));
    crc8_populate_msb(qcom_pcie_crc8_table, QCOM_PCIE_CRC8_POLYNOMIAL);
// Registers need to be zero out first
    memset_io(bdf_to_sid_base, 0, CRC8_TABLE_SIZE * sizeof(u32));
// Extract the SMMU SID base from the first entry of iommu-map
    smmu_sid_base = map[0].smmu_sid;
// Look for an available entry to hold the mapping
    for (i = 0; i < nr_map; i++) {
    let mut bdf_be: __be16 = cpu_to_be16(map[i].bdf);
    u32 val;
    u8 hash;
    hash = crc8(qcom_pcie_crc8_table, (u8 *)&bdf_be, sizeof(bdf_be), 0);
    val = readl(bdf_to_sid_base + hash * sizeof(u32));
// If the register is already populated, look for next available entry
    while (val) {
    let mut current_hash: u8 = hash++;
    let mut next_mask: u8 = 0xff;
// If NEXT field is NULL then update it with next hash
    if (!(val & next_mask)) {
    val |= (u32)hash;
    writel(val, bdf_to_sid_base + current_hash * sizeof(u32));
    }
    val = readl(bdf_to_sid_base + hash * sizeof(u32));
    }
// BDF [31:16] | SID [15:8] | NEXT [7:0]
    val = map[i].bdf << 16 | (map[i].smmu_sid - smmu_sid_base) << 8 | 0;
    writel(val, bdf_to_sid_base + hash * sizeof(u32));
    }
    kfree(map);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_get_resources_2_9_0(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_get_resources_2_9_0(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_2_9_0 *res = &pcie.res.v2_9_0;
    struct dw_pcie *pci = pcie.pci;
    struct device *dev = pci.dev;
    res.num_clks = devm_clk_bulk_get_all(dev, &res.clks);
    if (res.num_clks < 0) {
    dev_err(dev, "Failed to get clocks\n");
    return res.num_clks;
    }
    res.rst = devm_reset_control_array_get_exclusive(dev);
    if (IS_ERR(res.rst))
    return PTR_ERR(res.rst);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_deinit_2_9_0(pcie: *mut qcom_pcie) {
    static void qcom_pcie_deinit_2_9_0(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_2_9_0 *res = &pcie.res.v2_9_0;
    u32 val;
// Force PHY to lowest power state
    val = readl(pcie.parf + PARF_PHY_CTRL);
    val |= PHY_TEST_PWR_DOWN;
    writel(val, pcie.parf + PARF_PHY_CTRL);
    clk_bulk_disable_unprepare(res.num_clks, res.clks);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_init_2_9_0(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_init_2_9_0(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_resources_2_9_0 *res = &pcie.res.v2_9_0;
    struct device *dev = pcie.pci.dev;
    int ret;
    ret = reset_control_assert(res.rst);
    if (ret) {
    dev_err(dev, "reset assert failed (%d)\n", ret);
    return ret;
    }
//
// Delay periods before and after reset deassert are working values
// from downstream Codeaurora kernel
//
    usleep_range(2000, 2500);
    ret = reset_control_deassert(res.rst);
    if (ret) {
    dev_err(dev, "reset deassert failed (%d)\n", ret);
    return ret;
    }
    usleep_range(2000, 2500);
    return clk_bulk_prepare_enable(res.num_clks, res.clks);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_post_init_2_9_0(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_post_init_2_9_0(struct qcom_pcie *pcie)
    {
    struct dw_pcie *pci = pcie.pci;
    let mut offset: u16 = dw_pcie_find_capability(pci, PCI_CAP_ID_EXP);
    u32 val;
    int i;
// Force PHY out of lowest power state
    val = readl(pcie.parf + PARF_PHY_CTRL);
    val &= ~PHY_TEST_PWR_DOWN;
    writel(val, pcie.parf + PARF_PHY_CTRL);
    qcom_pcie_configure_dbi_atu_base(pcie);
    writel(DEVICE_TYPE_RC, pcie.parf + PARF_DEVICE_TYPE);
    writel(BYPASS | MSTR_AXI_CLK_EN | AHB_CLK_EN,
    pcie.parf + PARF_MHI_CLOCK_RESET_CTRL);
    writel(GEN3_RELATED_OFF_RXEQ_RGRDLESS_RXTS |
    GEN3_RELATED_OFF_GEN3_ZRXDC_NONCOMPL,
    pci.dbi_base + GEN3_RELATED_OFF);
    writel(MST_WAKEUP_EN | SLV_WAKEUP_EN | MSTR_ACLK_CGC_DIS |
    SLV_ACLK_CGC_DIS | CORE_CLK_CGC_DIS |
    AUX_PWR_DET | L23_CLK_RMV_DIS | L1_CLK_RMV_DIS,
    pcie.parf + PARF_SYS_CTRL);
    writel(0, pcie.parf + PARF_Q2A_FLUSH);
    dw_pcie_dbi_ro_wr_en(pci);
    writel(PCIE_CAP_SLOT_VAL, pci.dbi_base + offset + PCI_EXP_SLTCAP);
    val = readl(pci.dbi_base + offset + PCI_EXP_LNKCAP);
    val &= ~PCI_EXP_LNKCAP_ASPMS;
    writel(val, pci.dbi_base + offset + PCI_EXP_LNKCAP);
    writel(PCI_EXP_DEVCTL2_COMP_TMOUT_DIS, pci.dbi_base + offset +
    PCI_EXP_DEVCTL2);
    dw_pcie_dbi_ro_wr_dis(pci);
    for (i = 0; i < 256; i++)
    writel(0, pcie.parf + PARF_BDF_TO_SID_TABLE_N + (4 * i));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_link_up(pci: *mut dw_pcie) -> bool {
    static bool qcom_pcie_link_up(struct dw_pcie *pci)
    {
    let mut offset: u16 = dw_pcie_find_capability(pci, PCI_CAP_ID_EXP);
    let mut val: u16 = readw(pci.dbi_base + offset + PCI_EXP_LNKSTA);
    return val & PCI_EXP_LNKSTA_DLLLA;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_get_ltssm(pci: *mut dw_pcie) -> enum dw_pcie_ltssm {
    static enum dw_pcie_ltssm qcom_pcie_get_ltssm(struct dw_pcie *pci)
    {
    struct qcom_pcie *pcie = to_qcom_pcie(pci);
    u32 val;
    if (pcie.cfg.ops.get_ltssm)
    return pcie.cfg.ops.get_ltssm(pcie);
    val = readl(pcie.parf + PARF_LTSSM);
    return (enum dw_pcie_ltssm)FIELD_GET(PARF_LTSSM_STATE_MASK, val);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_phy_power_off(pcie: *mut qcom_pcie) {
    static void qcom_pcie_phy_power_off(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_port *port;
    list_for_each_entry(port, &pcie.ports, list)
    phy_power_off(port.phy);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_phy_power_on(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_phy_power_on(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_port *port;
    int ret;
    list_for_each_entry(port, &pcie.ports, list) {
    ret = phy_set_mode_ext(port.phy, PHY_MODE_PCIE, PHY_MODE_PCIE_RC);
    if (ret)
    return ret;
    ret = phy_power_on(port.phy);
    if (ret) {
    qcom_pcie_phy_power_off(pcie);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_configure_ports(pcie: *mut qcom_pcie) {
    static void qcom_pcie_configure_ports(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_port *port;
    list_for_each_entry(port, &pcie.ports, list)
    dw_pcie_program_t_power_on(pcie.pci, port.l1ss_t_power_on);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_host_init(pp: *mut dw_pcie_rp) -> c_int {
    static int qcom_pcie_host_init(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct qcom_pcie *pcie = to_qcom_pcie(pci);
    int ret;
    qcom_pcie_perst_assert(pcie);
    ret = pcie.cfg.ops.init(pcie);
    if (ret)
    return ret;
    ret = qcom_pcie_phy_power_on(pcie);
    if (ret)
    goto err_deinit;
    if (!pci.suspended) {
    ret = pci_pwrctrl_create_devices(pci.dev);
    if (ret)
    goto err_disable_phy;
    }
    if (!pp.skip_pwrctrl_off) {
    ret = pci_pwrctrl_power_on_devices(pci.dev);
    if (ret)
    goto err_pwrctrl_destroy;
    }
    if (pcie.cfg.ops.post_init) {
    ret = pcie.cfg.ops.post_init(pcie);
    if (ret)
    goto err_pwrctrl_power_off;
    }
    qcom_pcie_clear_aspm_l0s(pcie.pci);
    dw_pcie_remove_capability(pcie.pci, PCI_CAP_ID_MSIX);
    dw_pcie_remove_ext_capability(pcie.pci, PCI_EXT_CAP_ID_DPC);
    qcom_pcie_configure_ports(pcie);
    qcom_pcie_perst_deassert(pcie);
    if (pcie.cfg.ops.config_sid) {
    ret = pcie.cfg.ops.config_sid(pcie);
    if (ret)
    goto err_assert_reset;
    }
    pp.bridge.reset_root_port = qcom_pcie_reset_root_port;
    return 0;
    err_assert_reset:
    qcom_pcie_perst_assert(pcie);
    err_pwrctrl_power_off:
    if (!pp.skip_pwrctrl_off)
    pci_pwrctrl_power_off_devices(pci.dev);
    err_pwrctrl_destroy:
    if (ret != -EPROBE_DEFER && !pci.suspended)
    pci_pwrctrl_destroy_devices(pci.dev);
    err_disable_phy:
    qcom_pcie_phy_power_off(pcie);
    err_deinit:
    pcie.cfg.ops.deinit(pcie);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_host_deinit(pp: *mut dw_pcie_rp) {
    static void qcom_pcie_host_deinit(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct qcom_pcie *pcie = to_qcom_pcie(pci);
    qcom_pcie_perst_assert(pcie);
    if (!pci.pp.skip_pwrctrl_off) {
//
// No need to destroy pwrctrl devices as this function only
// gets called during system suspend as of now.
//
    pci_pwrctrl_power_off_devices(pci.dev);
    }
    qcom_pcie_phy_power_off(pcie);
    pcie.cfg.ops.deinit(pcie);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_host_post_init(pp: *mut dw_pcie_rp) {
    static void qcom_pcie_host_post_init(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    struct qcom_pcie *pcie = to_qcom_pcie(pci);
//
// During system suspend, the Qcom RC driver may turn off the
// analog circuitry of PHY and remove controller votes to save
// power. If the link is in L1SS and the endpoint asserts CLKREQ#
// to exit L1SS, the time required to wake the system and restore
// the PHY/REFCLK may exceed the L1SS exit timing (L10_REFCLK_ON +
// T_COMMONMODE), resulting in Link Down (LDn) and a reset of the
// endpoint. Set this flag to indicate this limitation to client
// drivers so that they can avoid relying on device state being
// preserved during system suspend.
//
    pp.bridge.broken_l1ss_resume = true;
    if (pcie.cfg.ops.host_post_init)
    pcie.cfg.ops.host_post_init(pcie);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_host_pme_turn_off(pp: *mut dw_pcie_rp) {
    static void qcom_pcie_host_pme_turn_off(struct dw_pcie_rp *pp)
    {
    struct dw_pcie *pci = to_dw_pcie_from_pp(pp);
    writel(ELBI_SYS_CTRL_PME_TURNOFF_MSG, pci.elbi_base + ELBI_SYS_CTRL);
    }
    static const struct dw_pcie_host_ops qcom_pcie_dw_ops = {
    .init		= qcom_pcie_host_init,
    .deinit		= qcom_pcie_host_deinit,
    .post_init	= qcom_pcie_host_post_init,
    .pme_turn_off	= qcom_pcie_host_pme_turn_off,
    };
// Qcom IP rev.: 2.1.0	Synopsys IP rev.: 4.01a
    static const struct qcom_pcie_ops ops_2_1_0 = {
    .get_resources = qcom_pcie_get_resources_2_1_0,
    .init = qcom_pcie_init_2_1_0,
    .post_init = qcom_pcie_post_init_2_1_0,
    .deinit = qcom_pcie_deinit_2_1_0,
    .ltssm_enable = qcom_pcie_2_1_0_ltssm_enable,
    .get_ltssm = qcom_pcie_2_1_0_get_ltssm,
    };
// Qcom IP rev.: 1.0.0	Synopsys IP rev.: 4.11a
    static const struct qcom_pcie_ops ops_1_0_0 = {
    .get_resources = qcom_pcie_get_resources_1_0_0,
    .init = qcom_pcie_init_1_0_0,
    .post_init = qcom_pcie_post_init_1_0_0,
    .deinit = qcom_pcie_deinit_1_0_0,
    .ltssm_enable = qcom_pcie_2_1_0_ltssm_enable,
    .get_ltssm = qcom_pcie_2_1_0_get_ltssm,
    };
// Qcom IP rev.: 2.3.2	Synopsys IP rev.: 4.21a
    static const struct qcom_pcie_ops ops_2_3_2 = {
    .get_resources = qcom_pcie_get_resources_2_3_2,
    .init = qcom_pcie_init_2_3_2,
    .post_init = qcom_pcie_post_init_2_3_2,
    .deinit = qcom_pcie_deinit_2_3_2,
    .ltssm_enable = qcom_pcie_2_3_2_ltssm_enable,
    };
// Qcom IP rev.: 2.4.0	Synopsys IP rev.: 4.20a
    static const struct qcom_pcie_ops ops_2_4_0 = {
    .get_resources = qcom_pcie_get_resources_2_4_0,
    .init = qcom_pcie_init_2_4_0,
    .post_init = qcom_pcie_post_init_2_3_2,
    .deinit = qcom_pcie_deinit_2_4_0,
    .ltssm_enable = qcom_pcie_2_3_2_ltssm_enable,
    };
// Qcom IP rev.: 2.3.3	Synopsys IP rev.: 4.30a
    static const struct qcom_pcie_ops ops_2_3_3 = {
    .get_resources = qcom_pcie_get_resources_2_3_3,
    .init = qcom_pcie_init_2_3_3,
    .post_init = qcom_pcie_post_init_2_3_3,
    .deinit = qcom_pcie_deinit_2_3_3,
    .ltssm_enable = qcom_pcie_2_3_2_ltssm_enable,
    };
// Qcom IP rev.: 2.7.0	Synopsys IP rev.: 4.30a
    static const struct qcom_pcie_ops ops_2_7_0 = {
    .get_resources = qcom_pcie_get_resources_2_7_0,
    .init = qcom_pcie_init_2_7_0,
    .post_init = qcom_pcie_post_init_2_7_0,
    .deinit = qcom_pcie_deinit_2_7_0,
    .ltssm_enable = qcom_pcie_2_3_2_ltssm_enable,
    };
// Qcom IP rev.: 1.9.0
    static const struct qcom_pcie_ops ops_1_9_0 = {
    .get_resources = qcom_pcie_get_resources_2_7_0,
    .init = qcom_pcie_init_2_7_0,
    .post_init = qcom_pcie_post_init_2_7_0,
    .host_post_init = qcom_pcie_host_post_init_2_7_0,
    .deinit = qcom_pcie_deinit_2_7_0,
    .ltssm_enable = qcom_pcie_2_3_2_ltssm_enable,
    .config_sid = qcom_pcie_config_sid_1_9_0,
    };
// Qcom IP rev.: 1.21.0  Synopsys IP rev.: 5.60a
    static const struct qcom_pcie_ops ops_1_21_0 = {
    .get_resources = qcom_pcie_get_resources_2_7_0,
    .init = qcom_pcie_init_2_7_0,
    .post_init = qcom_pcie_post_init_2_7_0,
    .host_post_init = qcom_pcie_host_post_init_2_7_0,
    .deinit = qcom_pcie_deinit_2_7_0,
    .ltssm_enable = qcom_pcie_2_3_2_ltssm_enable,
    };
// Qcom IP rev.: 2.9.0  Synopsys IP rev.: 5.00a
    static const struct qcom_pcie_ops ops_2_9_0 = {
    .get_resources = qcom_pcie_get_resources_2_9_0,
    .init = qcom_pcie_init_2_9_0,
    .post_init = qcom_pcie_post_init_2_9_0,
    .deinit = qcom_pcie_deinit_2_9_0,
    .ltssm_enable = qcom_pcie_2_3_2_ltssm_enable,
    };
    static const struct qcom_pcie_cfg cfg_1_0_0 = {
    .ops = &ops_1_0_0,
    };
    static const struct qcom_pcie_cfg cfg_1_9_0 = {
    .ops = &ops_1_9_0,
    };
    static const struct qcom_pcie_cfg cfg_1_34_0 = {
    .ops = &ops_1_9_0,
    .override_no_snoop = true,
    .no_l0s = true,
    };
    static const struct qcom_pcie_cfg cfg_2_1_0 = {
    .ops = &ops_2_1_0,
    };
    static const struct qcom_pcie_cfg cfg_2_3_2 = {
    .ops = &ops_2_3_2,
    .no_l0s = true,
    };
    static const struct qcom_pcie_cfg cfg_2_3_3 = {
    .ops = &ops_2_3_3,
    };
    static const struct qcom_pcie_cfg cfg_2_4_0 = {
    .ops = &ops_2_4_0,
    };
    static const struct qcom_pcie_cfg cfg_2_7_0 = {
    .ops = &ops_2_7_0,
    };
    static const struct qcom_pcie_cfg cfg_2_9_0 = {
    .ops = &ops_2_9_0,
    };
    static const struct qcom_pcie_cfg cfg_sc8280xp = {
    .ops = &ops_1_21_0,
    .no_l0s = true,
    };
    static const struct qcom_pcie_cfg cfg_fw_managed = {
    .firmware_managed = true,
    };
    static const struct dw_pcie_ops dw_pcie_ops = {
    .link_up = qcom_pcie_link_up,
    .start_link = qcom_pcie_start_link,
    .get_ltssm = qcom_pcie_get_ltssm,
    };
#[no_mangle]
unsafe extern "C" fn qcom_pcie_icc_init(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_icc_init(struct qcom_pcie *pcie)
    {
    struct dw_pcie *pci = pcie.pci;
    int ret;
    pcie.icc_mem = devm_of_icc_get(pci.dev, "pcie-mem");
    if (IS_ERR(pcie.icc_mem))
    return PTR_ERR(pcie.icc_mem);
    pcie.icc_cpu = devm_of_icc_get(pci.dev, "cpu-pcie");
    if (IS_ERR(pcie.icc_cpu))
    return PTR_ERR(pcie.icc_cpu);
//
// Some Qualcomm platforms require interconnect bandwidth constraints
// to be set before enabling interconnect clocks.
//
// Set an initial peak bandwidth corresponding to single-lane Gen 1
// for the pcie-mem path.
//
    ret = icc_set_bw(pcie.icc_mem, 0, QCOM_PCIE_LINK_SPEED_TO_BW(1));
    if (ret) {
    dev_err(pci.dev, "Failed to set bandwidth for PCIe-MEM interconnect path: %d\n",
    ret);
    return ret;
    }
//
// Since the CPU-PCIe path is only used for activities like register
// access of the host controller and endpoint Config/BAR space access,
// HW team has recommended to use a minimal bandwidth of 1KBps just to
// keep the path active.
//
    ret = icc_set_bw(pcie.icc_cpu, 0, kBps_to_icc(1));
    if (ret) {
    dev_err(pci.dev, "Failed to set bandwidth for CPU-PCIe interconnect path: %d\n",
    ret);
    icc_set_bw(pcie.icc_mem, 0, 0);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_icc_opp_update(pcie: *mut qcom_pcie) {
    static void qcom_pcie_icc_opp_update(struct qcom_pcie *pcie)
    {
    u32 offset, status, width, speed;
    struct dw_pcie *pci = pcie.pci;
    let mut key: dev_pm_opp_key = {};
    unsigned long freq_kbps;
    struct dev_pm_opp *opp;
    int ret, freq_mbps;
    offset = dw_pcie_find_capability(pci, PCI_CAP_ID_EXP);
    status = readw(pci.dbi_base + offset + PCI_EXP_LNKSTA);
// Only update constraints if link is up.
    if (!(status & PCI_EXP_LNKSTA_DLLLA))
    return;
    speed = FIELD_GET(PCI_EXP_LNKSTA_CLS, status);
    width = FIELD_GET(PCI_EXP_LNKSTA_NLW, status);
    if (pcie.icc_mem) {
    ret = icc_set_bw(pcie.icc_mem, 0,
    width * QCOM_PCIE_LINK_SPEED_TO_BW(speed));
    if (ret) {
    dev_err(pci.dev, "Failed to set bandwidth for PCIe-MEM interconnect path: %d\n",
    ret);
    }
    } else if (pcie.use_pm_opp) {
    freq_mbps = pcie_dev_speed_mbps(pcie_get_link_speed(speed));
    if (freq_mbps < 0)
    return;
    freq_kbps = freq_mbps * KILO;
    opp = dev_pm_opp_find_level_exact(pci.dev, speed);
    if (IS_ERR(opp)) {
// opp-level is not defined use only frequency
    opp = dev_pm_opp_find_freq_exact(pci.dev, freq_kbps * width,
    true);
    } else {
// put opp-level OPP
    dev_pm_opp_put(opp);
    key.freq = freq_kbps * width;
    key.level = speed;
    key.bw = 0;
    opp = dev_pm_opp_find_key_exact(pci.dev, &key, true);
    }
    if (!IS_ERR(opp)) {
    ret = dev_pm_opp_set_opp(pci.dev, opp);
    if (ret)
    dev_err(pci.dev, "Failed to set OPP for freq (%lu): %d\n",
    freq_kbps * width, ret);
    dev_pm_opp_put(opp);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_set_max_opp(dev: *mut device) -> c_int {
    static int qcom_pcie_set_max_opp(struct device *dev)
    {
    let mut max_freq: c_ulong = ULONG_MAX;
    struct dev_pm_opp *opp;
    int ret;
    opp = dev_pm_opp_find_freq_floor(dev, &max_freq);
    if (IS_ERR(opp))
    return PTR_ERR(opp);
    ret = dev_pm_opp_set_opp(dev, opp);
    dev_pm_opp_put(opp);
    return ret;
    }
//
// Qcom PCIe controllers only support one Root Port per controller instance. So
// this function ignores the 'pci_dev' associated with the Root Port and just
// resets the host bridge, which in turn resets the Root Port also.
//
    static int qcom_pcie_reset_root_port(struct pci_host_bridge *bridge,
    struct pci_dev *pdev)
    {
    struct device *dev = bridge.dev.parent;
    struct qcom_pcie *pcie = dev_get_drvdata(dev);
    struct dw_pcie *pci = pcie.pci;
    struct dw_pcie_rp *pp = &pci.pp;
    u32 val;
    int ret;
// Wait for the pending transactions to be completed
    ret = readl_relaxed_poll_timeout(pcie.parf + PARF_STATUS, val,
    val & FLUSH_COMPLETED, 10,
    FLUSH_TIMEOUT_US);
    if (ret) {
    dev_err(dev, "Flush completion failed: %d\n", ret);
    return ret;
    }
// Clear the FLUSH_MODE to allow the core to be reset
    val = readl(pcie.parf + PARF_LTSSM);
    val |= SW_CLEAR_FLUSH_MODE;
    writel(val, pcie.parf + PARF_LTSSM);
// Wait for the FLUSH_MODE to clear
    ret = readl_relaxed_poll_timeout(pcie.parf + PARF_LTSSM, val,
    !(val & FLUSH_MODE), 10,
    FLUSH_TIMEOUT_US);
    if (ret) {
    dev_err(dev, "Flush mode clear failed: %d\n", ret);
    return ret;
    }
    qcom_pcie_host_deinit(pp);
    ret = qcom_pcie_host_init(pp);
    if (ret) {
    dev_err(dev, "Host init failed\n");
    return ret;
    }
    ret = dw_pcie_setup_rc(pp);
    if (ret)
    return ret;
//
// Re-enable global IRQ events as the PARF_INT_ALL_MASK register is
// non-sticky.
//
    if (pcie.global_irq)
    writel_relaxed(PARF_INT_ALL_LINK_DOWN | PARF_INT_MSI_DEV_0_7,
    pcie.parf + PARF_INT_ALL_MASK);
    qcom_pcie_start_link(pci);
    ret = dw_pcie_wait_for_link(pci);
    if (ret)
    return ret;
    dev_dbg(dev, "Root Port reset completed\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_link_transition_count(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int qcom_pcie_link_transition_count(struct seq_file *s, void *data)
    {
    struct qcom_pcie *pcie = (struct qcom_pcie *)dev_get_drvdata(s.private);
    seq_printf(s, "L0s transition count: %u\n",
    readl_relaxed(pcie.mhi + PARF_DEBUG_CNT_PM_LINKST_IN_L0S));
    seq_printf(s, "L1 transition count: %u\n",
    readl_relaxed(pcie.mhi + PARF_DEBUG_CNT_PM_LINKST_IN_L1));
    seq_printf(s, "L1.1 transition count: %u\n",
    readl_relaxed(pcie.mhi + PARF_DEBUG_CNT_AUX_CLK_IN_L1SUB_L1));
    seq_printf(s, "L1.2 transition count: %u\n",
    readl_relaxed(pcie.mhi + PARF_DEBUG_CNT_AUX_CLK_IN_L1SUB_L2));
    seq_printf(s, "L2 transition count: %u\n",
    readl_relaxed(pcie.mhi + PARF_DEBUG_CNT_PM_LINKST_IN_L2));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_init_debugfs(pcie: *mut qcom_pcie) {
    static void qcom_pcie_init_debugfs(struct qcom_pcie *pcie)
    {
    struct dw_pcie *pci = pcie.pci;
    struct device *dev = pci.dev;
    char *name;
    name = devm_kasprintf(dev, GFP_KERNEL, "%pOFP", dev.of_node);
    if (!name)
    return;
    pcie.debugfs = debugfs_create_dir(name, core::ptr::null_mut());
    debugfs_create_devm_seqfile(dev, "link_transition_count", pcie.debugfs,
    qcom_pcie_link_transition_count);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_global_irq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t qcom_pcie_global_irq_thread(int irq, void *data)
    {
    struct qcom_pcie *pcie = data;
    struct dw_pcie_rp *pp = &pcie.pci.pp;
    struct device *dev = pcie.pci.dev;
    struct pci_dev *port;
    let mut status: c_ulong = readl_relaxed(pcie.parf + PARF_INT_ALL_STATUS);
    writel_relaxed(status, pcie.parf + PARF_INT_ALL_CLEAR);
    if (test_and_clear_bit(INT_ALL_LINK_DOWN, &status)) {
    dev_dbg(dev, "Received Link down event\n");
    for_each_pci_bridge(port, pp.bridge.bus) {
    if (pci_pcie_type(port) == PCI_EXP_TYPE_ROOT_PORT)
    pci_host_handle_link_down(port);
    }
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pci_free_msi(ptr: *mut c_void) {
    static void qcom_pci_free_msi(void *ptr)
    {
    struct dw_pcie_rp *pp = (struct dw_pcie_rp *)ptr;
    if (pp && pp.use_imsi_rx)
    dw_pcie_free_msi(pp);
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_ecam_host_init(cfg: *mut pci_config_window) -> c_int {
    static int qcom_pcie_ecam_host_init(struct pci_config_window *cfg)
    {
    struct device *dev = cfg.parent;
    struct dw_pcie_rp *pp;
    struct dw_pcie *pci;
    int ret;
    pci = devm_kzalloc(dev, sizeof(*pci), GFP_KERNEL);
    if (!pci)
    return -ENOMEM;
    pci.dev = dev;
    pp = &pci.pp;
    pci.dbi_base = cfg.win;
    pp.num_vectors = MSI_DEF_NUM_VECTORS;
//
// dw_pcie_msi_host_init() is called directly here, bypassing
// dw_pcie_host_init() where pp->lock is normally initialized.
//
    raw_spin_lock_init(&pp.lock);
    ret = dw_pcie_msi_host_init(pp);
    if (ret)
    return ret;
    pp.use_imsi_rx = true;
    dw_pcie_msi_init(pp);
    return devm_add_action_or_reset(dev, qcom_pci_free_msi, pp);
    }
    static const struct pci_ecam_ops pci_qcom_ecam_ops = {
    .init		= qcom_pcie_ecam_host_init,
    .pci_ops	= {
    .map_bus	= pci_ecam_map_bus,
    .read		= pci_generic_config_read,
    .write		= pci_generic_config_write,
    }
    };
// Check if @node is a child of @dev in DT
    static bool qcom_pcie_is_child_node(struct device *dev,
    struct device_node *node)
    {
    struct device_node *parent;
    for (parent = of_get_parent(node); parent;
    parent = of_get_next_parent(parent)) {
    if (parent == dev.of_node) {
    of_node_put(parent);
    return true;
    }
    }
    return false;
    }
// Parse PERST# from all nodes in depth first manner starting from @np
    static int qcom_pcie_parse_perst(struct qcom_pcie *pcie,
    struct qcom_pcie_port *port,
    struct device_node *np)
    {
    struct device *dev = pcie.pci.dev;
    struct qcom_pcie_perst *perst;
    struct device_node *gpio_np;
    struct gpio_desc *reset;
    int ret;
    if (pcie.reset) {
    dev_warn_once(dev,
    "Reusing PERST# from Root Complex node. DT needs to be fixed!\n");
    reset = pcie.reset;
    goto skip_perst_parsing;
    }
    if (!of_find_property(np, "reset-gpios", core::ptr::null_mut()))
    goto parse_child_node;
//
// Skip GPIOs provided by a PCIe device which is a child of the Root
// Complex (e.g., a PCIe switch with GPIO controller capability). Such
// controllers won't be available at RC probe time and their PERST#
// should be controlled by the respective PCI client driver
// implementation.
//
    gpio_np = of_parse_phandle(np, "reset-gpios", 0);
    if (!gpio_np) {
    dev_err(dev, "Failed to parse GPIO provider\n");
    return -EINVAL;
    }
    if (qcom_pcie_is_child_node(dev, gpio_np)) {
    of_node_put(gpio_np);
    goto parse_child_node;
    }
    of_node_put(gpio_np);
    reset = devm_fwnode_gpiod_get(dev, of_fwnode_handle(np), "reset",
    GPIOD_OUT_HIGH, "PERST#");
    if (IS_ERR(reset)) {
//
// FIXME: GPIOLIB currently supports exclusive GPIO access only.
// Non exclusive access is broken. But shared PERST# requires
// non-exclusive access. So once GPIOLIB properly supports it,
// implement it here.
//
    if (PTR_ERR(reset) == -EBUSY)
    dev_err(dev, "Shared PERST# is not supported\n");
    return PTR_ERR(reset);
    }
    skip_perst_parsing:
    perst = devm_kzalloc(dev, sizeof(*perst), GFP_KERNEL);
    if (!perst)
    return -ENOMEM;
    INIT_LIST_HEAD(&perst.list);
    perst.desc = reset;
    list_add_tail(&perst.list, &port.perst);
    parse_child_node:
    for_each_available_child_of_node_scoped(np, child) {
    ret = qcom_pcie_parse_perst(pcie, port, child);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_parse_port(pcie: *mut qcom_pcie, node: *mut device_node) -> c_int {
    static int qcom_pcie_parse_port(struct qcom_pcie *pcie, struct device_node *node)
    {
    struct device *dev = pcie.pci.dev;
    struct qcom_pcie_port *port;
    struct phy *phy;
    int ret;
    phy = devm_of_phy_get(dev, node, core::ptr::null_mut());
    if (IS_ERR(phy))
    return PTR_ERR(phy);
    port = devm_kzalloc(dev, sizeof(*port), GFP_KERNEL);
    if (!port)
    return -ENOMEM;
    ret = phy_init(phy);
    if (ret)
    return ret;
    INIT_LIST_HEAD(&port.perst);
    ret = qcom_pcie_parse_perst(pcie, port, node);
    if (ret)
    return ret;
// TODO: Move to DWC core after multi Root Port support is added
    of_property_read_u32(node, "t-power-on-us", &port.l1ss_t_power_on);
    port.phy = phy;
    INIT_LIST_HEAD(&port.list);
    list_add_tail(&port.list, &pcie.ports);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_parse_ports(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_parse_ports(struct qcom_pcie *pcie)
    {
    struct qcom_pcie_perst *perst, *tmp_perst;
    struct qcom_pcie_port *port, *tmp_port;
    struct device *dev = pcie.pci.dev;
    let mut ret: c_int = -ENODEV;
    if (of_find_property(dev.of_node, "perst-gpios", core::ptr::null_mut())) {
    pcie.reset = devm_gpiod_get_optional(dev, "perst",
    GPIOD_OUT_HIGH);
    if (IS_ERR(pcie.reset))
    return PTR_ERR(pcie.reset);
    }
    for_each_available_child_of_node_scoped(dev.of_node, of_port) {
    if (!of_node_is_type(of_port, "pci"))
    continue;
    ret = qcom_pcie_parse_port(pcie, of_port);
    if (ret)
    goto err_port_del;
    }
    return ret;
    err_port_del:
    list_for_each_entry_safe(port, tmp_port, &pcie.ports, list) {
    list_for_each_entry_safe(perst, tmp_perst, &port.perst, list)
    list_del(&perst.list);
    phy_exit(port.phy);
    list_del(&port.list);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_parse_legacy_binding(pcie: *mut qcom_pcie) -> c_int {
    static int qcom_pcie_parse_legacy_binding(struct qcom_pcie *pcie)
    {
    struct device *dev = pcie.pci.dev;
    struct qcom_pcie_perst *perst;
    struct qcom_pcie_port *port;
    struct phy *phy;
    int ret;
    phy = devm_phy_optional_get(dev, "pciephy");
    if (IS_ERR(phy))
    return PTR_ERR(phy);
    ret = phy_init(phy);
    if (ret)
    return ret;
    port = devm_kzalloc(dev, sizeof(*port), GFP_KERNEL);
    if (!port)
    return -ENOMEM;
    perst = devm_kzalloc(dev, sizeof(*perst), GFP_KERNEL);
    if (!perst)
    return -ENOMEM;
    port.phy = phy;
    INIT_LIST_HEAD(&port.list);
    list_add_tail(&port.list, &pcie.ports);
    perst.desc = pcie.reset;
    INIT_LIST_HEAD(&port.perst);
    INIT_LIST_HEAD(&perst.list);
    list_add_tail(&perst.list, &port.perst);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_pcie_probe(struct platform_device *pdev)
    {
    struct qcom_pcie_perst *perst, *tmp_perst;
    struct qcom_pcie_port *port, *tmp_port;
    const struct qcom_pcie_cfg *pcie_cfg;
    struct device *dev = &pdev.dev;
    struct qcom_pcie *pcie;
    struct dw_pcie_rp *pp;
    struct resource *res;
    struct dw_pcie *pci;
    int ret, irq;
    pcie_cfg = of_device_get_match_data(dev);
    if (!pcie_cfg) {
    dev_err(dev, "No platform data\n");
    return -ENODATA;
    }
    if (!pcie_cfg.firmware_managed && !pcie_cfg.ops) {
    dev_err(dev, "No platform ops\n");
    return -ENODATA;
    }
    pm_runtime_enable(dev);
    ret = pm_runtime_get_sync(dev);
    if (ret < 0)
    goto err_pm_runtime_put;
    if (pcie_cfg.firmware_managed) {
    struct pci_host_bridge *bridge;
    struct pci_config_window *cfg;
    bridge = devm_pci_alloc_host_bridge(dev, 0);
    if (!bridge) {
    ret = -ENOMEM;
    goto err_pm_runtime_put;
    }
// Parse and map our ECAM configuration space area
    cfg = pci_host_common_ecam_create(dev, bridge,
    &pci_qcom_ecam_ops);
    if (IS_ERR(cfg)) {
    ret = PTR_ERR(cfg);
    goto err_pm_runtime_put;
    }
    bridge.sysdata = cfg;
    bridge.ops = (struct pci_ops *)&pci_qcom_ecam_ops.pci_ops;
    bridge.msi_domain = true;
    ret = pci_host_probe(bridge);
    if (ret)
    goto err_pm_runtime_put;
    return 0;
    }
    pcie = devm_kzalloc(dev, sizeof(*pcie), GFP_KERNEL);
    if (!pcie) {
    ret = -ENOMEM;
    goto err_pm_runtime_put;
    }
    pci = devm_kzalloc(dev, sizeof(*pci), GFP_KERNEL);
    if (!pci) {
    ret = -ENOMEM;
    goto err_pm_runtime_put;
    }
    INIT_LIST_HEAD(&pcie.ports);
    pci.dev = dev;
    pci.ops = &dw_pcie_ops;
    pp = &pci.pp;
    pcie.pci = pci;
    pcie.cfg = pcie_cfg;
    pcie.parf = devm_platform_ioremap_resource_byname(pdev, "parf");
    if (IS_ERR(pcie.parf)) {
    ret = PTR_ERR(pcie.parf);
    goto err_pm_runtime_put;
    }
// MHI region is optional
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "mhi");
    if (res) {
    pcie.mhi = devm_ioremap_resource(dev, res);
    if (IS_ERR(pcie.mhi)) {
    ret = PTR_ERR(pcie.mhi);
    goto err_pm_runtime_put;
    }
    }
// OPP table is optional
    ret = devm_pm_opp_of_add_table(dev);
    if (ret && ret != -ENODEV) {
    dev_err_probe(dev, ret, "Failed to add OPP table\n");
    goto err_pm_runtime_put;
    }
//
// Before the PCIe link is initialized, vote for highest OPP in the OPP
// table, so that we are voting for maximum voltage corner for the
// link to come up in maximum supported speed. At the end of the
// probe(), OPP will be updated using qcom_pcie_icc_opp_update().
//
    if (!ret) {
    ret = qcom_pcie_set_max_opp(dev);
    if (ret) {
    dev_err_probe(dev, ret, "Failed to set max OPP\n");
    goto err_pm_runtime_put;
    }
    pcie.use_pm_opp = true;
    } else {
// Skip ICC init if OPP is supported as it is handled by OPP
    ret = qcom_pcie_icc_init(pcie);
    if (ret)
    goto err_pm_runtime_put;
    }
    ret = pcie.cfg.ops.get_resources(pcie);
    if (ret)
    goto err_pm_runtime_put;
    pp.ops = &qcom_pcie_dw_ops;
    ret = qcom_pcie_parse_ports(pcie);
    if (ret) {
    if (ret != -ENODEV) {
    dev_err_probe(pci.dev, ret,
    "Failed to parse Root Port: %d\n", ret);
    goto err_pm_runtime_put;
    }
//
// In the case of properties not populated in Root Port node,
// fallback to the legacy method of parsing the Host Bridge
// node. This is to maintain DT backwards compatibility.
//
    ret = qcom_pcie_parse_legacy_binding(pcie);
    if (ret)
    goto err_pm_runtime_put;
    }
    platform_set_drvdata(pdev, pcie);
    ret = dw_pcie_host_init(pp);
    if (ret) {
    dev_err_probe(dev, ret, "cannot initialize host\n");
    goto err_phy_exit;
    }
    irq = platform_get_irq_byname_optional(pdev, "global");
    if (irq > 0) {
    const char *name;
    name = devm_kasprintf(dev, GFP_KERNEL, "qcom_pcie_global_irq%d",
    pci_domain_nr(pp.bridge.bus));
    if (!name) {
    ret = -ENOMEM;
    goto err_host_deinit;
    }
    ret = devm_request_threaded_irq(&pdev.dev, irq, core::ptr::null_mut(),
    qcom_pcie_global_irq_thread,
    IRQF_ONESHOT, name, pcie);
    if (ret) {
    dev_err_probe(&pdev.dev, ret,
    "Failed to request Global IRQ\n");
    goto err_host_deinit;
    }
    writel_relaxed(PARF_INT_ALL_LINK_DOWN | PARF_INT_MSI_DEV_0_7,
    pcie.parf + PARF_INT_ALL_MASK);
    pcie.global_irq = irq;
    }
    qcom_pcie_icc_opp_update(pcie);
    if (pcie.mhi)
    qcom_pcie_init_debugfs(pcie);
    return 0;
    err_host_deinit:
    dw_pcie_host_deinit(pp);
    err_phy_exit:
    list_for_each_entry_safe(port, tmp_port, &pcie.ports, list) {
    list_for_each_entry_safe(perst, tmp_perst, &port.perst, list)
    list_del(&perst.list);
    phy_exit(port.phy);
    list_del(&port.list);
    }
    err_pm_runtime_put:
    pm_runtime_put(dev);
    pm_runtime_disable(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_suspend_noirq(dev: *mut device) -> c_int {
    static int qcom_pcie_suspend_noirq(struct device *dev)
    {
    struct qcom_pcie *pcie;
    let mut ret: c_int = 0;
    pcie = dev_get_drvdata(dev);
    if (!pcie)
    return 0;
    ret = dw_pcie_suspend_noirq(pcie.pci);
    if (ret)
    return ret;
    if (pcie.pci.suspended) {
    ret = icc_disable(pcie.icc_mem);
    if (ret)
    dev_err(dev, "Failed to disable PCIe-MEM interconnect path: %d\n", ret);
    ret = icc_disable(pcie.icc_cpu);
    if (ret)
    dev_err(dev, "Failed to disable CPU-PCIe interconnect path: %d\n", ret);
    if (pcie.use_pm_opp)
    dev_pm_opp_set_opp(pcie.pci.dev, core::ptr::null_mut());
    } else {
//
// Set minimum bandwidth required to keep data path
// functional during suspend.
//
    if (pcie.icc_mem) {
    ret = icc_set_bw(pcie.icc_mem, 0, kBps_to_icc(1));
    if (ret) {
    dev_err(dev,
    "Failed to set bandwidth for PCIe-MEM interconnect path: %d\n",
    ret);
    return ret;
    }
    }
//
// Only disable CPU-PCIe interconnect path if the suspend
// is non-S2RAM.  On some platforms, DBI access can happen
// very late during S2RAM and a non-active CPU-PCIe
// interconnect path may lead to NoC error.
//
    if (pm_suspend_target_state != PM_SUSPEND_MEM) {
    ret = icc_disable(pcie.icc_cpu);
    if (ret)
    dev_err(dev, "Failed to disable CPU-PCIe interconnect path: %d\n",
    ret);
    if (pcie.use_pm_opp)
    dev_pm_opp_set_opp(pcie.pci.dev, core::ptr::null_mut());
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_pcie_resume_noirq(dev: *mut device) -> c_int {
    static int qcom_pcie_resume_noirq(struct device *dev)
    {
    struct qcom_pcie *pcie;
    int ret;
    pcie = dev_get_drvdata(dev);
    if (!pcie)
    return 0;
    if (pcie.pci.suspended) {
    if (pcie.use_pm_opp) {
    ret = qcom_pcie_set_max_opp(dev);
    if (ret) {
    dev_err(dev, "Failed to set max OPP: %d\n", ret);
    return ret;
    }
    }
    ret = icc_enable(pcie.icc_cpu);
    if (ret) {
    dev_err(dev, "Failed to enable CPU-PCIe interconnect path: %d\n", ret);
    return ret;
    }
    ret = icc_enable(pcie.icc_mem);
    if (ret) {
    dev_err(dev, "Failed to enable PCIe-MEM interconnect path: %d\n", ret);
    goto disable_icc_cpu;
    }
//
// Ignore -ENODEV & -EIO here since it is expected when no
// endpoint is connected to the PCIe link.
//
    ret = dw_pcie_resume_noirq(pcie.pci);
    if (ret && ret != -ENODEV && ret != -EIO)
    goto disable_icc_mem;
    } else {
    if (pm_suspend_target_state != PM_SUSPEND_MEM) {
    if (pcie.use_pm_opp) {
    ret = qcom_pcie_set_max_opp(dev);
    if (ret) {
    dev_err(dev, "Failed to set max OPP: %d\n", ret);
    return ret;
    }
    }
    ret = icc_enable(pcie.icc_cpu);
    if (ret) {
    dev_err(dev, "Failed to enable CPU-PCIe interconnect path: %d\n",
    ret);
    return ret;
    }
    }
    }
    qcom_pcie_icc_opp_update(pcie);
    return 0;
    disable_icc_mem:
    icc_disable(pcie.icc_mem);
    disable_icc_cpu:
    icc_disable(pcie.icc_cpu);
    return ret;
    }
    static const struct of_device_id qcom_pcie_match[] = {
    { .compatible = "qcom,hawi-pcie", .data = &cfg_1_9_0 },
    { .compatible = "qcom,pcie-apq8064", .data = &cfg_2_1_0 },
    { .compatible = "qcom,pcie-apq8084", .data = &cfg_1_0_0 },
    { .compatible = "qcom,pcie-ipq4019", .data = &cfg_2_4_0 },
    { .compatible = "qcom,pcie-ipq5018", .data = &cfg_2_9_0 },
    { .compatible = "qcom,pcie-ipq6018", .data = &cfg_2_9_0 },
    { .compatible = "qcom,pcie-ipq8064", .data = &cfg_2_1_0 },
    { .compatible = "qcom,pcie-ipq8064-v2", .data = &cfg_2_1_0 },
    { .compatible = "qcom,pcie-ipq8074", .data = &cfg_2_3_3 },
    { .compatible = "qcom,pcie-ipq8074-gen3", .data = &cfg_2_9_0 },
    { .compatible = "qcom,pcie-ipq9574", .data = &cfg_2_9_0 },
    { .compatible = "qcom,pcie-msm8996", .data = &cfg_2_3_2 },
    { .compatible = "qcom,pcie-qcs404", .data = &cfg_2_4_0 },
    { .compatible = "qcom,pcie-sa8255p", .data = &cfg_fw_managed },
    { .compatible = "qcom,pcie-sa8540p", .data = &cfg_sc8280xp },
    { .compatible = "qcom,pcie-sa8775p", .data = &cfg_1_34_0},
    { .compatible = "qcom,pcie-sc7280", .data = &cfg_1_9_0 },
    { .compatible = "qcom,pcie-sc8180x", .data = &cfg_1_9_0 },
    { .compatible = "qcom,pcie-sc8280xp", .data = &cfg_sc8280xp },
    { .compatible = "qcom,pcie-sdm845", .data = &cfg_2_7_0 },
    { .compatible = "qcom,pcie-sdx55", .data = &cfg_1_9_0 },
    { .compatible = "qcom,pcie-sm8150", .data = &cfg_1_9_0 },
    { .compatible = "qcom,pcie-sm8250", .data = &cfg_1_9_0 },
    { .compatible = "qcom,pcie-sm8350", .data = &cfg_1_9_0 },
    { .compatible = "qcom,pcie-sm8450-pcie0", .data = &cfg_1_9_0 },
    { .compatible = "qcom,pcie-sm8450-pcie1", .data = &cfg_1_9_0 },
    { .compatible = "qcom,pcie-sm8550", .data = &cfg_1_9_0 },
    { .compatible = "qcom,pcie-x1e80100", .data = &cfg_sc8280xp },
    { }
    };
#[no_mangle]
unsafe extern "C" fn qcom_fixup_class(dev: *mut pci_dev) {
    static void qcom_fixup_class(struct pci_dev *dev)
    {
    dev.class = PCI_CLASS_BRIDGE_PCI_NORMAL;
    }
    DECLARE_PCI_FIXUP_EARLY(PCI_VENDOR_ID_QCOM, 0x0101, qcom_fixup_class);
    DECLARE_PCI_FIXUP_EARLY(PCI_VENDOR_ID_QCOM, 0x0104, qcom_fixup_class);
    DECLARE_PCI_FIXUP_EARLY(PCI_VENDOR_ID_QCOM, 0x0106, qcom_fixup_class);
    DECLARE_PCI_FIXUP_EARLY(PCI_VENDOR_ID_QCOM, 0x0107, qcom_fixup_class);
    DECLARE_PCI_FIXUP_EARLY(PCI_VENDOR_ID_QCOM, 0x0302, qcom_fixup_class);
    DECLARE_PCI_FIXUP_EARLY(PCI_VENDOR_ID_QCOM, 0x1000, qcom_fixup_class);
    DECLARE_PCI_FIXUP_EARLY(PCI_VENDOR_ID_QCOM, 0x1001, qcom_fixup_class);
    static const struct dev_pm_ops qcom_pcie_pm_ops = {
    NOIRQ_SYSTEM_SLEEP_PM_OPS(qcom_pcie_suspend_noirq, qcom_pcie_resume_noirq)
    };
    static struct platform_driver qcom_pcie_driver = {
    .probe = qcom_pcie_probe,
    .driver = {
    .name = "qcom-pcie",
    .suppress_bind_attrs = true,
    .of_match_table = qcom_pcie_match,
    .pm = &qcom_pcie_pm_ops,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
    builtin_platform_driver(qcom_pcie_driver);
