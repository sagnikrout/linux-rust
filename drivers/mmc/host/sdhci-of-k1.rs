//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/sdhci-of-k1.c
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
// Copyright (C) 2023-2025 SpacemiT (Hangzhou) Technology Co. Ltd
// Copyright (c) 2025 Yixun Lan <dlan@gentoo.org>
//

pub const SPACEMIT_SDHC_OP_EXT_REG: c_uint = 0x108;

pub const SPACEMIT_SDHC_LEGACY_CTRL_REG: c_uint = 0x10C;

pub const SPACEMIT_SDHC_MMC_CTRL_REG: c_uint = 0x114;

pub const SPACEMIT_SDHC_TX_CFG_REG: c_uint = 0x11C;

pub const SPACEMIT_SDHC_PHY_CTRL_REG: c_uint = 0x160;

pub const SPACEMIT_SDHC_PHY_FUNC_REG: c_uint = 0x164;

pub const SPACEMIT_SDHC_PHY_DLLCFG: c_uint = 0x168;

pub const SPACEMIT_SDHC_PHY_DLLCFG1: c_uint = 0x16C;

pub const SPACEMIT_SDHC_PHY_DLLSTS: c_uint = 0x170;

pub const SPACEMIT_SDHC_PHY_PADCFG_REG: c_uint = 0x178;

pub const SPACEMIT_SDHC_RX_CFG_REG: c_uint = 0x118;

pub const SPACEMIT_SDHC_DLINE_CTRL_REG: c_uint = 0x130;

pub const SPACEMIT_SDHC_DLINE_CFG_REG: c_uint = 0x134;

pub const SPACEMIT_RX_DLINE_REG: c_int = 9;
pub const SPACEMIT_RX_TUNE_DELAY_MIN: c_uint = 0x0;
pub const SPACEMIT_RX_TUNE_DELAY_MAX: c_uint = 0xFF;
pub const SPACEMIT_TX_TUNING_DLINE_REG: c_uint = 0x00;
pub const SPACEMIT_TX_TUNING_DELAYCODE: c_int = 127;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spacemit_sdhci_host {
    pub clk_core: *mut clk,
    pub clk_io: *mut clk,
    pub pinctrl: *mut pinctrl,
    pub pinctrl_default: *mut pinctrl_state,
    pub pinctrl_uhs: *mut pinctrl_state,
}

