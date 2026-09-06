//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/sdhci-msm.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// drivers/mmc/host/sdhci-msm.c - Qualcomm SDHCI Platform driver
//
// Copyright (c) 2013-2014, The Linux Foundation. All rights reserved.
//

pub const CORE_MCI_VERSION: c_uint = 0x50;
pub const CORE_VERSION_MAJOR_SHIFT: c_int = 28;

pub const CORE_VERSION_MINOR_MASK: c_uint = 0xff;
pub const CORE_MCI_GENERICS: c_uint = 0x70;

pub const HC_MODE_EN: c_uint = 0x1;
pub const CORE_POWER: c_uint = 0x0;

pub const INT_MASK: c_uint = 0xf;
pub const MAX_PHASES: c_int = 16;

pub const DLL_USR_CTL_POR_VAL: c_uint = 0x10800;

pub const DLL_CONFIG_3_LOW_FREQ_VAL: c_uint = 0x08;
pub const DLL_CONFIG_3_HIGH_FREQ_VAL: c_uint = 0x10;
pub const CORE_VENDOR_SPEC_POR_VAL: c_uint = 0xa9c;

pub const CORE_CSR_CDC_CTLR_CFG0: c_uint = 0x130;

pub const CORE_CSR_CDC_CTLR_CFG1: c_uint = 0x134;
pub const CORE_CSR_CDC_CAL_TIMER_CFG0: c_uint = 0x138;

pub const CORE_CSR_CDC_CAL_TIMER_CFG1: c_uint = 0x13C;
pub const CORE_CSR_CDC_REFCOUNT_CFG: c_uint = 0x140;
pub const CORE_CSR_CDC_COARSE_CAL_CFG: c_uint = 0x144;
pub const CORE_CDC_OFFSET_CFG: c_uint = 0x14C;
pub const CORE_CSR_CDC_DELAY_CFG: c_uint = 0x150;
pub const CORE_CDC_SLAVE_DDA_CFG: c_uint = 0x160;
pub const CORE_CSR_CDC_STATUS0: c_uint = 0x164;

pub const CORE_CDC_ERROR_CODE_MASK: c_uint = 0x7000000;
pub const CORE_CSR_CDC_GEN_CFG: c_uint = 0x178;

pub const DDR_CONFIG_POR_VAL: c_uint = 0x80040873;

pub const SDHCI_MSM_MIN_CLOCK: c_int = 400000;

pub const CDR_SELEXT_SHIFT: c_int = 20;

pub const CMUX_SHIFT_PHASE_SHIFT: c_int = 24;

pub const MSM_MMC_AUTOSUSPEND_DELAY_MS: c_int = 50;
// Timeout value to avoid infinite waiting for pwr_irq
pub const MSM_PWR_IRQ_TIMEOUT_MS: c_int = 5000;
// Max load for eMMC Vdd supply
pub const MMC_VMMC_MAX_LOAD_UA: c_int = 570000;
// Max load for eMMC Vdd-io supply
pub const MMC_VQMMC_MAX_LOAD_UA: c_int = 325000;
// Max load for SD Vdd supply
pub const SD_VMMC_MAX_LOAD_UA: c_int = 800000;
// Max load for SD Vdd-io supply
pub const SD_VQMMC_MAX_LOAD_UA: c_int = 22000;

    msm_host.var_ops.msm_readl_relaxed(host, offset)

    msm_host.var_ops.msm_writel_relaxed(val, host, offset)
// CQHCI vendor specific registers
pub const CQHCI_VENDOR_CFG1: c_uint = 0xA00;

// non command queue crypto enable register
pub const NONCQ_CRYPTO_PARM: c_uint = 0x70;
pub const NONCQ_CRYPTO_DUN: c_uint = 0x74;

pub const HC_VENDOR_SPECIFIC_FUNC4: c_uint = 0x260;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdhci_msm_offset {
    pub core_hc_mode: u32,
    pub core_mci_data_cnt: u32,
    pub core_mci_status: u32,
    pub core_mci_fifo_cnt: u32,
    pub core_mci_version: u32,
    pub core_generics: u32,
    pub core_testbus_config: u32,
    pub core_testbus_sel2_bit: u32,
    pub core_testbus_ena: u32,
    pub core_testbus_sel2: u32,
    pub core_pwrctl_status: u32,
    pub core_pwrctl_mask: u32,
    pub core_pwrctl_clear: u32,
    pub core_pwrctl_ctl: u32,
    pub core_sdcc_debug_reg: u32,
    pub core_dll_config: u32,
    pub core_dll_status: u32,
    pub core_vendor_spec: u32,
    pub core_vendor_spec_adma_err_addr0: u32,
    pub core_vendor_spec_adma_err_addr1: u32,
    pub core_vendor_spec_func2: u32,
    pub core_vendor_spec_capabilities0: u32,
    pub core_ddr_200_cfg: u32,
    pub core_vendor_spec3: u32,
    pub core_dll_config_2: u32,
    pub core_dll_config_3: u32,
    pub /: *mut *mut u32 core_ddr_config_old; / Applicable to sdcc minor ver < 0x49,
    pub core_ddr_config: u32,
    pub /: *mut *mut u32 core_dll_usr_ctl; / Present on SDCC5.1 onwards,
}

    static const struct sdhci_msm_offset sdhci_msm_v5_offset = {
    .core_mci_data_cnt = 0x35c,
    .core_mci_status = 0x324,
    .core_mci_fifo_cnt = 0x308,
    .core_mci_version = 0x318,
    .core_generics = 0x320,
    .core_testbus_config = 0x32c,
    .core_testbus_sel2_bit = 3,
    .core_testbus_ena = (1 << 31),
    .core_testbus_sel2 = (1 << 3),
    .core_pwrctl_status = 0x240,
    .core_pwrctl_mask = 0x244,
    .core_pwrctl_clear = 0x248,
    .core_pwrctl_ctl = 0x24c,
    .core_sdcc_debug_reg = 0x358,
    .core_dll_config = 0x200,
    .core_dll_status = 0x208,
    .core_vendor_spec = 0x20c,
    .core_vendor_spec_adma_err_addr0 = 0x214,
    .core_vendor_spec_adma_err_addr1 = 0x218,
    .core_vendor_spec_func2 = 0x210,
    .core_vendor_spec_capabilities0 = 0x21c,
    .core_ddr_200_cfg = 0x224,
    .core_vendor_spec3 = 0x250,
    .core_dll_config_2 = 0x254,
    .core_dll_config_3 = 0x258,
    .core_ddr_config = 0x25c,
    .core_dll_usr_ctl = 0x388,
    };
    static const struct sdhci_msm_offset sdhci_msm_mci_offset = {
    .core_hc_mode = 0x78,
    .core_mci_data_cnt = 0x30,
    .core_mci_status = 0x34,
    .core_mci_fifo_cnt = 0x44,
    .core_mci_version = 0x050,
    .core_generics = 0x70,
    .core_testbus_config = 0x0cc,
    .core_testbus_sel2_bit = 4,
    .core_testbus_ena = (1 << 3),
    .core_testbus_sel2 = (1 << 4),
    .core_pwrctl_status = 0xdc,
    .core_pwrctl_mask = 0xe0,
    .core_pwrctl_clear = 0xe4,
    .core_pwrctl_ctl = 0xe8,
    .core_sdcc_debug_reg = 0x124,
    .core_dll_config = 0x100,
    .core_dll_status = 0x108,
    .core_vendor_spec = 0x10c,
    .core_vendor_spec_adma_err_addr0 = 0x114,
    .core_vendor_spec_adma_err_addr1 = 0x118,
    .core_vendor_spec_func2 = 0x110,
    .core_vendor_spec_capabilities0 = 0x11c,
    .core_ddr_200_cfg = 0x184,
    .core_vendor_spec3 = 0x1b0,
    .core_dll_config_2 = 0x1b4,
    .core_ddr_config_old = 0x1b8,
    .core_ddr_config = 0x1bc,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdhci_msm_variant_ops {
    pub offset): *mut *mut *mut u32 (msm_readl_relaxed)(struct sdhci_host host, u32,
    void (*msm_writel_relaxed)(u32 val, struct sdhci_host *host,
    pub offset): u32,
}

//
// From V5, register spaces have changed. Wrap this info in a structure
// and choose the data_structure based on version info mentioned in DT.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdhci_msm_variant_info {
    pub mci_removed: bool,
    pub restore_dll_config: bool,
    pub var_ops: *const sdhci_msm_variant_ops,
    pub offset: *const sdhci_msm_offset,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdhci_msm_host {
    pub pdev: *mut platform_device,
    pub /: *mut *mut *mut void __iomem core_mem; / MSM SDCC mapped address,
    pub /: *mut *mut int pwr_irq; / power irq,
    pub /: *mut *mut *mut clk bus_clk; / SDHC bus voter clock,
    pub cm_dll*/: *mut *mut *mut clk xo_clk; / TCXO clk needed for FLL feature of,
// core, iface, cal and sleep clocks
    pub bulk_clks: [clk_bulk_data; 4],
    pub ice: *mut qcom_ice,

    pub clk_rate: c_ulong,
    pub mmc: *mut mmc_host,
    pub use_14lpp_dll_reset: bool,
    pub tuning_done: bool,
    pub calibration_done: bool,
    pub saved_tuning_phase: u8,
    pub use_cdclp533: bool,
    pub curr_pwr_state: u32,
    pub curr_io_level: u32,
    pub pwr_irq_wait: wait_queue_head_t,
    pub pwr_irq_flag: bool,
    pub caps_0: u32,
    pub mci_removed: bool,
    pub restore_dll_config: bool,
    pub var_ops: *const sdhci_msm_variant_ops,
    pub offset: *const sdhci_msm_offset,
    pub use_cdr: bool,
    pub transfer_mode: u32,
    pub updated_ddr_cfg: bool,
    pub uses_tassadar_dll: bool,
    pub dll_config: u32,
    pub ddr_config: u32,
    pub vqmmc_enabled: bool,
    pub non_cqe_ice_init_done: bool,
}

    static const struct sdhci_msm_offset *sdhci_priv_msm_offset(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    return msm_host.offset;
    }
//
// APIs to read/write to vendor specific registers which were there in the
// core_mem region before MCI was removed.
//
    static u32 sdhci_msm_mci_variant_readl_relaxed(struct sdhci_host *host,
    u32 offset)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    return readl_relaxed(msm_host.core_mem + offset);
    }
    static u32 sdhci_msm_v5_variant_readl_relaxed(struct sdhci_host *host,
    u32 offset)
    {
    return readl_relaxed(host.ioaddr + offset);
    }
    static void sdhci_msm_mci_variant_writel_relaxed(u32 val,
    struct sdhci_host *host, u32 offset)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    writel_relaxed(val, msm_host.core_mem + offset);
    }
    static void sdhci_msm_v5_variant_writel_relaxed(u32 val,
    struct sdhci_host *host, u32 offset)
    {
    writel_relaxed(val, host.ioaddr + offset);
    }
    static unsigned int msm_get_clock_mult_for_bus_mode(struct sdhci_host *host,
    unsigned int clock,
    unsigned int timing)
    {
//
// The SDHC requires internal clock frequency to be double the
// actual clock that will be set for DDR mode. The controller
// uses the faster clock(100/400MHz) for some of its parts and
// send the actual required clock (50/200MHz) to the card.
//
    if (timing == MMC_TIMING_UHS_DDR50 ||
    timing == MMC_TIMING_MMC_DDR52 ||
    (timing == MMC_TIMING_MMC_HS400 &&
    clock == MMC_HS200_MAX_DTR) ||
    host.flags & SDHCI_HS400_TUNING)
    return 2;
    return 1;
    }
    static void msm_set_clock_rate_for_bus_mode(struct sdhci_host *host,
    unsigned int clock,
    unsigned int timing)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    struct clk *core_clk = msm_host.bulk_clks[0].clk;
    unsigned long achieved_rate;
    unsigned int desired_rate;
    unsigned int mult;
    int rc;
    mult = msm_get_clock_mult_for_bus_mode(host, clock, timing);
    desired_rate = clock * mult;
    rc = dev_pm_opp_set_rate(mmc_dev(host.mmc), desired_rate);
    if (rc) {
    pr_err("%s: Failed to set clock at rate %u at timing %d\n",
    mmc_hostname(host.mmc), desired_rate, timing);
    return;
    }
//
// Qualcomm clock drivers by default round clock _up_ if they can't
// make the requested rate.  This is not good for SD.  Yell if we
// encounter it.
//
    achieved_rate = clk_get_rate(core_clk);
    if (achieved_rate > desired_rate)
    pr_warn("%s: Card appears overclocked; req %u Hz, actual %lu Hz\n",
    mmc_hostname(host.mmc), desired_rate, achieved_rate);
    host.mmc.actual_clock = achieved_rate / mult;
// Stash the rate we requested to use in sdhci_msm_runtime_resume()
    msm_host.clk_rate = desired_rate;
    pr_debug("%s: Setting clock at rate %lu at timing %d\n",
    mmc_hostname(host.mmc), achieved_rate, timing);
    }
