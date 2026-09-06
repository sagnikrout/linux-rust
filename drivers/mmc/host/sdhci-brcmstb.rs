//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/sdhci-brcmstb.c
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
// sdhci-brcmstb.c Support for SDHCI on Broadcom BRCMSTB SoC's
//
// Copyright (C) 2015 Broadcom Corporation
//

pub const SDHCI_VENDOR: c_uint = 0x78;
pub const SDHCI_VENDOR_ENHANCED_STRB: c_uint = 0x1;
pub const SDHCI_VENDOR_GATE_SDCLK_EN: c_uint = 0x2;

pub const SDHCI_ARASAN_CQE_BASE_ADDR: c_uint = 0x200;
pub const SDIO_CFG_CTRL: c_uint = 0x0;

pub const SDIO_CFG_OP_DLY: c_uint = 0x34;
pub const SDIO_CFG_OP_DLY_DEFAULT: c_uint = 0x80000003;
pub const SDIO_CFG_CQ_CAPABILITY: c_uint = 0x4c;

pub const SDIO_CFG_SD_PIN_SEL: c_uint = 0x44;
pub const SDIO_CFG_V1_SD_PIN_SEL: c_uint = 0x54;
pub const SDIO_CFG_PHY_SW_MODE_0_RX_CTRL: c_uint = 0x7C;
pub const SDIO_CFG_MAX_50MHZ_MODE: c_uint = 0x1ac;

pub const SDIO_BOOT_MAIN_CTL: c_uint = 0x0;