// All helper functions will update clr/set while preserve rest bits
#[no_mangle]
pub unsafe extern "C" fn spacemit_sdhci_setbits(host: *mut sdhci_host, val: u32, reg: c_int) {
    static inline void spacemit_sdhci_setbits(struct sdhci_host *host, u32 val, int reg)
    {
    sdhci_writel(host, sdhci_readl(host, reg) | val, reg);
    }
#[no_mangle]
pub unsafe extern "C" fn spacemit_sdhci_clrbits(host: *mut sdhci_host, val: u32, reg: c_int) {
    static inline void spacemit_sdhci_clrbits(struct sdhci_host *host, u32 val, int reg)
    {
    sdhci_writel(host, sdhci_readl(host, reg) & ~val, reg);
    }
#[no_mangle]
pub unsafe extern "C" fn spacemit_sdhci_clrsetbits(host: *mut sdhci_host, clr: u32, set: u32, reg: c_int) {
    static inline void spacemit_sdhci_clrsetbits(struct sdhci_host *host, u32 clr, u32 set, int reg)
    {
    let mut val: u32 = sdhci_readl(host, reg);
    val = (val & ~clr) | set;
    sdhci_writel(host, val, reg);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_sdhci_set_rx_delay(host: *mut sdhci_host, delay: u8) {
    static void spacemit_sdhci_set_rx_delay(struct sdhci_host *host, u8 delay)
    {
    spacemit_sdhci_clrsetbits(host, SDHC_RX_DLINE_CODE_MASK,
    FIELD_PREP(SDHC_RX_DLINE_CODE_MASK, delay),
    SPACEMIT_SDHC_DLINE_CTRL_REG);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_sdhci_set_tx_delay(host: *mut sdhci_host, delay: u8) {
    static void spacemit_sdhci_set_tx_delay(struct sdhci_host *host, u8 delay)
    {
    spacemit_sdhci_clrsetbits(host, SDHC_TX_DLINE_CODE_MASK,
    FIELD_PREP(SDHC_TX_DLINE_CODE_MASK, delay),
    SPACEMIT_SDHC_DLINE_CTRL_REG);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_sdhci_set_tx_dline_reg(host: *mut sdhci_host, dline_reg: u8) {
    static void spacemit_sdhci_set_tx_dline_reg(struct sdhci_host *host, u8 dline_reg)
    {
    spacemit_sdhci_clrsetbits(host, SDHC_TX_DLINE_REG_MASK,
    FIELD_PREP(SDHC_TX_DLINE_REG_MASK, dline_reg),
    SPACEMIT_SDHC_DLINE_CFG_REG);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_sdhci_tx_tuning_prepare(host: *mut sdhci_host) {
    static void spacemit_sdhci_tx_tuning_prepare(struct sdhci_host *host)
    {
    spacemit_sdhci_setbits(host, SDHC_TX_MUX_SEL, SPACEMIT_SDHC_TX_CFG_REG);
    spacemit_sdhci_setbits(host, SDHC_DLINE_PU, SPACEMIT_SDHC_DLINE_CTRL_REG);
    udelay(5);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_sdhci_prepare_tuning(host: *mut sdhci_host) {
    static void spacemit_sdhci_prepare_tuning(struct sdhci_host *host)
    {
    spacemit_sdhci_clrsetbits(host, SDHC_RX_DLINE_REG_MASK,
    FIELD_PREP(SDHC_RX_DLINE_REG_MASK, SPACEMIT_RX_DLINE_REG),
    SPACEMIT_SDHC_DLINE_CFG_REG);
    spacemit_sdhci_setbits(host, SDHC_DLINE_PU, SPACEMIT_SDHC_DLINE_CTRL_REG);
    udelay(5);
    spacemit_sdhci_clrsetbits(host, SDHC_RX_SDCLK_SEL1_MASK, SDHC_RX_SDCLK_SEL1,
    SPACEMIT_SDHC_RX_CFG_REG);
    if (host.mmc.ios.timing == MMC_TIMING_MMC_HS200)
    spacemit_sdhci_setbits(host, SDHC_HS200_USE_RFIFO, SPACEMIT_SDHC_PHY_FUNC_REG);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_sdhci_reset(host: *mut sdhci_host, mask: u8) {
    static void spacemit_sdhci_reset(struct sdhci_host *host, u8 mask)
    {
    sdhci_reset(host, mask);
    if (mask != SDHCI_RESET_ALL)
    return;
    spacemit_sdhci_setbits(host, SDHC_PHY_FUNC_EN | SDHC_PHY_PLL_LOCK,
    SPACEMIT_SDHC_PHY_CTRL_REG);
    spacemit_sdhci_clrsetbits(host, SDHC_PHY_DRIVE_SEL,
    SDHC_RX_BIAS_CTRL | FIELD_PREP(SDHC_PHY_DRIVE_SEL, 4),
    SPACEMIT_SDHC_PHY_PADCFG_REG);
    if (!(host.mmc.caps2 & MMC_CAP2_NO_MMC))
    spacemit_sdhci_setbits(host, SDHC_MMC_CARD_MODE, SPACEMIT_SDHC_MMC_CTRL_REG);
    spacemit_sdhci_setbits(host, SDHC_GEN_PAD_CLK_ON, SPACEMIT_SDHC_LEGACY_CTRL_REG);
    if (host.mmc.caps2 & MMC_CAP2_NO_MMC)
    spacemit_sdhci_setbits(host, SDHC_OVRRD_CLK_OEN | SDHC_FORCE_CLK_ON,
    SPACEMIT_SDHC_OP_EXT_REG);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_sdhci_set_uhs_signaling(host: *mut sdhci_host, timing: c_uint) {
    static void spacemit_sdhci_set_uhs_signaling(struct sdhci_host *host, unsigned int timing)
    {
    if (timing == MMC_TIMING_MMC_HS200)
    spacemit_sdhci_setbits(host, SDHC_MMC_HS200, SPACEMIT_SDHC_MMC_CTRL_REG);
    if (timing == MMC_TIMING_MMC_HS400)
    spacemit_sdhci_setbits(host, SDHC_MMC_HS400, SPACEMIT_SDHC_MMC_CTRL_REG);
    sdhci_set_uhs_signaling(host, timing);
    if (!(host.mmc.caps2 & MMC_CAP2_NO_SDIO))
    spacemit_sdhci_setbits(host, SDHCI_CTRL_VDD_180, SDHCI_HOST_CONTROL2);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_sdhci_set_clock(host: *mut sdhci_host, clock: c_uint) {
    static void spacemit_sdhci_set_clock(struct sdhci_host *host, unsigned int clock)
    {
    struct mmc_host *mmc = host.mmc;
    if (mmc.ios.timing <= MMC_TIMING_UHS_SDR50)
    spacemit_sdhci_setbits(host, SDHC_TX_INT_CLK_SEL, SPACEMIT_SDHC_TX_CFG_REG);
    else
    spacemit_sdhci_clrbits(host, SDHC_TX_INT_CLK_SEL, SPACEMIT_SDHC_TX_CFG_REG);
    sdhci_set_clock(host, clock);
    };
#[no_mangle]
unsafe extern "C" fn spacemit_sdhci_phy_dll_init(host: *mut sdhci_host) {
    static void spacemit_sdhci_phy_dll_init(struct sdhci_host *host)
    {
    u32 state;
    int ret;
    spacemit_sdhci_clrsetbits(host, SDHC_DLL_PREDLY_NUM |
    SDHC_DLL_FULLDLY_RANGE |
    SDHC_DLL_VREG_CTRL,
    FIELD_PREP(SDHC_DLL_PREDLY_NUM, 1) |
    FIELD_PREP(SDHC_DLL_FULLDLY_RANGE, 1) |
    FIELD_PREP(SDHC_DLL_VREG_CTRL, 1),
    SPACEMIT_SDHC_PHY_DLLCFG);
    spacemit_sdhci_clrsetbits(host, SDHC_DLL_REG1_CTRL,
    FIELD_PREP(SDHC_DLL_REG1_CTRL, 0x92),
    SPACEMIT_SDHC_PHY_DLLCFG1);
    spacemit_sdhci_setbits(host, SDHC_DLL_ENABLE, SPACEMIT_SDHC_PHY_DLLCFG);
    ret = readl_poll_timeout(host.ioaddr + SPACEMIT_SDHC_PHY_DLLSTS, state,
    state & SDHC_DLL_LOCK_STATE, 2, 100);
    if (ret == -ETIMEDOUT)
    dev_warn(mmc_dev(host.mmc), "fail to lock phy dll in 100us!\n");
    }
#[no_mangle]
unsafe extern "C" fn spacemit_sdhci_hs400_enhanced_strobe(mmc: *mut mmc_host, ios: *mut mmc_ios) {
    static void spacemit_sdhci_hs400_enhanced_strobe(struct mmc_host *mmc, struct mmc_ios *ios)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    if (!ios.enhanced_strobe) {
    spacemit_sdhci_clrbits(host, SDHC_ENHANCE_STROBE_EN, SPACEMIT_SDHC_MMC_CTRL_REG);
    return;
    }
    spacemit_sdhci_setbits(host, SDHC_ENHANCE_STROBE_EN, SPACEMIT_SDHC_MMC_CTRL_REG);
    spacemit_sdhci_phy_dll_init(host);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_sdhci_clk_get_max_clock(host: *mut sdhci_host) -> c_uint {
    static unsigned int spacemit_sdhci_clk_get_max_clock(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    return clk_get_rate(pltfm_host.clk);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_sdhci_execute_tuning(host: *mut sdhci_host, opcode: u32) -> c_int {
    static int spacemit_sdhci_execute_tuning(struct sdhci_host *host, u32 opcode)
    {
    let mut current_len: c_int = 0, current_start = 0;
    let mut max_pass_len: c_int = 0, max_pass_start = 0;
    struct mmc_host *mmc = host.mmc;
    let mut ios: mmc_ios = mmc.ios;
    u8 final_delay;
    let mut ret: c_int = 0;
    int i;
//
// Tuning is required for SDR50/SDR104, HS200/HS400 cards and
// if clock frequency is greater than 100MHz in these modes.
//
    if (host.clock < 100 * 1000 * 1000 ||
    !(ios.timing == MMC_TIMING_MMC_HS200 ||
    ios.timing == MMC_TIMING_UHS_SDR50 ||
    ios.timing == MMC_TIMING_UHS_SDR104))
    return 0;
    if (mmc.caps2 & MMC_CAP2_NO_MMC) {
    spacemit_sdhci_set_tx_dline_reg(host, SPACEMIT_TX_TUNING_DLINE_REG);
    spacemit_sdhci_set_tx_delay(host, SPACEMIT_TX_TUNING_DELAYCODE);
    spacemit_sdhci_tx_tuning_prepare(host);
    dev_dbg(mmc_dev(host.mmc), "TX tuning: dline_reg=%d, delaycode=%d\n",
    SPACEMIT_TX_TUNING_DLINE_REG, SPACEMIT_TX_TUNING_DELAYCODE);
    }
    spacemit_sdhci_prepare_tuning(host);
    for (i = SPACEMIT_RX_TUNE_DELAY_MIN; i <= SPACEMIT_RX_TUNE_DELAY_MAX; i++) {
    spacemit_sdhci_set_rx_delay(host, i);
    ret = mmc_send_tuning(host.mmc, opcode, core::ptr::null_mut());
    dev_dbg(mmc_dev(host.mmc), "RX delay %d: %s\n",
    i, ret == 0 ? "pass" : "fail");
    if (ret == 0) {
// Test passed - extend current window
    if (current_len == 0)
    current_start = i;
    current_len++;
    } else {
// Test failed - check if current window is best so far
    if (current_len > max_pass_len) {
    max_pass_len = current_len;
    max_pass_start = current_start;
    }
    current_len = 0;
    }
    }
    if (current_len > max_pass_len) {
    max_pass_len = current_len;
    max_pass_start = current_start;
    }
    if (max_pass_len < 3) {
    dev_err(mmc_dev(host.mmc), "Tuning failed: no stable window found\n");
    return -EIO;
    }
    final_delay = max_pass_start + max_pass_len / 2;
    spacemit_sdhci_set_rx_delay(host, final_delay);
    ret = mmc_send_tuning(host.mmc, opcode, core::ptr::null_mut());
    if (ret) {
    u8 retry_delays[] = {
    max_pass_start + max_pass_len / 4,
    max_pass_start + (3 * max_pass_len) / 4,
    max_pass_start,
    max_pass_start + max_pass_len - 1
    };
    let mut retry_count: c_int = ARRAY_SIZE(retry_delays);
    dev_warn(mmc_dev(mmc), "Primary delay %d failed, trying alternatives\n",
    final_delay);
    for (i = 0; i < retry_count; i++) {
    if (retry_delays[i] >= SPACEMIT_RX_TUNE_DELAY_MIN &&
    retry_delays[i] <= SPACEMIT_RX_TUNE_DELAY_MAX) {
    spacemit_sdhci_set_rx_delay(host, retry_delays[i]);
    ret = mmc_send_tuning(host.mmc, opcode, core::ptr::null_mut());
    if (!ret) {
    final_delay = retry_delays[i];
    dev_info(mmc_dev(mmc), "Retry successful with delay %d\n",
    final_delay);
    break;
    }
    }
    }
    if (ret) {
    dev_err(mmc_dev(mmc), "All retry attempts failed\n");
    return -EIO;
    }
    }
    dev_dbg(mmc_dev(host.mmc),
    "Tuning successful: window %d-%d, using delay %d\n",
    max_pass_start, max_pass_start + max_pass_len - 1, final_delay);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spacemit_sdhci_pre_select_hs400(mmc: *mut mmc_host) -> c_int {
    static int spacemit_sdhci_pre_select_hs400(struct mmc_host *mmc)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    spacemit_sdhci_setbits(host, SDHC_MMC_HS400, SPACEMIT_SDHC_MMC_CTRL_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spacemit_sdhci_post_select_hs400(mmc: *mut mmc_host) {
    static void spacemit_sdhci_post_select_hs400(struct mmc_host *mmc)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    spacemit_sdhci_phy_dll_init(host);
    }
#[no_mangle]
unsafe extern "C" fn spacemit_sdhci_pre_hs400_to_hs200(mmc: *mut mmc_host) {
    static void spacemit_sdhci_pre_hs400_to_hs200(struct mmc_host *mmc)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    spacemit_sdhci_clrbits(host, SDHC_PHY_FUNC_EN | SDHC_PHY_PLL_LOCK,
    SPACEMIT_SDHC_PHY_CTRL_REG);
    spacemit_sdhci_clrbits(host, SDHC_MMC_HS400 | SDHC_MMC_HS200 | SDHC_ENHANCE_STROBE_EN,
    SPACEMIT_SDHC_MMC_CTRL_REG);
    spacemit_sdhci_clrbits(host, SDHC_HS200_USE_RFIFO, SPACEMIT_SDHC_PHY_FUNC_REG);
    udelay(5);
    spacemit_sdhci_setbits(host, SDHC_PHY_FUNC_EN | SDHC_PHY_PLL_LOCK,
    SPACEMIT_SDHC_PHY_CTRL_REG);
    }
    static int spacemit_sdhci_start_signal_voltage_switch(struct mmc_host *mmc,
    struct mmc_ios *ios)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct spacemit_sdhci_host *sdhst = sdhci_pltfm_priv(pltfm_host);
    struct pinctrl_state *state;
    int ret;
    ret = sdhci_start_signal_voltage_switch(mmc, ios);
    if (ret)
    return ret;
    if (!sdhst.pinctrl)
    return 0;
// Select appropriate pinctrl state based on signal voltage
    switch (ios.signal_voltage) {
    case MMC_SIGNAL_VOLTAGE_330:
    state = sdhst.pinctrl_default;
    break;
    case MMC_SIGNAL_VOLTAGE_180:
    state = sdhst.pinctrl_uhs;
    break;
    default:
    dev_warn(mmc_dev(mmc), "unsupported voltage %d\n", ios.signal_voltage);
    return 0;
    }
    ret = pinctrl_select_state(sdhst.pinctrl, state);
    if (ret) {
    dev_warn(mmc_dev(mmc), "failed to select pinctrl state: %d\n", ret);
    return 0;
    }
    dev_dbg(mmc_dev(mmc), "switched to %s pinctrl state\n",
    ios.signal_voltage == MMC_SIGNAL_VOLTAGE_180 ? "UHS" : "default");
    return 0;
    }
    static inline int spacemit_sdhci_get_clocks(struct device *dev,
    struct sdhci_pltfm_host *pltfm_host)
    {
    struct spacemit_sdhci_host *sdhst = sdhci_pltfm_priv(pltfm_host);
    sdhst.clk_core = devm_clk_get_enabled(dev, "core");
    if (IS_ERR(sdhst.clk_core))
    return -EINVAL;
    sdhst.clk_io = devm_clk_get_enabled(dev, "io");
    if (IS_ERR(sdhst.clk_io))
    return -EINVAL;
    pltfm_host.clk = sdhst.clk_io;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn spacemit_sdhci_get_resets(dev: *mut device) -> c_int {
    static inline int spacemit_sdhci_get_resets(struct device *dev)
    {
    struct reset_control *rst;
    rst = devm_reset_control_get_optional_shared_deasserted(dev, "axi");
    if (IS_ERR(rst))
    return PTR_ERR(rst);
    rst = devm_reset_control_get_optional_exclusive_deasserted(dev, "sdh");
    if (IS_ERR(rst))
    return PTR_ERR(rst);
    return 0;
    }
    static inline void spacemit_sdhci_get_pins(struct device *dev,
    struct sdhci_pltfm_host *pltfm_host)
    {
    struct spacemit_sdhci_host *sdhst = sdhci_pltfm_priv(pltfm_host);
    sdhst.pinctrl = devm_pinctrl_get(dev);
    if (IS_ERR(sdhst.pinctrl)) {
    sdhst.pinctrl = core::ptr::null_mut();
    dev_dbg(dev, "pinctrl not available, voltage switching will work without it\n");
    return;
    }
    sdhst.pinctrl_default = pinctrl_lookup_state(sdhst.pinctrl, "default");
    if (IS_ERR(sdhst.pinctrl_default))
    sdhst.pinctrl_default = core::ptr::null_mut();
    sdhst.pinctrl_uhs = pinctrl_lookup_state(sdhst.pinctrl, "uhs");
    if (IS_ERR(sdhst.pinctrl_uhs))
    sdhst.pinctrl_uhs = core::ptr::null_mut();
    dev_dbg(dev, "pinctrl setup: default=%p, uhs=%p\n",
    sdhst.pinctrl_default, sdhst.pinctrl_uhs);
    }
    static const struct sdhci_ops spacemit_sdhci_ops = {
    .get_max_clock		= spacemit_sdhci_clk_get_max_clock,
    .reset			= spacemit_sdhci_reset,
    .set_bus_width		= sdhci_set_bus_width,
    .set_clock		= spacemit_sdhci_set_clock,
    .set_uhs_signaling	= spacemit_sdhci_set_uhs_signaling,
    .platform_execute_tuning = spacemit_sdhci_execute_tuning,
    };
    static const struct sdhci_pltfm_data spacemit_sdhci_k1_pdata = {
    .ops = &spacemit_sdhci_ops,
    .quirks = SDHCI_QUIRK_DATA_TIMEOUT_USES_SDCLK |
    SDHCI_QUIRK_NO_ENDATTR_IN_NOPDESC |
    SDHCI_QUIRK_32BIT_ADMA_SIZE |
    SDHCI_QUIRK_CAP_CLOCK_BASE_BROKEN |
    SDHCI_QUIRK_BROKEN_CARD_DETECTION |
    SDHCI_QUIRK_BROKEN_TIMEOUT_VAL,
    .quirks2 = SDHCI_QUIRK2_BROKEN_64_BIT_DMA |
    SDHCI_QUIRK2_PRESET_VALUE_BROKEN,
    };
    static const struct sdhci_pltfm_data spacemit_sdhci_k3_pdata = {
    .ops = &spacemit_sdhci_ops,
    .quirks = SDHCI_QUIRK_DATA_TIMEOUT_USES_SDCLK |
    SDHCI_QUIRK_NO_ENDATTR_IN_NOPDESC |
    SDHCI_QUIRK_32BIT_ADMA_SIZE |
    SDHCI_QUIRK_CAP_CLOCK_BASE_BROKEN |
    SDHCI_QUIRK_BROKEN_CARD_DETECTION |
    SDHCI_QUIRK_BROKEN_TIMEOUT_VAL,
    .quirks2 = SDHCI_QUIRK2_PRESET_VALUE_BROKEN,
    };
    static const struct of_device_id spacemit_sdhci_of_match[] = {
    { .compatible = "spacemit,k1-sdhci", .data = &spacemit_sdhci_k1_pdata },
    { .compatible = "spacemit,k3-sdhci", .data = &spacemit_sdhci_k3_pdata },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, spacemit_sdhci_of_match);
#[no_mangle]
unsafe extern "C" fn spacemit_sdhci_probe(pdev: *mut platform_device) -> c_int {
    static int spacemit_sdhci_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct spacemit_sdhci_host *sdhst;
    struct sdhci_pltfm_host *pltfm_host;
    struct sdhci_host *host;
    const struct sdhci_pltfm_data *data;
    struct mmc_host_ops *mops;
    int ret;
    data = of_device_get_match_data(&pdev.dev);
    host = sdhci_pltfm_init(pdev, data, sizeof(*sdhst));
    if (IS_ERR(host))
    return PTR_ERR(host);
    pltfm_host = sdhci_priv(host);
    ret = mmc_of_parse(host.mmc);
    if (ret)
    goto err_pltfm;
    sdhci_get_of_property(pdev);
    if (!(host.mmc.caps2 & MMC_CAP2_NO_MMC)) {
    mops = &host.mmc_host_ops;
    mops.hs400_prepare_ddr	= spacemit_sdhci_pre_select_hs400;
    mops.hs400_complete	= spacemit_sdhci_post_select_hs400;
    mops.hs400_downgrade	= spacemit_sdhci_pre_hs400_to_hs200;
    mops.hs400_enhanced_strobe = spacemit_sdhci_hs400_enhanced_strobe;
    }
    host.mmc.caps |= MMC_CAP_NEED_RSP_BUSY;
    spacemit_sdhci_get_pins(dev, pltfm_host);
    host.mmc_host_ops.start_signal_voltage_switch = spacemit_sdhci_start_signal_voltage_switch;
    ret = spacemit_sdhci_get_clocks(dev, pltfm_host);
    if (ret)
    goto err_pltfm;
    ret = spacemit_sdhci_get_resets(dev);
    if (ret)
    goto err_pltfm;
    ret = sdhci_add_host(host);
    if (ret)
    goto err_pltfm;
    return 0;
    err_pltfm:
    return ret;
    }
    static struct platform_driver spacemit_sdhci_driver = {
    .driver		= {
    .name	= "sdhci-spacemit",
    .of_match_table = spacemit_sdhci_of_match,
    },
    .probe		= spacemit_sdhci_probe,
    .remove		= sdhci_pltfm_remove,
    };
    module_platform_driver(spacemit_sdhci_driver);
    MODULE_DESCRIPTION("SpacemiT SDHCI platform driver");
    MODULE_LICENSE("GPL");
