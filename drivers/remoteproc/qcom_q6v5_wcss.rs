//! Automatically rewritten from C to Rust
//! Source: drivers/remoteproc/qcom_q6v5_wcss.c
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
// Copyright (C) 2016-2018 Linaro Ltd.
// Copyright (C) 2014 Sony Mobile Communications AB
// Copyright (c) 2012-2018, The Linux Foundation. All rights reserved.
//

pub const WCSS_CRASH_REASON: c_int = 421;
// Q6SS Register Offsets
pub const Q6SS_RESET_REG: c_uint = 0x014;
pub const Q6SS_GFMUX_CTL_REG: c_uint = 0x020;
pub const Q6SS_PWR_CTL_REG: c_uint = 0x030;
pub const Q6SS_MEM_PWR_CTL: c_uint = 0x0B0;
pub const Q6SS_STRAP_ACC: c_uint = 0x110;
pub const Q6SS_CGC_OVERRIDE: c_uint = 0x034;
pub const Q6SS_BCR_REG: c_uint = 0x6000;
// AXI Halt Register Offsets
pub const AXI_HALTREQ_REG: c_uint = 0x0;
pub const AXI_HALTACK_REG: c_uint = 0x4;
pub const AXI_IDLE_REG: c_uint = 0x8;
pub const HALT_ACK_TIMEOUT_MS: c_int = 100;
// Q6SS_RESET

// Q6SS_BRC_RESET

// Q6SS_GFMUX_CTL

// Q6SS_PWR_CTL

// Q6SS parameters

pub const HALT_CHECK_MAX_LOOPS: c_int = 200;

// Q6SS config/status registers
pub const TCSR_GLOBAL_CFG0: c_uint = 0x0;
pub const TCSR_GLOBAL_CFG1: c_uint = 0x4;
pub const SSCAON_CONFIG: c_uint = 0x8;
pub const SSCAON_STATUS: c_uint = 0xc;
pub const Q6SS_BHS_STATUS: c_uint = 0x78;
pub const Q6SS_RST_EVB: c_uint = 0x10;