// Platform specific tuning
#[no_mangle]
pub unsafe extern "C" fn msm_dll_poll_ck_out_en(host: *mut sdhci_host, poll: u8) -> c_int {
    static inline int msm_dll_poll_ck_out_en(struct sdhci_host *host, u8 poll)
    {
    let mut wait_cnt: u32 = 50;
    u8 ck_out_en;
    struct mmc_host *mmc = host.mmc;
    const struct sdhci_msm_offset *msm_offset =
    sdhci_priv_msm_offset(host);
// Poll for CK_OUT_EN bit.  max. poll time = 50us
    ck_out_en = !!(readl_relaxed(host.ioaddr +
    msm_offset.core_dll_config) & CORE_CK_OUT_EN);
    while (ck_out_en != poll) {
    if (--wait_cnt == 0) {
    dev_err(mmc_dev(mmc), "%s: CK_OUT_EN bit is not %d\n",
    mmc_hostname(mmc), poll);
    return -ETIMEDOUT;
    }
    udelay(1);
    ck_out_en = !!(readl_relaxed(host.ioaddr +
    msm_offset.core_dll_config) & CORE_CK_OUT_EN);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msm_config_cm_dll_phase(host: *mut sdhci_host, phase: u8) -> c_int {
    static int msm_config_cm_dll_phase(struct sdhci_host *host, u8 phase)
    {
    int rc;
    static const u8 grey_coded_phase_table[] = {
    0x0, 0x1, 0x3, 0x2, 0x6, 0x7, 0x5, 0x4,
    0xc, 0xd, 0xf, 0xe, 0xa, 0xb, 0x9, 0x8
    };
    unsigned long flags;
    u32 config;
    struct mmc_host *mmc = host.mmc;
    const struct sdhci_msm_offset *msm_offset =
    sdhci_priv_msm_offset(host);
    if (phase > 0xf)
    return -EINVAL;
    spin_lock_irqsave(&host.lock, flags);
    config = readl_relaxed(host.ioaddr + msm_offset.core_dll_config);
    config &= ~(CORE_CDR_EN | CORE_CK_OUT_EN);
    config |= (CORE_CDR_EXT_EN | CORE_DLL_EN);
    writel_relaxed(config, host.ioaddr + msm_offset.core_dll_config);
// Wait until CK_OUT_EN bit of DLL_CONFIG register becomes '0'
    rc = msm_dll_poll_ck_out_en(host, 0);
    if (rc)
    goto err_out;
//
// Write the selected DLL clock output phase (0 ... 15)
// to CDR_SELEXT bit field of DLL_CONFIG register.
//
    config = readl_relaxed(host.ioaddr + msm_offset.core_dll_config);
    config &= ~CDR_SELEXT_MASK;
    config |= grey_coded_phase_table[phase] << CDR_SELEXT_SHIFT;
    writel_relaxed(config, host.ioaddr + msm_offset.core_dll_config);
    config = readl_relaxed(host.ioaddr + msm_offset.core_dll_config);
    config |= CORE_CK_OUT_EN;
    writel_relaxed(config, host.ioaddr + msm_offset.core_dll_config);
// Wait until CK_OUT_EN bit of DLL_CONFIG register becomes '1'
    rc = msm_dll_poll_ck_out_en(host, 1);
    if (rc)
    goto err_out;
    config = readl_relaxed(host.ioaddr + msm_offset.core_dll_config);
    config |= CORE_CDR_EN;
    config &= ~CORE_CDR_EXT_EN;
    writel_relaxed(config, host.ioaddr + msm_offset.core_dll_config);
    goto out;
    err_out:
    dev_err(mmc_dev(mmc), "%s: Failed to set DLL phase: %d\n",
    mmc_hostname(mmc), phase);
    out:
    spin_unlock_irqrestore(&host.lock, flags);
    return rc;
    }
//
// Find out the greatest range of consecuitive selected
// DLL clock output phases that can be used as sampling
// setting for SD3.0 UHS-I card read operation (in SDR104
// timing mode) or for eMMC4.5 card read operation (in
// HS400/HS200 timing mode).
// Select the 3/4 of the range and configure the DLL with the
// selected DLL clock output phase.
//
    static int msm_find_most_appropriate_phase(struct sdhci_host *host,
    u8 *phase_table, u8 total_phases)
    {
    int ret;
    u8 ranges[MAX_PHASES][MAX_PHASES] = { {0}, {0} };
    u8 phases_per_row[MAX_PHASES] = { 0 };
    let mut row_index: c_int = 0, col_index = 0, selected_row_index = 0, curr_max = 0;
    int i, cnt, phase_0_raw_index = 0, phase_15_raw_index = 0;
    let mut phase_0_found: bool = false, phase_15_found = false;
    struct mmc_host *mmc = host.mmc;
    if (!total_phases || (total_phases > MAX_PHASES)) {
    dev_err(mmc_dev(mmc), "%s: Invalid argument: total_phases=%d\n",
    mmc_hostname(mmc), total_phases);
    return -EINVAL;
    }
    for (cnt = 0; cnt < total_phases; cnt++) {
    ranges[row_index][col_index] = phase_table[cnt];
    phases_per_row[row_index] += 1;
    col_index++;
    if ((cnt + 1) == total_phases) {
    continue;
// check if next phase in phase_table is consecutive or not
    } else if ((phase_table[cnt] + 1) != phase_table[cnt + 1]) {
    row_index++;
    col_index = 0;
    }
    }
    if (row_index >= MAX_PHASES)
    return -EINVAL;
// Check if phase-0 is present in first valid window?
    if (!ranges[0][0]) {
    phase_0_found = true;
    phase_0_raw_index = 0;
// Check if cycle exist between 2 valid windows
    for (cnt = 1; cnt <= row_index; cnt++) {
    if (phases_per_row[cnt]) {
    for (i = 0; i < phases_per_row[cnt]; i++) {
    if (ranges[cnt][i] == 15) {
    phase_15_found = true;
    phase_15_raw_index = cnt;
    break;
    }
    }
    }
    }
    }
// If 2 valid windows form cycle then merge them as single window
    if (phase_0_found && phase_15_found) {
// number of phases in raw where phase 0 is present
    let mut phases_0: u8 = phases_per_row[phase_0_raw_index];
// number of phases in raw where phase 15 is present
    let mut phases_15: u8 = phases_per_row[phase_15_raw_index];
    if (phases_0 + phases_15 >= MAX_PHASES)
//
// If there are more than 1 phase windows then total
// number of phases in both the windows should not be
// more than or equal to MAX_PHASES.
//
    return -EINVAL;
// Merge 2 cyclic windows
    i = phases_15;
    for (cnt = 0; cnt < phases_0; cnt++) {
    ranges[phase_15_raw_index][i] =
    ranges[phase_0_raw_index][cnt];
    if (++i >= MAX_PHASES)
    break;
    }
    phases_per_row[phase_0_raw_index] = 0;
    phases_per_row[phase_15_raw_index] = phases_15 + phases_0;
    }
    for (cnt = 0; cnt <= row_index; cnt++) {
    if (phases_per_row[cnt] > curr_max) {
    curr_max = phases_per_row[cnt];
    selected_row_index = cnt;
    }
    }
    i = (curr_max * 3) / 4;
    if (i)
    i--;
    ret = ranges[selected_row_index][i];
    if (ret >= MAX_PHASES) {
    ret = -EINVAL;
    dev_err(mmc_dev(mmc), "%s: Invalid phase selected=%d\n",
    mmc_hostname(mmc), ret);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn msm_cm_dll_set_freq(host: *mut sdhci_host) {
    static inline void msm_cm_dll_set_freq(struct sdhci_host *host)
    {
    let mut mclk_freq: u32 = 0, config;
    const struct sdhci_msm_offset *msm_offset =
    sdhci_priv_msm_offset(host);
// Program the MCLK value to MCLK_FREQ bit field
    if (host.clock <= 112000000)
    mclk_freq = 0;
#[no_mangle]
pub unsafe extern "C" fn if(125000000: host->clock <=) -> else {
    else if (host.clock <= 125000000)
    mclk_freq = 1;
#[no_mangle]
pub unsafe extern "C" fn if(137000000: host->clock <=) -> else {
    else if (host.clock <= 137000000)
    mclk_freq = 2;
#[no_mangle]
pub unsafe extern "C" fn if(150000000: host->clock <=) -> else {
    else if (host.clock <= 150000000)
    mclk_freq = 3;
#[no_mangle]
pub unsafe extern "C" fn if(162000000: host->clock <=) -> else {
    else if (host.clock <= 162000000)
    mclk_freq = 4;
#[no_mangle]
pub unsafe extern "C" fn if(175000000: host->clock <=) -> else {
    else if (host.clock <= 175000000)
    mclk_freq = 5;
#[no_mangle]
pub unsafe extern "C" fn if(187000000: host->clock <=) -> else {
    else if (host.clock <= 187000000)
    mclk_freq = 6;
#[no_mangle]
pub unsafe extern "C" fn if(200000000: host->clock <=) -> else {
    else if (host.clock <= 200000000)
    mclk_freq = 7;
    config = readl_relaxed(host.ioaddr + msm_offset.core_dll_config);
    config &= ~CMUX_SHIFT_PHASE_MASK;
    config |= mclk_freq << CMUX_SHIFT_PHASE_SHIFT;
    writel_relaxed(config, host.ioaddr + msm_offset.core_dll_config);
    }
// Initialize the DLL (Programmable Delay Line)
#[no_mangle]
unsafe extern "C" fn msm_init_cm_dll(host: *mut sdhci_host) -> c_int {
    static int msm_init_cm_dll(struct sdhci_host *host)
    {
    struct mmc_host *mmc = host.mmc;
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    let mut wait_cnt: c_int = 50;
    unsigned long flags, xo_clk = 0;
    u32 config;
    const struct sdhci_msm_offset *msm_offset =
    msm_host.offset;
    if (msm_host.use_14lpp_dll_reset && !IS_ERR_OR_NULL(msm_host.xo_clk))
    xo_clk = clk_get_rate(msm_host.xo_clk);
    spin_lock_irqsave(&host.lock, flags);
//
// Make sure that clock is always enabled when DLL
// tuning is in progress. Keeping PWRSAVE ON may
// turn off the clock.
//
    config = readl_relaxed(host.ioaddr + msm_offset.core_vendor_spec);
    config &= ~CORE_CLK_PWRSAVE;
    writel_relaxed(config, host.ioaddr + msm_offset.core_vendor_spec);
    if (msm_host.dll_config)
    writel_relaxed(msm_host.dll_config,
    host.ioaddr + msm_offset.core_dll_config);
    if (msm_host.use_14lpp_dll_reset) {
    config = readl_relaxed(host.ioaddr +
    msm_offset.core_dll_config);
    config &= ~CORE_CK_OUT_EN;
    writel_relaxed(config, host.ioaddr +
    msm_offset.core_dll_config);
    config = readl_relaxed(host.ioaddr +
    msm_offset.core_dll_config_2);
    config |= CORE_DLL_CLOCK_DISABLE;
    writel_relaxed(config, host.ioaddr +
    msm_offset.core_dll_config_2);
    }
    config = readl_relaxed(host.ioaddr +
    msm_offset.core_dll_config);
    config |= CORE_DLL_RST;
    writel_relaxed(config, host.ioaddr +
    msm_offset.core_dll_config);
    config = readl_relaxed(host.ioaddr +
    msm_offset.core_dll_config);
    config |= CORE_DLL_PDN;
    writel_relaxed(config, host.ioaddr +
    msm_offset.core_dll_config);
    if (!msm_host.dll_config)
    msm_cm_dll_set_freq(host);
    if (msm_host.use_14lpp_dll_reset &&
    !IS_ERR_OR_NULL(msm_host.xo_clk)) {
    let mut mclk_freq: u32 = 0;
    config = readl_relaxed(host.ioaddr +
    msm_offset.core_dll_config_2);
    config &= CORE_FLL_CYCLE_CNT;
    if (config)
    mclk_freq = DIV_ROUND_CLOSEST_ULL((host.clock * 8),
    xo_clk);
    else
    mclk_freq = DIV_ROUND_CLOSEST_ULL((host.clock * 4),
    xo_clk);
    config = readl_relaxed(host.ioaddr +
    msm_offset.core_dll_config_2);
    config &= ~(0xFF << 10);
    config |= mclk_freq << 10;
    writel_relaxed(config, host.ioaddr +
    msm_offset.core_dll_config_2);
// wait for 5us before enabling DLL clock
    udelay(5);
    }
    config = readl_relaxed(host.ioaddr +
    msm_offset.core_dll_config);
    config &= ~CORE_DLL_RST;
    writel_relaxed(config, host.ioaddr +
    msm_offset.core_dll_config);
    config = readl_relaxed(host.ioaddr +
    msm_offset.core_dll_config);
    config &= ~CORE_DLL_PDN;
    writel_relaxed(config, host.ioaddr +
    msm_offset.core_dll_config);
    if (msm_host.use_14lpp_dll_reset) {
    if (!msm_host.dll_config)
    msm_cm_dll_set_freq(host);
    config = readl_relaxed(host.ioaddr +
    msm_offset.core_dll_config_2);
    config &= ~CORE_DLL_CLOCK_DISABLE;
    writel_relaxed(config, host.ioaddr +
    msm_offset.core_dll_config_2);
    }
//
// Configure DLL user control register to enable DLL status.
// This setting is applicable to SDCC v5.1 onwards only.
//
    if (msm_host.uses_tassadar_dll) {
    config = DLL_USR_CTL_POR_VAL | FINE_TUNE_MODE_EN |
    ENABLE_DLL_LOCK_STATUS | BIAS_OK_SIGNAL;
    writel_relaxed(config, host.ioaddr +
    msm_offset.core_dll_usr_ctl);
    config = readl_relaxed(host.ioaddr +
    msm_offset.core_dll_config_3);
    config &= ~0xFF;
    if (msm_host.clk_rate < 150000000)
    config |= DLL_CONFIG_3_LOW_FREQ_VAL;
    else
    config |= DLL_CONFIG_3_HIGH_FREQ_VAL;
    writel_relaxed(config, host.ioaddr +
    msm_offset.core_dll_config_3);
    }
    config = readl_relaxed(host.ioaddr +
    msm_offset.core_dll_config);
    config |= CORE_DLL_EN;
    writel_relaxed(config, host.ioaddr +
    msm_offset.core_dll_config);
    config = readl_relaxed(host.ioaddr +
    msm_offset.core_dll_config);
    config |= CORE_CK_OUT_EN;
    writel_relaxed(config, host.ioaddr +
    msm_offset.core_dll_config);
// Wait until DLL_LOCK bit of DLL_STATUS register becomes '1'
    while (!(readl_relaxed(host.ioaddr + msm_offset.core_dll_status) &
    CORE_DLL_LOCK)) {
// max. wait for 50us sec for LOCK bit to be set
    if (--wait_cnt == 0) {
    dev_err(mmc_dev(mmc), "%s: DLL failed to LOCK\n",
    mmc_hostname(mmc));
    spin_unlock_irqrestore(&host.lock, flags);
    return -ETIMEDOUT;
    }
    udelay(1);
    }
    spin_unlock_irqrestore(&host.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msm_hc_select_default(host: *mut sdhci_host) {
    static void msm_hc_select_default(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    u32 config;
    const struct sdhci_msm_offset *msm_offset =
    msm_host.offset;
    if (!msm_host.use_cdclp533) {
    config = readl_relaxed(host.ioaddr +
    msm_offset.core_vendor_spec3);
    config &= ~CORE_PWRSAVE_DLL;
    writel_relaxed(config, host.ioaddr +
    msm_offset.core_vendor_spec3);
    }
    config = readl_relaxed(host.ioaddr + msm_offset.core_vendor_spec);
    config &= ~CORE_HC_MCLK_SEL_MASK;
    config |= CORE_HC_MCLK_SEL_DFLT;
    writel_relaxed(config, host.ioaddr + msm_offset.core_vendor_spec);
//
// Disable HC_SELECT_IN to be able to use the UHS mode select
// configuration from Host Control2 register for all other
// modes.
// Write 0 to HC_SELECT_IN and HC_SELECT_IN_EN field
// in VENDOR_SPEC_FUNC
//
    config = readl_relaxed(host.ioaddr + msm_offset.core_vendor_spec);
    config &= ~CORE_HC_SELECT_IN_EN;
    config &= ~CORE_HC_SELECT_IN_MASK;
    writel_relaxed(config, host.ioaddr + msm_offset.core_vendor_spec);
//
// Make sure above writes impacting free running MCLK are completed
// before changing the clk_rate at GCC.
//
    wmb();
    }
#[no_mangle]
unsafe extern "C" fn msm_hc_select_hs400(host: *mut sdhci_host) {
    static void msm_hc_select_hs400(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    let mut ios: mmc_ios = host.mmc.ios;
    u32 config, dll_lock;
    int rc;
    const struct sdhci_msm_offset *msm_offset =
    msm_host.offset;
// Select the divided clock (free running MCLK/2)
    config = readl_relaxed(host.ioaddr + msm_offset.core_vendor_spec);
    config &= ~CORE_HC_MCLK_SEL_MASK;
    config |= CORE_HC_MCLK_SEL_HS400;
    writel_relaxed(config, host.ioaddr + msm_offset.core_vendor_spec);
//
// Select HS400 mode using the HC_SELECT_IN from VENDOR SPEC
// register
//
    if ((msm_host.tuning_done || ios.enhanced_strobe) &&
    !msm_host.calibration_done) {
    config = readl_relaxed(host.ioaddr +
    msm_offset.core_vendor_spec);
    config |= CORE_HC_SELECT_IN_HS400;
    config |= CORE_HC_SELECT_IN_EN;
    writel_relaxed(config, host.ioaddr +
    msm_offset.core_vendor_spec);
    }
    if (!msm_host.clk_rate && !msm_host.use_cdclp533) {
//
// Poll on DLL_LOCK or DDR_DLL_LOCK bits in
// core_dll_status to be set. This should get set
// within 15 us at 200 MHz.
//
    rc = readl_relaxed_poll_timeout(host.ioaddr +
    msm_offset.core_dll_status,
    dll_lock,
    (dll_lock &
    (CORE_DLL_LOCK |
    CORE_DDR_DLL_LOCK)), 10,
    1000);
    if (rc == -ETIMEDOUT)
    pr_err("%s: Unable to get DLL_LOCK/DDR_DLL_LOCK, dll_status: 0x%08x\n",
    mmc_hostname(host.mmc), dll_lock);
    }
//
// Make sure above writes impacting free running MCLK are completed
// before changing the clk_rate at GCC.
//
    wmb();
    }
//
// sdhci_msm_hc_select_mode :- In general all timing modes are
// controlled via UHS mode select in Host Control2 register.
// eMMC specific HS200/HS400 doesn't have their respective modes
// defined here, hence we use these values.
//
// HS200 - SDR104 (Since they both are equivalent in functionality)
// HS400 - This involves multiple configurations
// Initially SDR104 - when tuning is required as HS200
// Then when switching to DDR @ 400MHz (HS400) we use
// the vendor specific HC_SELECT_IN to control the mode.
//
// In addition to controlling the modes we also need to select the
// correct input clock for DLL depending on the mode.
//
// HS400 - divided clock (free running MCLK/2)
// All other modes - default (free running MCLK)
//
#[no_mangle]
unsafe extern "C" fn sdhci_msm_hc_select_mode(host: *mut sdhci_host) {
    static void sdhci_msm_hc_select_mode(struct sdhci_host *host)
    {
    let mut ios: mmc_ios = host.mmc.ios;
    if (ios.timing == MMC_TIMING_MMC_HS400 ||
    host.flags & SDHCI_HS400_TUNING)
    msm_hc_select_hs400(host);
    else
    msm_hc_select_default(host);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_cdclp533_calibration(host: *mut sdhci_host) -> c_int {
    static int sdhci_msm_cdclp533_calibration(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    u32 config, calib_done;
    int ret;
    const struct sdhci_msm_offset *msm_offset =
    msm_host.offset;
    pr_debug("%s: %s: Enter\n", mmc_hostname(host.mmc), __func__);
//
// Retuning in HS400 (DDR mode) will fail, just reset the
// tuning block and restore the saved tuning phase.
//
    ret = msm_init_cm_dll(host);
    if (ret)
    goto out;
// Set the selected phase in delay line hw block
    ret = msm_config_cm_dll_phase(host, msm_host.saved_tuning_phase);
    if (ret)
    goto out;
    config = readl_relaxed(host.ioaddr + msm_offset.core_dll_config);
    config |= CORE_CMD_DAT_TRACK_SEL;
    writel_relaxed(config, host.ioaddr + msm_offset.core_dll_config);
    config = readl_relaxed(host.ioaddr + msm_offset.core_ddr_200_cfg);
    config &= ~CORE_CDC_T4_DLY_SEL;
    writel_relaxed(config, host.ioaddr + msm_offset.core_ddr_200_cfg);
    config = readl_relaxed(host.ioaddr + CORE_CSR_CDC_GEN_CFG);
    config &= ~CORE_CDC_SWITCH_BYPASS_OFF;
    writel_relaxed(config, host.ioaddr + CORE_CSR_CDC_GEN_CFG);
    config = readl_relaxed(host.ioaddr + CORE_CSR_CDC_GEN_CFG);
    config |= CORE_CDC_SWITCH_RC_EN;
    writel_relaxed(config, host.ioaddr + CORE_CSR_CDC_GEN_CFG);
    config = readl_relaxed(host.ioaddr + msm_offset.core_ddr_200_cfg);
    config &= ~CORE_START_CDC_TRAFFIC;
    writel_relaxed(config, host.ioaddr + msm_offset.core_ddr_200_cfg);
// Perform CDC Register Initialization Sequence
    writel_relaxed(0x11800EC, host.ioaddr + CORE_CSR_CDC_CTLR_CFG0);
    writel_relaxed(0x3011111, host.ioaddr + CORE_CSR_CDC_CTLR_CFG1);
    writel_relaxed(0x1201000, host.ioaddr + CORE_CSR_CDC_CAL_TIMER_CFG0);
    writel_relaxed(0x4, host.ioaddr + CORE_CSR_CDC_CAL_TIMER_CFG1);
    writel_relaxed(0xCB732020, host.ioaddr + CORE_CSR_CDC_REFCOUNT_CFG);
    writel_relaxed(0xB19, host.ioaddr + CORE_CSR_CDC_COARSE_CAL_CFG);
    writel_relaxed(0x4E2, host.ioaddr + CORE_CSR_CDC_DELAY_CFG);
    writel_relaxed(0x0, host.ioaddr + CORE_CDC_OFFSET_CFG);
    writel_relaxed(0x16334, host.ioaddr + CORE_CDC_SLAVE_DDA_CFG);
// CDC HW Calibration
    config = readl_relaxed(host.ioaddr + CORE_CSR_CDC_CTLR_CFG0);
    config |= CORE_SW_TRIG_FULL_CALIB;
    writel_relaxed(config, host.ioaddr + CORE_CSR_CDC_CTLR_CFG0);
    config = readl_relaxed(host.ioaddr + CORE_CSR_CDC_CTLR_CFG0);
    config &= ~CORE_SW_TRIG_FULL_CALIB;
    writel_relaxed(config, host.ioaddr + CORE_CSR_CDC_CTLR_CFG0);
    config = readl_relaxed(host.ioaddr + CORE_CSR_CDC_CTLR_CFG0);
    config |= CORE_HW_AUTOCAL_ENA;
    writel_relaxed(config, host.ioaddr + CORE_CSR_CDC_CTLR_CFG0);
    config = readl_relaxed(host.ioaddr + CORE_CSR_CDC_CAL_TIMER_CFG0);
    config |= CORE_TIMER_ENA;
    writel_relaxed(config, host.ioaddr + CORE_CSR_CDC_CAL_TIMER_CFG0);
    ret = readl_relaxed_poll_timeout(host.ioaddr + CORE_CSR_CDC_STATUS0,
    calib_done,
    (calib_done & CORE_CALIBRATION_DONE),
    1, 50);
    if (ret == -ETIMEDOUT) {
    pr_err("%s: %s: CDC calibration was not completed\n",
    mmc_hostname(host.mmc), __func__);
    goto out;
    }
    ret = readl_relaxed(host.ioaddr + CORE_CSR_CDC_STATUS0)
    & CORE_CDC_ERROR_CODE_MASK;
    if (ret) {
    pr_err("%s: %s: CDC error code %d\n",
    mmc_hostname(host.mmc), __func__, ret);
    ret = -EINVAL;
    goto out;
    }
    config = readl_relaxed(host.ioaddr + msm_offset.core_ddr_200_cfg);
    config |= CORE_START_CDC_TRAFFIC;
    writel_relaxed(config, host.ioaddr + msm_offset.core_ddr_200_cfg);
    out:
    pr_debug("%s: %s: Exit, ret %d\n", mmc_hostname(host.mmc),
    __func__, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_cm_dll_sdc4_calibration(host: *mut sdhci_host) -> c_int {
    static int sdhci_msm_cm_dll_sdc4_calibration(struct sdhci_host *host)
    {
    struct mmc_host *mmc = host.mmc;
    u32 dll_status, config, ddr_cfg_offset;
    int ret;
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    const struct sdhci_msm_offset *msm_offset =
    sdhci_priv_msm_offset(host);
    pr_debug("%s: %s: Enter\n", mmc_hostname(host.mmc), __func__);
//
// Currently the core_ddr_config register defaults to desired
// configuration on reset. Currently reprogramming the power on
// reset (POR) value in case it might have been modified by
// bootloaders. In the future, if this changes, then the desired
// values will need to be programmed appropriately.
//
    if (msm_host.updated_ddr_cfg)
    ddr_cfg_offset = msm_offset.core_ddr_config;
    else
    ddr_cfg_offset = msm_offset.core_ddr_config_old;
    writel_relaxed(msm_host.ddr_config, host.ioaddr + ddr_cfg_offset);
    if (mmc.ios.enhanced_strobe) {
    config = readl_relaxed(host.ioaddr +
    msm_offset.core_ddr_200_cfg);
    config |= CORE_CMDIN_RCLK_EN;
    writel_relaxed(config, host.ioaddr +
    msm_offset.core_ddr_200_cfg);
    }
    config = readl_relaxed(host.ioaddr + msm_offset.core_dll_config_2);
    config |= CORE_DDR_CAL_EN;
    writel_relaxed(config, host.ioaddr + msm_offset.core_dll_config_2);
    ret = readl_relaxed_poll_timeout(host.ioaddr +
    msm_offset.core_dll_status,
    dll_status,
    (dll_status & CORE_DDR_DLL_LOCK),
    10, 1000);
    if (ret == -ETIMEDOUT) {
    pr_err("%s: %s: CM_DLL_SDC4 calibration was not completed\n",
    mmc_hostname(host.mmc), __func__);
    goto out;
    }
//
// Set CORE_PWRSAVE_DLL bit in CORE_VENDOR_SPEC3.
// When MCLK is gated OFF, it is not gated for less than 0.5us
// and MCLK must be switched on for at-least 1us before DATA
// starts coming. Controllers with 14lpp and later tech DLL cannot
// guarantee above requirement. So PWRSAVE_DLL should not be
// turned on for host controllers using this DLL.
//
    if (!msm_host.use_14lpp_dll_reset) {
    config = readl_relaxed(host.ioaddr +
    msm_offset.core_vendor_spec3);
    config |= CORE_PWRSAVE_DLL;
    writel_relaxed(config, host.ioaddr +
    msm_offset.core_vendor_spec3);
    }
//
// Drain writebuffer to ensure above DLL calibration
// and PWRSAVE DLL is enabled.
//
    wmb();
    out:
    pr_debug("%s: %s: Exit, ret %d\n", mmc_hostname(host.mmc),
    __func__, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_hs400_dll_calibration(host: *mut sdhci_host) -> c_int {
    static int sdhci_msm_hs400_dll_calibration(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    struct mmc_host *mmc = host.mmc;
    int ret;
    u32 config;
    const struct sdhci_msm_offset *msm_offset =
    msm_host.offset;
    pr_debug("%s: %s: Enter\n", mmc_hostname(host.mmc), __func__);
//
// Retuning in HS400 (DDR mode) will fail, just reset the
// tuning block and restore the saved tuning phase.
//
    ret = msm_init_cm_dll(host);
    if (ret)
    goto out;
    if (!mmc.ios.enhanced_strobe) {
// Set the selected phase in delay line hw block
    ret = msm_config_cm_dll_phase(host,
    msm_host.saved_tuning_phase);
    if (ret)
    goto out;
    config = readl_relaxed(host.ioaddr +
    msm_offset.core_dll_config);
    config |= CORE_CMD_DAT_TRACK_SEL;
    writel_relaxed(config, host.ioaddr +
    msm_offset.core_dll_config);
    }
    if (msm_host.use_cdclp533)
    ret = sdhci_msm_cdclp533_calibration(host);
    else
    ret = sdhci_msm_cm_dll_sdc4_calibration(host);
    out:
    pr_debug("%s: %s: Exit, ret %d\n", mmc_hostname(host.mmc),
    __func__, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_is_tuning_needed(host: *mut sdhci_host) -> bool {
    static bool sdhci_msm_is_tuning_needed(struct sdhci_host *host)
    {
    struct mmc_ios *ios = &host.mmc.ios;
    if (ios.timing == MMC_TIMING_UHS_SDR50 &&
    host.flags & SDHCI_SDR50_NEEDS_TUNING)
    return true;
//
// Tuning is required for SDR104, HS200 and HS400 cards and
// if clock frequency is greater than 100MHz in these modes.
//
    if (host.clock <= CORE_FREQ_100MHZ ||
    !(ios.timing == MMC_TIMING_MMC_HS400 ||
    ios.timing == MMC_TIMING_MMC_HS200 ||
    ios.timing == MMC_TIMING_UHS_SDR104) ||
    ios.enhanced_strobe)
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_restore_sdr_dll_config(host: *mut sdhci_host) -> c_int {
    static int sdhci_msm_restore_sdr_dll_config(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    int ret;
//
// SDR DLL comes into picture only for timing modes which needs
// tuning.
//
    if (!sdhci_msm_is_tuning_needed(host))
    return 0;
// Reset the tuning block
    ret = msm_init_cm_dll(host);
    if (ret)
    return ret;
// Restore the tuning block
    ret = msm_config_cm_dll_phase(host, msm_host.saved_tuning_phase);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_set_cdr(host: *mut sdhci_host, enable: bool) {
    static void sdhci_msm_set_cdr(struct sdhci_host *host, bool enable)
    {
    const struct sdhci_msm_offset *msm_offset = sdhci_priv_msm_offset(host);
    u32 config, oldconfig = readl_relaxed(host.ioaddr +
    msm_offset.core_dll_config);
    config = oldconfig;
    if (enable) {
    config |= CORE_CDR_EN;
    config &= ~CORE_CDR_EXT_EN;
    } else {
    config &= ~CORE_CDR_EN;
    config |= CORE_CDR_EXT_EN;
    }
    if (config != oldconfig) {
    writel_relaxed(config, host.ioaddr +
    msm_offset.core_dll_config);
    }
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_execute_tuning(mmc: *mut mmc_host, opcode: u32) -> c_int {
    static int sdhci_msm_execute_tuning(struct mmc_host *mmc, u32 opcode)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    let mut tuning_seq_cnt: c_int = 10;
    u8 phase, tuned_phases[16], tuned_phase_cnt = 0;
    int rc;
    let mut ios: mmc_ios = host.mmc.ios;
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    const struct sdhci_msm_offset *msm_offset = msm_host.offset;
    u32 config;
    if (!sdhci_msm_is_tuning_needed(host)) {
    msm_host.use_cdr = false;
    sdhci_msm_set_cdr(host, false);
    return 0;
    }
// Clock-Data-Recovery used to dynamically adjust RX sampling point
    msm_host.use_cdr = true;
//
// Clear tuning_done flag before tuning to ensure proper
// HS400 settings.
//
    msm_host.tuning_done = 0;
    if (ios.timing == MMC_TIMING_UHS_SDR50 &&
    host.flags & SDHCI_SDR50_NEEDS_TUNING) {
    config = readl_relaxed(host.ioaddr + msm_offset.core_vendor_spec);
    config &= ~CORE_HC_SELECT_IN_MASK;
    config |= CORE_HC_SELECT_IN_EN | CORE_HC_SELECT_IN_SDR50;
    writel_relaxed(config, host.ioaddr + msm_offset.core_vendor_spec);
    }
//
// For HS400 tuning in HS200 timing requires:
// - select MCLK/2 in VENDOR_SPEC
// - program MCLK to 400MHz (or nearest supported) in GCC
//
    if (host.flags & SDHCI_HS400_TUNING) {
    sdhci_msm_hc_select_mode(host);
    msm_set_clock_rate_for_bus_mode(host, ios.clock, ios.timing);
    host.flags &= ~SDHCI_HS400_TUNING;
    }
    retry:
// First of all reset the tuning block
    rc = msm_init_cm_dll(host);
    if (rc)
    return rc;
    phase = 0;
    do {
// Set the phase in delay line hw block
    rc = msm_config_cm_dll_phase(host, phase);
    if (rc)
    return rc;
    rc = mmc_send_tuning(mmc, opcode, core::ptr::null_mut());
    if (!rc) {
// Tuning is successful at this tuning point
    tuned_phases[tuned_phase_cnt++] = phase;
    dev_dbg(mmc_dev(mmc), "%s: Found good phase = %d\n",
    mmc_hostname(mmc), phase);
    }
    } while (++phase < ARRAY_SIZE(tuned_phases));
    if (tuned_phase_cnt) {
    if (tuned_phase_cnt == ARRAY_SIZE(tuned_phases)) {
//
// All phases valid is _almost_ as bad as no phases
// valid.  Probably all phases are not really reliable
// but we didn't detect where the unreliable place is.
// That means we'll essentially be guessing and hoping
// we get a good phase.  Better to try a few times.
//
    dev_dbg(mmc_dev(mmc), "%s: All phases valid; try again\n",
    mmc_hostname(mmc));
    if (--tuning_seq_cnt) {
    tuned_phase_cnt = 0;
    goto retry;
    }
    }
    rc = msm_find_most_appropriate_phase(host, tuned_phases,
    tuned_phase_cnt);
    if (rc < 0)
    return rc;
    else
    phase = rc;
//
// Finally set the selected phase in delay
// line hw block.
//
    rc = msm_config_cm_dll_phase(host, phase);
    if (rc)
    return rc;
    msm_host.saved_tuning_phase = phase;
    dev_dbg(mmc_dev(mmc), "%s: Setting the tuning phase to %d\n",
    mmc_hostname(mmc), phase);
    } else {
    if (--tuning_seq_cnt)
    goto retry;
// Tuning failed
    dev_dbg(mmc_dev(mmc), "%s: No tuning point found\n",
    mmc_hostname(mmc));
    rc = -EIO;
    }
    if (!rc)
    msm_host.tuning_done = true;
    return rc;
    }
//
// sdhci_msm_hs400 - Calibrate the DLL for HS400 bus speed mode operation.
// This needs to be done for both tuning and enhanced_strobe mode.
// DLL operation is only needed for clock > 100MHz. For clock <= 100MHz
// fixed feedback clock is used.
//
#[no_mangle]
unsafe extern "C" fn sdhci_msm_hs400(host: *mut sdhci_host, ios: *mut mmc_ios) {
    static void sdhci_msm_hs400(struct sdhci_host *host, struct mmc_ios *ios)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    int ret;
    if (host.clock > CORE_FREQ_100MHZ &&
    (msm_host.tuning_done || ios.enhanced_strobe) &&
    !msm_host.calibration_done) {
    ret = sdhci_msm_hs400_dll_calibration(host);
    if (!ret)
    msm_host.calibration_done = true;
    else
    pr_err("%s: Failed to calibrate DLL for hs400 mode (%d)\n",
    mmc_hostname(host.mmc), ret);
    }
    }
    static void sdhci_msm_set_uhs_signaling(struct sdhci_host *host,
    unsigned int uhs)
    {
    struct mmc_host *mmc = host.mmc;
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    u16 ctrl_2;
    u32 config;
    const struct sdhci_msm_offset *msm_offset =
    msm_host.offset;
    ctrl_2 = sdhci_readw(host, SDHCI_HOST_CONTROL2);
// Select Bus Speed Mode for host
    ctrl_2 &= ~SDHCI_CTRL_UHS_MASK;
    switch (uhs) {
    case MMC_TIMING_UHS_SDR12:
    ctrl_2 |= SDHCI_CTRL_UHS_SDR12;
    break;
    case MMC_TIMING_UHS_SDR25:
    ctrl_2 |= SDHCI_CTRL_UHS_SDR25;
    break;
    case MMC_TIMING_UHS_SDR50:
    ctrl_2 |= SDHCI_CTRL_UHS_SDR50;
    break;
    case MMC_TIMING_MMC_HS400:
    case MMC_TIMING_MMC_HS200:
    case MMC_TIMING_UHS_SDR104:
    ctrl_2 |= SDHCI_CTRL_UHS_SDR104;
    break;
    case MMC_TIMING_UHS_DDR50:
    case MMC_TIMING_MMC_DDR52:
    ctrl_2 |= SDHCI_CTRL_UHS_DDR50;
    break;
    }
//
// When clock frequency is less than 100MHz, the feedback clock must be
// provided and DLL must not be used so that tuning can be skipped. To
// provide feedback clock, the mode selection can be any value less
// than 3'b011 in bits [2:0] of HOST CONTROL2 register.
//
    if (host.clock <= CORE_FREQ_100MHZ) {
    if (uhs == MMC_TIMING_MMC_HS400 ||
    uhs == MMC_TIMING_MMC_HS200 ||
    uhs == MMC_TIMING_UHS_SDR104)
    ctrl_2 &= ~SDHCI_CTRL_UHS_MASK;
//
// DLL is not required for clock <= 100MHz
// Thus, make sure DLL it is disabled when not required
//
    config = readl_relaxed(host.ioaddr +
    msm_offset.core_dll_config);
    config |= CORE_DLL_RST;
    writel_relaxed(config, host.ioaddr +
    msm_offset.core_dll_config);
    config = readl_relaxed(host.ioaddr +
    msm_offset.core_dll_config);
    config |= CORE_DLL_PDN;
    writel_relaxed(config, host.ioaddr +
    msm_offset.core_dll_config);
//
// The DLL needs to be restored and CDCLP533 recalibrated
// when the clock frequency is set back to 400MHz.
//
    msm_host.calibration_done = false;
    }
    dev_dbg(mmc_dev(mmc), "%s: clock=%u uhs=%u ctrl_2=0x%x\n",
    mmc_hostname(host.mmc), host.clock, uhs, ctrl_2);
    sdhci_writew(host, ctrl_2, SDHCI_HOST_CONTROL2);
    if (mmc.ios.timing == MMC_TIMING_MMC_HS400)
    sdhci_msm_hs400(host, &mmc.ios);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_set_pincfg(msm_host: *mut sdhci_msm_host, level: bool) -> c_int {
    static int sdhci_msm_set_pincfg(struct sdhci_msm_host *msm_host, bool level)
    {
    struct platform_device *pdev = msm_host.pdev;
    int ret;
    if (level)
    ret = pinctrl_pm_select_default_state(&pdev.dev);
    else
    ret = pinctrl_pm_select_sleep_state(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn msm_config_vmmc_regulator(mmc: *mut mmc_host, hpm: bool) {
    static void msm_config_vmmc_regulator(struct mmc_host *mmc, bool hpm)
    {
    int load;
    if (!hpm)
    load = 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !mmc->card) -> else {
    else if (!mmc.card)
    load = max(MMC_VMMC_MAX_LOAD_UA, SD_VMMC_MAX_LOAD_UA);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: mmc_card_mmc(mmc->card)) -> else {
    else if (mmc_card_mmc(mmc.card))
    load = MMC_VMMC_MAX_LOAD_UA;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: mmc_card_sd(mmc->card)) -> else {
    else if (mmc_card_sd(mmc.card))
    load = SD_VMMC_MAX_LOAD_UA;
    else
    return;
    regulator_set_load(mmc.supply.vmmc, load);
    }
#[no_mangle]
unsafe extern "C" fn msm_config_vqmmc_regulator(mmc: *mut mmc_host, hpm: bool) {
    static void msm_config_vqmmc_regulator(struct mmc_host *mmc, bool hpm)
    {
    int load;
    if (!hpm)
    load = 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !mmc->card) -> else {
    else if (!mmc.card)
    load = max(MMC_VQMMC_MAX_LOAD_UA, SD_VQMMC_MAX_LOAD_UA);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: mmc_card_sd(mmc->card)) -> else {
    else if (mmc_card_sd(mmc.card))
    load = SD_VQMMC_MAX_LOAD_UA;
    else
    return;
    regulator_set_load(mmc.supply.vqmmc, load);
    }
    static int sdhci_msm_set_vmmc(struct sdhci_msm_host *msm_host,
    struct mmc_host *mmc, bool hpm)
    {
    if (IS_ERR(mmc.supply.vmmc))
    return 0;
    msm_config_vmmc_regulator(mmc, hpm);
    return mmc_regulator_set_ocr(mmc, mmc.supply.vmmc, mmc.ios.vdd);
    }
    static int msm_toggle_vqmmc(struct sdhci_msm_host *msm_host,
    struct mmc_host *mmc, bool level)
    {
    int ret;
    struct mmc_ios ios;
    if (msm_host.vqmmc_enabled == level)
    return 0;
    msm_config_vqmmc_regulator(mmc, level);
    if (level) {
// Set the IO voltage regulator to default voltage level
    if (msm_host.caps_0 & CORE_3_0V_SUPPORT)
    ios.signal_voltage = MMC_SIGNAL_VOLTAGE_330;
#[no_mangle]
pub unsafe extern "C" fn if(CORE_1_8V_SUPPORT: msm_host->caps_0 &) -> else {
    else if (msm_host.caps_0 & CORE_1_8V_SUPPORT)
    ios.signal_voltage = MMC_SIGNAL_VOLTAGE_180;
    if (msm_host.caps_0 & CORE_VOLT_SUPPORT) {
    ret = mmc_regulator_set_vqmmc(mmc, &ios);
    if (ret < 0) {
    dev_err(mmc_dev(mmc), "%s: vqmmc set volgate failed: %d\n",
    mmc_hostname(mmc), ret);
    goto out;
    }
    }
    ret = regulator_enable(mmc.supply.vqmmc);
    } else {
    ret = regulator_disable(mmc.supply.vqmmc);
    }
    if (ret)
    dev_err(mmc_dev(mmc), "%s: vqmm %sable failed: %d\n",
    mmc_hostname(mmc), level ? "en":"dis", ret);
    else
    msm_host.vqmmc_enabled = level;
    out:
    return ret;
    }
    static int msm_config_vqmmc_mode(struct sdhci_msm_host *msm_host,
    struct mmc_host *mmc, bool hpm)
    {
    int load, ret;
    load = hpm ? MMC_VQMMC_MAX_LOAD_UA : 0;
    ret = regulator_set_load(mmc.supply.vqmmc, load);
    if (ret)
    dev_err(mmc_dev(mmc), "%s: vqmmc set load failed: %d\n",
    mmc_hostname(mmc), ret);
    return ret;
    }
    static int sdhci_msm_set_vqmmc(struct sdhci_msm_host *msm_host,
    struct mmc_host *mmc, bool level)
    {
    int ret;
    bool always_on;
    if (IS_ERR(mmc.supply.vqmmc) ||
    (mmc.ios.power_mode == MMC_POWER_UNDEFINED))
    return 0;
//
// For eMMC don't turn off Vqmmc, Instead just configure it in LPM
// and HPM modes by setting the corresponding load.
//
// Till eMMC is initialized (i.e. always_on == 0), just turn on/off
// Vqmmc. Vqmmc gets turned off only if init fails and mmc_power_off
// gets invoked. Once eMMC is initialized (i.e. always_on == 1),
// Vqmmc should remain ON, So just set the load instead of turning it
// off/on.
//
    always_on = !mmc_card_is_removable(mmc) &&
    mmc.card && mmc_card_mmc(mmc.card);
    if (always_on)
    ret = msm_config_vqmmc_mode(msm_host, mmc, level);
    else
    ret = msm_toggle_vqmmc(msm_host, mmc, level);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn sdhci_msm_init_pwr_irq_wait(msm_host: *mut sdhci_msm_host) {
    static inline void sdhci_msm_init_pwr_irq_wait(struct sdhci_msm_host *msm_host)
    {
    init_waitqueue_head(&msm_host.pwr_irq_wait);
    }
    static inline void sdhci_msm_complete_pwr_irq_wait(
    struct sdhci_msm_host *msm_host)
    {
    wake_up(&msm_host.pwr_irq_wait);
    }
//
// sdhci_msm_check_power_status API should be called when registers writes
// which can toggle sdhci IO bus ON/OFF or change IO lines HIGH/LOW happens.
// To what state the register writes will change the IO lines should be passed
// as the argument req_type. This API will check whether the IO line's state
// is already the expected state and will wait for power irq only if
// power irq is expected to be triggered based on the current IO line state
// and expected IO line state.
//
#[no_mangle]
unsafe extern "C" fn sdhci_msm_check_power_status(host: *mut sdhci_host, req_type: u32) {
    static void sdhci_msm_check_power_status(struct sdhci_host *host, u32 req_type)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    struct mmc_host *mmc = host.mmc;
    let mut done: bool = false;
    let mut val: u32 = SWITCHABLE_SIGNALING_VOLTAGE;
    const struct sdhci_msm_offset *msm_offset =
    msm_host.offset;
    pr_debug("%s: %s: request %d curr_pwr_state %x curr_io_level %x\n",
    mmc_hostname(host.mmc), __func__, req_type,
    msm_host.curr_pwr_state, msm_host.curr_io_level);
//
// The power interrupt will not be generated for signal voltage
// switches if SWITCHABLE_SIGNALING_VOLTAGE in MCI_GENERICS is not set.
// Since sdhci-msm-v5, this bit has been removed and SW must consider
// it as always set.
//
    if (!msm_host.mci_removed)
    val = msm_host_readl(msm_host, host,
    msm_offset.core_generics);
    if ((req_type & REQ_IO_HIGH || req_type & REQ_IO_LOW) &&
    !(val & SWITCHABLE_SIGNALING_VOLTAGE)) {
    return;
    }
//
// The IRQ for request type IO High/LOW will be generated when -
// there is a state change in 1.8V enable bit (bit 3) of
// SDHCI_HOST_CONTROL2 register. The reset state of that bit is 0
// which indicates 3.3V IO voltage. So, when MMC core layer tries
// to set it to 3.3V before card detection happens, the
// IRQ doesn't get triggered as there is no state change in this bit.
// The driver already handles this case by changing the IO voltage
// level to high as part of controller power up sequence. Hence, check
// for host->pwr to handle a case where IO voltage high request is
// issued even before controller power up.
//
    if ((req_type & REQ_IO_HIGH) && !host.pwr) {
    pr_debug("%s: do not wait for power IRQ that never comes, req_type: %d\n",
    mmc_hostname(host.mmc), req_type);
    return;
    }
    if ((req_type & msm_host.curr_pwr_state) ||
    (req_type & msm_host.curr_io_level))
    done = true;
//
// This is needed here to handle cases where register writes will
// not change the current bus state or io level of the controller.
// In this case, no power irq will be triggerred and we should
// not wait.
//
    if (!done) {
    if (!wait_event_timeout(msm_host.pwr_irq_wait,
    msm_host.pwr_irq_flag,
    msecs_to_jiffies(MSM_PWR_IRQ_TIMEOUT_MS)))
    dev_warn(&msm_host.pdev.dev,
    "%s: pwr_irq for req: (%d) timed out\n",
    mmc_hostname(host.mmc), req_type);
    }
    if ((req_type & REQ_BUS_ON) && mmc.card && !mmc.ops.get_cd(mmc)) {
    sdhci_writeb(host, 0, SDHCI_POWER_CONTROL);
    host.pwr = 0;
    }
    pr_debug("%s: %s: request %d done\n", mmc_hostname(host.mmc),
    __func__, req_type);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_dump_pwr_ctrl_regs(host: *mut sdhci_host) {
    static void sdhci_msm_dump_pwr_ctrl_regs(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    const struct sdhci_msm_offset *msm_offset =
    msm_host.offset;
    pr_err("%s: PWRCTL_STATUS: 0x%08x | PWRCTL_MASK: 0x%08x | PWRCTL_CTL: 0x%08x\n",
    mmc_hostname(host.mmc),
    msm_host_readl(msm_host, host, msm_offset.core_pwrctl_status),
    msm_host_readl(msm_host, host, msm_offset.core_pwrctl_mask),
    msm_host_readl(msm_host, host, msm_offset.core_pwrctl_ctl));
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_handle_pwr_irq(host: *mut sdhci_host, irq: c_int) {
    static void sdhci_msm_handle_pwr_irq(struct sdhci_host *host, int irq)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    struct mmc_host *mmc = host.mmc;
    u32 irq_status, irq_ack = 0;
    let mut retry: c_int = 10, ret;
    let mut pwr_state: u32 = 0, io_level = 0;
    u32 config;
    const struct sdhci_msm_offset *msm_offset = msm_host.offset;
    irq_status = msm_host_readl(msm_host, host,
    msm_offset.core_pwrctl_status);
    irq_status &= INT_MASK;
    msm_host_writel(msm_host, irq_status, host,
    msm_offset.core_pwrctl_clear);
//
// There is a rare HW scenario where the first clear pulse could be
// lost when actual reset and clear/read of status register is
// happening at a time. Hence, retry for at least 10 times to make
// sure status register is cleared. Otherwise, this will result in
// a spurious power IRQ resulting in system instability.
//
    while (irq_status & msm_host_readl(msm_host, host,
    msm_offset.core_pwrctl_status)) {
    if (retry == 0) {
    pr_err("%s: Timedout clearing (0x%x) pwrctl status register\n",
    mmc_hostname(host.mmc), irq_status);
    sdhci_msm_dump_pwr_ctrl_regs(host);
    WARN_ON(1);
    break;
    }
    msm_host_writel(msm_host, irq_status, host,
    msm_offset.core_pwrctl_clear);
    retry--;
    udelay(10);
    }
    if ((irq_status & CORE_PWRCTL_BUS_ON) && mmc.card &&
    !mmc.ops.get_cd(mmc)) {
    msm_host_writel(msm_host, CORE_PWRCTL_BUS_FAIL, host,
    msm_offset.core_pwrctl_ctl);
    return;
    }
// Handle BUS ON/OFF
    if (irq_status & CORE_PWRCTL_BUS_ON) {
    pwr_state = REQ_BUS_ON;
    io_level = REQ_IO_HIGH;
    }
    if (irq_status & CORE_PWRCTL_BUS_OFF) {
    pwr_state = REQ_BUS_OFF;
    io_level = REQ_IO_LOW;
    }
    if (pwr_state) {
    ret = sdhci_msm_set_vmmc(msm_host, mmc,
    pwr_state & REQ_BUS_ON);
    if (!ret)
    ret = sdhci_msm_set_vqmmc(msm_host, mmc,
    pwr_state & REQ_BUS_ON);
    if (!ret)
    ret = sdhci_msm_set_pincfg(msm_host,
    pwr_state & REQ_BUS_ON);
    if (!ret)
    irq_ack |= CORE_PWRCTL_BUS_SUCCESS;
    else
    irq_ack |= CORE_PWRCTL_BUS_FAIL;
    }
// Handle IO LOW/HIGH
    if (irq_status & CORE_PWRCTL_IO_LOW)
    io_level = REQ_IO_LOW;
    if (irq_status & CORE_PWRCTL_IO_HIGH)
    io_level = REQ_IO_HIGH;
    if (io_level)
    irq_ack |= CORE_PWRCTL_IO_SUCCESS;
    if (io_level && !IS_ERR(mmc.supply.vqmmc) && !pwr_state) {
    ret = mmc_regulator_set_vqmmc(mmc, &mmc.ios);
    if (ret < 0) {
    dev_err(mmc_dev(mmc), "%s: IO_level setting failed(%d). signal_voltage: %d, vdd: %d irq_status: 0x%08x\n",
    mmc_hostname(mmc), ret,
    mmc.ios.signal_voltage, mmc.ios.vdd,
    irq_status);
    irq_ack |= CORE_PWRCTL_IO_FAIL;
    }
    }
//
// The driver has to acknowledge the interrupt, switch voltages and
// report back if it succeded or not to this register. The voltage
// switches are handled by the sdhci core, so just report success.
//
    msm_host_writel(msm_host, irq_ack, host,
    msm_offset.core_pwrctl_ctl);
//
// If we don't have info regarding the voltage levels supported by
// regulators, don't change the IO PAD PWR SWITCH.
//
    if (msm_host.caps_0 & CORE_VOLT_SUPPORT) {
    u32 new_config;
//
// We should unset IO PAD PWR switch only if the register write
// can set IO lines high and the regulator also switches to 3 V.
// Else, we should keep the IO PAD PWR switch set.
// This is applicable to certain targets where eMMC vccq supply
// is only 1.8V. In such targets, even during REQ_IO_HIGH, the
// IO PAD PWR switch must be kept set to reflect actual
// regulator voltage. This way, during initialization of
// controllers with only 1.8V, we will set the IO PAD bit
// without waiting for a REQ_IO_LOW.
//
    config = readl_relaxed(host.ioaddr +
    msm_offset.core_vendor_spec);
    new_config = config;
    if ((io_level & REQ_IO_HIGH) &&
    (msm_host.caps_0 & CORE_3_0V_SUPPORT))
    new_config &= ~CORE_IO_PAD_PWR_SWITCH;
    else if ((io_level & REQ_IO_LOW) ||
    (msm_host.caps_0 & CORE_1_8V_SUPPORT))
    new_config |= CORE_IO_PAD_PWR_SWITCH;
    if (config ^ new_config)
    writel_relaxed(new_config, host.ioaddr +
    msm_offset.core_vendor_spec);
    }
    if (pwr_state)
    msm_host.curr_pwr_state = pwr_state;
    if (io_level)
    msm_host.curr_io_level = io_level;
    dev_dbg(mmc_dev(mmc), "%s: %s: Handled IRQ(%d), irq_status=0x%x, ack=0x%x\n",
    mmc_hostname(msm_host.mmc), __func__, irq, irq_status,
    irq_ack);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_pwr_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t sdhci_msm_pwr_irq(int irq, void *data)
    {
    struct sdhci_host *host = (struct sdhci_host *)data;
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    sdhci_msm_handle_pwr_irq(host, irq);
    msm_host.pwr_irq_flag = 1;
    sdhci_msm_complete_pwr_irq_wait(msm_host);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_get_max_clock(host: *mut sdhci_host) -> c_uint {
    static unsigned int sdhci_msm_get_max_clock(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    struct clk *core_clk = msm_host.bulk_clks[0].clk;
    return clk_round_rate(core_clk, ULONG_MAX);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_get_min_clock(host: *mut sdhci_host) -> c_uint {
    static unsigned int sdhci_msm_get_min_clock(struct sdhci_host *host)
    {
    return SDHCI_MSM_MIN_CLOCK;
    }
//
// __sdhci_msm_set_clock - sdhci_msm clock control.
//
// Description:
// MSM controller does not use internal divider and
// instead directly control the GCC clock as per
// HW recommendation.
//
#[no_mangle]
unsafe extern "C" fn __sdhci_msm_set_clock(host: *mut sdhci_host, clock: c_uint) {
    static void __sdhci_msm_set_clock(struct sdhci_host *host, unsigned int clock)
    {
    u16 clk;
    sdhci_writew(host, 0, SDHCI_CLOCK_CONTROL);
    if (clock == 0)
    return;
//
// MSM controller do not use clock divider.
// Thus read SDHCI_CLOCK_CONTROL and only enable
// clock with no divider value programmed.
//
    clk = sdhci_readw(host, SDHCI_CLOCK_CONTROL);
    sdhci_enable_clk(host, clk);
    }
// sdhci_msm_set_clock - Called with (host->lock) spinlock held.
#[no_mangle]
unsafe extern "C" fn sdhci_msm_set_clock(host: *mut sdhci_host, clock: c_uint) {
    static void sdhci_msm_set_clock(struct sdhci_host *host, unsigned int clock)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    let mut ios: mmc_ios = host.mmc.ios;
    if (!clock) {
    host.mmc.actual_clock = msm_host.clk_rate = 0;
    goto out;
    }
    sdhci_msm_hc_select_mode(host);
    msm_set_clock_rate_for_bus_mode(host, ios.clock, ios.timing);
    out:
    __sdhci_msm_set_clock(host, clock);
    }
// \
//
// Inline Crypto Engine (ICE) support
//
    \*****************************************************************************/

    static const struct blk_crypto_ll_ops sdhci_msm_crypto_ops; /* forward decl */
    static int sdhci_msm_ice_init(struct sdhci_msm_host *msm_host,
    struct cqhci_host *cq_host)
    {
    struct mmc_host *mmc = msm_host.mmc;
    struct blk_crypto_profile *profile = &mmc.crypto_profile;
    struct device *dev = mmc_dev(mmc);
    struct qcom_ice *ice;
    union cqhci_crypto_capabilities caps;
    union cqhci_crypto_cap_entry cap;
    int err;
    int i;
    if (!(cqhci_readl(cq_host, CQHCI_CAP) & CQHCI_CAP_CS))
    return 0;
    ice = devm_of_qcom_ice_get(dev);
    if (IS_ERR(ice)) {
    if (ice != ERR_PTR(-EOPNOTSUPP))
    return PTR_ERR(ice);
    dev_warn(dev, "Disabling inline encryption support\n");
    return 0;
    }
    msm_host.ice = ice;
// Initialize the blk_crypto_profile
    caps.reg_val = cpu_to_le32(cqhci_readl(cq_host, CQHCI_CCAP));
// The number of keyslots supported is (CFGC+1)
    err = devm_blk_crypto_profile_init(dev, profile, caps.config_count + 1);
    if (err)
    return err;
    profile.ll_ops = sdhci_msm_crypto_ops;
    profile.max_dun_bytes_supported = 4;
    profile.key_types_supported = qcom_ice_get_supported_key_type(ice);
    profile.dev = dev;
//
// Currently this driver only supports AES-256-XTS.  All known versions
// of ICE support it, but to be safe make sure it is really declared in
// the crypto capability registers.  The crypto capability registers
// also give the supported data unit size(s).
//
    for (i = 0; i < caps.num_crypto_cap; i++) {
    cap.reg_val = cpu_to_le32(cqhci_readl(cq_host,
    CQHCI_CRYPTOCAP +
    i * sizeof(__le32)));
    if (cap.algorithm_id == CQHCI_CRYPTO_ALG_AES_XTS &&
    cap.key_size == CQHCI_CRYPTO_KEY_SIZE_256)
    profile.modes_supported[BLK_ENCRYPTION_MODE_AES_256_XTS] |=
    cap.sdus_mask * 512;
    }
    mmc.caps2 |= MMC_CAP2_CRYPTO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_ice_enable(msm_host: *mut sdhci_msm_host) {
    static void sdhci_msm_ice_enable(struct sdhci_msm_host *msm_host)
    {
    if (msm_host.mmc.caps2 & MMC_CAP2_CRYPTO)
    qcom_ice_enable(msm_host.ice);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_ice_resume(msm_host: *mut sdhci_msm_host) -> c_int {
    static int sdhci_msm_ice_resume(struct sdhci_msm_host *msm_host)
    {
    if (msm_host.mmc.caps2 & MMC_CAP2_CRYPTO)
    return qcom_ice_resume(msm_host.ice);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_ice_suspend(msm_host: *mut sdhci_msm_host) -> c_int {
    static int sdhci_msm_ice_suspend(struct sdhci_msm_host *msm_host)
    {
    if (msm_host.mmc.caps2 & MMC_CAP2_CRYPTO)
    return qcom_ice_suspend(msm_host.ice);
    return 0;
    }
    static inline struct sdhci_msm_host *
    sdhci_msm_host_from_crypto_profile(struct blk_crypto_profile *profile)
    {
    struct mmc_host *mmc = mmc_from_crypto_profile(profile);
    struct sdhci_host *host = mmc_priv(mmc);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    return msm_host;
    }
//
// Program a key into a QC ICE keyslot.  QC ICE requires a QC-specific SCM call
// for this; it doesn't support the standard way.
//
    static int sdhci_msm_ice_keyslot_program(struct blk_crypto_profile *profile,
    const struct blk_crypto_key *key,
    unsigned int slot)
    {
    struct sdhci_msm_host *msm_host =
    sdhci_msm_host_from_crypto_profile(profile);
    return qcom_ice_program_key(msm_host.ice, slot, key);
    }
    static int sdhci_msm_ice_keyslot_evict(struct blk_crypto_profile *profile,
    const struct blk_crypto_key *key,
    unsigned int slot)
    {
    struct sdhci_msm_host *msm_host =
    sdhci_msm_host_from_crypto_profile(profile);
    return qcom_ice_evict_key(msm_host.ice, slot);
    }
    static int sdhci_msm_ice_derive_sw_secret(struct blk_crypto_profile *profile,
    const u8 *eph_key, size_t eph_key_size,
    u8 sw_secret[BLK_CRYPTO_SW_SECRET_SIZE])
    {
    struct sdhci_msm_host *msm_host = sdhci_msm_host_from_crypto_profile(profile);
    return qcom_ice_derive_sw_secret(msm_host.ice, eph_key, eph_key_size,
    sw_secret);
    }
    static int sdhci_msm_ice_import_key(struct blk_crypto_profile *profile,
    const u8 *raw_key, size_t raw_key_size,
    u8 lt_key[BLK_CRYPTO_MAX_HW_WRAPPED_KEY_SIZE])
    {
    struct sdhci_msm_host *msm_host = sdhci_msm_host_from_crypto_profile(profile);
    return qcom_ice_import_key(msm_host.ice, raw_key, raw_key_size, lt_key);
    }
    static int sdhci_msm_ice_generate_key(struct blk_crypto_profile *profile,
    u8 lt_key[BLK_CRYPTO_MAX_HW_WRAPPED_KEY_SIZE])
    {
    struct sdhci_msm_host *msm_host = sdhci_msm_host_from_crypto_profile(profile);
    return qcom_ice_generate_key(msm_host.ice, lt_key);
    }
    static int sdhci_msm_ice_prepare_key(struct blk_crypto_profile *profile,
    const u8 *lt_key, size_t lt_key_size,
    u8 eph_key[BLK_CRYPTO_MAX_HW_WRAPPED_KEY_SIZE])
    {
    struct sdhci_msm_host *msm_host = sdhci_msm_host_from_crypto_profile(profile);
    return qcom_ice_prepare_key(msm_host.ice, lt_key, lt_key_size, eph_key);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_non_cqe_ice_init(host: *mut sdhci_host) {
    static void sdhci_msm_non_cqe_ice_init(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    struct mmc_host *mmc = msm_host.mmc;
    struct cqhci_host *cq_host = mmc.cqe_private;
    u32 config;
    config = sdhci_readl(host, HC_VENDOR_SPECIFIC_FUNC4);
    config &= ~DISABLE_CRYPTO;
    sdhci_writel(host, config, HC_VENDOR_SPECIFIC_FUNC4);
    config = cqhci_readl(cq_host, CQHCI_CFG);
    config |= CRYPTO_GENERAL_ENABLE;
    cqhci_writel(cq_host, config, CQHCI_CFG);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_ice_cfg(host: *mut sdhci_host, mrq: *mut mmc_request) {
    static void sdhci_msm_ice_cfg(struct sdhci_host *host, struct mmc_request *mrq)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    struct mmc_host *mmc = msm_host.mmc;
    struct cqhci_host *cq_host = mmc.cqe_private;
    let mut crypto_params: c_uint = 0;
    int key_index;
    if (mrq.crypto_ctx) {
    if (!msm_host.non_cqe_ice_init_done) {
    sdhci_msm_non_cqe_ice_init(host);
    msm_host.non_cqe_ice_init_done = true;
    }
    key_index = mrq.crypto_key_slot;
    crypto_params = FIELD_PREP(ICE_HCI_PARAM_CE, 1) |
    FIELD_PREP(ICE_HCI_PARAM_CCI, key_index);
    cqhci_writel(cq_host, crypto_params, NONCQ_CRYPTO_PARM);
    cqhci_writel(cq_host, lower_32_bits(mrq.crypto_ctx.bc_dun[0]),
    NONCQ_CRYPTO_DUN);
    } else {
    cqhci_writel(cq_host, crypto_params, NONCQ_CRYPTO_PARM);
    }
// Ensure crypto configuration is written before proceeding
    wmb();
    }
//
// Handle non-CQE MMC requests with ICE crypto support.
// Configures ICE registers before passing the request to
// the standard SDHCI handler.
//
#[no_mangle]
unsafe extern "C" fn sdhci_msm_request(mmc: *mut mmc_host, mrq: *mut mmc_request) {
    static void sdhci_msm_request(struct mmc_host *mmc, struct mmc_request *mrq)
    {
    struct sdhci_host *host = mmc_priv(mmc);
// Only need to handle non-CQE crypto requests in this path
    if (mmc.caps2 & MMC_CAP2_CRYPTO)
    sdhci_msm_ice_cfg(host, mrq);
    sdhci_request(mmc, mrq);
    }
    static const struct blk_crypto_ll_ops sdhci_msm_crypto_ops = {
    .keyslot_program	= sdhci_msm_ice_keyslot_program,
    .keyslot_evict		= sdhci_msm_ice_keyslot_evict,
    .derive_sw_secret	= sdhci_msm_ice_derive_sw_secret,
    .import_key		= sdhci_msm_ice_import_key,
    .generate_key		= sdhci_msm_ice_generate_key,
    .prepare_key		= sdhci_msm_ice_prepare_key,
    };

    static inline int sdhci_msm_ice_init(struct sdhci_msm_host *msm_host,
    struct cqhci_host *cq_host)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sdhci_msm_ice_enable(msm_host: *mut sdhci_msm_host) {
    static inline void sdhci_msm_ice_enable(struct sdhci_msm_host *msm_host)
    {
    }
    static inline int
    sdhci_msm_ice_resume(struct sdhci_msm_host *msm_host)
    {
    return 0;
    }
    static inline int
    sdhci_msm_ice_suspend(struct sdhci_msm_host *msm_host)
    {
    return 0;
    }

// \
//
// MSM Command Queue Engine (CQE)
//
    \*****************************************************************************/
#[no_mangle]
unsafe extern "C" fn sdhci_msm_cqe_irq(host: *mut sdhci_host, intmask: u32) -> u32 {
    static u32 sdhci_msm_cqe_irq(struct sdhci_host *host, u32 intmask)
    {
    let mut cmd_error: c_int = 0;
    let mut data_error: c_int = 0;
    if (!sdhci_cqe_irq(host, intmask, &cmd_error, &data_error))
    return intmask;
    cqhci_irq(host.mmc, cmd_error, data_error);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_cqe_enable(mmc: *mut mmc_host) {
    static void sdhci_msm_cqe_enable(struct mmc_host *mmc)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    sdhci_cqe_enable(mmc);
    sdhci_msm_ice_enable(msm_host);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_cqe_disable(mmc: *mut mmc_host, recovery: bool) {
    static void sdhci_msm_cqe_disable(struct mmc_host *mmc, bool recovery)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    unsigned long flags;
    u32 ctrl;
//
// When CQE is halted, the legacy SDHCI path operates only
// on 16-byte descriptors in 64bit mode.
//
    if (host.flags & SDHCI_USE_64_BIT_DMA)
    host.desc_sz = 16;
    spin_lock_irqsave(&host.lock, flags);
//
// During CQE command transfers, command complete bit gets latched.
// So s/w should clear command complete interrupt status when CQE is
// either halted or disabled. Otherwise unexpected SDCHI legacy
// interrupt gets triggered when CQE is halted/disabled.
//
    ctrl = sdhci_readl(host, SDHCI_INT_ENABLE);
    ctrl |= SDHCI_INT_RESPONSE;
    sdhci_writel(host,  ctrl, SDHCI_INT_ENABLE);
    sdhci_writel(host, SDHCI_INT_RESPONSE, SDHCI_INT_STATUS);
    spin_unlock_irqrestore(&host.lock, flags);
    sdhci_cqe_disable(mmc, recovery);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_set_timeout(host: *mut sdhci_host, cmd: *mut mmc_command) {
    static void sdhci_msm_set_timeout(struct sdhci_host *host, struct mmc_command *cmd)
    {
    u32 count, start = 15;
    __sdhci_set_timeout(host, cmd);
    count = sdhci_readb(host, SDHCI_TIMEOUT_CONTROL);
//
// Update software timeout value if its value is less than hardware data
// timeout value. Qcom SoC hardware data timeout value was calculated
// using 4 * MCLK * 2^(count + 13). where MCLK = 1 / host->clock.
//
    if (cmd && cmd.data && host.clock > 400000 &&
    host.clock <= 50000000 &&
    ((1 << (count + start)) > (10 * host.clock)))
    host.data_timeout = 22LL * NSEC_PER_SEC;
    }
    static const struct cqhci_host_ops sdhci_msm_cqhci_ops = {
    .enable		= sdhci_msm_cqe_enable,
    .disable	= sdhci_msm_cqe_disable,

    .uses_custom_crypto_profile = true,

    };
    static int sdhci_msm_cqe_add_host(struct sdhci_host *host,
    struct platform_device *pdev)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    struct cqhci_host *cq_host;
    bool dma64;
    u32 cqcfg;
    int ret;
//
// When CQE is halted, SDHC operates only on 16byte ADMA descriptors.
// So ensure ADMA table is allocated for 16byte descriptors.
//
    if (host.caps & SDHCI_CAN_64BIT)
    host.alloc_desc_sz = 16;
    ret = sdhci_setup_host(host);
    if (ret)
    return ret;
    cq_host = cqhci_pltfm_init(pdev);
    if (IS_ERR(cq_host)) {
    ret = PTR_ERR(cq_host);
    dev_err(&pdev.dev, "cqhci-pltfm init: failed: %d\n", ret);
    goto cleanup;
    }
    msm_host.mmc.caps2 |= MMC_CAP2_CQE | MMC_CAP2_CQE_DCMD;
    cq_host.ops = &sdhci_msm_cqhci_ops;
    dma64 = host.flags & SDHCI_USE_64_BIT_DMA;
    ret = sdhci_msm_ice_init(msm_host, cq_host);
    if (ret)
    goto cleanup;
    ret = cqhci_init(cq_host, host.mmc, dma64);
    if (ret) {
    dev_err(&pdev.dev, "%s: CQE init: failed (%d)\n",
    mmc_hostname(host.mmc), ret);
    goto cleanup;
    }
// Disable cqe reset due to cqe enable signal
    cqcfg = cqhci_readl(cq_host, CQHCI_VENDOR_CFG1);
    cqcfg |= CQHCI_VENDOR_DIS_RST_ON_CQ_EN;
    cqhci_writel(cq_host, cqcfg, CQHCI_VENDOR_CFG1);
//
// SDHC expects 12byte ADMA descriptors till CQE is enabled.
// So limit desc_sz to 12 so that the data commands that are sent
// during card initialization (before CQE gets enabled) would
// get executed without any issues.
//
    if (host.flags & SDHCI_USE_64_BIT_DMA)
    host.desc_sz = 12;
    ret = __sdhci_add_host(host);
    if (ret)
    goto cleanup;
    dev_info(&pdev.dev, "%s: CQE init: success\n",
    mmc_hostname(host.mmc));
    return ret;
    cleanup:
    sdhci_cleanup_host(host);
    return ret;
    }
//
// Platform specific register write functions. This is so that, if any
// register write needs to be followed up by platform specific actions,
// they can be added here. These functions can go to sleep when writes
// to certain registers are done.
// These functions are relying on sdhci_set_ios not using spinlock.
//
#[no_mangle]
unsafe extern "C" fn __sdhci_msm_check_write(host: *mut sdhci_host, val: u16, reg: c_int) -> c_int {
    static int __sdhci_msm_check_write(struct sdhci_host *host, u16 val, int reg)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    let mut req_type: u32 = 0;
    switch (reg) {
    case SDHCI_HOST_CONTROL2:
    req_type = (val & SDHCI_CTRL_VDD_180) ? REQ_IO_LOW :
    REQ_IO_HIGH;
    break;
    case SDHCI_SOFTWARE_RESET:
    if (host.pwr && (val & SDHCI_RESET_ALL))
    req_type = REQ_BUS_OFF;
    break;
    case SDHCI_POWER_CONTROL:
    req_type = !val ? REQ_BUS_OFF : REQ_BUS_ON;
    break;
    case SDHCI_TRANSFER_MODE:
    msm_host.transfer_mode = val;
    break;
    case SDHCI_COMMAND:
    if (!msm_host.use_cdr)
    break;
    if ((msm_host.transfer_mode & SDHCI_TRNS_READ) &&
    !mmc_op_tuning(SDHCI_GET_CMD(val)))
    sdhci_msm_set_cdr(host, true);
    else
    sdhci_msm_set_cdr(host, false);
    break;
    }
    if (req_type) {
    msm_host.pwr_irq_flag = 0;
//
// Since this register write may trigger a power irq, ensure
// all previous register writes are complete by this point.
//
    mb();
    }
    return req_type;
    }
// This function may sleep
#[no_mangle]
unsafe extern "C" fn sdhci_msm_writew(host: *mut sdhci_host, val: u16, reg: c_int) {
    static void sdhci_msm_writew(struct sdhci_host *host, u16 val, int reg)
    {
    let mut req_type: u32 = 0;
    req_type = __sdhci_msm_check_write(host, val, reg);
    writew_relaxed(val, host.ioaddr + reg);
    if (req_type)
    sdhci_msm_check_power_status(host, req_type);
    }
// This function may sleep
#[no_mangle]
unsafe extern "C" fn sdhci_msm_writeb(host: *mut sdhci_host, val: u8, reg: c_int) {
    static void sdhci_msm_writeb(struct sdhci_host *host, u8 val, int reg)
    {
    let mut req_type: u32 = 0;
    req_type = __sdhci_msm_check_write(host, val, reg);
    writeb_relaxed(val, host.ioaddr + reg);
    if (req_type)
    sdhci_msm_check_power_status(host, req_type);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_set_regulator_caps(msm_host: *mut sdhci_msm_host) {
    static void sdhci_msm_set_regulator_caps(struct sdhci_msm_host *msm_host)
    {
    struct mmc_host *mmc = msm_host.mmc;
    struct regulator *supply = mmc.supply.vqmmc;
    let mut caps: u32 = 0, config;
    struct sdhci_host *host = mmc_priv(mmc);
    const struct sdhci_msm_offset *msm_offset = msm_host.offset;
    if (!IS_ERR(mmc.supply.vqmmc)) {
    if (regulator_is_supported_voltage(supply, 1700000, 1950000))
    caps |= CORE_1_8V_SUPPORT;
    if (regulator_is_supported_voltage(supply, 2700000, 3600000))
    caps |= CORE_3_0V_SUPPORT;
    if (!caps)
    pr_warn("%s: 1.8/3V not supported for vqmmc\n",
    mmc_hostname(mmc));
    }
    if (caps) {
//
// Set the PAD_PWR_SWITCH_EN bit so that the PAD_PWR_SWITCH
// bit can be used as required later on.
//
    let mut io_level: u32 = msm_host.curr_io_level;
    config = readl_relaxed(host.ioaddr +
    msm_offset.core_vendor_spec);
    config |= CORE_IO_PAD_PWR_SWITCH_EN;
    if ((io_level & REQ_IO_HIGH) && (caps &	CORE_3_0V_SUPPORT))
    config &= ~CORE_IO_PAD_PWR_SWITCH;
#[no_mangle]
pub unsafe extern "C" fn if(CORE_1_8V_SUPPORT): (io_level & REQ_IO_LOW) || (caps &) -> else {
    else if ((io_level & REQ_IO_LOW) || (caps & CORE_1_8V_SUPPORT))
    config |= CORE_IO_PAD_PWR_SWITCH;
    writel_relaxed(config,
    host.ioaddr + msm_offset.core_vendor_spec);
    }
    msm_host.caps_0 |= caps;
    pr_debug("%s: supported caps: 0x%08x\n", mmc_hostname(mmc), caps);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_register_vreg(msm_host: *mut sdhci_msm_host) -> c_int {
    static int sdhci_msm_register_vreg(struct sdhci_msm_host *msm_host)
    {
    int ret;
    ret = mmc_regulator_get_supply(msm_host.mmc);
    if (ret)
    return ret;
    sdhci_msm_set_regulator_caps(msm_host);
    return 0;
    }
    static int sdhci_msm_start_signal_voltage_switch(struct mmc_host *mmc,
    struct mmc_ios *ios)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    u16 ctrl, status;
//
// Signal Voltage Switching is only applicable for Host Controllers
// v3.00 and above.
//
    if (host.version < SDHCI_SPEC_300)
    return 0;
    ctrl = sdhci_readw(host, SDHCI_HOST_CONTROL2);
    switch (ios.signal_voltage) {
    case MMC_SIGNAL_VOLTAGE_330:
    if (!(host.flags & SDHCI_SIGNALING_330))
    return -EINVAL;
// Set 1.8V Signal Enable in the Host Control2 register to 0
    ctrl &= ~SDHCI_CTRL_VDD_180;
    break;
    case MMC_SIGNAL_VOLTAGE_180:
    if (!(host.flags & SDHCI_SIGNALING_180))
    return -EINVAL;
// Enable 1.8V Signal Enable in the Host Control2 register
    ctrl |= SDHCI_CTRL_VDD_180;
    break;
    default:
    return -EINVAL;
    }
    sdhci_writew(host, ctrl, SDHCI_HOST_CONTROL2);
// Wait for 5ms
    usleep_range(5000, 5500);
// regulator output should be stable within 5 ms
    status = ctrl & SDHCI_CTRL_VDD_180;
    ctrl = sdhci_readw(host, SDHCI_HOST_CONTROL2);
    if ((ctrl & SDHCI_CTRL_VDD_180) == status)
    return 0;
    dev_warn(mmc_dev(mmc), "%s: Regulator output did not became stable\n",
    mmc_hostname(mmc));
    return -EAGAIN;
    }

    pr_err("%s: " DRIVER_NAME ": " f, mmc_hostname(host.mmc), ## x)
#[no_mangle]
unsafe extern "C" fn sdhci_msm_dump_vendor_regs(host: *mut sdhci_host) {
    static void sdhci_msm_dump_vendor_regs(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    const struct sdhci_msm_offset *msm_offset = msm_host.offset;
    SDHCI_MSM_DUMP("----------- VENDOR REGISTER DUMP -----------\n");
    SDHCI_MSM_DUMP(
    "DLL sts: 0x%08x | DLL cfg:  0x%08x | DLL cfg2: 0x%08x\n",
    readl_relaxed(host.ioaddr + msm_offset.core_dll_status),
    readl_relaxed(host.ioaddr + msm_offset.core_dll_config),
    readl_relaxed(host.ioaddr + msm_offset.core_dll_config_2));
    SDHCI_MSM_DUMP(
    "DLL cfg3: 0x%08x | DLL usr ctl:  0x%08x | DDR cfg: 0x%08x\n",
    readl_relaxed(host.ioaddr + msm_offset.core_dll_config_3),
    readl_relaxed(host.ioaddr + msm_offset.core_dll_usr_ctl),
    readl_relaxed(host.ioaddr + msm_offset.core_ddr_config));
    SDHCI_MSM_DUMP(
    "Vndr func: 0x%08x | Vndr func2 : 0x%08x Vndr func3: 0x%08x\n",
    readl_relaxed(host.ioaddr + msm_offset.core_vendor_spec),
    readl_relaxed(host.ioaddr +
    msm_offset.core_vendor_spec_func2),
    readl_relaxed(host.ioaddr + msm_offset.core_vendor_spec3));
    }
    static const struct sdhci_msm_variant_ops mci_var_ops = {
    .msm_readl_relaxed = sdhci_msm_mci_variant_readl_relaxed,
    .msm_writel_relaxed = sdhci_msm_mci_variant_writel_relaxed,
    };
    static const struct sdhci_msm_variant_ops v5_var_ops = {
    .msm_readl_relaxed = sdhci_msm_v5_variant_readl_relaxed,
    .msm_writel_relaxed = sdhci_msm_v5_variant_writel_relaxed,
    };
    static const struct sdhci_msm_variant_info sdhci_msm_mci_var = {
    .var_ops = &mci_var_ops,
    .offset = &sdhci_msm_mci_offset,
    };
    static const struct sdhci_msm_variant_info sdhci_msm_v5_var = {
    .mci_removed = true,
    .var_ops = &v5_var_ops,
    .offset = &sdhci_msm_v5_offset,
    };
    static const struct sdhci_msm_variant_info sdm845_sdhci_var = {
    .mci_removed = true,
    .restore_dll_config = true,
    .var_ops = &v5_var_ops,
    .offset = &sdhci_msm_v5_offset,
    };
    static const struct of_device_id sdhci_msm_dt_match[] = {
//
// Do not add new variants to the driver which are compatible with
// generic ones, unless they need customization.
//
    {.compatible = "qcom,sdhci-msm-v4", .data = &sdhci_msm_mci_var},
    {.compatible = "qcom,sdhci-msm-v5", .data = &sdhci_msm_v5_var},
    {.compatible = "qcom,sdm670-sdhci", .data = &sdm845_sdhci_var},
    {.compatible = "qcom,sdm845-sdhci", .data = &sdm845_sdhci_var},
    {.compatible = "qcom,sc7180-sdhci", .data = &sdm845_sdhci_var},
    {},
    };
    MODULE_DEVICE_TABLE(of, sdhci_msm_dt_match);
    static const struct sdhci_ops sdhci_msm_ops = {
    .reset = sdhci_and_cqhci_reset,
    .set_clock = sdhci_msm_set_clock,
    .get_min_clock = sdhci_msm_get_min_clock,
    .get_max_clock = sdhci_msm_get_max_clock,
    .set_bus_width = sdhci_set_bus_width,
    .set_uhs_signaling = sdhci_msm_set_uhs_signaling,
    .write_w = sdhci_msm_writew,
    .write_b = sdhci_msm_writeb,
    .irq	= sdhci_msm_cqe_irq,
    .dump_vendor_regs = sdhci_msm_dump_vendor_regs,
    .set_power = sdhci_set_power_noreg,
    .set_timeout = sdhci_msm_set_timeout,
    };
    static const struct sdhci_pltfm_data sdhci_msm_pdata = {
    .quirks = SDHCI_QUIRK_BROKEN_CARD_DETECTION |
    SDHCI_QUIRK_SINGLE_POWER_WRITE |
    SDHCI_QUIRK_CAP_CLOCK_BASE_BROKEN |
    SDHCI_QUIRK_MULTIBLOCK_READ_ACMD12,
    .quirks2 = SDHCI_QUIRK2_PRESET_VALUE_BROKEN,
    .ops = &sdhci_msm_ops,
    };
    static inline void sdhci_msm_get_of_property(struct platform_device *pdev,
    struct sdhci_host *host)
    {
    struct device_node *node = pdev.dev.of_node;
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    if (of_property_read_u32(node, "qcom,ddr-config",
    &msm_host.ddr_config))
    msm_host.ddr_config = DDR_CONFIG_POR_VAL;
    of_property_read_u32(node, "qcom,dll-config", &msm_host.dll_config);
    if (of_device_is_compatible(node, "qcom,msm8916-sdhci"))
    host.quirks2 |= SDHCI_QUIRK2_BROKEN_64_BIT_DMA;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_gcc_reset(dev: *mut device, host: *mut sdhci_host) -> c_int {
    static int sdhci_msm_gcc_reset(struct device *dev, struct sdhci_host *host)
    {
    struct reset_control *reset;
    let mut ret: c_int = 0;
    reset = reset_control_get_optional_exclusive(dev, core::ptr::null_mut());
    if (IS_ERR(reset))
    return dev_err_probe(dev, PTR_ERR(reset),
    "unable to acquire core_reset\n");
    if (!reset)
    return ret;
    ret = reset_control_assert(reset);
    if (ret) {
    reset_control_put(reset);
    return dev_err_probe(dev, ret, "core_reset assert failed\n");
    }
//
// The hardware requirement for delay between assert/deassert
// is at least 3-4 sleep clock (32.7KHz) cycles, which comes to
// ~125us (4/32768). To be on the safe side add 200us delay.
//
    usleep_range(200, 210);
    ret = reset_control_deassert(reset);
    if (ret) {
    reset_control_put(reset);
    return dev_err_probe(dev, ret, "core_reset deassert failed\n");
    }
    usleep_range(200, 210);
    reset_control_put(reset);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_probe(pdev: *mut platform_device) -> c_int {
    static int sdhci_msm_probe(struct platform_device *pdev)
    {
    struct sdhci_host *host;
    struct sdhci_pltfm_host *pltfm_host;
    struct sdhci_msm_host *msm_host;
    struct clk *clk;
    int ret;
    u16 host_version, core_minor;
    u32 core_version, config;
    u8 core_major;
    const struct sdhci_msm_offset *msm_offset;
    const struct sdhci_msm_variant_info *var_info;
    struct device_node *node = pdev.dev.of_node;
    host = sdhci_pltfm_init(pdev, &sdhci_msm_pdata, sizeof(*msm_host));
    if (IS_ERR(host))
    return PTR_ERR(host);
    host.sdma_boundary = 0;
    pltfm_host = sdhci_priv(host);
    msm_host = sdhci_pltfm_priv(pltfm_host);
    msm_host.mmc = host.mmc;
    msm_host.pdev = pdev;
    ret = mmc_of_parse(host.mmc);
    if (ret)
    return ret;
//
// Based on the compatible string, load the required msm host info from
// the data associated with the version info.
//
    var_info = of_device_get_match_data(&pdev.dev);
    msm_host.mci_removed = var_info.mci_removed;
    msm_host.restore_dll_config = var_info.restore_dll_config;
    msm_host.var_ops = var_info.var_ops;
    msm_host.offset = var_info.offset;
    msm_offset = msm_host.offset;
    sdhci_get_of_property(pdev);
    sdhci_msm_get_of_property(pdev, host);
    msm_host.saved_tuning_phase = INVALID_TUNING_PHASE;
    ret = sdhci_msm_gcc_reset(&pdev.dev, host);
    if (ret)
    return ret;
// Setup SDCC bus voter clock.
    msm_host.bus_clk = devm_clk_get(&pdev.dev, "bus");
    if (!IS_ERR(msm_host.bus_clk)) {
// Vote for max. clk rate for max. performance
    ret = clk_set_rate(msm_host.bus_clk, INT_MAX);
    if (ret)
    return ret;
    ret = clk_prepare_enable(msm_host.bus_clk);
    if (ret)
    return ret;
    }
// Setup main peripheral bus clock
    clk = devm_clk_get(&pdev.dev, "iface");
    if (IS_ERR(clk)) {
    ret = PTR_ERR(clk);
    dev_err(&pdev.dev, "Peripheral clk setup failed (%d)\n", ret);
    goto bus_clk_disable;
    }
    msm_host.bulk_clks[1].clk = clk;
// Setup SDC MMC clock
    clk = devm_clk_get(&pdev.dev, "core");
    if (IS_ERR(clk)) {
    ret = PTR_ERR(clk);
    dev_err(&pdev.dev, "SDC MMC clk setup failed (%d)\n", ret);
    goto bus_clk_disable;
    }
    msm_host.bulk_clks[0].clk = clk;
// Check for optional interconnect paths
    ret = dev_pm_opp_of_find_icc_paths(&pdev.dev, core::ptr::null_mut());
    if (ret)
    goto bus_clk_disable;
    ret = devm_pm_opp_set_clkname(&pdev.dev, "core");
    if (ret)
    goto bus_clk_disable;
// OPP table is optional
    ret = devm_pm_opp_of_add_table(&pdev.dev);
    if (ret && ret != -ENODEV) {
    dev_err(&pdev.dev, "Invalid OPP table in Device tree\n");
    goto bus_clk_disable;
    }
// Vote for maximum clock rate for maximum performance
    ret = dev_pm_opp_set_rate(&pdev.dev, INT_MAX);
    if (ret)
    dev_warn(&pdev.dev, "core clock boost failed\n");
    clk = devm_clk_get(&pdev.dev, "cal");
    if (IS_ERR(clk))
    clk = core::ptr::null_mut();
    msm_host.bulk_clks[2].clk = clk;
    clk = devm_clk_get(&pdev.dev, "sleep");
    if (IS_ERR(clk))
    clk = core::ptr::null_mut();
    msm_host.bulk_clks[3].clk = clk;
    ret = clk_bulk_prepare_enable(ARRAY_SIZE(msm_host.bulk_clks),
    msm_host.bulk_clks);
    if (ret)
    goto bus_clk_disable;
//
// xo clock is needed for FLL feature of cm_dll.
// In case if xo clock is not mentioned in DT, warn and proceed.
//
    msm_host.xo_clk = devm_clk_get(&pdev.dev, "xo");
    if (IS_ERR(msm_host.xo_clk)) {
    ret = PTR_ERR(msm_host.xo_clk);
    dev_warn(&pdev.dev, "TCXO clk not present (%d)\n", ret);
    }
    if (!msm_host.mci_removed) {
    msm_host.core_mem = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(msm_host.core_mem)) {
    ret = PTR_ERR(msm_host.core_mem);
    goto clk_disable;
    }
    }
// Reset the vendor spec register to power on reset state
    writel_relaxed(CORE_VENDOR_SPEC_POR_VAL,
    host.ioaddr + msm_offset.core_vendor_spec);
    if (!msm_host.mci_removed) {
// Set HC_MODE_EN bit in HC_MODE register
    msm_host_writel(msm_host, HC_MODE_EN, host,
    msm_offset.core_hc_mode);
    config = msm_host_readl(msm_host, host,
    msm_offset.core_hc_mode);
    config |= FF_CLK_SW_RST_DIS;
    msm_host_writel(msm_host, config, host,
    msm_offset.core_hc_mode);
    }
    host_version = readw_relaxed((host.ioaddr + SDHCI_HOST_VERSION));
    dev_dbg(&pdev.dev, "Host Version: 0x%x Vendor Version 0x%x\n",
    host_version, ((host_version & SDHCI_VENDOR_VER_MASK) >>
    SDHCI_VENDOR_VER_SHIFT));
    core_version = msm_host_readl(msm_host, host,
    msm_offset.core_mci_version);
    core_major = (core_version & CORE_VERSION_MAJOR_MASK) >>
    CORE_VERSION_MAJOR_SHIFT;
    core_minor = core_version & CORE_VERSION_MINOR_MASK;
    dev_dbg(&pdev.dev, "MCI Version: 0x%08x, major: 0x%04x, minor: 0x%02x\n",
    core_version, core_major, core_minor);
    if (core_major == 1 && core_minor >= 0x42)
    msm_host.use_14lpp_dll_reset = true;
//
// SDCC 5 controller with major version 1, minor version 0x34 and later
// with HS 400 mode support will use CM DLL instead of CDC LP 533 DLL.
//
    if (core_major == 1 && core_minor < 0x34)
    msm_host.use_cdclp533 = true;
//
// Support for some capabilities is not advertised by newer
// controller versions and must be explicitly enabled.
//
    if (core_major >= 1 && core_minor != 0x11 && core_minor != 0x12) {
    config = readl_relaxed(host.ioaddr + SDHCI_CAPABILITIES);
    config |= SDHCI_CAN_VDD_300 | SDHCI_CAN_DO_8BIT;
    writel_relaxed(config, host.ioaddr +
    msm_offset.core_vendor_spec_capabilities0);
    }
    if (core_major == 1 && core_minor >= 0x49)
    msm_host.updated_ddr_cfg = true;
    if (core_major == 1 && core_minor >= 0x71)
    msm_host.uses_tassadar_dll = true;
    ret = sdhci_msm_register_vreg(msm_host);
    if (ret)
    goto clk_disable;
//
// Power on reset state may trigger power irq if previous status of
// PWRCTL was either BUS_ON or IO_HIGH_V. So before enabling pwr irq
// interrupt in GIC, any pending power irq interrupt should be
// acknowledged. Otherwise power irq interrupt handler would be
// fired prematurely.
//
    sdhci_msm_handle_pwr_irq(host, 0);
//
// Ensure that above writes are propagated before interrupt enablement
// in GIC.
//
    mb();
// Setup IRQ for handling power/voltage tasks with PMIC
    msm_host.pwr_irq = platform_get_irq_byname(pdev, "pwr_irq");
    if (msm_host.pwr_irq < 0) {
    ret = msm_host.pwr_irq;
    goto clk_disable;
    }
    sdhci_msm_init_pwr_irq_wait(msm_host);
// Enable pwr irq interrupts
    msm_host_writel(msm_host, INT_MASK, host,
    msm_offset.core_pwrctl_mask);
    ret = devm_request_threaded_irq(&pdev.dev, msm_host.pwr_irq, core::ptr::null_mut(),
    sdhci_msm_pwr_irq, IRQF_ONESHOT,
    dev_name(&pdev.dev), host);
    if (ret)
    goto clk_disable;
    msm_host.mmc.caps |= MMC_CAP_WAIT_WHILE_BUSY | MMC_CAP_NEED_RSP_BUSY;

    host.mmc_host_ops.request = sdhci_msm_request;

// Set the timeout value to max possible
    host.max_timeout_count = 0xF;
    pm_runtime_get_noresume(&pdev.dev);
    pm_runtime_set_active(&pdev.dev);
    pm_runtime_enable(&pdev.dev);
    pm_runtime_set_autosuspend_delay(&pdev.dev,
    MSM_MMC_AUTOSUSPEND_DELAY_MS);
    pm_runtime_use_autosuspend(&pdev.dev);
    host.mmc_host_ops.start_signal_voltage_switch =
    sdhci_msm_start_signal_voltage_switch;
    host.mmc_host_ops.execute_tuning = sdhci_msm_execute_tuning;
    if (of_property_read_bool(node, "supports-cqe"))
    ret = sdhci_msm_cqe_add_host(host, pdev);
    else
    ret = sdhci_add_host(host);
    if (ret)
    goto pm_runtime_disable;
    pm_runtime_put_autosuspend(&pdev.dev);
    return 0;
    pm_runtime_disable:
    pm_runtime_disable(&pdev.dev);
    pm_runtime_set_suspended(&pdev.dev);
    pm_runtime_put_noidle(&pdev.dev);
    clk_disable:
    clk_bulk_disable_unprepare(ARRAY_SIZE(msm_host.bulk_clks),
    msm_host.bulk_clks);
    bus_clk_disable:
    if (!IS_ERR(msm_host.bus_clk))
    clk_disable_unprepare(msm_host.bus_clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_remove(pdev: *mut platform_device) {
    static void sdhci_msm_remove(struct platform_device *pdev)
    {
    struct sdhci_host *host = platform_get_drvdata(pdev);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    int dead = (readl_relaxed(host.ioaddr + SDHCI_INT_STATUS) ==
    0xffffffff);
    sdhci_remove_host(host, dead);
    pm_runtime_get_sync(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    pm_runtime_put_noidle(&pdev.dev);
    clk_bulk_disable_unprepare(ARRAY_SIZE(msm_host.bulk_clks),
    msm_host.bulk_clks);
    if (!IS_ERR(msm_host.bus_clk))
    clk_disable_unprepare(msm_host.bus_clk);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_runtime_suspend(dev: *mut device) -> c_int {
    static int sdhci_msm_runtime_suspend(struct device *dev)
    {
    struct sdhci_host *host = dev_get_drvdata(dev);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    unsigned long flags;
    spin_lock_irqsave(&host.lock, flags);
    host.runtime_suspended = true;
    spin_unlock_irqrestore(&host.lock, flags);
// Drop the performance vote
    dev_pm_opp_set_rate(dev, 0);
    clk_bulk_disable_unprepare(ARRAY_SIZE(msm_host.bulk_clks),
    msm_host.bulk_clks);
    return sdhci_msm_ice_suspend(msm_host);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_msm_runtime_resume(dev: *mut device) -> c_int {
    static int sdhci_msm_runtime_resume(struct device *dev)
    {
    struct sdhci_host *host = dev_get_drvdata(dev);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_msm_host *msm_host = sdhci_pltfm_priv(pltfm_host);
    unsigned long flags;
    int ret;
    ret = clk_bulk_prepare_enable(ARRAY_SIZE(msm_host.bulk_clks),
    msm_host.bulk_clks);
    if (ret)
    return ret;
//
// Whenever core-clock is gated dynamically, it's needed to
// restore the SDR DLL settings when the clock is ungated.
//
    if (msm_host.restore_dll_config && msm_host.clk_rate) {
    ret = sdhci_msm_restore_sdr_dll_config(host);
    if (ret)
    return ret;
    }
    dev_pm_opp_set_rate(dev, msm_host.clk_rate);
    ret = sdhci_msm_ice_resume(msm_host);
    if (ret)
    return ret;
    spin_lock_irqsave(&host.lock, flags);
    host.runtime_suspended = false;
    spin_unlock_irqrestore(&host.lock, flags);
    return ret;
    }
    static const struct dev_pm_ops sdhci_msm_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
    RUNTIME_PM_OPS(sdhci_msm_runtime_suspend, sdhci_msm_runtime_resume, core::ptr::null_mut())
    };
    static struct platform_driver sdhci_msm_driver = {
    .probe = sdhci_msm_probe,
    .remove = sdhci_msm_remove,
    .driver = {
    .name = "sdhci_msm",
    .of_match_table = sdhci_msm_dt_match,
    .pm = pm_ptr(&sdhci_msm_pm_ops),
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
    module_platform_driver(sdhci_msm_driver);
    MODULE_DESCRIPTION("Qualcomm Secure Digital Host Controller Interface driver");
    MODULE_LICENSE("GPL v2");
