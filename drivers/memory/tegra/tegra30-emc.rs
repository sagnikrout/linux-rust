//! Automatically rewritten from C to Rust
//! Source: drivers/memory/tegra/tegra30-emc.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Tegra30 External Memory Controller driver
//
// Based on downstream driver from NVIDIA and tegra124-emc.c
// Copyright (C) 2011-2014 NVIDIA Corporation
//
// Author: Dmitry Osipenko <digetx@gmail.com>
// Copyright (C) 2019 GRATE-DRIVER project
//

pub const EMC_INTSTATUS: c_uint = 0x000;
pub const EMC_INTMASK: c_uint = 0x004;
pub const EMC_DBG: c_uint = 0x008;
pub const EMC_ADR_CFG: c_uint = 0x010;
pub const EMC_CFG: c_uint = 0x00c;
pub const EMC_REFCTRL: c_uint = 0x020;
pub const EMC_TIMING_CONTROL: c_uint = 0x028;
pub const EMC_RC: c_uint = 0x02c;
pub const EMC_RFC: c_uint = 0x030;
pub const EMC_RAS: c_uint = 0x034;
pub const EMC_RP: c_uint = 0x038;
pub const EMC_R2W: c_uint = 0x03c;
pub const EMC_W2R: c_uint = 0x040;
pub const EMC_R2P: c_uint = 0x044;
pub const EMC_W2P: c_uint = 0x048;
pub const EMC_RD_RCD: c_uint = 0x04c;
pub const EMC_WR_RCD: c_uint = 0x050;
pub const EMC_RRD: c_uint = 0x054;
pub const EMC_REXT: c_uint = 0x058;
pub const EMC_WDV: c_uint = 0x05c;
pub const EMC_QUSE: c_uint = 0x060;
pub const EMC_QRST: c_uint = 0x064;
pub const EMC_QSAFE: c_uint = 0x068;
pub const EMC_RDV: c_uint = 0x06c;
pub const EMC_REFRESH: c_uint = 0x070;
pub const EMC_BURST_REFRESH_NUM: c_uint = 0x074;
pub const EMC_PDEX2WR: c_uint = 0x078;
pub const EMC_PDEX2RD: c_uint = 0x07c;
pub const EMC_PCHG2PDEN: c_uint = 0x080;
pub const EMC_ACT2PDEN: c_uint = 0x084;
pub const EMC_AR2PDEN: c_uint = 0x088;
pub const EMC_RW2PDEN: c_uint = 0x08c;
pub const EMC_TXSR: c_uint = 0x090;
pub const EMC_TCKE: c_uint = 0x094;
pub const EMC_TFAW: c_uint = 0x098;
pub const EMC_TRPAB: c_uint = 0x09c;
pub const EMC_TCLKSTABLE: c_uint = 0x0a0;
pub const EMC_TCLKSTOP: c_uint = 0x0a4;
pub const EMC_TREFBW: c_uint = 0x0a8;
pub const EMC_QUSE_EXTRA: c_uint = 0x0ac;
pub const EMC_ODT_WRITE: c_uint = 0x0b0;
pub const EMC_ODT_READ: c_uint = 0x0b4;
pub const EMC_WEXT: c_uint = 0x0b8;
pub const EMC_CTT: c_uint = 0x0bc;
pub const EMC_MRS_WAIT_CNT: c_uint = 0x0c8;
pub const EMC_MRS: c_uint = 0x0cc;
pub const EMC_EMRS: c_uint = 0x0d0;
pub const EMC_SELF_REF: c_uint = 0x0e0;
pub const EMC_MRW: c_uint = 0x0e8;
pub const EMC_MRR: c_uint = 0x0ec;
pub const EMC_XM2DQSPADCTRL3: c_uint = 0x0f8;
pub const EMC_FBIO_SPARE: c_uint = 0x100;
pub const EMC_FBIO_CFG5: c_uint = 0x104;
pub const EMC_FBIO_CFG6: c_uint = 0x114;
pub const EMC_CFG_RSV: c_uint = 0x120;
pub const EMC_AUTO_CAL_CONFIG: c_uint = 0x2a4;
pub const EMC_AUTO_CAL_INTERVAL: c_uint = 0x2a8;
pub const EMC_AUTO_CAL_STATUS: c_uint = 0x2ac;
pub const EMC_STATUS: c_uint = 0x2b4;
pub const EMC_CFG_2: c_uint = 0x2b8;
pub const EMC_CFG_DIG_DLL: c_uint = 0x2bc;
pub const EMC_CFG_DIG_DLL_PERIOD: c_uint = 0x2c0;
pub const EMC_CTT_DURATION: c_uint = 0x2d8;
pub const EMC_CTT_TERM_CTRL: c_uint = 0x2dc;
pub const EMC_ZCAL_INTERVAL: c_uint = 0x2e0;
pub const EMC_ZCAL_WAIT_CNT: c_uint = 0x2e4;
pub const EMC_ZQ_CAL: c_uint = 0x2ec;
pub const EMC_XM2CMDPADCTRL: c_uint = 0x2f0;
pub const EMC_XM2DQSPADCTRL2: c_uint = 0x2fc;
pub const EMC_XM2DQPADCTRL2: c_uint = 0x304;
pub const EMC_XM2CLKPADCTRL: c_uint = 0x308;
pub const EMC_XM2COMPPADCTRL: c_uint = 0x30c;
pub const EMC_XM2VTTGENPADCTRL: c_uint = 0x310;
pub const EMC_XM2VTTGENPADCTRL2: c_uint = 0x314;
pub const EMC_XM2QUSEPADCTRL: c_uint = 0x318;
pub const EMC_DLL_XFORM_DQS0: c_uint = 0x328;
pub const EMC_DLL_XFORM_DQS1: c_uint = 0x32c;
pub const EMC_DLL_XFORM_DQS2: c_uint = 0x330;
pub const EMC_DLL_XFORM_DQS3: c_uint = 0x334;
pub const EMC_DLL_XFORM_DQS4: c_uint = 0x338;
pub const EMC_DLL_XFORM_DQS5: c_uint = 0x33c;
pub const EMC_DLL_XFORM_DQS6: c_uint = 0x340;
pub const EMC_DLL_XFORM_DQS7: c_uint = 0x344;
pub const EMC_DLL_XFORM_QUSE0: c_uint = 0x348;
pub const EMC_DLL_XFORM_QUSE1: c_uint = 0x34c;
pub const EMC_DLL_XFORM_QUSE2: c_uint = 0x350;
pub const EMC_DLL_XFORM_QUSE3: c_uint = 0x354;
pub const EMC_DLL_XFORM_QUSE4: c_uint = 0x358;
pub const EMC_DLL_XFORM_QUSE5: c_uint = 0x35c;
pub const EMC_DLL_XFORM_QUSE6: c_uint = 0x360;
pub const EMC_DLL_XFORM_QUSE7: c_uint = 0x364;
pub const EMC_DLL_XFORM_DQ0: c_uint = 0x368;
pub const EMC_DLL_XFORM_DQ1: c_uint = 0x36c;
pub const EMC_DLL_XFORM_DQ2: c_uint = 0x370;
pub const EMC_DLL_XFORM_DQ3: c_uint = 0x374;
pub const EMC_DLI_TRIM_TXDQS0: c_uint = 0x3a8;
pub const EMC_DLI_TRIM_TXDQS1: c_uint = 0x3ac;
pub const EMC_DLI_TRIM_TXDQS2: c_uint = 0x3b0;
pub const EMC_DLI_TRIM_TXDQS3: c_uint = 0x3b4;
pub const EMC_DLI_TRIM_TXDQS4: c_uint = 0x3b8;
pub const EMC_DLI_TRIM_TXDQS5: c_uint = 0x3bc;
pub const EMC_DLI_TRIM_TXDQS6: c_uint = 0x3c0;
pub const EMC_DLI_TRIM_TXDQS7: c_uint = 0x3c4;
pub const EMC_STALL_THEN_EXE_BEFORE_CLKCHANGE: c_uint = 0x3c8;
pub const EMC_STALL_THEN_EXE_AFTER_CLKCHANGE: c_uint = 0x3cc;
pub const EMC_UNSTALL_RW_AFTER_CLKCHANGE: c_uint = 0x3d0;
pub const EMC_SEL_DPD_CTRL: c_uint = 0x3d8;
pub const EMC_PRE_REFRESH_REQ_CNT: c_uint = 0x3dc;
pub const EMC_DYN_SELF_REF_CONTROL: c_uint = 0x3e0;
pub const EMC_TXSRDLL: c_uint = 0x3e4;

    ((num) > 1 ? DRAM_DEV_SEL_ALL : DRAM_DEV_SEL_0)

    (DRAM_DEV_SEL_0 | EMC_ZQ_CAL_LONG | EMC_ZQ_CAL_CMD)

    (DRAM_DEV_SEL_1 | EMC_ZQ_CAL_LONG | EMC_ZQ_CAL_CMD)