// Select all SD UHS type I SDR speed above 50MB/s

    enum cfg_core_ver {
    SDIO_CFG_CORE_V1 = 1,
    SDIO_CFG_CORE_V2,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdhci_brcmstb_saved_regs {
    pub sd_pin_sel: u32,
    pub phy_sw_mode0_rxctrl: u32,
    pub max_50mhz_mode: u32,
    pub boot_main_ctl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmstb_match_priv {
    pub host): *mut *mut void (cfginit)(struct sdhci_host,
    pub ios): *mut *mut *mut void (hs400es)(struct mmc_host mmc, struct mmc_ios,
    pub save): *mut *mut *mut void (save_restore_regs)(struct mmc_host mmc, int,
    pub ops: *mut sdhci_ops,
    pub flags: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdhci_brcmstb_priv {
    pub cfg_regs: *mut void __iomem,
    pub boot_regs: *mut void __iomem,
    pub saved_regs: sdhci_brcmstb_saved_regs,
    pub flags: c_uint,
    pub base_clk: *mut clk,
    pub base_freq_hz: u32,
    pub match_priv: *const brcmstb_match_priv,
}

#[no_mangle]
unsafe extern "C" fn sdhci_brcmstb_save_regs(mmc: *mut mmc_host, ver: enum cfg_core_ver) {
    static void sdhci_brcmstb_save_regs(struct mmc_host *mmc, enum cfg_core_ver ver)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_brcmstb_priv *priv = sdhci_pltfm_priv(pltfm_host);
    struct sdhci_brcmstb_saved_regs *sr = &priv.saved_regs;
    void __iomem *cr = priv.cfg_regs;
    let mut is_emmc: bool = mmc.caps & MMC_CAP_NONREMOVABLE;
    if (is_emmc && priv.boot_regs)
    sr.boot_main_ctl = readl(priv.boot_regs + SDIO_BOOT_MAIN_CTL);
    if (ver == SDIO_CFG_CORE_V1) {
    sr.sd_pin_sel = readl(cr + SDIO_CFG_V1_SD_PIN_SEL);
    return;
    }
    sr.sd_pin_sel = readl(cr + SDIO_CFG_SD_PIN_SEL);
    sr.phy_sw_mode0_rxctrl = readl(cr + SDIO_CFG_PHY_SW_MODE_0_RX_CTRL);
    sr.max_50mhz_mode = readl(cr + SDIO_CFG_MAX_50MHZ_MODE);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_brcmstb_restore_regs(mmc: *mut mmc_host, ver: enum cfg_core_ver) {
    static void sdhci_brcmstb_restore_regs(struct mmc_host *mmc, enum cfg_core_ver ver)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_brcmstb_priv *priv = sdhci_pltfm_priv(pltfm_host);
    struct sdhci_brcmstb_saved_regs *sr = &priv.saved_regs;
    void __iomem *cr = priv.cfg_regs;
    let mut is_emmc: bool = mmc.caps & MMC_CAP_NONREMOVABLE;
    if (is_emmc && priv.boot_regs)
    writel(sr.boot_main_ctl, priv.boot_regs + SDIO_BOOT_MAIN_CTL);
    if (ver == SDIO_CFG_CORE_V1) {
    writel(sr.sd_pin_sel, cr + SDIO_CFG_V1_SD_PIN_SEL);
    return;
    }
    writel(sr.sd_pin_sel, cr + SDIO_CFG_SD_PIN_SEL);
    writel(sr.phy_sw_mode0_rxctrl, cr + SDIO_CFG_PHY_SW_MODE_0_RX_CTRL);
    writel(sr.max_50mhz_mode, cr + SDIO_CFG_MAX_50MHZ_MODE);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_brcmstb_save_restore_regs_v1(mmc: *mut mmc_host, save: c_int) {
    static void sdhci_brcmstb_save_restore_regs_v1(struct mmc_host *mmc, int save)
    {
    if (save)
    sdhci_brcmstb_save_regs(mmc, SDIO_CFG_CORE_V1);
    else
    sdhci_brcmstb_restore_regs(mmc, SDIO_CFG_CORE_V1);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_brcmstb_save_restore_regs_v2(mmc: *mut mmc_host, save: c_int) {
    static void sdhci_brcmstb_save_restore_regs_v2(struct mmc_host *mmc, int save)
    {
    if (save)
    sdhci_brcmstb_save_regs(mmc, SDIO_CFG_CORE_V2);
    else
    sdhci_brcmstb_restore_regs(mmc, SDIO_CFG_CORE_V2);
    }
#[no_mangle]
pub unsafe extern "C" fn enable_clock_gating(host: *mut sdhci_host) {
    static inline void enable_clock_gating(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_brcmstb_priv *priv = sdhci_pltfm_priv(pltfm_host);
    u32 reg;
    if (!(priv.flags & BRCMSTB_PRIV_FLAGS_GATE_CLOCK))
    return;
    reg = sdhci_readl(host, SDHCI_VENDOR);
    reg |= SDHCI_VENDOR_GATE_SDCLK_EN;
    sdhci_writel(host, reg, SDHCI_VENDOR);
    }
#[no_mangle]
unsafe extern "C" fn brcmstb_reset(host: *mut sdhci_host, mask: u8) {
    static void brcmstb_reset(struct sdhci_host *host, u8 mask)
    {
    sdhci_and_cqhci_reset(host, mask);
// Reset will clear this, so re-enable it
    enable_clock_gating(host);
    }
#[no_mangle]
unsafe extern "C" fn brcmstb_sdhci_reset_cmd_data(host: *mut sdhci_host, mask: u8) {
    static void brcmstb_sdhci_reset_cmd_data(struct sdhci_host *host, u8 mask)
    {
    let mut new_mask: u32 = (mask &  (SDHCI_RESET_CMD | SDHCI_RESET_DATA)) << 24;
    int ret;
    u32 reg;
//
// SDHCI_CLOCK_CONTROL register CARD_EN and CLOCK_INT_EN bits shall
// be set along with SOFTWARE_RESET register RESET_CMD or RESET_DATA
// bits, hence access SDHCI_CLOCK_CONTROL register as 32-bit register
//
    new_mask |= SDHCI_CLOCK_CARD_EN | SDHCI_CLOCK_INT_EN;
    reg = sdhci_readl(host, SDHCI_CLOCK_CONTROL);
    sdhci_writel(host, reg | new_mask, SDHCI_CLOCK_CONTROL);
    reg = sdhci_readb(host, SDHCI_SOFTWARE_RESET);
    ret = read_poll_timeout_atomic(sdhci_readb, reg, !(reg & mask),
    10, 10000, false,
    host, SDHCI_SOFTWARE_RESET);
    if (ret) {
    pr_err("%s: Reset 0x%x never completed.\n",
    mmc_hostname(host.mmc), (int)mask);
    sdhci_err_stats_inc(host, CTRL_TIMEOUT);
    sdhci_dumpregs(host);
    }
    }
#[no_mangle]
unsafe extern "C" fn brcmstb_reset_74165b0(host: *mut sdhci_host, mask: u8) {
    static void brcmstb_reset_74165b0(struct sdhci_host *host, u8 mask)
    {
// take care of RESET_ALL as usual
    if (mask & SDHCI_RESET_ALL)
    sdhci_and_cqhci_reset(host, SDHCI_RESET_ALL);
// cmd and/or data treated differently on this core
    if (mask & (SDHCI_RESET_CMD | SDHCI_RESET_DATA))
    brcmstb_sdhci_reset_cmd_data(host, mask);
// Reset will clear this, so re-enable it
    enable_clock_gating(host);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_brcmstb_hs400es(mmc: *mut mmc_host, ios: *mut mmc_ios) {
    static void sdhci_brcmstb_hs400es(struct mmc_host *mmc, struct mmc_ios *ios)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    u32 reg;
    dev_dbg(mmc_dev(mmc), "%s(): Setting HS400-Enhanced-Strobe mode\n",
    __func__);
    reg = readl(host.ioaddr + SDHCI_VENDOR);
    if (ios.enhanced_strobe)
    reg |= SDHCI_VENDOR_ENHANCED_STRB;
    else
    reg &= ~SDHCI_VENDOR_ENHANCED_STRB;
    writel(reg, host.ioaddr + SDHCI_VENDOR);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_brcmstb_set_clock(host: *mut sdhci_host, clock: c_uint) {
    static void sdhci_brcmstb_set_clock(struct sdhci_host *host, unsigned int clock)
    {
    u16 clk;
    host.mmc.actual_clock = 0;
    clk = sdhci_calc_clk(host, clock, &host.mmc.actual_clock);
    sdhci_writew(host, clk, SDHCI_CLOCK_CONTROL);
    if (clock == 0)
    return;
    sdhci_enable_clk(host, clk);
    }
    static void sdhci_brcmstb_set_uhs_signaling(struct sdhci_host *host,
    unsigned int timing)
    {
    u16 ctrl_2;
    dev_dbg(mmc_dev(host.mmc), "%s: Setting UHS signaling for %d timing\n",
    __func__, timing);
    ctrl_2 = sdhci_readw(host, SDHCI_HOST_CONTROL2);
// Select Bus Speed Mode for host
    ctrl_2 &= ~SDHCI_CTRL_UHS_MASK;
    if ((timing == MMC_TIMING_MMC_HS200) ||
    (timing == MMC_TIMING_UHS_SDR104))
    ctrl_2 |= SDHCI_CTRL_UHS_SDR104;
#[no_mangle]
pub unsafe extern "C" fn if(MMC_TIMING_UHS_SDR12: timing ==) -> else {
    else if (timing == MMC_TIMING_UHS_SDR12)
    ctrl_2 |= SDHCI_CTRL_UHS_SDR12;
    else if (timing == MMC_TIMING_SD_HS ||
    timing == MMC_TIMING_MMC_HS ||
    timing == MMC_TIMING_UHS_SDR25)
    ctrl_2 |= SDHCI_CTRL_UHS_SDR25;
#[no_mangle]
pub unsafe extern "C" fn if(MMC_TIMING_UHS_SDR50: timing ==) -> else {
    else if (timing == MMC_TIMING_UHS_SDR50)
    ctrl_2 |= SDHCI_CTRL_UHS_SDR50;
    else if ((timing == MMC_TIMING_UHS_DDR50) ||
    (timing == MMC_TIMING_MMC_DDR52))
    ctrl_2 |= SDHCI_CTRL_UHS_DDR50;
#[no_mangle]
pub unsafe extern "C" fn if(MMC_TIMING_MMC_HS400: timing ==) -> else {
    else if (timing == MMC_TIMING_MMC_HS400)
    ctrl_2 |= SDHCI_CTRL_HS400; /* Non-standard */
    sdhci_writew(host, ctrl_2, SDHCI_HOST_CONTROL2);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_brcmstb_cfginit_2712(host: *mut sdhci_host) {
    static void sdhci_brcmstb_cfginit_2712(struct sdhci_host *host)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_brcmstb_priv *brcmstb_priv = sdhci_pltfm_priv(pltfm_host);
    u32 reg;
//
// If we support a speed that requires tuning,
// then select the delay line PHY as the clock source.
//
    if ((host.mmc.caps & MMC_CAP_UHS_I_SDR_MASK) || (host.mmc.caps2 & MMC_CAP_HSE_MASK)) {
    reg = readl(brcmstb_priv.cfg_regs + SDIO_CFG_MAX_50MHZ_MODE);
    reg &= ~SDIO_CFG_MAX_50MHZ_MODE_ENABLE;
    reg |= SDIO_CFG_MAX_50MHZ_MODE_STRAP_OVERRIDE;
    writel(reg, brcmstb_priv.cfg_regs + SDIO_CFG_MAX_50MHZ_MODE);
    }
    if ((host.mmc.caps & MMC_CAP_NONREMOVABLE) ||
    (host.mmc.caps & MMC_CAP_NEEDS_POLL)) {
// Force presence
    reg = readl(brcmstb_priv.cfg_regs + SDIO_CFG_CTRL);
    reg &= ~SDIO_CFG_CTRL_SDCD_N_TEST_LEV;
    reg |= SDIO_CFG_CTRL_SDCD_N_TEST_EN;
    writel(reg, brcmstb_priv.cfg_regs + SDIO_CFG_CTRL);
    }
    }
#[no_mangle]
unsafe extern "C" fn sdhci_brcmstb_set_72116_uhs_signaling(host: *mut sdhci_host, timing: c_uint) {
    static void sdhci_brcmstb_set_72116_uhs_signaling(struct sdhci_host *host, unsigned int timing)
    {
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_brcmstb_priv *priv = sdhci_pltfm_priv(pltfm_host);
    u32 reg;
// no change to SDIO_CFG_OP_DLY_DEFAULT when using preset clk rate
    if (!(host.quirks2 & SDHCI_QUIRK2_PRESET_VALUE_BROKEN))
    return;
    reg = (timing == MMC_TIMING_MMC_HS200) ? 0 : SDIO_CFG_OP_DLY_DEFAULT;
    writel(reg, priv.cfg_regs + SDIO_CFG_OP_DLY);
    sdhci_set_uhs_signaling(host, timing);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_brcmstb_dumpregs(mmc: *mut mmc_host) {
    static void sdhci_brcmstb_dumpregs(struct mmc_host *mmc)
    {
    sdhci_dumpregs(mmc_priv(mmc));
    }
#[no_mangle]
unsafe extern "C" fn sdhci_brcmstb_cqe_enable(mmc: *mut mmc_host) {
    static void sdhci_brcmstb_cqe_enable(struct mmc_host *mmc)
    {
    struct sdhci_host *host = mmc_priv(mmc);
    u32 reg;
    reg = sdhci_readl(host, SDHCI_PRESENT_STATE);
    while (reg & SDHCI_DATA_AVAILABLE) {
    sdhci_readl(host, SDHCI_BUFFER);
    reg = sdhci_readl(host, SDHCI_PRESENT_STATE);
    }
    sdhci_cqe_enable(mmc);
    }
    static const struct cqhci_host_ops sdhci_brcmstb_cqhci_ops = {
    .enable         = sdhci_brcmstb_cqe_enable,
    .disable        = sdhci_cqe_disable,
    .dumpregs       = sdhci_brcmstb_dumpregs,
    };
    static struct sdhci_ops sdhci_brcmstb_ops = {
    .set_clock = sdhci_set_clock,
    .set_bus_width = sdhci_set_bus_width,
    .reset = sdhci_reset,
    .set_uhs_signaling = sdhci_set_uhs_signaling,
    };
    static struct sdhci_ops sdhci_brcmstb_ops_2712 = {
    .set_clock = sdhci_set_clock,
    .set_power = sdhci_set_power_and_bus_voltage,
    .set_bus_width = sdhci_set_bus_width,
    .reset = sdhci_reset,
    .set_uhs_signaling = sdhci_set_uhs_signaling,
    };
    static struct sdhci_ops sdhci_brcmstb_ops_72116 = {
    .set_clock = sdhci_set_clock,
    .set_bus_width = sdhci_set_bus_width,
    .reset = sdhci_reset,
    .set_uhs_signaling = sdhci_brcmstb_set_72116_uhs_signaling,
    };
    static struct sdhci_ops sdhci_brcmstb_ops_7216 = {
    .set_clock = sdhci_brcmstb_set_clock,
    .set_bus_width = sdhci_set_bus_width,
    .reset = brcmstb_reset,
    .set_uhs_signaling = sdhci_brcmstb_set_uhs_signaling,
    };
    static struct sdhci_ops sdhci_brcmstb_ops_74165b0 = {
    .set_clock = sdhci_brcmstb_set_clock,
    .set_bus_width = sdhci_set_bus_width,
    .reset = brcmstb_reset_74165b0,
    .set_uhs_signaling = sdhci_brcmstb_set_uhs_signaling,
    };
    static const struct brcmstb_match_priv match_priv_2712 = {
    .cfginit = sdhci_brcmstb_cfginit_2712,
    .ops = &sdhci_brcmstb_ops_2712,
    };
    static struct brcmstb_match_priv match_priv_7425 = {
    .flags = BRCMSTB_MATCH_FLAGS_NO_64BIT |
    BRCMSTB_MATCH_FLAGS_BROKEN_TIMEOUT,
    .ops = &sdhci_brcmstb_ops,
    };
    static struct brcmstb_match_priv match_priv_74371 = {
    .flags = BRCMSTB_MATCH_FLAGS_BROKEN_TIMEOUT,
    .ops = &sdhci_brcmstb_ops,
    };
    static struct brcmstb_match_priv match_priv_7445 = {
    .flags = BRCMSTB_MATCH_FLAGS_BROKEN_TIMEOUT,
    .save_restore_regs = sdhci_brcmstb_save_restore_regs_v1,
    .ops = &sdhci_brcmstb_ops,
    };
    static struct brcmstb_match_priv match_priv_72116 = {
    .flags = BRCMSTB_MATCH_FLAGS_BROKEN_TIMEOUT,
    .save_restore_regs = sdhci_brcmstb_save_restore_regs_v1,
    .ops = &sdhci_brcmstb_ops_72116,
    };
    static const struct brcmstb_match_priv match_priv_7216 = {
    .flags = BRCMSTB_MATCH_FLAGS_HAS_CLOCK_GATE,
    .save_restore_regs = sdhci_brcmstb_save_restore_regs_v2,
    .hs400es = sdhci_brcmstb_hs400es,
    .ops = &sdhci_brcmstb_ops_7216,
    };
    static struct brcmstb_match_priv match_priv_74165b0 = {
    .flags = BRCMSTB_MATCH_FLAGS_HAS_CLOCK_GATE,
    .save_restore_regs = sdhci_brcmstb_save_restore_regs_v2,
    .hs400es = sdhci_brcmstb_hs400es,
    .ops = &sdhci_brcmstb_ops_74165b0,
    };
    static const struct of_device_id __maybe_unused sdhci_brcm_of_match[] = {
    { .compatible = "brcm,bcm2712-sdhci", .data = &match_priv_2712 },
    { .compatible = "brcm,bcm7425-sdhci", .data = &match_priv_7425 },
    { .compatible = "brcm,bcm74371-sdhci", .data = &match_priv_74371 },
    { .compatible = "brcm,bcm7445-sdhci", .data = &match_priv_7445 },
    { .compatible = "brcm,bcm72116-sdhci", .data = &match_priv_72116 },
    { .compatible = "brcm,bcm7216-sdhci", .data = &match_priv_7216 },
    { .compatible = "brcm,bcm74165b0-sdhci", .data = &match_priv_74165b0 },
    {},
    };
#[no_mangle]
unsafe extern "C" fn sdhci_brcmstb_cqhci_irq(host: *mut sdhci_host, intmask: u32) -> u32 {
    static u32 sdhci_brcmstb_cqhci_irq(struct sdhci_host *host, u32 intmask)
    {
    let mut cmd_error: c_int = 0;
    let mut data_error: c_int = 0;
    if (!sdhci_cqe_irq(host, intmask, &cmd_error, &data_error))
    return intmask;
    cqhci_irq(host.mmc, cmd_error, data_error);
    return 0;
    }
    static int sdhci_brcmstb_add_host(struct sdhci_host *host,
    struct sdhci_brcmstb_priv *priv)
    {
    struct cqhci_host *cq_host;
    bool dma64;
    int ret;
    if ((priv.flags & BRCMSTB_PRIV_FLAGS_HAS_CQE) == 0)
    return sdhci_add_host(host);
    dev_dbg(mmc_dev(host.mmc), "CQE is enabled\n");
    host.mmc.caps2 |= MMC_CAP2_CQE | MMC_CAP2_CQE_DCMD;
    ret = sdhci_setup_host(host);
    if (ret)
    return ret;
    cq_host = devm_kzalloc(mmc_dev(host.mmc),
    sizeof(*cq_host), GFP_KERNEL);
    if (!cq_host) {
    ret = -ENOMEM;
    goto cleanup;
    }
    cq_host.mmio = host.ioaddr + SDHCI_ARASAN_CQE_BASE_ADDR;
    cq_host.ops = &sdhci_brcmstb_cqhci_ops;
    dma64 = host.flags & SDHCI_USE_64_BIT_DMA;
    if (dma64) {
    dev_dbg(mmc_dev(host.mmc), "Using 64 bit DMA\n");
    cq_host.caps |= CQHCI_TASK_DESC_SZ_128;
    }
    ret = cqhci_init(cq_host, host.mmc, dma64);
    if (ret)
    goto cleanup;
    ret = __sdhci_add_host(host);
    if (ret)
    goto cleanup;
    return 0;
    cleanup:
    sdhci_cleanup_host(host);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_brcmstb_probe(pdev: *mut platform_device) -> c_int {
    static int sdhci_brcmstb_probe(struct platform_device *pdev)
    {
    const struct brcmstb_match_priv *match_priv;
    struct sdhci_pltfm_data brcmstb_pdata;
    struct sdhci_pltfm_host *pltfm_host;
    const struct of_device_id *match;
    struct sdhci_brcmstb_priv *priv;
    u32 actual_clock_mhz;
    struct sdhci_host *host;
    struct clk *clk;
    struct clk *base_clk = core::ptr::null_mut();
    int res;
    match = of_match_node(sdhci_brcm_of_match, pdev.dev.of_node);
    match_priv = match.data;
    dev_dbg(&pdev.dev, "Probe found match for %s\n",  match.compatible);
    clk = devm_clk_get_optional_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(clk),
    "Failed to get and enable clock from Device Tree\n");
    memset(&brcmstb_pdata, 0, sizeof(brcmstb_pdata));
    brcmstb_pdata.ops = match_priv.ops;
    host = sdhci_pltfm_init(pdev, &brcmstb_pdata,
    sizeof(struct sdhci_brcmstb_priv));
    if (IS_ERR(host))
    return PTR_ERR(host);
    pltfm_host = sdhci_priv(host);
    priv = sdhci_pltfm_priv(pltfm_host);
    priv.match_priv = match.data;
    if (device_property_read_bool(&pdev.dev, "supports-cqe")) {
    priv.flags |= BRCMSTB_PRIV_FLAGS_HAS_CQE;
    match_priv.ops.irq = sdhci_brcmstb_cqhci_irq;
    }
// Map in the non-standard CFG registers
    priv.cfg_regs = devm_platform_get_and_ioremap_resource(pdev, 1, core::ptr::null_mut());
    if (IS_ERR(priv.cfg_regs)) {
    res = PTR_ERR(priv.cfg_regs);
    goto err;
    }
    sdhci_get_of_property(pdev);
    res = mmc_of_parse(host.mmc);
    if (res)
    goto err;
// map non-standard BOOT registers if present
    if (host.mmc.caps & MMC_CAP_NONREMOVABLE) {
    priv.boot_regs = devm_platform_get_and_ioremap_resource(pdev, 2, core::ptr::null_mut());
    if (IS_ERR(priv.boot_regs))
    priv.boot_regs = core::ptr::null_mut();
    }
//
// Automatic clock gating does not work for SD cards that may
// voltage switch so only enable it for non-removable devices.
//
    if ((match_priv.flags & BRCMSTB_MATCH_FLAGS_HAS_CLOCK_GATE) &&
    (host.mmc.caps & MMC_CAP_NONREMOVABLE))
    priv.flags |= BRCMSTB_PRIV_FLAGS_GATE_CLOCK;
//
// If the chip has enhanced strobe and it's enabled, add
// callback
//
    if (match_priv.hs400es &&
    (host.mmc.caps2 & MMC_CAP2_HS400_ES))
    host.mmc_host_ops.hs400_enhanced_strobe = match_priv.hs400es;
    if (match_priv.cfginit)
    match_priv.cfginit(host);
//
// Supply the existing CAPS, but clear the UHS modes. This
// will allow these modes to be specified by device tree
// properties through mmc_of_parse().
//
    sdhci_read_caps(host);
    if (match_priv.flags & BRCMSTB_MATCH_FLAGS_NO_64BIT)
    host.caps &= ~SDHCI_CAN_64BIT;
    host.caps1 &= ~(SDHCI_SUPPORT_SDR50 | SDHCI_SUPPORT_SDR104 |
    SDHCI_SUPPORT_DDR50);
    if (match_priv.flags & BRCMSTB_MATCH_FLAGS_BROKEN_TIMEOUT)
    host.quirks |= SDHCI_QUIRK_BROKEN_TIMEOUT_VAL;
    if (!(match_priv.flags & BRCMSTB_MATCH_FLAGS_USE_CARD_BUSY))
    host.mmc_host_ops.card_busy = core::ptr::null_mut();
// Change the base clock frequency if the DT property exists
    if (device_property_read_u32(&pdev.dev, "clock-frequency",
    &priv.base_freq_hz) != 0)
    goto add_host;
    base_clk = devm_clk_get_optional(&pdev.dev, "sdio_freq");
    if (IS_ERR(base_clk)) {
    dev_warn(&pdev.dev, "Clock for \"sdio_freq\" not found\n");
    goto add_host;
    }
    res = clk_prepare_enable(base_clk);
    if (res)
    goto err;
// set improved clock rate
    clk_set_rate(base_clk, priv.base_freq_hz);
    actual_clock_mhz = clk_get_rate(base_clk) / 1000000;
    host.caps &= ~SDHCI_CLOCK_V3_BASE_MASK;
    host.caps |= (actual_clock_mhz << SDHCI_CLOCK_BASE_SHIFT);
// Disable presets because they are now incorrect
    host.quirks2 |= SDHCI_QUIRK2_PRESET_VALUE_BROKEN;
    dev_dbg(&pdev.dev, "Base Clock Frequency changed to %dMHz\n",
    actual_clock_mhz);
    priv.base_clk = base_clk;
    add_host:
    res = sdhci_brcmstb_add_host(host, priv);
    if (res)
    goto err;
    pltfm_host.clk = clk;
    return res;
    err:
    clk_disable_unprepare(base_clk);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn sdhci_brcmstb_shutdown(pdev: *mut platform_device) {
    static void sdhci_brcmstb_shutdown(struct platform_device *pdev)
    {
    sdhci_pltfm_suspend(&pdev.dev);
    }
    MODULE_DEVICE_TABLE(of, sdhci_brcm_of_match);
#[no_mangle]
unsafe extern "C" fn sdhci_brcmstb_suspend(dev: *mut device) -> c_int {
    static int sdhci_brcmstb_suspend(struct device *dev)
    {
    struct sdhci_host *host = dev_get_drvdata(dev);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_brcmstb_priv *priv = sdhci_pltfm_priv(pltfm_host);
    const struct brcmstb_match_priv *match_priv = priv.match_priv;
    int ret;
    if (match_priv.save_restore_regs)
    match_priv.save_restore_regs(host.mmc, 1);
    clk_disable_unprepare(priv.base_clk);
    if (host.mmc.caps2 & MMC_CAP2_CQE) {
    ret = cqhci_suspend(host.mmc);
    if (ret)
    return ret;
    }
    return sdhci_pltfm_suspend(dev);
    }
#[no_mangle]
unsafe extern "C" fn sdhci_brcmstb_resume(dev: *mut device) -> c_int {
    static int sdhci_brcmstb_resume(struct device *dev)
    {
    struct sdhci_host *host = dev_get_drvdata(dev);
    struct sdhci_pltfm_host *pltfm_host = sdhci_priv(host);
    struct sdhci_brcmstb_priv *priv = sdhci_pltfm_priv(pltfm_host);
    const struct brcmstb_match_priv *match_priv = priv.match_priv;
    int ret;
    ret = sdhci_pltfm_resume(dev);
    if (!ret && priv.base_freq_hz) {
    ret = clk_prepare_enable(priv.base_clk);
//
// Note: using clk_get_rate() below as clk_get_rate()
// honors CLK_GET_RATE_NOCACHE attribute, but clk_set_rate()
// may do implicit get_rate() calls that do not honor
// CLK_GET_RATE_NOCACHE.
//
    if (!ret &&
    (clk_get_rate(priv.base_clk) != priv.base_freq_hz))
    ret = clk_set_rate(priv.base_clk, priv.base_freq_hz);
    }
    if (match_priv.save_restore_regs)
    match_priv.save_restore_regs(host.mmc, 0);
    if (host.mmc.caps2 & MMC_CAP2_CQE)
    ret = cqhci_resume(host.mmc);
    return ret;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(sdhci_brcmstb_pmops, sdhci_brcmstb_suspend, sdhci_brcmstb_resume);
    static struct platform_driver sdhci_brcmstb_driver = {
    .driver		= {
    .name	= "sdhci-brcmstb",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .pm	= pm_sleep_ptr(&sdhci_brcmstb_pmops),
    .of_match_table = of_match_ptr(sdhci_brcm_of_match),
    },
    .probe		= sdhci_brcmstb_probe,
    .remove		= sdhci_pltfm_remove,
    .shutdown	= sdhci_brcmstb_shutdown,
    };
    module_platform_driver(sdhci_brcmstb_driver);
    MODULE_DESCRIPTION("SDHCI driver for Broadcom BRCMSTB SoCs");
    MODULE_AUTHOR("Broadcom");
    MODULE_LICENSE("GPL v2");