pub const MEM_BANKS: c_int = 19;
pub const TCSR_WCSS_CLK_MASK: c_uint = 0x1F;
pub const TCSR_WCSS_CLK_ENABLE: c_uint = 0x14;
pub const MAX_HALT_REG: c_int = 4;
    enum {
    WCSS_IPQ8074,
    WCSS_QCS404,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcss_data {
    pub firmware_name: *const c_char,
    pub crash_reason_smem: c_uint,
    pub version: u32,
    pub aon_reset_required: bool,
    pub ssr_name: *const c_char,
    pub sysmon_name: *const c_char,
    pub ssctl_id: c_int,
    pub ops: *const rproc_ops,
    pub requires_force_stop: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct q6v5_wcss {
    pub dev: *mut device,
    pub reg_base: *mut void __iomem,
    pub rmb_base: *mut void __iomem,
    pub halt_map: *mut regmap,
    pub halt_q6: u32,
    pub halt_wcss: u32,
    pub halt_nc: u32,
    pub xo: *mut clk,
    pub ahbfabric_cbcr_clk: *mut clk,
    pub gcc_abhs_cbcr: *mut clk,
    pub gcc_axim_cbcr: *mut clk,
    pub lcc_csr_cbcr: *mut clk,
    pub ahbs_cbcr: *mut clk,
    pub tcm_slave_cbcr: *mut clk,
    pub qdsp6ss_abhm_cbcr: *mut clk,
    pub qdsp6ss_sleep_cbcr: *mut clk,
    pub qdsp6ss_axim_cbcr: *mut clk,
    pub qdsp6ss_xo_cbcr: *mut clk,
    pub qdsp6ss_core_gfmux: *mut clk,
    pub lcc_bcr_sleep: *mut clk,
    pub cx_supply: *mut regulator,
    pub sysmon: *mut qcom_sysmon,
    pub wcss_aon_reset: *mut reset_control,
    pub wcss_reset: *mut reset_control,
    pub wcss_q6_reset: *mut reset_control,
    pub q6v5: qcom_q6v5,
    pub mem_phys: phys_addr_t,
    pub mem_reloc: phys_addr_t,
    pub mem_region: *mut c_void,
    pub mem_size: usize,
    pub crash_reason_smem: c_uint,
    pub version: u32,
    pub requires_force_stop: bool,
    pub glink_subdev: qcom_rproc_glink,
    pub pdm_subdev: qcom_rproc_pdm,
    pub ssr_subdev: qcom_rproc_ssr,
}

#[no_mangle]
unsafe extern "C" fn q6v5_wcss_reset(wcss: *mut q6v5_wcss) -> c_int {
    static int q6v5_wcss_reset(struct q6v5_wcss *wcss)
    {
    int ret;
    u32 val;
    int i;
// Assert resets, stop core
    val = readl(wcss.reg_base + Q6SS_RESET_REG);
    val |= Q6SS_CORE_ARES | Q6SS_BUS_ARES_ENABLE | Q6SS_STOP_CORE;
    writel(val, wcss.reg_base + Q6SS_RESET_REG);
// BHS require xo cbcr to be enabled
    val = readl(wcss.reg_base + Q6SS_XO_CBCR);
    val |= 0x1;
    writel(val, wcss.reg_base + Q6SS_XO_CBCR);
// Read CLKOFF bit to go low indicating CLK is enabled
    ret = readl_poll_timeout(wcss.reg_base + Q6SS_XO_CBCR,
    val, !(val & BIT(31)), 1,
    HALT_CHECK_MAX_LOOPS);
    if (ret) {
    dev_err(wcss.dev,
    "xo cbcr enabling timed out (rc:%d)\n", ret);
    return ret;
    }
// Enable power block headswitch and wait for it to stabilize
    val = readl(wcss.reg_base + Q6SS_PWR_CTL_REG);
    val |= Q6SS_BHS_ON;
    writel(val, wcss.reg_base + Q6SS_PWR_CTL_REG);
    udelay(1);
// Put LDO in bypass mode
    val |= Q6SS_LDO_BYP;
    writel(val, wcss.reg_base + Q6SS_PWR_CTL_REG);
// Deassert Q6 compiler memory clamp
    val = readl(wcss.reg_base + Q6SS_PWR_CTL_REG);
    val &= ~Q6SS_CLAMP_QMC_MEM;
    writel(val, wcss.reg_base + Q6SS_PWR_CTL_REG);
// Deassert memory peripheral sleep and L2 memory standby
    val |= Q6SS_L2DATA_STBY_N | Q6SS_SLP_RET_N;
    writel(val, wcss.reg_base + Q6SS_PWR_CTL_REG);
// Turn on L1, L2, ETB and JU memories 1 at a time
    val = readl(wcss.reg_base + Q6SS_MEM_PWR_CTL);
    for (i = MEM_BANKS; i >= 0; i--) {
    val |= BIT(i);
    writel(val, wcss.reg_base + Q6SS_MEM_PWR_CTL);
//
// Read back value to ensure the write is done then
// wait for 1us for both memory peripheral and data
// array to turn on.
//
    val |= readl(wcss.reg_base + Q6SS_MEM_PWR_CTL);
    udelay(1);
    }
// Remove word line clamp
    val = readl(wcss.reg_base + Q6SS_PWR_CTL_REG);
    val &= ~Q6SS_CLAMP_WL;
    writel(val, wcss.reg_base + Q6SS_PWR_CTL_REG);
// Remove IO clamp
    val &= ~Q6SS_CLAMP_IO;
    writel(val, wcss.reg_base + Q6SS_PWR_CTL_REG);
// Bring core out of reset
    val = readl(wcss.reg_base + Q6SS_RESET_REG);
    val &= ~Q6SS_CORE_ARES;
    writel(val, wcss.reg_base + Q6SS_RESET_REG);
// Turn on core clock
    val = readl(wcss.reg_base + Q6SS_GFMUX_CTL_REG);
    val |= Q6SS_CLK_ENABLE;
    writel(val, wcss.reg_base + Q6SS_GFMUX_CTL_REG);
// Start core execution
    val = readl(wcss.reg_base + Q6SS_RESET_REG);
    val &= ~Q6SS_STOP_CORE;
    writel(val, wcss.reg_base + Q6SS_RESET_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_wcss_start(rproc: *mut rproc) -> c_int {
    static int q6v5_wcss_start(struct rproc *rproc)
    {
    struct q6v5_wcss *wcss = rproc.priv;
    int ret;
    qcom_q6v5_prepare(&wcss.q6v5);
// Release Q6 and WCSS reset
    ret = reset_control_deassert(wcss.wcss_reset);
    if (ret) {
    dev_err(wcss.dev, "wcss_reset failed\n");
    return ret;
    }
    ret = reset_control_deassert(wcss.wcss_q6_reset);
    if (ret) {
    dev_err(wcss.dev, "wcss_q6_reset failed\n");
    goto wcss_reset;
    }
// Lithium configuration - clock gating and bus arbitration
    ret = regmap_update_bits(wcss.halt_map,
    wcss.halt_nc + TCSR_GLOBAL_CFG0,
    TCSR_WCSS_CLK_MASK,
    TCSR_WCSS_CLK_ENABLE);
    if (ret)
    goto wcss_q6_reset;
    ret = regmap_update_bits(wcss.halt_map,
    wcss.halt_nc + TCSR_GLOBAL_CFG1,
    1, 0);
    if (ret)
    goto wcss_q6_reset;
// Write bootaddr to EVB so that Q6WCSS will jump there after reset
    writel(rproc.bootaddr >> 4, wcss.reg_base + Q6SS_RST_EVB);
    ret = q6v5_wcss_reset(wcss);
    if (ret)
    goto wcss_q6_reset;
    ret = qcom_q6v5_wait_for_start(&wcss.q6v5, 5 * HZ);
    if (ret == -ETIMEDOUT)
    dev_err(wcss.dev, "start timed out\n");
    return ret;
    wcss_q6_reset:
    reset_control_assert(wcss.wcss_q6_reset);
    wcss_reset:
    reset_control_assert(wcss.wcss_reset);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_wcss_qcs404_power_on(wcss: *mut q6v5_wcss) -> c_int {
    static int q6v5_wcss_qcs404_power_on(struct q6v5_wcss *wcss)
    {
    unsigned long val;
    int ret, idx;
// Toggle the restart
    reset_control_assert(wcss.wcss_reset);
    usleep_range(200, 300);
    reset_control_deassert(wcss.wcss_reset);
    usleep_range(200, 300);
// Enable GCC_WDSP_Q6SS_AHBS_CBCR clock
    ret = clk_prepare_enable(wcss.gcc_abhs_cbcr);
    if (ret)
    return ret;
// Remove reset to the WCNSS QDSP6SS
    reset_control_deassert(wcss.wcss_q6_reset);
// Enable Q6SSTOP_AHBFABRIC_CBCR clock
    ret = clk_prepare_enable(wcss.ahbfabric_cbcr_clk);
    if (ret)
    goto disable_gcc_abhs_cbcr_clk;
// Enable the LCCCSR CBC clock, Q6SSTOP_Q6SSTOP_LCC_CSR_CBCR clock
    ret = clk_prepare_enable(wcss.lcc_csr_cbcr);
    if (ret)
    goto disable_ahbfabric_cbcr_clk;
// Enable the Q6AHBS CBC, Q6SSTOP_Q6SS_AHBS_CBCR clock
    ret = clk_prepare_enable(wcss.ahbs_cbcr);
    if (ret)
    goto disable_csr_cbcr_clk;
// Enable the TCM slave CBC, Q6SSTOP_Q6SS_TCM_SLAVE_CBCR clock
    ret = clk_prepare_enable(wcss.tcm_slave_cbcr);
    if (ret)
    goto disable_ahbs_cbcr_clk;
// Enable the Q6SS AHB master CBC, Q6SSTOP_Q6SS_AHBM_CBCR clock
    ret = clk_prepare_enable(wcss.qdsp6ss_abhm_cbcr);
    if (ret)
    goto disable_tcm_slave_cbcr_clk;
// Enable the Q6SS AXI master CBC, Q6SSTOP_Q6SS_AXIM_CBCR clock
    ret = clk_prepare_enable(wcss.qdsp6ss_axim_cbcr);
    if (ret)
    goto disable_abhm_cbcr_clk;
// Enable the Q6SS XO CBC
    val = readl(wcss.reg_base + Q6SS_XO_CBCR);
    val |= BIT(0);
    writel(val, wcss.reg_base + Q6SS_XO_CBCR);
// Read CLKOFF bit to go low indicating CLK is enabled
    ret = readl_poll_timeout(wcss.reg_base + Q6SS_XO_CBCR,
    val, !(val & BIT(31)), 1,
    HALT_CHECK_MAX_LOOPS);
    if (ret) {
    dev_err(wcss.dev,
    "xo cbcr enabling timed out (rc:%d)\n", ret);
    goto disable_xo_cbcr_clk;
    }
    writel(0, wcss.reg_base + Q6SS_CGC_OVERRIDE);
// Enable QDSP6 sleep clock clock
    val = readl(wcss.reg_base + Q6SS_SLEEP_CBCR);
    val |= BIT(0);
    writel(val, wcss.reg_base + Q6SS_SLEEP_CBCR);
// Enable the Enable the Q6 AXI clock, GCC_WDSP_Q6SS_AXIM_CBCR
    ret = clk_prepare_enable(wcss.gcc_axim_cbcr);
    if (ret)
    goto disable_sleep_cbcr_clk;
// Assert resets, stop core
    val = readl(wcss.reg_base + Q6SS_RESET_REG);
    val |= Q6SS_CORE_ARES | Q6SS_BUS_ARES_ENABLE | Q6SS_STOP_CORE;
    writel(val, wcss.reg_base + Q6SS_RESET_REG);
// Program the QDSP6SS PWR_CTL register
    writel(0x01700000, wcss.reg_base + Q6SS_PWR_CTL_REG);
    writel(0x03700000, wcss.reg_base + Q6SS_PWR_CTL_REG);
    writel(0x03300000, wcss.reg_base + Q6SS_PWR_CTL_REG);
    writel(0x033C0000, wcss.reg_base + Q6SS_PWR_CTL_REG);
//
// Enable memories by turning on the QDSP6 memory foot/head switch, one
// bank at a time to avoid in-rush current
//
    for (idx = 28; idx >= 0; idx--) {
    writel((readl(wcss.reg_base + Q6SS_MEM_PWR_CTL) |
    (1 << idx)), wcss.reg_base + Q6SS_MEM_PWR_CTL);
    }
    writel(0x031C0000, wcss.reg_base + Q6SS_PWR_CTL_REG);
    writel(0x030C0000, wcss.reg_base + Q6SS_PWR_CTL_REG);
    val = readl(wcss.reg_base + Q6SS_RESET_REG);
    val &= ~Q6SS_CORE_ARES;
    writel(val, wcss.reg_base + Q6SS_RESET_REG);
// Enable the Q6 core clock at the GFM, Q6SSTOP_QDSP6SS_GFMUX_CTL
    val = readl(wcss.reg_base + Q6SS_GFMUX_CTL_REG);
    val |= Q6SS_CLK_ENABLE | Q6SS_SWITCH_CLK_SRC;
    writel(val, wcss.reg_base + Q6SS_GFMUX_CTL_REG);
// Enable sleep clock branch needed for BCR circuit
    ret = clk_prepare_enable(wcss.lcc_bcr_sleep);
    if (ret)
    goto disable_core_gfmux_clk;
    return 0;
    disable_core_gfmux_clk:
    val = readl(wcss.reg_base + Q6SS_GFMUX_CTL_REG);
    val &= ~(Q6SS_CLK_ENABLE | Q6SS_SWITCH_CLK_SRC);
    writel(val, wcss.reg_base + Q6SS_GFMUX_CTL_REG);
    clk_disable_unprepare(wcss.gcc_axim_cbcr);
    disable_sleep_cbcr_clk:
    val = readl(wcss.reg_base + Q6SS_SLEEP_CBCR);
    val &= ~Q6SS_CLK_ENABLE;
    writel(val, wcss.reg_base + Q6SS_SLEEP_CBCR);
    disable_xo_cbcr_clk:
    val = readl(wcss.reg_base + Q6SS_XO_CBCR);
    val &= ~Q6SS_CLK_ENABLE;
    writel(val, wcss.reg_base + Q6SS_XO_CBCR);
    clk_disable_unprepare(wcss.qdsp6ss_axim_cbcr);
    disable_abhm_cbcr_clk:
    clk_disable_unprepare(wcss.qdsp6ss_abhm_cbcr);
    disable_tcm_slave_cbcr_clk:
    clk_disable_unprepare(wcss.tcm_slave_cbcr);
    disable_ahbs_cbcr_clk:
    clk_disable_unprepare(wcss.ahbs_cbcr);
    disable_csr_cbcr_clk:
    clk_disable_unprepare(wcss.lcc_csr_cbcr);
    disable_ahbfabric_cbcr_clk:
    clk_disable_unprepare(wcss.ahbfabric_cbcr_clk);
    disable_gcc_abhs_cbcr_clk:
    clk_disable_unprepare(wcss.gcc_abhs_cbcr);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn q6v5_wcss_qcs404_reset(wcss: *mut q6v5_wcss) -> c_int {
    static inline int q6v5_wcss_qcs404_reset(struct q6v5_wcss *wcss)
    {
    unsigned long val;
    writel(0x80800000, wcss.reg_base + Q6SS_STRAP_ACC);
// Start core execution
    val = readl(wcss.reg_base + Q6SS_RESET_REG);
    val &= ~Q6SS_STOP_CORE;
    writel(val, wcss.reg_base + Q6SS_RESET_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_qcs404_wcss_start(rproc: *mut rproc) -> c_int {
    static int q6v5_qcs404_wcss_start(struct rproc *rproc)
    {
    struct q6v5_wcss *wcss = rproc.priv;
    int ret;
    ret = clk_prepare_enable(wcss.xo);
    if (ret)
    return ret;
    ret = regulator_enable(wcss.cx_supply);
    if (ret)
    goto disable_xo_clk;
    qcom_q6v5_prepare(&wcss.q6v5);
    ret = q6v5_wcss_qcs404_power_on(wcss);
    if (ret) {
    dev_err(wcss.dev, "wcss clk_enable failed\n");
    goto disable_cx_supply;
    }
    writel(rproc.bootaddr >> 4, wcss.reg_base + Q6SS_RST_EVB);
    q6v5_wcss_qcs404_reset(wcss);
    ret = qcom_q6v5_wait_for_start(&wcss.q6v5, 5 * HZ);
    if (ret == -ETIMEDOUT) {
    dev_err(wcss.dev, "start timed out\n");
    goto disable_cx_supply;
    }
    return 0;
    disable_cx_supply:
    regulator_disable(wcss.cx_supply);
    disable_xo_clk:
    clk_disable_unprepare(wcss.xo);
    return ret;
    }
    static void q6v5_wcss_halt_axi_port(struct q6v5_wcss *wcss,
    struct regmap *halt_map,
    u32 offset)
    {
    unsigned long timeout;
    unsigned int val;
    int ret;
// Check if we're already idle
    ret = regmap_read(halt_map, offset + AXI_IDLE_REG, &val);
    if (!ret && val)
    return;
// Assert halt request
    regmap_write(halt_map, offset + AXI_HALTREQ_REG, 1);
// Wait for halt
    timeout = jiffies + msecs_to_jiffies(HALT_ACK_TIMEOUT_MS);
    for (;;) {
    ret = regmap_read(halt_map, offset + AXI_HALTACK_REG, &val);
    if (ret || val || time_after(jiffies, timeout))
    break;
    msleep(1);
    }
    ret = regmap_read(halt_map, offset + AXI_IDLE_REG, &val);
    if (ret || !val)
    dev_err(wcss.dev, "port failed halt\n");
// Clear halt request (port will remain halted until reset)
    regmap_write(halt_map, offset + AXI_HALTREQ_REG, 0);
    }
#[no_mangle]
unsafe extern "C" fn q6v5_qcs404_wcss_shutdown(wcss: *mut q6v5_wcss) -> c_int {
    static int q6v5_qcs404_wcss_shutdown(struct q6v5_wcss *wcss)
    {
    unsigned long val;
    int ret;
    q6v5_wcss_halt_axi_port(wcss, wcss.halt_map, wcss.halt_wcss);
// assert clamps to avoid MX current inrush
    val = readl(wcss.reg_base + Q6SS_PWR_CTL_REG);
    val |= (Q6SS_CLAMP_IO | Q6SS_CLAMP_WL | Q6SS_CLAMP_QMC_MEM);
    writel(val, wcss.reg_base + Q6SS_PWR_CTL_REG);
// Disable memories by turning off memory foot/headswitch
    writel((readl(wcss.reg_base + Q6SS_MEM_PWR_CTL) &
    ~QDSS_Q6_MEMORIES),
    wcss.reg_base + Q6SS_MEM_PWR_CTL);
// Clear the BHS_ON bit
    val = readl(wcss.reg_base + Q6SS_PWR_CTL_REG);
    val &= ~Q6SS_BHS_ON;
    writel(val, wcss.reg_base + Q6SS_PWR_CTL_REG);
    clk_disable_unprepare(wcss.ahbfabric_cbcr_clk);
    clk_disable_unprepare(wcss.lcc_csr_cbcr);
    clk_disable_unprepare(wcss.tcm_slave_cbcr);
    clk_disable_unprepare(wcss.qdsp6ss_abhm_cbcr);
    clk_disable_unprepare(wcss.qdsp6ss_axim_cbcr);
    val = readl(wcss.reg_base + Q6SS_SLEEP_CBCR);
    val &= ~BIT(0);
    writel(val, wcss.reg_base + Q6SS_SLEEP_CBCR);
    val = readl(wcss.reg_base + Q6SS_XO_CBCR);
    val &= ~BIT(0);
    writel(val, wcss.reg_base + Q6SS_XO_CBCR);
    clk_disable_unprepare(wcss.ahbs_cbcr);
    clk_disable_unprepare(wcss.lcc_bcr_sleep);
    val = readl(wcss.reg_base + Q6SS_GFMUX_CTL_REG);
    val &= ~(Q6SS_CLK_ENABLE | Q6SS_SWITCH_CLK_SRC);
    writel(val, wcss.reg_base + Q6SS_GFMUX_CTL_REG);
    clk_disable_unprepare(wcss.gcc_abhs_cbcr);
    ret = reset_control_assert(wcss.wcss_reset);
    if (ret) {
    dev_err(wcss.dev, "wcss_reset failed\n");
    return ret;
    }
    usleep_range(200, 300);
    ret = reset_control_deassert(wcss.wcss_reset);
    if (ret) {
    dev_err(wcss.dev, "wcss_reset failed\n");
    return ret;
    }
    usleep_range(200, 300);
    clk_disable_unprepare(wcss.gcc_axim_cbcr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_wcss_powerdown(wcss: *mut q6v5_wcss) -> c_int {
    static int q6v5_wcss_powerdown(struct q6v5_wcss *wcss)
    {
    int ret;
    u32 val;
// 1 - Assert WCSS/Q6 HALTREQ
    q6v5_wcss_halt_axi_port(wcss, wcss.halt_map, wcss.halt_wcss);
// 2 - Enable WCSSAON_CONFIG
    val = readl(wcss.rmb_base + SSCAON_CONFIG);
    val |= SSCAON_ENABLE;
    writel(val, wcss.rmb_base + SSCAON_CONFIG);
// 3 - Set SSCAON_CONFIG
    val |= SSCAON_BUS_EN;
    val &= ~SSCAON_BUS_MUX_MASK;
    writel(val, wcss.rmb_base + SSCAON_CONFIG);
// 4 - SSCAON_CONFIG 1
    val |= BIT(1);
    writel(val, wcss.rmb_base + SSCAON_CONFIG);
// 5 - wait for SSCAON_STATUS
    ret = readl_poll_timeout(wcss.rmb_base + SSCAON_STATUS,
    val, (val & 0xffff) == 0x400, 1000,
    HALT_CHECK_MAX_LOOPS);
    if (ret) {
    dev_err(wcss.dev,
    "can't get SSCAON_STATUS rc:%d)\n", ret);
    return ret;
    }
// 6 - De-assert WCSS_AON reset
    reset_control_assert(wcss.wcss_aon_reset);
// 7 - Disable WCSSAON_CONFIG 13
    val = readl(wcss.rmb_base + SSCAON_CONFIG);
    val &= ~SSCAON_ENABLE;
    writel(val, wcss.rmb_base + SSCAON_CONFIG);
// 8 - De-assert WCSS/Q6 HALTREQ
    reset_control_assert(wcss.wcss_reset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_q6_powerdown(wcss: *mut q6v5_wcss) -> c_int {
    static int q6v5_q6_powerdown(struct q6v5_wcss *wcss)
    {
    int ret;
    u32 val;
    int i;
// 1 - Halt Q6 bus interface
    q6v5_wcss_halt_axi_port(wcss, wcss.halt_map, wcss.halt_q6);
// 2 - Disable Q6 Core clock
    val = readl(wcss.reg_base + Q6SS_GFMUX_CTL_REG);
    val &= ~Q6SS_CLK_ENABLE;
    writel(val, wcss.reg_base + Q6SS_GFMUX_CTL_REG);
// 3 - Clamp I/O
    val = readl(wcss.reg_base + Q6SS_PWR_CTL_REG);
    val |= Q6SS_CLAMP_IO;
    writel(val, wcss.reg_base + Q6SS_PWR_CTL_REG);
// 4 - Clamp WL
    val |= QDSS_BHS_ON;
    writel(val, wcss.reg_base + Q6SS_PWR_CTL_REG);
// 5 - Clear Erase standby
    val &= ~Q6SS_L2DATA_STBY_N;
    writel(val, wcss.reg_base + Q6SS_PWR_CTL_REG);
// 6 - Clear Sleep RTN
    val &= ~Q6SS_SLP_RET_N;
    writel(val, wcss.reg_base + Q6SS_PWR_CTL_REG);
// 7 - turn off Q6 memory foot/head switch one bank at a time
    for (i = 0; i < 20; i++) {
    val = readl(wcss.reg_base + Q6SS_MEM_PWR_CTL);
    val &= ~BIT(i);
    writel(val, wcss.reg_base + Q6SS_MEM_PWR_CTL);
    mdelay(1);
    }
// 8 - Assert QMC memory RTN
    val = readl(wcss.reg_base + Q6SS_PWR_CTL_REG);
    val |= Q6SS_CLAMP_QMC_MEM;
    writel(val, wcss.reg_base + Q6SS_PWR_CTL_REG);
// 9 - Turn off BHS
    val &= ~Q6SS_BHS_ON;
    writel(val, wcss.reg_base + Q6SS_PWR_CTL_REG);
    udelay(1);
// 10 - Wait till BHS Reset is done
    ret = readl_poll_timeout(wcss.reg_base + Q6SS_BHS_STATUS,
    val, !(val & BHS_EN_REST_ACK), 1000,
    HALT_CHECK_MAX_LOOPS);
    if (ret) {
    dev_err(wcss.dev, "BHS_STATUS not OFF (rc:%d)\n", ret);
    return ret;
    }
// 11 -  Assert WCSS reset
    reset_control_assert(wcss.wcss_reset);
// 12 - Assert Q6 reset
    reset_control_assert(wcss.wcss_q6_reset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_wcss_stop(rproc: *mut rproc) -> c_int {
    static int q6v5_wcss_stop(struct rproc *rproc)
    {
    struct q6v5_wcss *wcss = rproc.priv;
    int ret;
// WCSS powerdown
    if (wcss.requires_force_stop) {
    ret = qcom_q6v5_request_stop(&wcss.q6v5, core::ptr::null_mut());
    if (ret == -ETIMEDOUT) {
    dev_err(wcss.dev, "timed out on wait\n");
    return ret;
    }
    }
    if (wcss.version == WCSS_QCS404) {
    ret = q6v5_qcs404_wcss_shutdown(wcss);
    if (ret)
    return ret;
    } else {
    ret = q6v5_wcss_powerdown(wcss);
    if (ret)
    return ret;
// Q6 Power down
    ret = q6v5_q6_powerdown(wcss);
    if (ret)
    return ret;
    }
    qcom_q6v5_unprepare(&wcss.q6v5);
    return 0;
    }
    static void *q6v5_wcss_da_to_va(struct rproc *rproc, u64 da, size_t len, bool *is_iomem)
    {
    struct q6v5_wcss *wcss = rproc.priv;
    int offset;
    offset = da - wcss.mem_reloc;
    if (offset < 0 || offset + len > wcss.mem_size)
    return core::ptr::null_mut();
    return wcss.mem_region + offset;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_wcss_load(rproc: *mut rproc, fw: *const firmware) -> c_int {
    static int q6v5_wcss_load(struct rproc *rproc, const struct firmware *fw)
    {
    struct q6v5_wcss *wcss = rproc.priv;
    int ret;
    ret = qcom_mdt_load_no_init(wcss.dev, fw, rproc.firmware,
    wcss.mem_region, wcss.mem_phys,
    wcss.mem_size, &wcss.mem_reloc);
    if (ret)
    return ret;
    qcom_pil_info_store("wcnss", wcss.mem_phys, wcss.mem_size);
    return ret;
    }
    static const struct rproc_ops q6v5_wcss_ipq8074_ops = {
    .start = q6v5_wcss_start,
    .stop = q6v5_wcss_stop,
    .da_to_va = q6v5_wcss_da_to_va,
    .load = q6v5_wcss_load,
    .get_boot_addr = rproc_elf_get_boot_addr,
    };
    static const struct rproc_ops q6v5_wcss_qcs404_ops = {
    .start = q6v5_qcs404_wcss_start,
    .stop = q6v5_wcss_stop,
    .da_to_va = q6v5_wcss_da_to_va,
    .load = q6v5_wcss_load,
    .get_boot_addr = rproc_elf_get_boot_addr,
    .parse_fw = qcom_register_dump_segments,
    };
    static int q6v5_wcss_init_reset(struct q6v5_wcss *wcss,
    const struct wcss_data *desc)
    {
    struct device *dev = wcss.dev;
    if (desc.aon_reset_required) {
    wcss.wcss_aon_reset = devm_reset_control_get_exclusive(dev, "wcss_aon_reset");
    if (IS_ERR(wcss.wcss_aon_reset)) {
    dev_err(wcss.dev, "fail to acquire wcss_aon_reset\n");
    return PTR_ERR(wcss.wcss_aon_reset);
    }
    }
    wcss.wcss_reset = devm_reset_control_get_exclusive(dev, "wcss_reset");
    if (IS_ERR(wcss.wcss_reset)) {
    dev_err(wcss.dev, "unable to acquire wcss_reset\n");
    return PTR_ERR(wcss.wcss_reset);
    }
    wcss.wcss_q6_reset = devm_reset_control_get_exclusive(dev, "wcss_q6_reset");
    if (IS_ERR(wcss.wcss_q6_reset)) {
    dev_err(wcss.dev, "unable to acquire wcss_q6_reset\n");
    return PTR_ERR(wcss.wcss_q6_reset);
    }
    return 0;
    }
    static int q6v5_wcss_init_mmio(struct q6v5_wcss *wcss,
    struct platform_device *pdev)
    {
    unsigned int halt_reg[MAX_HALT_REG] = {0};
    struct device_node *syscon;
    struct resource *res;
    int ret;
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "qdsp6");
    if (!res)
    return -EINVAL;
    wcss.reg_base = devm_ioremap(&pdev.dev, res.start,
    resource_size(res));
    if (!wcss.reg_base)
    return -ENOMEM;
    if (wcss.version == WCSS_IPQ8074) {
    wcss.rmb_base = devm_platform_ioremap_resource_byname(pdev, "rmb");
    if (IS_ERR(wcss.rmb_base))
    return PTR_ERR(wcss.rmb_base);
    }
    syscon = of_parse_phandle(pdev.dev.of_node,
    "qcom,halt-regs", 0);
    if (!syscon) {
    dev_err(&pdev.dev, "failed to parse qcom,halt-regs\n");
    return -EINVAL;
    }
    wcss.halt_map = syscon_node_to_regmap(syscon);
    of_node_put(syscon);
    if (IS_ERR(wcss.halt_map))
    return PTR_ERR(wcss.halt_map);
    ret = of_property_read_variable_u32_array(pdev.dev.of_node,
    "qcom,halt-regs",
    halt_reg, 0,
    MAX_HALT_REG);
    if (ret < 0) {
    dev_err(&pdev.dev, "failed to parse qcom,halt-regs\n");
    return -EINVAL;
    }
    wcss.halt_q6 = halt_reg[1];
    wcss.halt_wcss = halt_reg[2];
    wcss.halt_nc = halt_reg[3];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_alloc_memory_region(wcss: *mut q6v5_wcss) -> c_int {
    static int q6v5_alloc_memory_region(struct q6v5_wcss *wcss)
    {
    struct device *dev = wcss.dev;
    struct resource res;
    int ret;
    ret = of_reserved_mem_region_to_resource(dev.of_node, 0, &res);
    if (ret) {
    dev_err(dev, "unable to acquire memory-region\n");
    return ret;
    }
    wcss.mem_phys = res.start;
    wcss.mem_reloc = res.start;
    wcss.mem_size = resource_size(&res);
    wcss.mem_region = devm_ioremap_resource_wc(dev, &res);
    if (IS_ERR(wcss.mem_region)) {
    dev_err(dev, "unable to map memory region: %pR\n", &res);
    return PTR_ERR(wcss.mem_region);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_wcss_init_clock(wcss: *mut q6v5_wcss) -> c_int {
    static int q6v5_wcss_init_clock(struct q6v5_wcss *wcss)
    {
    wcss.xo = devm_clk_get(wcss.dev, "xo");
    if (IS_ERR(wcss.xo))
    return dev_err_probe(wcss.dev, PTR_ERR(wcss.xo),
    "failed to get xo clock");
    wcss.gcc_abhs_cbcr = devm_clk_get(wcss.dev, "gcc_abhs_cbcr");
    if (IS_ERR(wcss.gcc_abhs_cbcr))
    return dev_err_probe(wcss.dev, PTR_ERR(wcss.gcc_abhs_cbcr),
    "failed to get gcc abhs clock");
    wcss.gcc_axim_cbcr = devm_clk_get(wcss.dev, "gcc_axim_cbcr");
    if (IS_ERR(wcss.gcc_axim_cbcr))
    return dev_err_probe(wcss.dev, PTR_ERR(wcss.gcc_axim_cbcr),
    "failed to get gcc axim clock\n");
    wcss.ahbfabric_cbcr_clk = devm_clk_get(wcss.dev,
    "lcc_ahbfabric_cbc");
    if (IS_ERR(wcss.ahbfabric_cbcr_clk))
    return dev_err_probe(wcss.dev, PTR_ERR(wcss.ahbfabric_cbcr_clk),
    "failed to get ahbfabric clock\n");
    wcss.lcc_csr_cbcr = devm_clk_get(wcss.dev, "tcsr_lcc_cbc");
    if (IS_ERR(wcss.lcc_csr_cbcr))
    return dev_err_probe(wcss.dev, PTR_ERR(wcss.lcc_csr_cbcr),
    "failed to get csr cbcr clk\n");
    wcss.ahbs_cbcr = devm_clk_get(wcss.dev,
    "lcc_abhs_cbc");
    if (IS_ERR(wcss.ahbs_cbcr))
    return dev_err_probe(wcss.dev, PTR_ERR(wcss.ahbs_cbcr),
    "failed to get ahbs_cbcr clk\n");
    wcss.tcm_slave_cbcr = devm_clk_get(wcss.dev,
    "lcc_tcm_slave_cbc");
    if (IS_ERR(wcss.tcm_slave_cbcr))
    return dev_err_probe(wcss.dev, PTR_ERR(wcss.tcm_slave_cbcr),
    "failed to get tcm cbcr clk\n");
    wcss.qdsp6ss_abhm_cbcr = devm_clk_get(wcss.dev, "lcc_abhm_cbc");
    if (IS_ERR(wcss.qdsp6ss_abhm_cbcr))
    return dev_err_probe(wcss.dev, PTR_ERR(wcss.qdsp6ss_abhm_cbcr),
    "failed to get abhm cbcr clk\n");
    wcss.qdsp6ss_axim_cbcr = devm_clk_get(wcss.dev, "lcc_axim_cbc");
    if (IS_ERR(wcss.qdsp6ss_axim_cbcr))
    return dev_err_probe(wcss.dev, PTR_ERR(wcss.qdsp6ss_axim_cbcr),
    "failed to get axim cbcr clk\n");
    wcss.lcc_bcr_sleep = devm_clk_get(wcss.dev, "lcc_bcr_sleep");
    if (IS_ERR(wcss.lcc_bcr_sleep))
    return dev_err_probe(wcss.dev, PTR_ERR(wcss.lcc_bcr_sleep),
    "failed to get bcr cbcr clk\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_wcss_init_regulator(wcss: *mut q6v5_wcss) -> c_int {
    static int q6v5_wcss_init_regulator(struct q6v5_wcss *wcss)
    {
    wcss.cx_supply = devm_regulator_get(wcss.dev, "cx");
    if (IS_ERR(wcss.cx_supply))
    return PTR_ERR(wcss.cx_supply);
    regulator_set_load(wcss.cx_supply, 100000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_wcss_probe(pdev: *mut platform_device) -> c_int {
    static int q6v5_wcss_probe(struct platform_device *pdev)
    {
    const struct wcss_data *desc;
    struct q6v5_wcss *wcss;
    struct rproc *rproc;
    int ret;
    desc = device_get_match_data(&pdev.dev);
    if (!desc)
    return -EINVAL;
    rproc = devm_rproc_alloc(&pdev.dev, pdev.name, desc.ops,
    desc.firmware_name, sizeof(*wcss));
    if (!rproc) {
    dev_err(&pdev.dev, "failed to allocate rproc\n");
    return -ENOMEM;
    }
    wcss = rproc.priv;
    wcss.dev = &pdev.dev;
    wcss.version = desc.version;
    wcss.requires_force_stop = desc.requires_force_stop;
    ret = q6v5_wcss_init_mmio(wcss, pdev);
    if (ret)
    return ret;
    ret = q6v5_alloc_memory_region(wcss);
    if (ret)
    return ret;
    if (wcss.version == WCSS_QCS404) {
    ret = q6v5_wcss_init_clock(wcss);
    if (ret)
    return ret;
    ret = q6v5_wcss_init_regulator(wcss);
    if (ret)
    return ret;
    }
    ret = q6v5_wcss_init_reset(wcss, desc);
    if (ret)
    return ret;
    ret = qcom_q6v5_init(&wcss.q6v5, pdev, rproc, desc.crash_reason_smem, core::ptr::null_mut(), core::ptr::null_mut());
    if (ret)
    return ret;
    qcom_add_glink_subdev(rproc, &wcss.glink_subdev, "q6wcss");
    qcom_add_pdm_subdev(rproc, &wcss.pdm_subdev);
    qcom_add_ssr_subdev(rproc, &wcss.ssr_subdev, "q6wcss");
    if (desc.ssctl_id) {
    wcss.sysmon = qcom_add_sysmon_subdev(rproc,
    desc.sysmon_name,
    desc.ssctl_id);
    if (IS_ERR(wcss.sysmon)) {
    ret = PTR_ERR(wcss.sysmon);
    goto deinit_remove_subdevs;
    }
    }
    ret = rproc_add(rproc);
    if (ret)
    goto remove_sysmon_subdev;
    platform_set_drvdata(pdev, rproc);
    return 0;
    remove_sysmon_subdev:
    if (desc.ssctl_id)
    qcom_remove_sysmon_subdev(wcss.sysmon);
    deinit_remove_subdevs:
    qcom_q6v5_deinit(&wcss.q6v5);
    qcom_remove_glink_subdev(rproc, &wcss.glink_subdev);
    qcom_remove_pdm_subdev(rproc, &wcss.pdm_subdev);
    qcom_remove_ssr_subdev(rproc, &wcss.ssr_subdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn q6v5_wcss_remove(pdev: *mut platform_device) {
    static void q6v5_wcss_remove(struct platform_device *pdev)
    {
    struct rproc *rproc = platform_get_drvdata(pdev);
    struct q6v5_wcss *wcss = rproc.priv;
    qcom_q6v5_deinit(&wcss.q6v5);
    qcom_remove_pdm_subdev(rproc, &wcss.pdm_subdev);
    rproc_del(rproc);
    }
    static const struct wcss_data wcss_ipq8074_res_init = {
    .firmware_name = "IPQ8074/q6_fw.mdt",
    .crash_reason_smem = WCSS_CRASH_REASON,
    .aon_reset_required = true,
    .ops = &q6v5_wcss_ipq8074_ops,
    .requires_force_stop = true,
    };
    static const struct wcss_data wcss_qcs404_res_init = {
    .crash_reason_smem = WCSS_CRASH_REASON,
    .firmware_name = "wcnss.mdt",
    .version = WCSS_QCS404,
    .aon_reset_required = false,
    .ssr_name = "mpss",
    .sysmon_name = "wcnss",
    .ssctl_id = 0x12,
    .ops = &q6v5_wcss_qcs404_ops,
    .requires_force_stop = false,
    };
    static const struct of_device_id q6v5_wcss_of_match[] = {
    { .compatible = "qcom,ipq8074-wcss-pil", .data = &wcss_ipq8074_res_init },
    { .compatible = "qcom,qcs404-wcss-pil", .data = &wcss_qcs404_res_init },
    { },
    };
    MODULE_DEVICE_TABLE(of, q6v5_wcss_of_match);
    static struct platform_driver q6v5_wcss_driver = {
    .probe = q6v5_wcss_probe,
    .remove = q6v5_wcss_remove,
    .driver = {
    .name = "qcom-q6v5-wcss-pil",
    .of_match_table = q6v5_wcss_of_match,
    },
    };
    module_platform_driver(q6v5_wcss_driver);
    MODULE_DESCRIPTION("Hexagon WCSS Peripheral Image Loader");
    MODULE_LICENSE("GPL v2");