pub const EMC_CFG5_QUSE_MODE_SHIFT: c_int = 13;

pub const EMC_CFG5_QUSE_MODE_INTERNAL_LPBK: c_int = 2;
pub const EMC_CFG5_QUSE_MODE_PULSE_INTERN: c_int = 3;

pub const EMC_FBIO_CFG5_DRAM_TYPE_MASK: c_uint = 0x3;
pub const EMC_MRS_WAIT_CNT_SHORT_WAIT_MASK: c_uint = 0x3ff;
pub const EMC_MRS_WAIT_CNT_LONG_WAIT_SHIFT: c_int = 16;

    (0x3ff << EMC_MRS_WAIT_CNT_LONG_WAIT_SHIFT)
pub const EMC_REFCTRL_DEV_SEL_MASK: c_uint = 0x3;

    (((num) > 1 ? 0 : 2) | EMC_REFCTRL_ENABLE)

    enum emc_dram_type {
    DRAM_TYPE_DDR3,
    DRAM_TYPE_DDR1,
    DRAM_TYPE_LPDDR2,
    DRAM_TYPE_DDR2,
    };
    enum emc_dll_change {
    DLL_CHANGE_NONE,
    DLL_CHANGE_ON,
    DLL_CHANGE_OFF
    };
    static const u16 emc_timing_registers[] = {
    [0] = EMC_RC,
    [1] = EMC_RFC,
    [2] = EMC_RAS,
    [3] = EMC_RP,
    [4] = EMC_R2W,
    [5] = EMC_W2R,
    [6] = EMC_R2P,
    [7] = EMC_W2P,
    [8] = EMC_RD_RCD,
    [9] = EMC_WR_RCD,
    [10] = EMC_RRD,
    [11] = EMC_REXT,
    [12] = EMC_WEXT,
    [13] = EMC_WDV,
    [14] = EMC_QUSE,
    [15] = EMC_QRST,
    [16] = EMC_QSAFE,
    [17] = EMC_RDV,
    [18] = EMC_REFRESH,
    [19] = EMC_BURST_REFRESH_NUM,
    [20] = EMC_PRE_REFRESH_REQ_CNT,
    [21] = EMC_PDEX2WR,
    [22] = EMC_PDEX2RD,
    [23] = EMC_PCHG2PDEN,
    [24] = EMC_ACT2PDEN,
    [25] = EMC_AR2PDEN,
    [26] = EMC_RW2PDEN,
    [27] = EMC_TXSR,
    [28] = EMC_TXSRDLL,
    [29] = EMC_TCKE,
    [30] = EMC_TFAW,
    [31] = EMC_TRPAB,
    [32] = EMC_TCLKSTABLE,
    [33] = EMC_TCLKSTOP,
    [34] = EMC_TREFBW,
    [35] = EMC_QUSE_EXTRA,
    [36] = EMC_FBIO_CFG6,
    [37] = EMC_ODT_WRITE,
    [38] = EMC_ODT_READ,
    [39] = EMC_FBIO_CFG5,
    [40] = EMC_CFG_DIG_DLL,
    [41] = EMC_CFG_DIG_DLL_PERIOD,
    [42] = EMC_DLL_XFORM_DQS0,
    [43] = EMC_DLL_XFORM_DQS1,
    [44] = EMC_DLL_XFORM_DQS2,
    [45] = EMC_DLL_XFORM_DQS3,
    [46] = EMC_DLL_XFORM_DQS4,
    [47] = EMC_DLL_XFORM_DQS5,
    [48] = EMC_DLL_XFORM_DQS6,
    [49] = EMC_DLL_XFORM_DQS7,
    [50] = EMC_DLL_XFORM_QUSE0,
    [51] = EMC_DLL_XFORM_QUSE1,
    [52] = EMC_DLL_XFORM_QUSE2,
    [53] = EMC_DLL_XFORM_QUSE3,
    [54] = EMC_DLL_XFORM_QUSE4,
    [55] = EMC_DLL_XFORM_QUSE5,
    [56] = EMC_DLL_XFORM_QUSE6,
    [57] = EMC_DLL_XFORM_QUSE7,
    [58] = EMC_DLI_TRIM_TXDQS0,
    [59] = EMC_DLI_TRIM_TXDQS1,
    [60] = EMC_DLI_TRIM_TXDQS2,
    [61] = EMC_DLI_TRIM_TXDQS3,
    [62] = EMC_DLI_TRIM_TXDQS4,
    [63] = EMC_DLI_TRIM_TXDQS5,
    [64] = EMC_DLI_TRIM_TXDQS6,
    [65] = EMC_DLI_TRIM_TXDQS7,
    [66] = EMC_DLL_XFORM_DQ0,
    [67] = EMC_DLL_XFORM_DQ1,
    [68] = EMC_DLL_XFORM_DQ2,
    [69] = EMC_DLL_XFORM_DQ3,
    [70] = EMC_XM2CMDPADCTRL,
    [71] = EMC_XM2DQSPADCTRL2,
    [72] = EMC_XM2DQPADCTRL2,
    [73] = EMC_XM2CLKPADCTRL,
    [74] = EMC_XM2COMPPADCTRL,
    [75] = EMC_XM2VTTGENPADCTRL,
    [76] = EMC_XM2VTTGENPADCTRL2,
    [77] = EMC_XM2QUSEPADCTRL,
    [78] = EMC_XM2DQSPADCTRL3,
    [79] = EMC_CTT_TERM_CTRL,
    [80] = EMC_ZCAL_INTERVAL,
    [81] = EMC_ZCAL_WAIT_CNT,
    [82] = EMC_MRS_WAIT_CNT,
    [83] = EMC_AUTO_CAL_CONFIG,
    [84] = EMC_CTT,
    [85] = EMC_CTT_DURATION,
    [86] = EMC_DYN_SELF_REF_CONTROL,
    [87] = EMC_FBIO_SPARE,
    [88] = EMC_CFG_RSV,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emc_timing {
    pub rate: c_ulong,
    pub data: [u32; ARRAY_SIZE(emc_timing_registers)],
    pub emc_auto_cal_interval: u32,
    pub emc_mode_1: u32,
    pub emc_mode_2: u32,
    pub emc_mode_reset: u32,
    pub emc_zcal_cnt_long: u32,
    pub emc_cfg_periodic_qrst: bool,
    pub emc_cfg_dyn_self_ref: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_emc {
    pub dev: *mut device,
    pub mc: *mut tegra_mc,
    pub provider: icc_provider,
    pub clk_nb: notifier_block,
    pub clk: *mut clk,
    pub regs: *mut void __iomem,
    pub irq: c_uint,
    pub bad_state: bool,
    pub new_timing: *mut emc_timing,
    pub timings: *mut emc_timing,
    pub num_timings: c_uint,
    pub mc_override: u32,
    pub emc_cfg: u32,
    pub emc_mode_1: u32,
    pub emc_mode_2: u32,
    pub emc_mode_reset: u32,
    pub 1: bool vref_cal_toggle :,
    pub 1: bool zcal_long :,
    pub 1: bool dll_on :,
    struct {
    pub root: *mut dentry,
    pub min_rate: c_ulong,
    pub max_rate: c_ulong,
    pub debugfs: },
    pub reqs: tegra_emc_rate_requests,
    pub mrr_error: bool,
}

#[no_mangle]
unsafe extern "C" fn emc_seq_update_timing(emc: *mut tegra_emc) -> c_int {
    static int emc_seq_update_timing(struct tegra_emc *emc)
    {
    u32 val;
    int err;
    writel_relaxed(EMC_TIMING_UPDATE, emc.regs + EMC_TIMING_CONTROL);
    err = readl_relaxed_poll_timeout_atomic(emc.regs + EMC_STATUS, val,
    !(val & EMC_STATUS_TIMING_UPDATE_STALLED),
    1, 200);
    if (err) {
    dev_err(emc.dev, "failed to update timing: %d\n", err);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra30_emc_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t tegra30_emc_isr(int irq, void *data)
    {
    struct tegra_emc *emc = data;
    let mut intmask: u32 = EMC_REFRESH_OVERFLOW_INT;
    u32 status;
    status = readl_relaxed(emc.regs + EMC_INTSTATUS) & intmask;
    if (!status)
    return IRQ_NONE;
// notify about HW problem
    if (status & EMC_REFRESH_OVERFLOW_INT)
    dev_err_ratelimited(emc.dev,
    "refresh request overflow timeout\n");
// clear interrupts
    writel_relaxed(status, emc.regs + EMC_INTSTATUS);
    return IRQ_HANDLED;
    }
    static struct emc_timing *emc_find_timing(struct tegra_emc *emc,
    unsigned long rate)
    {
    struct emc_timing *timing = core::ptr::null_mut();
    unsigned int i;
    for (i = 0; i < emc.num_timings; i++) {
    if (emc.timings[i].rate >= rate) {
    timing = &emc.timings[i];
    break;
    }
    }
    if (!timing) {
    dev_err(emc.dev, "no timing for rate %lu\n", rate);
    return core::ptr::null_mut();
    }
    return timing;
    }
    static bool emc_dqs_preset(struct tegra_emc *emc, struct emc_timing *timing,
    bool *schmitt_to_vref)
    {
    let mut preset: bool = false;
    u32 val;
    if (timing.data[71] & EMC_XM2DQSPADCTRL2_VREF_ENABLE) {
    val = readl_relaxed(emc.regs + EMC_XM2DQSPADCTRL2);
    if (!(val & EMC_XM2DQSPADCTRL2_VREF_ENABLE)) {
    val |= EMC_XM2DQSPADCTRL2_VREF_ENABLE;
    writel_relaxed(val, emc.regs + EMC_XM2DQSPADCTRL2);
    preset = true;
    }
    }
    if (timing.data[78] & EMC_XM2DQSPADCTRL3_VREF_ENABLE) {
    val = readl_relaxed(emc.regs + EMC_XM2DQSPADCTRL3);
    if (!(val & EMC_XM2DQSPADCTRL3_VREF_ENABLE)) {
    val |= EMC_XM2DQSPADCTRL3_VREF_ENABLE;
    writel_relaxed(val, emc.regs + EMC_XM2DQSPADCTRL3);
    preset = true;
    }
    }
    if (timing.data[77] & EMC_XM2QUSEPADCTRL_IVREF_ENABLE) {
    val = readl_relaxed(emc.regs + EMC_XM2QUSEPADCTRL);
    if (!(val & EMC_XM2QUSEPADCTRL_IVREF_ENABLE)) {
    val |= EMC_XM2QUSEPADCTRL_IVREF_ENABLE;
    writel_relaxed(val, emc.regs + EMC_XM2QUSEPADCTRL);
// schmitt_to_vref = true;
    preset = true;
    }
    }
    return preset;
    }
#[no_mangle]
unsafe extern "C" fn emc_prepare_mc_clk_cfg(emc: *mut tegra_emc, rate: c_ulong) -> c_int {
    static int emc_prepare_mc_clk_cfg(struct tegra_emc *emc, unsigned long rate)
    {
    struct tegra_mc *mc = emc.mc;
    let mut misc0_index: c_uint = 16;
    unsigned int i;
    bool same;
    for (i = 0; i < mc.num_timings; i++) {
    if (mc.timings[i].rate != rate)
    continue;
    if (mc.timings[i].emem_data[misc0_index] & BIT(27))
    same = true;
    else
    same = false;
    return tegra20_clk_prepare_emc_mc_same_freq(emc.clk, same);
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn emc_prepare_timing_change(emc: *mut tegra_emc, rate: c_ulong) -> c_int {
    static int emc_prepare_timing_change(struct tegra_emc *emc, unsigned long rate)
    {
    struct emc_timing *timing = emc_find_timing(emc, rate);
    enum emc_dll_change dll_change;
    enum emc_dram_type dram_type;
    let mut schmitt_to_vref: bool = false;
    let mut pre_wait: c_uint = 0;
    let mut qrst_used: bool = false;
    unsigned int dram_num;
    unsigned int i;
    u32 fbio_cfg5;
    u32 emc_dbg;
    u32 val;
    int err;
    if (!timing || emc.bad_state)
    return -EINVAL;
    dev_dbg(emc.dev, "%s: using timing rate %lu for requested rate %lu\n",
    __func__, timing.rate, rate);
    emc.bad_state = true;
    err = emc_prepare_mc_clk_cfg(emc, rate);
    if (err) {
    dev_err(emc.dev, "mc clock preparation failed: %d\n", err);
    return err;
    }
    emc.vref_cal_toggle = false;
    emc.mc_override = mc_readl(emc.mc, MC_EMEM_ARB_OVERRIDE);
    emc.emc_cfg = readl_relaxed(emc.regs + EMC_CFG);
    emc_dbg = readl_relaxed(emc.regs + EMC_DBG);
    if (emc.dll_on == !(timing.emc_mode_1 & 0x1))
    dll_change = DLL_CHANGE_NONE;
#[no_mangle]
pub unsafe extern "C" fn if(0x1): !(timing->emc_mode_1 &) -> else {
    else if (!(timing.emc_mode_1 & 0x1))
    dll_change = DLL_CHANGE_ON;
    else
    dll_change = DLL_CHANGE_OFF;
    emc.dll_on = !(timing.emc_mode_1 & 0x1);
    if (timing.data[80] && !readl_relaxed(emc.regs + EMC_ZCAL_INTERVAL))
    emc.zcal_long = true;
    else
    emc.zcal_long = false;
    fbio_cfg5 = readl_relaxed(emc.regs + EMC_FBIO_CFG5);
    dram_type = fbio_cfg5 & EMC_FBIO_CFG5_DRAM_TYPE_MASK;
    dram_num = tegra_mc_get_emem_device_count(emc.mc);
// disable dynamic self-refresh
    if (emc.emc_cfg & EMC_CFG_DYN_SREF_ENABLE) {
    emc.emc_cfg &= ~EMC_CFG_DYN_SREF_ENABLE;
    writel_relaxed(emc.emc_cfg, emc.regs + EMC_CFG);
    pre_wait = 5;
    }
// update MC arbiter settings
    val = mc_readl(emc.mc, MC_EMEM_ARB_OUTSTANDING_REQ);
    if (!(val & MC_EMEM_ARB_OUTSTANDING_REQ_HOLDOFF_OVERRIDE) ||
    ((val & MC_EMEM_ARB_OUTSTANDING_REQ_MAX_MASK) > 0x50)) {
    val = MC_EMEM_ARB_OUTSTANDING_REQ_LIMIT_ENABLE |
    MC_EMEM_ARB_OUTSTANDING_REQ_HOLDOFF_OVERRIDE | 0x50;
    mc_writel(emc.mc, val, MC_EMEM_ARB_OUTSTANDING_REQ);
    mc_writel(emc.mc, MC_TIMING_UPDATE, MC_TIMING_CONTROL);
    }
    if (emc.mc_override & MC_EMEM_ARB_OVERRIDE_EACK_MASK)
    mc_writel(emc.mc,
    emc.mc_override & ~MC_EMEM_ARB_OVERRIDE_EACK_MASK,
    MC_EMEM_ARB_OVERRIDE);
// check DQ/DQS VREF delay
    if (emc_dqs_preset(emc, timing, &schmitt_to_vref)) {
    if (pre_wait < 3)
    pre_wait = 3;
    }
    if (pre_wait) {
    err = emc_seq_update_timing(emc);
    if (err)
    return err;
    udelay(pre_wait);
    }
// disable auto-calibration if VREF mode is switching
    if (timing.emc_auto_cal_interval) {
    val = readl_relaxed(emc.regs + EMC_XM2COMPPADCTRL);
    val ^= timing.data[74];
    if (val & EMC_XM2COMPPADCTRL_VREF_CAL_ENABLE) {
    writel_relaxed(0, emc.regs + EMC_AUTO_CAL_INTERVAL);
    err = readl_relaxed_poll_timeout_atomic(
    emc.regs + EMC_AUTO_CAL_STATUS, val,
    !(val & EMC_AUTO_CAL_STATUS_ACTIVE), 1, 300);
    if (err) {
    dev_err(emc.dev,
    "auto-cal finish timeout: %d\n", err);
    return err;
    }
    emc.vref_cal_toggle = true;
    }
    }
// program shadow registers
    for (i = 0; i < ARRAY_SIZE(timing.data); i++) {
// EMC_XM2CLKPADCTRL should be programmed separately
    if (i != 73)
    writel_relaxed(timing.data[i],
    emc.regs + emc_timing_registers[i]);
    }
    err = tegra_mc_write_emem_configuration(emc.mc, timing.rate);
    if (err)
    return err;
// DDR3: predict MRS long wait count
    if (dram_type == DRAM_TYPE_DDR3 && dll_change == DLL_CHANGE_ON) {
    let mut cnt: u32 = 512;
    if (emc.zcal_long)
    cnt -= dram_num * 256;
    val = timing.data[82] & EMC_MRS_WAIT_CNT_SHORT_WAIT_MASK;
    if (cnt < val)
    cnt = val;
    val = timing.data[82] & ~EMC_MRS_WAIT_CNT_LONG_WAIT_MASK;
    val |= (cnt << EMC_MRS_WAIT_CNT_LONG_WAIT_SHIFT) &
    EMC_MRS_WAIT_CNT_LONG_WAIT_MASK;
    writel_relaxed(val, emc.regs + EMC_MRS_WAIT_CNT);
    }
// this read also completes the writes
    val = readl_relaxed(emc.regs + EMC_SEL_DPD_CTRL);
    if (!(val & EMC_SEL_DPD_CTRL_QUSE_DPD_ENABLE) && schmitt_to_vref) {
    u32 cur_mode, new_mode;
    cur_mode = fbio_cfg5 & EMC_CFG5_QUSE_MODE_MASK;
    cur_mode >>= EMC_CFG5_QUSE_MODE_SHIFT;
    new_mode = timing.data[39] & EMC_CFG5_QUSE_MODE_MASK;
    new_mode >>= EMC_CFG5_QUSE_MODE_SHIFT;
    if ((cur_mode != EMC_CFG5_QUSE_MODE_PULSE_INTERN &&
    cur_mode != EMC_CFG5_QUSE_MODE_INTERNAL_LPBK) ||
    (new_mode != EMC_CFG5_QUSE_MODE_PULSE_INTERN &&
    new_mode != EMC_CFG5_QUSE_MODE_INTERNAL_LPBK))
    qrst_used = true;
    }
// flow control marker 1
    writel_relaxed(0x1, emc.regs + EMC_STALL_THEN_EXE_BEFORE_CLKCHANGE);
// enable periodic reset
    if (qrst_used) {
    writel_relaxed(emc_dbg | EMC_DBG_WRITE_MUX_ACTIVE,
    emc.regs + EMC_DBG);
    writel_relaxed(emc.emc_cfg | EMC_CFG_PERIODIC_QRST,
    emc.regs + EMC_CFG);
    writel_relaxed(emc_dbg, emc.regs + EMC_DBG);
    }
// disable auto-refresh to save time after clock change
    writel_relaxed(EMC_REFCTRL_DISABLE_ALL(dram_num),
    emc.regs + EMC_REFCTRL);
// turn off DLL and enter self-refresh on DDR3
    if (dram_type == DRAM_TYPE_DDR3) {
    if (dll_change == DLL_CHANGE_OFF)
    writel_relaxed(timing.emc_mode_1,
    emc.regs + EMC_EMRS);
    writel_relaxed(DRAM_BROADCAST(dram_num) |
    EMC_SELF_REF_CMD_ENABLED,
    emc.regs + EMC_SELF_REF);
    }
// flow control marker 2
    writel_relaxed(0x1, emc.regs + EMC_STALL_THEN_EXE_AFTER_CLKCHANGE);
// enable write-active MUX, update unshadowed pad control
    writel_relaxed(emc_dbg | EMC_DBG_WRITE_MUX_ACTIVE, emc.regs + EMC_DBG);
    writel_relaxed(timing.data[73], emc.regs + EMC_XM2CLKPADCTRL);
// restore periodic QRST and disable write-active MUX
    val = !!(emc.emc_cfg & EMC_CFG_PERIODIC_QRST);
    if (qrst_used || timing.emc_cfg_periodic_qrst != val) {
    if (timing.emc_cfg_periodic_qrst)
    emc.emc_cfg |= EMC_CFG_PERIODIC_QRST;
    else
    emc.emc_cfg &= ~EMC_CFG_PERIODIC_QRST;
    writel_relaxed(emc.emc_cfg, emc.regs + EMC_CFG);
    }
    writel_relaxed(emc_dbg, emc.regs + EMC_DBG);
// exit self-refresh on DDR3
    if (dram_type == DRAM_TYPE_DDR3)
    writel_relaxed(DRAM_BROADCAST(dram_num),
    emc.regs + EMC_SELF_REF);
// set DRAM-mode registers
    if (dram_type == DRAM_TYPE_DDR3) {
    if (timing.emc_mode_1 != emc.emc_mode_1)
    writel_relaxed(timing.emc_mode_1,
    emc.regs + EMC_EMRS);
    if (timing.emc_mode_2 != emc.emc_mode_2)
    writel_relaxed(timing.emc_mode_2,
    emc.regs + EMC_EMRS);
    if (timing.emc_mode_reset != emc.emc_mode_reset ||
    dll_change == DLL_CHANGE_ON) {
    val = timing.emc_mode_reset;
    if (dll_change == DLL_CHANGE_ON) {
    val |= EMC_MODE_SET_DLL_RESET;
    val |= EMC_MODE_SET_LONG_CNT;
    } else {
    val &= ~EMC_MODE_SET_DLL_RESET;
    }
    writel_relaxed(val, emc.regs + EMC_MRS);
    }
    } else {
    if (timing.emc_mode_2 != emc.emc_mode_2)
    writel_relaxed(timing.emc_mode_2,
    emc.regs + EMC_MRW);
    if (timing.emc_mode_1 != emc.emc_mode_1)
    writel_relaxed(timing.emc_mode_1,
    emc.regs + EMC_MRW);
    }
    emc.emc_mode_1 = timing.emc_mode_1;
    emc.emc_mode_2 = timing.emc_mode_2;
    emc.emc_mode_reset = timing.emc_mode_reset;
// issue ZCAL command if turning ZCAL on
    if (emc.zcal_long) {
    writel_relaxed(EMC_ZQ_CAL_LONG_CMD_DEV0,
    emc.regs + EMC_ZQ_CAL);
    if (dram_num > 1)
    writel_relaxed(EMC_ZQ_CAL_LONG_CMD_DEV1,
    emc.regs + EMC_ZQ_CAL);
    }
// flow control marker 3
    writel_relaxed(0x1, emc.regs + EMC_UNSTALL_RW_AFTER_CLKCHANGE);
//
// Read and discard an arbitrary MC register (Note: EMC registers
// can't be used) to ensure the register writes are completed.
//
    mc_readl(emc.mc, MC_EMEM_ARB_OVERRIDE);
    return 0;
    }
    static int emc_complete_timing_change(struct tegra_emc *emc,
    unsigned long rate)
    {
    struct emc_timing *timing = emc_find_timing(emc, rate);
    unsigned int dram_num;
    int err;
    u32 v;
    err = readl_relaxed_poll_timeout_atomic(emc.regs + EMC_INTSTATUS, v,
    v & EMC_CLKCHANGE_COMPLETE_INT,
    1, 100);
    if (err) {
    dev_err(emc.dev, "emc-car handshake timeout: %d\n", err);
    return err;
    }
// re-enable auto-refresh
    dram_num = tegra_mc_get_emem_device_count(emc.mc);
    writel_relaxed(EMC_REFCTRL_ENABLE_ALL(dram_num),
    emc.regs + EMC_REFCTRL);
// restore auto-calibration
    if (emc.vref_cal_toggle)
    writel_relaxed(timing.emc_auto_cal_interval,
    emc.regs + EMC_AUTO_CAL_INTERVAL);
// restore dynamic self-refresh
    if (timing.emc_cfg_dyn_self_ref) {
    emc.emc_cfg |= EMC_CFG_DYN_SREF_ENABLE;
    writel_relaxed(emc.emc_cfg, emc.regs + EMC_CFG);
    }
// set number of clocks to wait after each ZQ command
    if (emc.zcal_long)
    writel_relaxed(timing.emc_zcal_cnt_long,
    emc.regs + EMC_ZCAL_WAIT_CNT);
// wait for writes to settle
    udelay(2);
// update restored timing
    err = emc_seq_update_timing(emc);
    if (!err)
    emc.bad_state = false;
// restore early ACK
    mc_writel(emc.mc, emc.mc_override, MC_EMEM_ARB_OVERRIDE);
    return err;
    }
    static int emc_unprepare_timing_change(struct tegra_emc *emc,
    unsigned long rate)
    {
    if (!emc.bad_state) {
// shouldn't ever happen in practice
    dev_err(emc.dev, "timing configuration can't be reverted\n");
    emc.bad_state = true;
    }
    return 0;
    }
    static int emc_clk_change_notify(struct notifier_block *nb,
    unsigned long msg, void *data)
    {
    struct tegra_emc *emc = container_of(nb, struct tegra_emc, clk_nb);
    struct clk_notifier_data *cnd = data;
    int err;
    switch (msg) {
    case PRE_RATE_CHANGE:
//
// Disable interrupt since read accesses are prohibited after
// stalling.
//
    disable_irq(emc.irq);
    err = emc_prepare_timing_change(emc, cnd.new_rate);
    enable_irq(emc.irq);
    break;
    case ABORT_RATE_CHANGE:
    err = emc_unprepare_timing_change(emc, cnd.old_rate);
    break;
    case POST_RATE_CHANGE:
    err = emc_complete_timing_change(emc, cnd.new_rate);
    break;
    default:
    return NOTIFY_DONE;
    }
    return notifier_from_errno(err);
    }
    static int load_one_timing_from_dt(struct tegra_emc *emc,
    struct emc_timing *timing,
    struct device_node *node)
    {
    u32 value;
    int err;
    err = of_property_read_u32(node, "clock-frequency", &value);
    if (err) {
    dev_err(emc.dev, "timing %pOF: failed to read rate: %d\n",
    node, err);
    return err;
    }
    timing.rate = value;
    err = of_property_read_u32_array(node, "nvidia,emc-configuration",
    timing.data,
    ARRAY_SIZE(emc_timing_registers));
    if (err) {
    dev_err(emc.dev,
    "timing %pOF: failed to read emc timing data: %d\n",
    node, err);
    return err;
    }

    timing.prop = of_property_read_bool(node, dtprop);

    err = of_property_read_u32(node, dtprop, &timing.prop); \
    if (err) { \
    dev_err(emc.dev, \
    "timing %pOFn: failed to read " #prop ": %d\n", \
    node, err); \
    return err; \
    }
    EMC_READ_U32(emc_auto_cal_interval, "nvidia,emc-auto-cal-interval")
    EMC_READ_U32(emc_mode_1, "nvidia,emc-mode-1")
    EMC_READ_U32(emc_mode_2, "nvidia,emc-mode-2")
    EMC_READ_U32(emc_mode_reset, "nvidia,emc-mode-reset")
    EMC_READ_U32(emc_zcal_cnt_long, "nvidia,emc-zcal-cnt-long")
    EMC_READ_BOOL(emc_cfg_dyn_self_ref, "nvidia,emc-cfg-dyn-self-ref")
    EMC_READ_BOOL(emc_cfg_periodic_qrst, "nvidia,emc-cfg-periodic-qrst")

    dev_dbg(emc.dev, "%s: %pOF: rate %lu\n", __func__, node, timing.rate);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmp_timings(_a: *const c_void, _b: *const c_void) -> c_int {
    static int cmp_timings(const void *_a, const void *_b)
    {
    const struct emc_timing *a = _a;
    const struct emc_timing *b = _b;
    if (a.rate < b.rate)
    return -1;
    if (a.rate > b.rate)
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn emc_check_mc_timings(emc: *mut tegra_emc) -> c_int {
    static int emc_check_mc_timings(struct tegra_emc *emc)
    {
    struct tegra_mc *mc = emc.mc;
    unsigned int i;
    if (emc.num_timings != mc.num_timings) {
    dev_err(emc.dev, "emc/mc timings number mismatch: %u %u\n",
    emc.num_timings, mc.num_timings);
    return -EINVAL;
    }
    for (i = 0; i < mc.num_timings; i++) {
    if (emc.timings[i].rate != mc.timings[i].rate) {
    dev_err(emc.dev,
    "emc/mc timing rate mismatch: %lu %lu\n",
    emc.timings[i].rate, mc.timings[i].rate);
    return -EINVAL;
    }
    }
    return 0;
    }
    static int emc_load_timings_from_dt(struct tegra_emc *emc,
    struct device_node *node)
    {
    struct emc_timing *timing;
    int child_count;
    int err;
    child_count = of_get_child_count(node);
    if (!child_count) {
    dev_err(emc.dev, "no memory timings in: %pOF\n", node);
    return -EINVAL;
    }
    emc.timings = devm_kcalloc(emc.dev, child_count, sizeof(*timing),
    GFP_KERNEL);
    if (!emc.timings)
    return -ENOMEM;
    emc.num_timings = child_count;
    timing = emc.timings;
    for_each_child_of_node_scoped(node, child) {
    err = load_one_timing_from_dt(emc, timing++, child);
    if (err)
    return err;
    }
    sort(emc.timings, emc.num_timings, sizeof(*timing), cmp_timings,
    core::ptr::null_mut());
    err = emc_check_mc_timings(emc);
    if (err)
    return err;
    dev_info_once(emc.dev,
    "got %u timings for RAM code %u (min %luMHz max %luMHz)\n",
    emc.num_timings,
    tegra_read_ram_code(),
    emc.timings[0].rate / 1000000,
    emc.timings[emc.num_timings - 1].rate / 1000000);
    return 0;
    }
    static struct device_node *emc_find_node_by_ram_code(struct tegra_emc *emc)
    {
    struct device *dev = emc.dev;
    struct device_node *np;
    u32 value, ram_code;
    int err;
    if (emc.mrr_error) {
    dev_warn(dev, "memory timings skipped due to MRR error\n");
    return core::ptr::null_mut();
    }
    if (of_get_child_count(dev.of_node) == 0) {
    dev_info_once(dev, "device-tree doesn't have memory timings\n");
    return core::ptr::null_mut();
    }
    ram_code = tegra_read_ram_code();
    for_each_child_of_node(dev.of_node, np) {
    err = of_property_read_u32(np, "nvidia,ram-code", &value);
    if (err || value != ram_code)
    continue;
    return np;
    }
    dev_err(dev, "no memory timings for RAM code %u found in device-tree\n",
    ram_code);
    return core::ptr::null_mut();
    }
    static int emc_read_lpddr_mode_register(struct tegra_emc *emc,
    unsigned int emem_dev,
    unsigned int register_addr,
    unsigned int *register_data)
    {
    let mut memory_dev: u32 = emem_dev ? 1 : 2;
    u32 val, mr_mask = 0xff;
    int err;
// clear data-valid interrupt status
    writel_relaxed(EMC_MRR_DIVLD_INT, emc.regs + EMC_INTSTATUS);
// issue mode register read request
    val  = FIELD_PREP(EMC_MRR_DEV_SELECTN, memory_dev);
    val |= FIELD_PREP(EMC_MRR_MRR_MA, register_addr);
    writel_relaxed(val, emc.regs + EMC_MRR);
// wait for the LPDDR2 data-valid interrupt
    err = readl_relaxed_poll_timeout_atomic(emc.regs + EMC_INTSTATUS, val,
    val & EMC_MRR_DIVLD_INT,
    1, 100);
    if (err) {
    dev_err(emc.dev, "mode register %u read failed: %d\n",
    register_addr, err);
    emc.mrr_error = true;
    return err;
    }
// read out mode register data
    val = readl_relaxed(emc.regs + EMC_MRR);
// register_data = FIELD_GET(EMC_MRR_MRR_DATA, val) & mr_mask;
    return 0;
    }
    static void emc_read_lpddr_sdram_info(struct tegra_emc *emc,
    unsigned int emem_dev)
    {
    union lpddr2_basic_config4 basic_conf4;
    unsigned int manufacturer_id;
    unsigned int revision_id1;
    unsigned int revision_id2;
// these registers are standard for all LPDDR JEDEC memory chips
    emc_read_lpddr_mode_register(emc, emem_dev, 5, &manufacturer_id);
    emc_read_lpddr_mode_register(emc, emem_dev, 6, &revision_id1);
    emc_read_lpddr_mode_register(emc, emem_dev, 7, &revision_id2);
    emc_read_lpddr_mode_register(emc, emem_dev, 8, &basic_conf4.value);
    dev_info(emc.dev, "SDRAM[dev%u]: manufacturer: 0x%x (%s) rev1: 0x%x rev2: 0x%x prefetch: S%u density: %uMbit iowidth: %ubit\n",
    emem_dev, manufacturer_id,
    lpddr2_jedec_manufacturer(manufacturer_id),
    revision_id1, revision_id2,
    4 >> basic_conf4.arch_type,
    64 << basic_conf4.density,
    32 >> basic_conf4.io_width);
    }
#[no_mangle]
unsafe extern "C" fn emc_setup_hw(emc: *mut tegra_emc) -> c_int {
    static int emc_setup_hw(struct tegra_emc *emc)
    {
    u32 fbio_cfg5, emc_cfg, emc_dbg, emc_adr_cfg;
    let mut intmask: u32 = EMC_REFRESH_OVERFLOW_INT;
    static bool print_sdram_info_once;
    enum emc_dram_type dram_type;
    const char *dram_type_str;
    unsigned int emem_numdev;
    fbio_cfg5 = readl_relaxed(emc.regs + EMC_FBIO_CFG5);
    dram_type = fbio_cfg5 & EMC_FBIO_CFG5_DRAM_TYPE_MASK;
    emc_cfg = readl_relaxed(emc.regs + EMC_CFG_2);
// enable EMC and CAR to handshake on PLL divider/source changes
    emc_cfg |= EMC_CLKCHANGE_REQ_ENABLE;
// configure clock change mode accordingly to DRAM type
    switch (dram_type) {
    case DRAM_TYPE_LPDDR2:
    emc_cfg |= EMC_CLKCHANGE_PD_ENABLE;
    emc_cfg &= ~EMC_CLKCHANGE_SR_ENABLE;
    break;
    default:
    emc_cfg &= ~EMC_CLKCHANGE_SR_ENABLE;
    emc_cfg &= ~EMC_CLKCHANGE_PD_ENABLE;
    break;
    }
    writel_relaxed(emc_cfg, emc.regs + EMC_CFG_2);
// initialize interrupt
    writel_relaxed(intmask, emc.regs + EMC_INTMASK);
    writel_relaxed(0xffffffff, emc.regs + EMC_INTSTATUS);
// ensure that unwanted debug features are disabled
    emc_dbg = readl_relaxed(emc.regs + EMC_DBG);
    emc_dbg |= EMC_DBG_CFG_PRIORITY;
    emc_dbg &= ~EMC_DBG_READ_MUX_ASSEMBLY;
    emc_dbg &= ~EMC_DBG_WRITE_MUX_ACTIVE;
    emc_dbg &= ~EMC_DBG_FORCE_UPDATE;
    writel_relaxed(emc_dbg, emc.regs + EMC_DBG);
    switch (dram_type) {
    case DRAM_TYPE_DDR1:
    dram_type_str = "DDR1";
    break;
    case DRAM_TYPE_LPDDR2:
    dram_type_str = "LPDDR2";
    break;
    case DRAM_TYPE_DDR2:
    dram_type_str = "DDR2";
    break;
    case DRAM_TYPE_DDR3:
    dram_type_str = "DDR3";
    break;
    }
    emc_adr_cfg = readl_relaxed(emc.regs + EMC_ADR_CFG);
    emem_numdev = FIELD_GET(EMC_ADR_CFG_EMEM_NUMDEV, emc_adr_cfg) + 1;
    dev_info_once(emc.dev, "%u %s %s attached\n", emem_numdev,
    dram_type_str, emem_numdev == 2 ? "devices" : "device");
    if (dram_type == DRAM_TYPE_LPDDR2 && !print_sdram_info_once) {
    while (emem_numdev--)
    emc_read_lpddr_sdram_info(emc, emem_numdev);
    print_sdram_info_once = true;
    }
    return 0;
    }
    static long emc_round_rate(unsigned long rate,
    unsigned long min_rate,
    unsigned long max_rate,
    void *arg)
    {
    struct emc_timing *timing = core::ptr::null_mut();
    struct tegra_emc *emc = arg;
    unsigned int i;
    if (!emc.num_timings)
    return clk_get_rate(emc.clk);
    min_rate = min(min_rate, emc.timings[emc.num_timings - 1].rate);
    for (i = 0; i < emc.num_timings; i++) {
    if (emc.timings[i].rate < rate && i != emc.num_timings - 1)
    continue;
    if (emc.timings[i].rate > max_rate) {
    i = max(i, 1u) - 1;
    if (emc.timings[i].rate < min_rate)
    break;
    }
    if (emc.timings[i].rate < min_rate)
    continue;
    timing = &emc.timings[i];
    break;
    }
    if (!timing) {
    dev_err(emc.dev, "no timing for rate %lu min %lu max %lu\n",
    rate, min_rate, max_rate);
    return -EINVAL;
    }
    return timing.rate;
    }
//
// debugfs interface
//
// The memory controller driver exposes some files in debugfs that can be used
// to control the EMC frequency. The top-level directory can be found here:
//
// /sys/kernel/debug/emc
//
// It contains the following files:
//
// - available_rates: This file contains a list of valid, space-separated
// EMC frequencies.
//
// - min_rate: Writing a value to this file sets the given frequency as the
// floor of the permitted range. If this is higher than the currently
// configured EMC frequency, this will cause the frequency to be
// increased so that it stays within the valid range.
//
// - max_rate: Similarily to the min_rate file, writing a value to this file
// sets the given frequency as the ceiling of the permitted range. If
// the value is lower than the currently configured EMC frequency, this
// will cause the frequency to be decreased so that it stays within the
// valid range.
//
#[no_mangle]
unsafe extern "C" fn tegra30_emc_validate_rate(emc: *mut tegra_emc, rate: c_ulong) -> bool {
    static bool tegra30_emc_validate_rate(struct tegra_emc *emc, unsigned long rate)
    {
    unsigned int i;
    for (i = 0; i < emc.num_timings; i++)
    if (rate == emc.timings[i].rate)
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn tegra30_emc_debug_available_rates_show(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int tegra30_emc_debug_available_rates_show(struct seq_file *s, void *data)
    {
    struct tegra_emc *emc = s.private;
    const char *prefix = "";
    unsigned int i;
    for (i = 0; i < emc.num_timings; i++) {
    seq_printf(s, "%s%lu", prefix, emc.timings[i].rate);
    prefix = " ";
    }
    seq_puts(s, "\n");
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(tegra30_emc_debug_available_rates);
#[no_mangle]
unsafe extern "C" fn tegra30_emc_debug_min_rate_get(data: *mut c_void, rate: *mut u64) -> c_int {
    static int tegra30_emc_debug_min_rate_get(void *data, u64 *rate)
    {
    struct tegra_emc *emc = data;
// rate = emc->debugfs.min_rate;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra30_emc_debug_min_rate_set(data: *mut c_void, rate: u64) -> c_int {
    static int tegra30_emc_debug_min_rate_set(void *data, u64 rate)
    {
    struct tegra_emc *emc = data;
    int err;
    if (!tegra30_emc_validate_rate(emc, rate))
    return -EINVAL;
    err = tegra_emc_set_min_rate(&emc.reqs, rate, TEGRA_EMC_RATE_DEBUG);
    if (err < 0)
    return err;
    emc.debugfs.min_rate = rate;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(tegra30_emc_debug_min_rate_fops,
    tegra30_emc_debug_min_rate_get,
    tegra30_emc_debug_min_rate_set, "%llu\n");
#[no_mangle]
unsafe extern "C" fn tegra30_emc_debug_max_rate_get(data: *mut c_void, rate: *mut u64) -> c_int {
    static int tegra30_emc_debug_max_rate_get(void *data, u64 *rate)
    {
    struct tegra_emc *emc = data;
// rate = emc->debugfs.max_rate;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra30_emc_debug_max_rate_set(data: *mut c_void, rate: u64) -> c_int {
    static int tegra30_emc_debug_max_rate_set(void *data, u64 rate)
    {
    struct tegra_emc *emc = data;
    int err;
    if (!tegra30_emc_validate_rate(emc, rate))
    return -EINVAL;
    err = tegra_emc_set_max_rate(&emc.reqs, rate, TEGRA_EMC_RATE_DEBUG);
    if (err < 0)
    return err;
    emc.debugfs.max_rate = rate;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(tegra30_emc_debug_max_rate_fops,
    tegra30_emc_debug_max_rate_get,
    tegra30_emc_debug_max_rate_set, "%llu\n");
#[no_mangle]
unsafe extern "C" fn tegra30_emc_debugfs_init(emc: *mut tegra_emc) {
    static void tegra30_emc_debugfs_init(struct tegra_emc *emc)
    {
    struct device *dev = emc.dev;
    unsigned int i;
    int err;
    emc.debugfs.min_rate = ULONG_MAX;
    emc.debugfs.max_rate = 0;
    for (i = 0; i < emc.num_timings; i++) {
    if (emc.timings[i].rate < emc.debugfs.min_rate)
    emc.debugfs.min_rate = emc.timings[i].rate;
    if (emc.timings[i].rate > emc.debugfs.max_rate)
    emc.debugfs.max_rate = emc.timings[i].rate;
    }
    if (!emc.num_timings) {
    emc.debugfs.min_rate = clk_get_rate(emc.clk);
    emc.debugfs.max_rate = emc.debugfs.min_rate;
    }
    err = clk_set_rate_range(emc.clk, emc.debugfs.min_rate,
    emc.debugfs.max_rate);
    if (err < 0) {
    dev_err(dev, "failed to set rate range [%lu-%lu] for %pC\n",
    emc.debugfs.min_rate, emc.debugfs.max_rate,
    emc.clk);
    }
    emc.debugfs.root = debugfs_create_dir("emc", core::ptr::null_mut());
    debugfs_create_file("available_rates", 0444, emc.debugfs.root,
    emc, &tegra30_emc_debug_available_rates_fops);
    debugfs_create_file("min_rate", 0644, emc.debugfs.root,
    emc, &tegra30_emc_debug_min_rate_fops);
    debugfs_create_file("max_rate", 0644, emc.debugfs.root,
    emc, &tegra30_emc_debug_max_rate_fops);
    }
    static inline struct tegra_emc *
    to_tegra_emc_provider(struct icc_provider *provider)
    {
    return container_of(provider, struct tegra_emc, provider);
    }
    static struct icc_node_data *
    emc_of_icc_xlate_extended(const struct of_phandle_args *spec, void *data)
    {
    struct icc_provider *provider = data;
    struct icc_node_data *ndata;
    struct icc_node *node;
// External Memory is the only possible ICC route
    list_for_each_entry(node, &provider.nodes, node_list) {
    if (node.id != TEGRA_ICC_EMEM)
    continue;
    ndata = kzalloc_obj(*ndata);
    if (!ndata)
    return ERR_PTR(-ENOMEM);
//
// SRC and DST nodes should have matching TAG in order to have
// it set by default for a requested path.
//
    ndata.tag = TEGRA_MC_ICC_TAG_ISO;
    ndata.node = node;
    return ndata;
    }
    return ERR_PTR(-EPROBE_DEFER);
    }
#[no_mangle]
unsafe extern "C" fn emc_icc_set(src: *mut icc_node, dst: *mut icc_node) -> c_int {
    static int emc_icc_set(struct icc_node *src, struct icc_node *dst)
    {
    struct tegra_emc *emc = to_tegra_emc_provider(dst.provider);
    let mut peak_bw: c_ulonglong = icc_units_to_bps(dst.peak_bw);
    let mut avg_bw: c_ulonglong = icc_units_to_bps(dst.avg_bw);
    let mut rate: c_ulonglong = max(avg_bw, peak_bw);
    let mut dram_data_bus_width_bytes: c_uint = 4;
    let mut ddr: c_uint = 2;
    int err;
//
// Tegra30 EMC runs on a clock rate of SDRAM bus.  This means that
// EMC clock rate is twice smaller than the peak data rate because
// data is sampled on both EMC clock edges.
//
    do_div(rate, ddr * dram_data_bus_width_bytes);
    rate = min_t(u64, rate, U32_MAX);
    err = tegra_emc_set_min_rate(&emc.reqs, rate, TEGRA_EMC_RATE_ICC);
    if (err)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra30_emc_interconnect_init(emc: *mut tegra_emc) -> c_int {
    static int tegra30_emc_interconnect_init(struct tegra_emc *emc)
    {
    const struct tegra_mc_soc *soc = emc.mc.soc;
    struct icc_node *node;
    int err;
    emc.provider.dev = emc.dev;
    emc.provider.set = emc_icc_set;
    emc.provider.data = &emc.provider;
    emc.provider.aggregate = soc.icc_ops.aggregate;
    emc.provider.xlate_extended = emc_of_icc_xlate_extended;
    icc_provider_init(&emc.provider);
// create External Memory Controller node
    node = icc_node_create(TEGRA_ICC_EMC);
    if (IS_ERR(node))
    return PTR_ERR(node);
    node.name = "External Memory Controller";
    icc_node_add(node, &emc.provider);
// link External Memory Controller to External Memory (DRAM)
    err = icc_link_create(node, TEGRA_ICC_EMEM);
    if (err)
    goto remove_nodes;
// create External Memory node
    node = icc_node_create(TEGRA_ICC_EMEM);
    if (IS_ERR(node)) {
    err = PTR_ERR(node);
    goto remove_nodes;
    }
    node.name = "External Memory (DRAM)";
    icc_node_add(node, &emc.provider);
    err = icc_provider_register(&emc.provider);
    if (err)
    goto remove_nodes;
    return 0;
    remove_nodes:
    icc_nodes_remove(&emc.provider);
    return dev_err_probe(emc.dev, err, "failed to initialize ICC\n");
    }
#[no_mangle]
unsafe extern "C" fn devm_tegra30_emc_unset_callback(data: *mut c_void) {
    static void devm_tegra30_emc_unset_callback(void *data)
    {
    tegra20_clk_set_emc_round_callback(core::ptr::null_mut(), core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn devm_tegra30_emc_unreg_clk_notifier(data: *mut c_void) {
    static void devm_tegra30_emc_unreg_clk_notifier(void *data)
    {
    struct tegra_emc *emc = data;
    clk_notifier_unregister(emc.clk, &emc.clk_nb);
    }
#[no_mangle]
unsafe extern "C" fn tegra30_emc_init_clk(emc: *mut tegra_emc) -> c_int {
    static int tegra30_emc_init_clk(struct tegra_emc *emc)
    {
    int err;
    tegra20_clk_set_emc_round_callback(emc_round_rate, emc);
    err = devm_add_action_or_reset(emc.dev, devm_tegra30_emc_unset_callback,
    core::ptr::null_mut());
    if (err)
    return err;
    emc.clk = devm_clk_get(emc.dev, core::ptr::null_mut());
    if (IS_ERR(emc.clk))
    return dev_err_probe(emc.dev, PTR_ERR(emc.clk),
    "failed to get EMC clock\n");
    err = clk_notifier_register(emc.clk, &emc.clk_nb);
    if (err)
    return dev_err_probe(emc.dev, err, "failed to register clk notifier\n");
    err = devm_add_action_or_reset(emc.dev,
    devm_tegra30_emc_unreg_clk_notifier, emc);
    if (err)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra30_emc_probe(pdev: *mut platform_device) -> c_int {
    static int tegra30_emc_probe(struct platform_device *pdev)
    {
    let mut opp_params: tegra_core_opp_params = {};
    struct device_node *np;
    struct tegra_emc *emc;
    int err;
    emc = devm_kzalloc(&pdev.dev, sizeof(*emc), GFP_KERNEL);
    if (!emc)
    return -ENOMEM;
    emc.mc = devm_tegra_memory_controller_get(&pdev.dev);
    if (IS_ERR(emc.mc))
    return PTR_ERR(emc.mc);
    emc.clk_nb.notifier_call = emc_clk_change_notify;
    emc.dev = &pdev.dev;
    emc.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(emc.regs))
    return PTR_ERR(emc.regs);
    err = emc_setup_hw(emc);
    if (err)
    return err;
    np = emc_find_node_by_ram_code(emc);
    if (np) {
    err = emc_load_timings_from_dt(emc, np);
    of_node_put(np);
    if (err)
    return err;
    }
    err = platform_get_irq(pdev, 0);
    if (err < 0)
    return err;
    emc.irq = err;
    err = devm_request_irq(&pdev.dev, emc.irq, tegra30_emc_isr, 0,
    dev_name(&pdev.dev), emc);
    if (err)
    return dev_err_probe(&pdev.dev, err, "failed to request irq\n");
    err = tegra30_emc_init_clk(emc);
    if (err)
    return err;
    opp_params.init_state = true;
    err = devm_tegra_core_dev_init_opp_table(&pdev.dev, &opp_params);
    if (err)
    return err;
    platform_set_drvdata(pdev, emc);
    tegra_emc_rate_requests_init(&emc.reqs, &pdev.dev);
    tegra30_emc_debugfs_init(emc);
    tegra30_emc_interconnect_init(emc);
//
// Don't allow the kernel module to be unloaded. Unloading adds some
// extra complexity which doesn't really worth the effort in a case of
// this driver.
//
    try_module_get(THIS_MODULE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra30_emc_suspend(dev: *mut device) -> c_int {
    static int tegra30_emc_suspend(struct device *dev)
    {
    struct tegra_emc *emc = dev_get_drvdata(dev);
    int err;
// take exclusive control over the clock's rate
    err = clk_rate_exclusive_get(emc.clk);
    if (err) {
    dev_err(emc.dev, "failed to acquire clk: %d\n", err);
    return err;
    }
// suspending in a bad state will hang machine
    if (WARN(emc.bad_state, "hardware in a bad state\n"))
    return -EINVAL;
    emc.bad_state = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra30_emc_resume(dev: *mut device) -> c_int {
    static int tegra30_emc_resume(struct device *dev)
    {
    struct tegra_emc *emc = dev_get_drvdata(dev);
    emc_setup_hw(emc);
    emc.bad_state = false;
    clk_rate_exclusive_put(emc.clk);
    return 0;
    }
    static const struct dev_pm_ops tegra30_emc_pm_ops = {
    .suspend = tegra30_emc_suspend,
    .resume = tegra30_emc_resume,
    };
    static const struct of_device_id tegra30_emc_of_match[] = {
    { .compatible = "nvidia,tegra30-emc", },
    {},
    };
    MODULE_DEVICE_TABLE(of, tegra30_emc_of_match);
    static struct platform_driver tegra30_emc_driver = {
    .probe = tegra30_emc_probe,
    .driver = {
    .name = "tegra30-emc",
    .of_match_table = tegra30_emc_of_match,
    .pm = &tegra30_emc_pm_ops,
    .suppress_bind_attrs = true,
    .sync_state = icc_sync_state,
    },
    };
    module_platform_driver(tegra30_emc_driver);
    MODULE_AUTHOR("Dmitry Osipenko <digetx@gmail.com>");
    MODULE_DESCRIPTION("NVIDIA Tegra30 EMC driver");
    MODULE_LICENSE("GPL v2");
