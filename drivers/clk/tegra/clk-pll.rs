//! Automatically rewritten from C to Rust
//! Source: drivers/clk/tegra/clk-pll.c
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
// Copyright (c) 2012, 2013, NVIDIA CORPORATION.  All rights reserved.
//

pub const PLL_BASE_DIVP_SHIFT: c_int = 20;
pub const PLL_BASE_DIVP_WIDTH: c_int = 3;
pub const PLL_BASE_DIVN_SHIFT: c_int = 8;
pub const PLL_BASE_DIVN_WIDTH: c_int = 10;
pub const PLL_BASE_DIVM_SHIFT: c_int = 0;
pub const PLL_BASE_DIVM_WIDTH: c_int = 5;
pub const PLLU_POST_DIVP_MASK: c_uint = 0x1;
pub const PLL_MISC_DCCON_SHIFT: c_int = 20;
pub const PLL_MISC_CPCON_SHIFT: c_int = 8;
pub const PLL_MISC_CPCON_WIDTH: c_int = 4;

pub const PLL_MISC_LFCON_SHIFT: c_int = 4;
pub const PLL_MISC_LFCON_WIDTH: c_int = 4;

pub const PLL_MISC_VCOCON_SHIFT: c_int = 0;
pub const PLL_MISC_VCOCON_WIDTH: c_int = 4;

pub const OUT_OF_TABLE_CPCON: c_int = 8;
pub const PMC_PLLP_WB0_OVERRIDE: c_uint = 0xf8;

pub const PLL_POST_LOCK_DELAY: c_int = 50;
pub const PLLDU_LFCON_SET_DIVN: c_int = 600;
pub const PLLE_BASE_DIVCML_SHIFT: c_int = 24;
pub const PLLE_BASE_DIVCML_MASK: c_uint = 0xf;
pub const PLLE_BASE_DIVP_SHIFT: c_int = 16;
pub const PLLE_BASE_DIVP_WIDTH: c_int = 6;
pub const PLLE_BASE_DIVN_SHIFT: c_int = 8;
pub const PLLE_BASE_DIVN_WIDTH: c_int = 8;
pub const PLLE_BASE_DIVM_SHIFT: c_int = 0;
pub const PLLE_BASE_DIVM_WIDTH: c_int = 8;

pub const PLLE_MISC_SETUP_BASE_SHIFT: c_int = 16;

pub const PLLE_MISC_SETUP_EX_SHIFT: c_int = 2;

    PLLE_MISC_SETUP_EX_MASK)

pub const PLLE_SS_CTRL: c_uint = 0x68;

    PLLE_SS_CNTL_SSC_BYP)
pub const PLLE_SS_MAX_MASK: c_uint = 0x1ff;
pub const PLLE_SS_MAX_VAL_TEGRA114: c_uint = 0x25;
pub const PLLE_SS_MAX_VAL_TEGRA210: c_uint = 0x21;

    (PLLE_SS_MAX_MASK | PLLE_SS_INC_MASK | PLLE_SS_INCINTRV_MASK)

    (PLLE_SS_MAX_VAL_TEGRA114 | PLLE_SS_INC_VAL |\
    PLLE_SS_INCINTRV_VAL_TEGRA114)

    (PLLE_SS_MAX_VAL_TEGRA210 | PLLE_SS_INC_VAL |\
    PLLE_SS_INCINTRV_VAL_TEGRA210)

pub const XUSBIO_PLL_CFG0: c_uint = 0x51c;

pub const SATA_PLL_CFG0: c_uint = 0x490;

pub const PLLE_MISC_VREG_BG_CTRL_SHIFT: c_int = 4;

pub const PLLE_MISC_VREG_CTRL_SHIFT: c_int = 2;

pub const PLLCX_MISC_SDM_DIV_SHIFT: c_int = 28;

pub const PLLCX_MISC_FILT_DIV_SHIFT: c_int = 26;

pub const PLLCX_MISC_ALPHA_SHIFT: c_int = 18;

    ((0x1 << PLLCX_MISC_SDM_DIV_SHIFT) | \
    (0x1 << PLLCX_MISC_FILT_DIV_SHIFT))

    ((0x2 << PLLCX_MISC_SDM_DIV_SHIFT) | \
    (0x2 << PLLCX_MISC_FILT_DIV_SHIFT))

    ((0x14 << PLLCX_MISC_KA_SHIFT) | (0x38 << PLLCX_MISC_KB_SHIFT))
pub const PLLCX_MISC_KA_SHIFT: c_int = 2;
pub const PLLCX_MISC_KB_SHIFT: c_int = 9;

    (0x19 << PLLCX_MISC_ALPHA_SHIFT) | \
    PLLCX_MISC_DIV_LOW_RANGE | \
    PLLCX_MISC_RESET)
pub const PLLCX_MISC1_DEFAULT: c_uint = 0x000d2308;
pub const PLLCX_MISC2_DEFAULT: c_uint = 0x30211200;
pub const PLLCX_MISC3_DEFAULT: c_uint = 0x200;
pub const PMC_SATA_PWRGT: c_uint = 0x1ac;

pub const PLLSS_MISC_KCP: c_int = 0;
pub const PLLSS_MISC_KVCO: c_int = 0;
pub const PLLSS_MISC_SETUP: c_int = 0;
pub const PLLSS_EN_SDM: c_int = 0;
pub const PLLSS_EN_SSC: c_int = 0;
pub const PLLSS_EN_DITHER2: c_int = 0;
pub const PLLSS_EN_DITHER: c_int = 1;
pub const PLLSS_SDM_RESET: c_int = 0;
pub const PLLSS_CLAMP: c_int = 0;
pub const PLLSS_SDM_SSC_MAX: c_int = 0;
pub const PLLSS_SDM_SSC_MIN: c_int = 0;
pub const PLLSS_SDM_SSC_STEP: c_int = 0;
pub const PLLSS_SDM_DIN: c_int = 0;

    (PLLSS_MISC_KVCO << 24) | \
    PLLSS_MISC_SETUP)

    (PLLSS_EN_SSC << 30) | \
    (PLLSS_EN_DITHER2 << 29) | \
    (PLLSS_EN_DITHER << 28) | \
    (PLLSS_SDM_RESET) << 27 | \
    (PLLSS_CLAMP << 22))

    ((PLLSS_SDM_SSC_MAX << 16) | PLLSS_SDM_SSC_MIN)

    ((PLLSS_SDM_SSC_STEP << 16) | PLLSS_SDM_DIN)

pub const PLLSS_REF_SRC_SEL_SHIFT: c_int = 25;

pub const UTMIP_PLL_CFG1: c_uint = 0x484;

pub const UTMIP_PLL_CFG2: c_uint = 0x488;

pub const UTMIPLL_HW_PWRDN_CFG0: c_uint = 0x52c;

pub const PLLU_HW_PWRDN_CFG0: c_uint = 0x530;

pub const XUSB_PLL_CFG0: c_uint = 0x534;
pub const XUSB_PLL_CFG0_UTMIPLL_LOCK_DLY: c_uint = 0x3ff;

    mask(p.params.div_nmp.divp_width))

    static struct div_nmp default_nmp = {
    .divn_shift = PLL_BASE_DIVN_SHIFT,
    .divn_width = PLL_BASE_DIVN_WIDTH,
    .divm_shift = PLL_BASE_DIVM_SHIFT,
    .divm_width = PLL_BASE_DIVM_WIDTH,
    .divp_shift = PLL_BASE_DIVP_SHIFT,
    .divp_width = PLL_BASE_DIVP_WIDTH,
    };
#[no_mangle]
unsafe extern "C" fn clk_pll_enable_lock(pll: *mut tegra_clk_pll) {
    static void clk_pll_enable_lock(struct tegra_clk_pll *pll)
    {
    u32 val;
    if (!(pll.params.flags & TEGRA_PLL_USE_LOCK))
    return;
    if (!(pll.params.flags & TEGRA_PLL_HAS_LOCK_ENABLE))
    return;
    val = pll_readl_misc(pll);
    val |= BIT(pll.params.lock_enable_bit_idx);
    pll_writel_misc(val, pll);
    }
#[no_mangle]
unsafe extern "C" fn clk_pll_wait_for_lock(pll: *mut tegra_clk_pll) -> c_int {
    static int clk_pll_wait_for_lock(struct tegra_clk_pll *pll)
    {
    int i;
    u32 val, lock_mask;
    void __iomem *lock_addr;
    if (!(pll.params.flags & TEGRA_PLL_USE_LOCK)) {
    udelay(pll.params.lock_delay);
    return 0;
    }
    lock_addr = pll.clk_base;
    if (pll.params.flags & TEGRA_PLL_LOCK_MISC)
    lock_addr += pll.params.misc_reg;
    else
    lock_addr += pll.params.base_reg;
    lock_mask = pll.params.lock_mask;
    for (i = 0; i < pll.params.lock_delay; i++) {
    val = readl_relaxed(lock_addr);
    if ((val & lock_mask) == lock_mask) {
    udelay(PLL_POST_LOCK_DELAY);
    return 0;
    }
    udelay(2); /* timeout = 2 * lock time */
    }
    pr_err("%s: Timed out waiting for pll %s lock\n", __func__,
    clk_hw_get_name(&pll.hw));
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_pll_wait_for_lock(pll: *mut tegra_clk_pll) -> c_int {
    int tegra_pll_wait_for_lock(struct tegra_clk_pll *pll)
    {
    return clk_pll_wait_for_lock(pll);
    }
#[no_mangle]
unsafe extern "C" fn pllm_clk_is_gated_by_pmc(pll: *mut tegra_clk_pll) -> bool {
    static bool pllm_clk_is_gated_by_pmc(struct tegra_clk_pll *pll)
    {
    let mut val: u32 = readl_relaxed(pll.pmc + PMC_PLLP_WB0_OVERRIDE);
    return (val & PMC_PLLP_WB0_OVERRIDE_PLLM_OVERRIDE) &&
    !(val & PMC_PLLP_WB0_OVERRIDE_PLLM_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn clk_pll_is_enabled(hw: *mut clk_hw) -> c_int {
    static int clk_pll_is_enabled(struct clk_hw *hw)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    u32 val;
//
// Power Management Controller (PMC) can override the PLLM clock
// settings, including the enable-state. The PLLM is enabled when
// PLLM's CaR state is ON and when PLLM isn't gated by PMC.
//
    if ((pll.params.flags & TEGRA_PLLM) && pllm_clk_is_gated_by_pmc(pll))
    return 0;
    val = pll_readl_base(pll);
    return val & PLL_BASE_ENABLE ? 1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn _clk_pll_enable(hw: *mut clk_hw) {
    static void _clk_pll_enable(struct clk_hw *hw)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    u32 val;
    if (pll.params.iddq_reg) {
    val = pll_readl(pll.params.iddq_reg, pll);
    val &= ~BIT(pll.params.iddq_bit_idx);
    pll_writel(val, pll.params.iddq_reg, pll);
    udelay(5);
    }
    if (pll.params.reset_reg) {
    val = pll_readl(pll.params.reset_reg, pll);
    val &= ~BIT(pll.params.reset_bit_idx);
    pll_writel(val, pll.params.reset_reg, pll);
    }
    clk_pll_enable_lock(pll);
    val = pll_readl_base(pll);
    if (pll.params.flags & TEGRA_PLL_BYPASS)
    val &= ~PLL_BASE_BYPASS;
    val |= PLL_BASE_ENABLE;
    pll_writel_base(val, pll);
    if (pll.params.flags & TEGRA_PLLM) {
    val = readl_relaxed(pll.pmc + PMC_PLLP_WB0_OVERRIDE);
    val |= PMC_PLLP_WB0_OVERRIDE_PLLM_ENABLE;
    writel_relaxed(val, pll.pmc + PMC_PLLP_WB0_OVERRIDE);
    }
    }
#[no_mangle]
unsafe extern "C" fn _clk_pll_disable(hw: *mut clk_hw) {
    static void _clk_pll_disable(struct clk_hw *hw)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    u32 val;
    val = pll_readl_base(pll);
    if (pll.params.flags & TEGRA_PLL_BYPASS)
    val &= ~PLL_BASE_BYPASS;
    val &= ~PLL_BASE_ENABLE;
    pll_writel_base(val, pll);
    if (pll.params.flags & TEGRA_PLLM) {
    val = readl_relaxed(pll.pmc + PMC_PLLP_WB0_OVERRIDE);
    val &= ~PMC_PLLP_WB0_OVERRIDE_PLLM_ENABLE;
    writel_relaxed(val, pll.pmc + PMC_PLLP_WB0_OVERRIDE);
    }
    if (pll.params.reset_reg) {
    val = pll_readl(pll.params.reset_reg, pll);
    val |= BIT(pll.params.reset_bit_idx);
    pll_writel(val, pll.params.reset_reg, pll);
    }
    if (pll.params.iddq_reg) {
    val = pll_readl(pll.params.iddq_reg, pll);
    val |= BIT(pll.params.iddq_bit_idx);
    pll_writel(val, pll.params.iddq_reg, pll);
    udelay(2);
    }
    }
#[no_mangle]
unsafe extern "C" fn pll_clk_start_ss(pll: *mut tegra_clk_pll) {
    static void pll_clk_start_ss(struct tegra_clk_pll *pll)
    {
    if (pll.params.defaults_set && pll.params.ssc_ctrl_reg) {
    let mut val: u32 = pll_readl(pll.params.ssc_ctrl_reg, pll);
    val |= pll.params.ssc_ctrl_en_mask;
    pll_writel(val, pll.params.ssc_ctrl_reg, pll);
    }
    }
#[no_mangle]
unsafe extern "C" fn pll_clk_stop_ss(pll: *mut tegra_clk_pll) {
    static void pll_clk_stop_ss(struct tegra_clk_pll *pll)
    {
    if (pll.params.defaults_set && pll.params.ssc_ctrl_reg) {
    let mut val: u32 = pll_readl(pll.params.ssc_ctrl_reg, pll);
    val &= ~pll.params.ssc_ctrl_en_mask;
    pll_writel(val, pll.params.ssc_ctrl_reg, pll);
    }
    }
#[no_mangle]
unsafe extern "C" fn clk_pll_enable(hw: *mut clk_hw) -> c_int {
    static int clk_pll_enable(struct clk_hw *hw)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    let mut flags: c_ulong = 0;
    int ret;
    if (clk_pll_is_enabled(hw))
    return 0;
    if (pll.lock)
    spin_lock_irqsave(pll.lock, flags);
    _clk_pll_enable(hw);
    ret = clk_pll_wait_for_lock(pll);
    pll_clk_start_ss(pll);
    if (pll.lock)
    spin_unlock_irqrestore(pll.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn clk_pll_disable(hw: *mut clk_hw) {
    static void clk_pll_disable(struct clk_hw *hw)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    let mut flags: c_ulong = 0;
    if (pll.lock)
    spin_lock_irqsave(pll.lock, flags);
    pll_clk_stop_ss(pll);
    _clk_pll_disable(hw);
    if (pll.lock)
    spin_unlock_irqrestore(pll.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn _p_div_to_hw(hw: *mut clk_hw, p_div: u8) -> c_int {
    static int _p_div_to_hw(struct clk_hw *hw, u8 p_div)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    const struct pdiv_map *p_tohw = pll.params.pdiv_tohw;
    if (p_tohw) {
    while (p_tohw.pdiv) {
    if (p_div <= p_tohw.pdiv)
    return p_tohw.hw_val;
    p_tohw++;
    }
    return -EINVAL;
    }
    return -EINVAL;
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_pll_p_div_to_hw(pll: *mut tegra_clk_pll, p_div: u8) -> c_int {
    int tegra_pll_p_div_to_hw(struct tegra_clk_pll *pll, u8 p_div)
    {
    return _p_div_to_hw(&pll.hw, p_div);
    }
#[no_mangle]
unsafe extern "C" fn _hw_to_p_div(hw: *mut clk_hw, p_div_hw: u8) -> c_int {
    static int _hw_to_p_div(struct clk_hw *hw, u8 p_div_hw)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    const struct pdiv_map *p_tohw = pll.params.pdiv_tohw;
    if (p_tohw) {
    while (p_tohw.pdiv) {
    if (p_div_hw == p_tohw.hw_val)
    return p_tohw.pdiv;
    p_tohw++;
    }
    return -EINVAL;
    }
    return 1 << p_div_hw;
    }
    static int _get_table_rate(struct clk_hw *hw,
    struct tegra_clk_pll_freq_table *cfg,
    unsigned long rate, unsigned long parent_rate)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    struct tegra_clk_pll_freq_table *sel;
    int p;
    for (sel = pll.params.freq_table; sel.input_rate != 0; sel++)
    if (sel.input_rate == parent_rate &&
    sel.output_rate == rate)
    break;
    if (sel.input_rate == 0)
    return -EINVAL;
    if (pll.params.pdiv_tohw) {
    p = _p_div_to_hw(hw, sel.p);
    if (p < 0)
    return p;
    } else {
    p = ilog2(sel.p);
    }
    cfg.input_rate = sel.input_rate;
    cfg.output_rate = sel.output_rate;
    cfg.m = sel.m;
    cfg.n = sel.n;
    cfg.p = p;
    cfg.cpcon = sel.cpcon;
    cfg.sdm_data = sel.sdm_data;
    return 0;
    }
    static int _calc_rate(struct clk_hw *hw, struct tegra_clk_pll_freq_table *cfg,
    unsigned long rate, unsigned long parent_rate)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    unsigned long cfreq;
    let mut p_div: u32 = 0;
    int ret;
    if (!rate)
    return -EINVAL;
    switch (parent_rate) {
    case 12000000:
    case 26000000:
    cfreq = (rate <= 1000000 * 1000) ? 1000000 : 2000000;
    break;
    case 13000000:
    cfreq = (rate <= 1000000 * 1000) ? 1000000 : 2600000;
    break;
    case 16800000:
    case 19200000:
    cfreq = (rate <= 1200000 * 1000) ? 1200000 : 2400000;
    break;
    case 9600000:
    case 28800000:
    case 48000000:
//
// PLL_P_OUT1 rate is not listed in PLLA table
//
    cfreq = parent_rate / (parent_rate / 1000000);
    break;
    default:
    pr_err("%s Unexpected reference rate %lu\n",
    __func__, parent_rate);
    BUG();
    }
// Raise VCO to guarantee 0.5% accuracy
    for (cfg.output_rate = rate; cfg.output_rate < 200 * cfreq;
    cfg.output_rate <<= 1)
    p_div++;
    cfg.m = parent_rate / cfreq;
    cfg.n = cfg.output_rate / cfreq;
    cfg.cpcon = OUT_OF_TABLE_CPCON;
    if (cfg.m == 0 || cfg.m > divm_max(pll) ||
    cfg.n > divn_max(pll) || (1 << p_div) > divp_max(pll) ||
    cfg.output_rate > pll.params.vco_max) {
    return -EINVAL;
    }
    cfg.output_rate = cfg.n * DIV_ROUND_UP(parent_rate, cfg.m);
    cfg.output_rate >>= p_div;
    if (pll.params.pdiv_tohw) {
    ret = _p_div_to_hw(hw, 1 << p_div);
    if (ret < 0)
    return ret;
    else
    cfg.p = ret;
    } else
    cfg.p = p_div;
    return 0;
    }
//
// SDM (Sigma Delta Modulator) divisor is 16-bit 2's complement signed number
// within (-2^12 ... 2^12-1) range. Represented in PLL data structure as
// unsigned 16-bit value, with "0" divisor mapped to 0xFFFF. Data "0" is used
// to indicate that SDM is disabled.
//
// Effective ndiv value when SDM is enabled: ndiv + 1/2 + sdm_din/2^13
//
    static void clk_pll_set_sdm_data(struct clk_hw *hw,
    struct tegra_clk_pll_freq_table *cfg)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    u32 val;
    bool enabled;
    if (!pll.params.sdm_din_reg)
    return;
    if (cfg.sdm_data) {
    val = pll_readl_sdm_din(pll) & (~sdm_din_mask(pll));
    val |= sdin_data_to_din(cfg.sdm_data) & sdm_din_mask(pll);
    pll_writel_sdm_din(val, pll);
    }
    val = pll_readl_sdm_ctrl(pll);
    enabled = (val & sdm_en_mask(pll));
    if (cfg.sdm_data == 0 && enabled)
    val &= ~pll.params.sdm_ctrl_en_mask;
    if (cfg.sdm_data != 0 && !enabled)
    val |= pll.params.sdm_ctrl_en_mask;
    pll_writel_sdm_ctrl(val, pll);
    }
    static void _update_pll_mnp(struct tegra_clk_pll *pll,
    struct tegra_clk_pll_freq_table *cfg)
    {
    u32 val;
    struct tegra_clk_pll_params *params = pll.params;
    struct div_nmp *div_nmp = params.div_nmp;
    if ((params.flags & (TEGRA_PLLM | TEGRA_PLLMB)) &&
    (pll_override_readl(PMC_PLLP_WB0_OVERRIDE, pll) &
    PMC_PLLP_WB0_OVERRIDE_PLLM_OVERRIDE)) {
    val = pll_override_readl(params.pmc_divp_reg, pll);
    val &= ~(divp_mask(pll) << div_nmp.override_divp_shift);
    val |= cfg.p << div_nmp.override_divp_shift;
    pll_override_writel(val, params.pmc_divp_reg, pll);
    val = pll_override_readl(params.pmc_divnm_reg, pll);
    val &= ~((divm_mask(pll) << div_nmp.override_divm_shift) |
    (divn_mask(pll) << div_nmp.override_divn_shift));
    val |= (cfg.m << div_nmp.override_divm_shift) |
    (cfg.n << div_nmp.override_divn_shift);
    pll_override_writel(val, params.pmc_divnm_reg, pll);
    } else {
    val = pll_readl_base(pll);
    val &= ~(divm_mask_shifted(pll) | divn_mask_shifted(pll) |
    divp_mask_shifted(pll));
    val |= (cfg.m << divm_shift(pll)) |
    (cfg.n << divn_shift(pll)) |
    (cfg.p << divp_shift(pll));
    pll_writel_base(val, pll);
    clk_pll_set_sdm_data(&pll.hw, cfg);
    }
    }
    static void _get_pll_mnp(struct tegra_clk_pll *pll,
    struct tegra_clk_pll_freq_table *cfg)
    {
    u32 val;
    struct tegra_clk_pll_params *params = pll.params;
    struct div_nmp *div_nmp = params.div_nmp;
// cfg = (struct tegra_clk_pll_freq_table) { };
    if ((params.flags & (TEGRA_PLLM | TEGRA_PLLMB)) &&
    (pll_override_readl(PMC_PLLP_WB0_OVERRIDE, pll) &
    PMC_PLLP_WB0_OVERRIDE_PLLM_OVERRIDE)) {
    val = pll_override_readl(params.pmc_divp_reg, pll);
    cfg.p = (val >> div_nmp.override_divp_shift) & divp_mask(pll);
    val = pll_override_readl(params.pmc_divnm_reg, pll);
    cfg.m = (val >> div_nmp.override_divm_shift) & divm_mask(pll);
    cfg.n = (val >> div_nmp.override_divn_shift) & divn_mask(pll);
    }  else {
    val = pll_readl_base(pll);
    cfg.m = (val >> div_nmp.divm_shift) & divm_mask(pll);
    cfg.n = (val >> div_nmp.divn_shift) & divn_mask(pll);
    cfg.p = (val >> div_nmp.divp_shift) & divp_mask(pll);
    if (pll.params.sdm_din_reg) {
    if (sdm_en_mask(pll) & pll_readl_sdm_ctrl(pll)) {
    val = pll_readl_sdm_din(pll);
    val &= sdm_din_mask(pll);
    cfg.sdm_data = sdin_din_to_data(val);
    }
    }
    }
    }
    static void _update_pll_cpcon(struct tegra_clk_pll *pll,
    struct tegra_clk_pll_freq_table *cfg,
    unsigned long rate)
    {
    u32 val;
    val = pll_readl_misc(pll);
    val &= ~(PLL_MISC_CPCON_MASK << PLL_MISC_CPCON_SHIFT);
    val |= cfg.cpcon << PLL_MISC_CPCON_SHIFT;
    if (pll.params.flags & TEGRA_PLL_SET_LFCON) {
    val &= ~(PLL_MISC_LFCON_MASK << PLL_MISC_LFCON_SHIFT);
    if (cfg.n >= PLLDU_LFCON_SET_DIVN)
    val |= 1 << PLL_MISC_LFCON_SHIFT;
    } else if (pll.params.flags & TEGRA_PLL_SET_DCCON) {
    val &= ~(1 << PLL_MISC_DCCON_SHIFT);
    if (rate >= (pll.params.vco_max >> 1))
    val |= 1 << PLL_MISC_DCCON_SHIFT;
    }
    pll_writel_misc(val, pll);
    }
    static int _program_pll(struct clk_hw *hw, struct tegra_clk_pll_freq_table *cfg,
    unsigned long rate)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    struct tegra_clk_pll_freq_table old_cfg;
    int state, ret = 0;
    state = clk_pll_is_enabled(hw);
    if (state && pll.params.pre_rate_change) {
    ret = pll.params.pre_rate_change();
    if (WARN_ON(ret))
    return ret;
    }
    _get_pll_mnp(pll, &old_cfg);
    if (state && pll.params.defaults_set && pll.params.dyn_ramp &&
    (cfg.m == old_cfg.m) && (cfg.p == old_cfg.p)) {
    ret = pll.params.dyn_ramp(pll, cfg);
    if (!ret)
    goto done;
    }
    if (state) {
    pll_clk_stop_ss(pll);
    _clk_pll_disable(hw);
    }
    if (!pll.params.defaults_set && pll.params.set_defaults)
    pll.params.set_defaults(pll);
    _update_pll_mnp(pll, cfg);
    if (pll.params.flags & TEGRA_PLL_HAS_CPCON)
    _update_pll_cpcon(pll, cfg, rate);
    if (state) {
    _clk_pll_enable(hw);
    ret = clk_pll_wait_for_lock(pll);
    pll_clk_start_ss(pll);
    }
    done:
    if (state && pll.params.post_rate_change)
    pll.params.post_rate_change();
    return ret;
    }
    static int clk_pll_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    struct tegra_clk_pll_freq_table cfg, old_cfg;
    let mut flags: c_ulong = 0;
    let mut ret: c_int = 0;
    if (pll.params.flags & TEGRA_PLL_FIXED) {
    if (rate != pll.params.fixed_rate) {
    pr_err("%s: Can not change %s fixed rate %lu to %lu\n",
    __func__, clk_hw_get_name(hw),
    pll.params.fixed_rate, rate);
    return -EINVAL;
    }
    return 0;
    }
    if (_get_table_rate(hw, &cfg, rate, parent_rate) &&
    pll.params.calc_rate(hw, &cfg, rate, parent_rate)) {
    pr_err("%s: Failed to set %s rate %lu\n", __func__,
    clk_hw_get_name(hw), rate);
    WARN_ON(1);
    return -EINVAL;
    }
    if (pll.lock)
    spin_lock_irqsave(pll.lock, flags);
    _get_pll_mnp(pll, &old_cfg);
    if (pll.params.flags & TEGRA_PLL_VCO_OUT)
    cfg.p = old_cfg.p;
    if (old_cfg.m != cfg.m || old_cfg.n != cfg.n || old_cfg.p != cfg.p ||
    old_cfg.sdm_data != cfg.sdm_data)
    ret = _program_pll(hw, &cfg, rate);
    if (pll.lock)
    spin_unlock_irqrestore(pll.lock, flags);
    return ret;
    }
    static int clk_pll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    struct tegra_clk_pll_freq_table cfg;
    if (pll.params.flags & TEGRA_PLL_FIXED) {
// PLLM/MB are used for memory; we do not change rate
    if (pll.params.flags & (TEGRA_PLLM | TEGRA_PLLMB))
    req.rate = clk_hw_get_rate(hw);
    else
    req.rate = pll.params.fixed_rate;
    return 0;
    }
    if (_get_table_rate(hw, &cfg, req.rate, req.best_parent_rate) &&
    pll.params.calc_rate(hw, &cfg, req.rate, req.best_parent_rate))
    return -EINVAL;
    req.rate = cfg.output_rate;
    return 0;
    }
    static unsigned long clk_pll_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    struct tegra_clk_pll_freq_table cfg;
    u32 val;
    let mut rate: u64 = parent_rate;
    int pdiv;
    val = pll_readl_base(pll);
    if ((pll.params.flags & TEGRA_PLL_BYPASS) && (val & PLL_BASE_BYPASS))
    return parent_rate;
    if ((pll.params.flags & TEGRA_PLL_FIXED) &&
    !(pll.params.flags & (TEGRA_PLLM | TEGRA_PLLMB)) &&
    !(val & PLL_BASE_OVERRIDE)) {
    struct tegra_clk_pll_freq_table sel;
    if (_get_table_rate(hw, &sel, pll.params.fixed_rate,
    parent_rate)) {
    pr_err("Clock %s has unknown fixed frequency\n",
    clk_hw_get_name(hw));
    BUG();
    }
    return pll.params.fixed_rate;
    }
    _get_pll_mnp(pll, &cfg);
    if (pll.params.flags & TEGRA_PLL_VCO_OUT) {
    pdiv = 1;
    } else {
    pdiv = _hw_to_p_div(hw, cfg.p);
    if (pdiv < 0) {
    WARN(1, "Clock %s has invalid pdiv value : 0x%x\n",
    clk_hw_get_name(hw), cfg.p);
    pdiv = 1;
    }
    }
    if (pll.params.set_gain)
    pll.params.set_gain(&cfg);
    cfg.m *= pdiv;
    rate *= cfg.n;
    do_div(rate, cfg.m);
    return rate;
    }
#[no_mangle]
unsafe extern "C" fn clk_plle_training(pll: *mut tegra_clk_pll) -> c_int {
    static int clk_plle_training(struct tegra_clk_pll *pll)
    {
    u32 val;
    unsigned long timeout;
    if (!pll.pmc)
    return -ENOSYS;
//
// PLLE is already disabled, and setup cleared;
// create falling edge on PLLE IDDQ input.
//
    val = readl(pll.pmc + PMC_SATA_PWRGT);
    val |= PMC_SATA_PWRGT_PLLE_IDDQ_VALUE;
    writel(val, pll.pmc + PMC_SATA_PWRGT);
    val = readl(pll.pmc + PMC_SATA_PWRGT);
    val |= PMC_SATA_PWRGT_PLLE_IDDQ_SWCTL;
    writel(val, pll.pmc + PMC_SATA_PWRGT);
    val = readl(pll.pmc + PMC_SATA_PWRGT);
    val &= ~PMC_SATA_PWRGT_PLLE_IDDQ_VALUE;
    writel(val, pll.pmc + PMC_SATA_PWRGT);
    val = pll_readl_misc(pll);
    timeout = jiffies + msecs_to_jiffies(100);
    while (1) {
    val = pll_readl_misc(pll);
    if (val & PLLE_MISC_READY)
    break;
    if (time_after(jiffies, timeout)) {
    pr_err("%s: timeout waiting for PLLE\n", __func__);
    return -EBUSY;
    }
    udelay(300);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_plle_enable(hw: *mut clk_hw) -> c_int {
    static int clk_plle_enable(struct clk_hw *hw)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    struct tegra_clk_pll_freq_table sel;
    unsigned long input_rate;
    u32 val;
    int err;
    if (clk_pll_is_enabled(hw))
    return 0;
    input_rate = clk_hw_get_rate(clk_hw_get_parent(hw));
    if (_get_table_rate(hw, &sel, pll.params.fixed_rate, input_rate))
    return -EINVAL;
    clk_pll_disable(hw);
    val = pll_readl_misc(pll);
    val &= ~(PLLE_MISC_LOCK_ENABLE | PLLE_MISC_SETUP_MASK);
    pll_writel_misc(val, pll);
    val = pll_readl_misc(pll);
    if (!(val & PLLE_MISC_READY)) {
    err = clk_plle_training(pll);
    if (err)
    return err;
    }
    if (pll.params.flags & TEGRA_PLLE_CONFIGURE) {
// configure dividers
    val = pll_readl_base(pll);
    val &= ~(divp_mask_shifted(pll) | divn_mask_shifted(pll) |
    divm_mask_shifted(pll));
    val &= ~(PLLE_BASE_DIVCML_MASK << PLLE_BASE_DIVCML_SHIFT);
    val |= sel.m << divm_shift(pll);
    val |= sel.n << divn_shift(pll);
    val |= sel.p << divp_shift(pll);
    val |= sel.cpcon << PLLE_BASE_DIVCML_SHIFT;
    pll_writel_base(val, pll);
    }
    val = pll_readl_misc(pll);
    val |= PLLE_MISC_SETUP_VALUE;
    val |= PLLE_MISC_LOCK_ENABLE;
    pll_writel_misc(val, pll);
    val = readl(pll.clk_base + PLLE_SS_CTRL);
    val &= ~PLLE_SS_COEFFICIENTS_MASK;
    val |= PLLE_SS_DISABLE;
    writel(val, pll.clk_base + PLLE_SS_CTRL);
    val = pll_readl_base(pll);
    val |= (PLL_BASE_BYPASS | PLL_BASE_ENABLE);
    pll_writel_base(val, pll);
    clk_pll_wait_for_lock(pll);
    return 0;
    }
    static unsigned long clk_plle_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    let mut val: u32 = pll_readl_base(pll);
    let mut divn: u32 = 0, divm = 0, divp = 0;
    let mut rate: u64 = parent_rate;
    divp = (val >> pll.params.div_nmp.divp_shift) & (divp_mask(pll));
    divn = (val >> pll.params.div_nmp.divn_shift) & (divn_mask(pll));
    divm = (val >> pll.params.div_nmp.divm_shift) & (divm_mask(pll));
    divm *= divp;
    rate *= divn;
    do_div(rate, divm);
    return rate;
    }
#[no_mangle]
unsafe extern "C" fn tegra_clk_pll_restore_context(hw: *mut clk_hw) {
    static void tegra_clk_pll_restore_context(struct clk_hw *hw)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    struct clk_hw *parent = clk_hw_get_parent(hw);
    let mut parent_rate: c_ulong = clk_hw_get_rate(parent);
    let mut rate: c_ulong = clk_hw_get_rate(hw);
    if (clk_pll_is_enabled(hw))
    return;
    if (pll.params.set_defaults)
    pll.params.set_defaults(pll);
    clk_pll_set_rate(hw, rate, parent_rate);
    if (!__clk_get_enable_count(hw.clk))
    clk_pll_disable(hw);
    else
    clk_pll_enable(hw);
    }
    const struct clk_ops tegra_clk_pll_ops = {
    .is_enabled = clk_pll_is_enabled,
    .enable = clk_pll_enable,
    .disable = clk_pll_disable,
    .recalc_rate = clk_pll_recalc_rate,
    .determine_rate = clk_pll_determine_rate,
    .set_rate = clk_pll_set_rate,
    .restore_context = tegra_clk_pll_restore_context,
    };
    const struct clk_ops tegra_clk_plle_ops = {
    .recalc_rate = clk_plle_recalc_rate,
    .is_enabled = clk_pll_is_enabled,
    .disable = clk_pll_disable,
    .enable = clk_plle_enable,
    };
//
// Structure defining the fields for USB UTMI clocks Parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct utmi_clk_param {
// Oscillator Frequency in Hz
    pub osc_frequency: u32,
// UTMIP PLL Enable Delay Count
    pub enable_delay_count: u8,
// UTMIP PLL Stable count
    pub stable_count: u8,
// UTMIP PLL Active delay count
    pub active_delay_count: u8,
// UTMIP PLL Xtal frequency count
    pub xtal_freq_count: u8,
}

    static const struct utmi_clk_param utmi_parameters[] = {
    {
    .osc_frequency = 13000000, .enable_delay_count = 0x02,
    .stable_count = 0x33, .active_delay_count = 0x05,
    .xtal_freq_count = 0x7f
    }, {
    .osc_frequency = 19200000, .enable_delay_count = 0x03,
    .stable_count = 0x4b, .active_delay_count = 0x06,
    .xtal_freq_count = 0xbb
    }, {
    .osc_frequency = 12000000, .enable_delay_count = 0x02,
    .stable_count = 0x2f, .active_delay_count = 0x04,
    .xtal_freq_count = 0x76
    }, {
    .osc_frequency = 26000000, .enable_delay_count = 0x04,
    .stable_count = 0x66, .active_delay_count = 0x09,
    .xtal_freq_count = 0xfe
    }, {
    .osc_frequency = 16800000, .enable_delay_count = 0x03,
    .stable_count = 0x41, .active_delay_count = 0x0a,
    .xtal_freq_count = 0xa4
    }, {
    .osc_frequency = 38400000, .enable_delay_count = 0x0,
    .stable_count = 0x0, .active_delay_count = 0x6,
    .xtal_freq_count = 0x80
    },
    };
#[no_mangle]
unsafe extern "C" fn clk_pllu_enable(hw: *mut clk_hw) -> c_int {
    static int clk_pllu_enable(struct clk_hw *hw)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    struct clk_hw *pll_ref = clk_hw_get_parent(hw);
    struct clk_hw *osc = clk_hw_get_parent(pll_ref);
    const struct utmi_clk_param *params = core::ptr::null_mut();
    let mut flags: c_ulong = 0, input_rate;
    unsigned int i;
    let mut ret: c_int = 0;
    u32 value;
    if (!osc) {
    pr_err("%s: failed to get OSC clock\n", __func__);
    return -EINVAL;
    }
    input_rate = clk_hw_get_rate(osc);
    if (pll.lock)
    spin_lock_irqsave(pll.lock, flags);
    if (!clk_pll_is_enabled(hw))
    _clk_pll_enable(hw);
    ret = clk_pll_wait_for_lock(pll);
    if (ret < 0)
    goto out;
    for (i = 0; i < ARRAY_SIZE(utmi_parameters); i++) {
    if (input_rate == utmi_parameters[i].osc_frequency) {
    params = &utmi_parameters[i];
    break;
    }
    }
    if (!params) {
    pr_err("%s: unexpected input rate %lu Hz\n", __func__,
    input_rate);
    ret = -EINVAL;
    goto out;
    }
    value = pll_readl_base(pll);
    value &= ~PLLU_BASE_OVERRIDE;
    pll_writel_base(value, pll);
    value = readl_relaxed(pll.clk_base + UTMIP_PLL_CFG2);
// Program UTMIP PLL stable and active counts
    value &= ~UTMIP_PLL_CFG2_STABLE_COUNT(~0);
    value |= UTMIP_PLL_CFG2_STABLE_COUNT(params.stable_count);
    value &= ~UTMIP_PLL_CFG2_ACTIVE_DLY_COUNT(~0);
    value |= UTMIP_PLL_CFG2_ACTIVE_DLY_COUNT(params.active_delay_count);
// Remove power downs from UTMIP PLL control bits
    value &= ~UTMIP_PLL_CFG2_FORCE_PD_SAMP_A_POWERDOWN;
    value &= ~UTMIP_PLL_CFG2_FORCE_PD_SAMP_B_POWERDOWN;
    value &= ~UTMIP_PLL_CFG2_FORCE_PD_SAMP_C_POWERDOWN;
    writel_relaxed(value, pll.clk_base + UTMIP_PLL_CFG2);
    value = readl_relaxed(pll.clk_base + UTMIP_PLL_CFG1);
// Program UTMIP PLL delay and oscillator frequency counts
    value &= ~UTMIP_PLL_CFG1_ENABLE_DLY_COUNT(~0);
    value |= UTMIP_PLL_CFG1_ENABLE_DLY_COUNT(params.enable_delay_count);
    value &= ~UTMIP_PLL_CFG1_XTAL_FREQ_COUNT(~0);
    value |= UTMIP_PLL_CFG1_XTAL_FREQ_COUNT(params.xtal_freq_count);
// Remove power downs from UTMIP PLL control bits
    value &= ~UTMIP_PLL_CFG1_FORCE_PLL_ENABLE_POWERDOWN;
    value &= ~UTMIP_PLL_CFG1_FORCE_PLL_ACTIVE_POWERDOWN;
    value &= ~UTMIP_PLL_CFG1_FORCE_PLLU_POWERDOWN;
    writel_relaxed(value, pll.clk_base + UTMIP_PLL_CFG1);
    out:
    if (pll.lock)
    spin_unlock_irqrestore(pll.lock, flags);
    return ret;
    }
    static const struct clk_ops tegra_clk_pllu_ops = {
    .is_enabled = clk_pll_is_enabled,
    .enable = clk_pllu_enable,
    .disable = clk_pll_disable,
    .recalc_rate = clk_pll_recalc_rate,
    .determine_rate = clk_pll_determine_rate,
    .set_rate = clk_pll_set_rate,
    };
    static int _pll_fixed_mdiv(struct tegra_clk_pll_params *pll_params,
    unsigned long parent_rate)
    {
    let mut mdiv: u16 = parent_rate / pll_params.cf_min;
    if (pll_params.flags & TEGRA_MDIV_NEW)
    return (!pll_params.mdiv_default ? mdiv :
    min(mdiv, pll_params.mdiv_default));
    if (pll_params.mdiv_default)
    return pll_params.mdiv_default;
    if (parent_rate > pll_params.cf_max)
    return 2;
    else
    return 1;
    }
    static int _calc_dynamic_ramp_rate(struct clk_hw *hw,
    struct tegra_clk_pll_freq_table *cfg,
    unsigned long rate, unsigned long parent_rate)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    unsigned int p;
    int p_div;
    if (!rate)
    return -EINVAL;
    p = DIV_ROUND_UP(pll.params.vco_min, rate);
    cfg.m = _pll_fixed_mdiv(pll.params, parent_rate);
    cfg.output_rate = rate * p;
    cfg.n = cfg.output_rate * cfg.m / parent_rate;
    cfg.input_rate = parent_rate;
    p_div = _p_div_to_hw(hw, p);
    if (p_div < 0)
    return p_div;
    cfg.p = p_div;
    if (cfg.n > divn_max(pll) || cfg.output_rate > pll.params.vco_max)
    return -EINVAL;
    return 0;
    }

    defined(CONFIG_ARCH_TEGRA_124_SOC) || \
    defined(CONFIG_ARCH_TEGRA_132_SOC) || \
    defined(CONFIG_ARCH_TEGRA_210_SOC)
#[no_mangle]
pub unsafe extern "C" fn tegra_pll_get_fixed_mdiv(hw: *mut clk_hw, input_rate: c_ulong) -> u16 {
    u16 tegra_pll_get_fixed_mdiv(struct clk_hw *hw, unsigned long input_rate)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    return (u16)_pll_fixed_mdiv(pll.params, input_rate);
    }
    static unsigned long _clip_vco_min(unsigned long vco_min,
    unsigned long parent_rate)
    {
    return DIV_ROUND_UP(vco_min, parent_rate) * parent_rate;
    }
    static int _setup_dynamic_ramp(struct tegra_clk_pll_params *pll_params,
    void __iomem *clk_base,
    unsigned long parent_rate)
    {
    u32 val;
    u32 step_a, step_b;
    switch (parent_rate) {
    case 12000000:
    case 13000000:
    case 26000000:
    step_a = 0x2B;
    step_b = 0x0B;
    break;
    case 16800000:
    step_a = 0x1A;
    step_b = 0x09;
    break;
    case 19200000:
    step_a = 0x12;
    step_b = 0x08;
    break;
    default:
    pr_err("%s: Unexpected reference rate %lu\n",
    __func__, parent_rate);
    WARN_ON(1);
    return -EINVAL;
    }
    val = step_a << pll_params.stepa_shift;
    val |= step_b << pll_params.stepb_shift;
    writel_relaxed(val, clk_base + pll_params.dyn_ramp_reg);
    return 0;
    }
    static int _pll_ramp_calc_pll(struct clk_hw *hw,
    struct tegra_clk_pll_freq_table *cfg,
    unsigned long rate, unsigned long parent_rate)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    let mut err: c_int = 0;
    err = _get_table_rate(hw, cfg, rate, parent_rate);
    if (err < 0)
    err = _calc_dynamic_ramp_rate(hw, cfg, rate, parent_rate);
    else {
    if (cfg.m != _pll_fixed_mdiv(pll.params, parent_rate)) {
    WARN_ON(1);
    err = -EINVAL;
    goto out;
    }
    }
    if (cfg.p >  pll.params.max_p)
    err = -EINVAL;
    out:
    return err;
    }
    static int clk_pllxc_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    struct tegra_clk_pll_freq_table cfg, old_cfg;
    let mut flags: c_ulong = 0;
    int ret;
    ret = _pll_ramp_calc_pll(hw, &cfg, rate, parent_rate);
    if (ret < 0)
    return ret;
    if (pll.lock)
    spin_lock_irqsave(pll.lock, flags);
    _get_pll_mnp(pll, &old_cfg);
    if (pll.params.flags & TEGRA_PLL_VCO_OUT)
    cfg.p = old_cfg.p;
    if (old_cfg.m != cfg.m || old_cfg.n != cfg.n || old_cfg.p != cfg.p)
    ret = _program_pll(hw, &cfg, rate);
    if (pll.lock)
    spin_unlock_irqrestore(pll.lock, flags);
    return ret;
    }
    static int clk_pll_ramp_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    struct tegra_clk_pll_freq_table cfg;
    int ret, p_div;
    let mut output_rate: u64 = req.best_parent_rate;
    ret = _pll_ramp_calc_pll(hw, &cfg, req.rate, req.best_parent_rate);
    if (ret < 0)
    return ret;
    p_div = _hw_to_p_div(hw, cfg.p);
    if (p_div < 0)
    return p_div;
    if (pll.params.set_gain)
    pll.params.set_gain(&cfg);
    output_rate *= cfg.n;
    do_div(output_rate, cfg.m * p_div);
    req.rate = output_rate;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn _pllcx_strobe(pll: *mut tegra_clk_pll) {
    static void _pllcx_strobe(struct tegra_clk_pll *pll)
    {
    u32 val;
    val = pll_readl_misc(pll);
    val |= PLLCX_MISC_STROBE;
    pll_writel_misc(val, pll);
    udelay(2);
    val &= ~PLLCX_MISC_STROBE;
    pll_writel_misc(val, pll);
    }
#[no_mangle]
unsafe extern "C" fn clk_pllc_enable(hw: *mut clk_hw) -> c_int {
    static int clk_pllc_enable(struct clk_hw *hw)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    u32 val;
    int ret;
    let mut flags: c_ulong = 0;
    if (clk_pll_is_enabled(hw))
    return 0;
    if (pll.lock)
    spin_lock_irqsave(pll.lock, flags);
    _clk_pll_enable(hw);
    udelay(2);
    val = pll_readl_misc(pll);
    val &= ~PLLCX_MISC_RESET;
    pll_writel_misc(val, pll);
    udelay(2);
    _pllcx_strobe(pll);
    ret = clk_pll_wait_for_lock(pll);
    if (pll.lock)
    spin_unlock_irqrestore(pll.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn _clk_pllc_disable(hw: *mut clk_hw) {
    static void _clk_pllc_disable(struct clk_hw *hw)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    u32 val;
    _clk_pll_disable(hw);
    val = pll_readl_misc(pll);
    val |= PLLCX_MISC_RESET;
    pll_writel_misc(val, pll);
    udelay(2);
    }
#[no_mangle]
unsafe extern "C" fn clk_pllc_disable(hw: *mut clk_hw) {
    static void clk_pllc_disable(struct clk_hw *hw)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    let mut flags: c_ulong = 0;
    if (pll.lock)
    spin_lock_irqsave(pll.lock, flags);
    _clk_pllc_disable(hw);
    if (pll.lock)
    spin_unlock_irqrestore(pll.lock, flags);
    }
    static int _pllcx_update_dynamic_coef(struct tegra_clk_pll *pll,
    unsigned long input_rate, u32 n)
    {
    u32 val, n_threshold;
    switch (input_rate) {
    case 12000000:
    n_threshold = 70;
    break;
    case 13000000:
    case 26000000:
    n_threshold = 71;
    break;
    case 16800000:
    n_threshold = 55;
    break;
    case 19200000:
    n_threshold = 48;
    break;
    default:
    pr_err("%s: Unexpected reference rate %lu\n",
    __func__, input_rate);
    return -EINVAL;
    }
    val = pll_readl_misc(pll);
    val &= ~(PLLCX_MISC_SDM_DIV_MASK | PLLCX_MISC_FILT_DIV_MASK);
    val |= n <= n_threshold ?
    PLLCX_MISC_DIV_LOW_RANGE : PLLCX_MISC_DIV_HIGH_RANGE;
    pll_writel_misc(val, pll);
    return 0;
    }
    static int clk_pllc_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct tegra_clk_pll_freq_table cfg, old_cfg;
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    let mut flags: c_ulong = 0;
    int state, ret = 0;
    if (pll.lock)
    spin_lock_irqsave(pll.lock, flags);
    ret = _pll_ramp_calc_pll(hw, &cfg, rate, parent_rate);
    if (ret < 0)
    goto out;
    _get_pll_mnp(pll, &old_cfg);
    if (cfg.m != old_cfg.m) {
    WARN_ON(1);
    goto out;
    }
    if (old_cfg.n == cfg.n && old_cfg.p == cfg.p)
    goto out;
    state = clk_pll_is_enabled(hw);
    if (state)
    _clk_pllc_disable(hw);
    ret = _pllcx_update_dynamic_coef(pll, parent_rate, cfg.n);
    if (ret < 0)
    goto out;
    _update_pll_mnp(pll, &cfg);
    if (state)
    ret = clk_pllc_enable(hw);
    out:
    if (pll.lock)
    spin_unlock_irqrestore(pll.lock, flags);
    return ret;
    }
    static long _pllre_calc_rate(struct tegra_clk_pll *pll,
    struct tegra_clk_pll_freq_table *cfg,
    unsigned long rate, unsigned long parent_rate)
    {
    u16 m, n;
    let mut output_rate: u64 = parent_rate;
    m = _pll_fixed_mdiv(pll.params, parent_rate);
    n = rate * m / parent_rate;
    output_rate *= n;
    do_div(output_rate, m);
    if (cfg) {
    cfg.m = m;
    cfg.n = n;
    }
    return output_rate;
    }
    static int clk_pllre_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct tegra_clk_pll_freq_table cfg, old_cfg;
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    let mut flags: c_ulong = 0;
    int state, ret = 0;
    if (pll.lock)
    spin_lock_irqsave(pll.lock, flags);
    _pllre_calc_rate(pll, &cfg, rate, parent_rate);
    _get_pll_mnp(pll, &old_cfg);
    cfg.p = old_cfg.p;
    if (cfg.m != old_cfg.m || cfg.n != old_cfg.n) {
    state = clk_pll_is_enabled(hw);
    if (state)
    _clk_pll_disable(hw);
    _update_pll_mnp(pll, &cfg);
    if (state) {
    _clk_pll_enable(hw);
    ret = clk_pll_wait_for_lock(pll);
    }
    }
    if (pll.lock)
    spin_unlock_irqrestore(pll.lock, flags);
    return ret;
    }
    static unsigned long clk_pllre_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct tegra_clk_pll_freq_table cfg;
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    let mut rate: u64 = parent_rate;
    _get_pll_mnp(pll, &cfg);
    rate *= cfg.n;
    do_div(rate, cfg.m);
    return rate;
    }
    static int clk_pllre_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    req.rate = _pllre_calc_rate(pll, core::ptr::null_mut(), req.rate,
    req.best_parent_rate);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_plle_tegra114_enable(hw: *mut clk_hw) -> c_int {
    static int clk_plle_tegra114_enable(struct clk_hw *hw)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    struct tegra_clk_pll_freq_table sel;
    u32 val;
    int ret;
    let mut flags: c_ulong = 0;
    unsigned long input_rate;
    input_rate = clk_hw_get_rate(clk_hw_get_parent(hw));
    if (_get_table_rate(hw, &sel, pll.params.fixed_rate, input_rate))
    return -EINVAL;
    if (pll.lock)
    spin_lock_irqsave(pll.lock, flags);
    val = pll_readl_base(pll);
    val &= ~BIT(29); /* Disable lock override */
    pll_writel_base(val, pll);
    val = pll_readl(pll.params.aux_reg, pll);
    val |= PLLE_AUX_ENABLE_SWCTL;
    val &= ~PLLE_AUX_SEQ_ENABLE;
    pll_writel(val, pll.params.aux_reg, pll);
    udelay(1);
    val = pll_readl_misc(pll);
    val |= PLLE_MISC_LOCK_ENABLE;
    val |= PLLE_MISC_IDDQ_SW_CTRL;
    val &= ~PLLE_MISC_IDDQ_SW_VALUE;
    val |= PLLE_MISC_PLLE_PTS;
    val &= ~(PLLE_MISC_VREG_BG_CTRL_MASK | PLLE_MISC_VREG_CTRL_MASK);
    pll_writel_misc(val, pll);
    udelay(5);
    val = pll_readl(PLLE_SS_CTRL, pll);
    val |= PLLE_SS_DISABLE;
    pll_writel(val, PLLE_SS_CTRL, pll);
    val = pll_readl_base(pll);
    val &= ~(divp_mask_shifted(pll) | divn_mask_shifted(pll) |
    divm_mask_shifted(pll));
    val &= ~(PLLE_BASE_DIVCML_MASK << PLLE_BASE_DIVCML_SHIFT);
    val |= sel.m << divm_shift(pll);
    val |= sel.n << divn_shift(pll);
    val |= sel.cpcon << PLLE_BASE_DIVCML_SHIFT;
    pll_writel_base(val, pll);
    udelay(1);
    _clk_pll_enable(hw);
    ret = clk_pll_wait_for_lock(pll);
    if (ret < 0)
    goto out;
    val = pll_readl(PLLE_SS_CTRL, pll);
    val &= ~(PLLE_SS_CNTL_CENTER | PLLE_SS_CNTL_INVERT);
    val &= ~PLLE_SS_COEFFICIENTS_MASK;
    val |= PLLE_SS_COEFFICIENTS_VAL_TEGRA114;
    pll_writel(val, PLLE_SS_CTRL, pll);
    val &= ~(PLLE_SS_CNTL_SSC_BYP | PLLE_SS_CNTL_BYPASS_SS);
    pll_writel(val, PLLE_SS_CTRL, pll);
    udelay(1);
    val &= ~PLLE_SS_CNTL_INTERP_RESET;
    pll_writel(val, PLLE_SS_CTRL, pll);
    udelay(1);
// Enable HW control of XUSB brick PLL
    val = pll_readl_misc(pll);
    val &= ~PLLE_MISC_IDDQ_SW_CTRL;
    pll_writel_misc(val, pll);
    val = pll_readl(pll.params.aux_reg, pll);
    val |= (PLLE_AUX_USE_LOCKDET | PLLE_AUX_SEQ_START_STATE);
    val &= ~(PLLE_AUX_ENABLE_SWCTL | PLLE_AUX_SS_SWCTL);
    pll_writel(val, pll.params.aux_reg, pll);
    udelay(1);
    val |= PLLE_AUX_SEQ_ENABLE;
    pll_writel(val, pll.params.aux_reg, pll);
    val = pll_readl(XUSBIO_PLL_CFG0, pll);
    val |= (XUSBIO_PLL_CFG0_PADPLL_USE_LOCKDET |
    XUSBIO_PLL_CFG0_SEQ_START_STATE);
    val &= ~(XUSBIO_PLL_CFG0_CLK_ENABLE_SWCTL |
    XUSBIO_PLL_CFG0_PADPLL_RESET_SWCTL);
    pll_writel(val, XUSBIO_PLL_CFG0, pll);
    udelay(1);
    val |= XUSBIO_PLL_CFG0_SEQ_ENABLE;
    pll_writel(val, XUSBIO_PLL_CFG0, pll);
// Enable HW control of SATA PLL
    val = pll_readl(SATA_PLL_CFG0, pll);
    val &= ~SATA_PLL_CFG0_PADPLL_RESET_SWCTL;
    val |= SATA_PLL_CFG0_PADPLL_USE_LOCKDET;
    val |= SATA_PLL_CFG0_SEQ_START_STATE;
    pll_writel(val, SATA_PLL_CFG0, pll);
    udelay(1);
    val = pll_readl(SATA_PLL_CFG0, pll);
    val |= SATA_PLL_CFG0_SEQ_ENABLE;
    pll_writel(val, SATA_PLL_CFG0, pll);
    out:
    if (pll.lock)
    spin_unlock_irqrestore(pll.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn clk_plle_tegra114_disable(hw: *mut clk_hw) {
    static void clk_plle_tegra114_disable(struct clk_hw *hw)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    let mut flags: c_ulong = 0;
    u32 val;
    if (pll.lock)
    spin_lock_irqsave(pll.lock, flags);
    _clk_pll_disable(hw);
    val = pll_readl_misc(pll);
    val |= PLLE_MISC_IDDQ_SW_CTRL | PLLE_MISC_IDDQ_SW_VALUE;
    pll_writel_misc(val, pll);
    udelay(1);
    if (pll.lock)
    spin_unlock_irqrestore(pll.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn clk_pllu_tegra114_enable(hw: *mut clk_hw) -> c_int {
    static int clk_pllu_tegra114_enable(struct clk_hw *hw)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    const struct utmi_clk_param *params = core::ptr::null_mut();
    struct clk *osc = __clk_lookup("osc");
    let mut flags: c_ulong = 0, input_rate;
    unsigned int i;
    let mut ret: c_int = 0;
    u32 value;
    if (!osc) {
    pr_err("%s: failed to get OSC clock\n", __func__);
    return -EINVAL;
    }
    input_rate = clk_hw_get_rate(__clk_get_hw(osc));
    if (pll.lock)
    spin_lock_irqsave(pll.lock, flags);
    if (!clk_pll_is_enabled(hw))
    _clk_pll_enable(hw);
    ret = clk_pll_wait_for_lock(pll);
    if (ret < 0)
    goto out;
    for (i = 0; i < ARRAY_SIZE(utmi_parameters); i++) {
    if (input_rate == utmi_parameters[i].osc_frequency) {
    params = &utmi_parameters[i];
    break;
    }
    }
    if (!params) {
    pr_err("%s: unexpected input rate %lu Hz\n", __func__,
    input_rate);
    ret = -EINVAL;
    goto out;
    }
    value = pll_readl_base(pll);
    value &= ~PLLU_BASE_OVERRIDE;
    pll_writel_base(value, pll);
    value = readl_relaxed(pll.clk_base + UTMIP_PLL_CFG2);
// Program UTMIP PLL stable and active counts
    value &= ~UTMIP_PLL_CFG2_STABLE_COUNT(~0);
    value |= UTMIP_PLL_CFG2_STABLE_COUNT(params.stable_count);
    value &= ~UTMIP_PLL_CFG2_ACTIVE_DLY_COUNT(~0);
    value |= UTMIP_PLL_CFG2_ACTIVE_DLY_COUNT(params.active_delay_count);
// Remove power downs from UTMIP PLL control bits
    value &= ~UTMIP_PLL_CFG2_FORCE_PD_SAMP_A_POWERDOWN;
    value &= ~UTMIP_PLL_CFG2_FORCE_PD_SAMP_B_POWERDOWN;
    value &= ~UTMIP_PLL_CFG2_FORCE_PD_SAMP_C_POWERDOWN;
    writel_relaxed(value, pll.clk_base + UTMIP_PLL_CFG2);
    value = readl_relaxed(pll.clk_base + UTMIP_PLL_CFG1);
// Program UTMIP PLL delay and oscillator frequency counts
    value &= ~UTMIP_PLL_CFG1_ENABLE_DLY_COUNT(~0);
    value |= UTMIP_PLL_CFG1_ENABLE_DLY_COUNT(params.enable_delay_count);
    value &= ~UTMIP_PLL_CFG1_XTAL_FREQ_COUNT(~0);
    value |= UTMIP_PLL_CFG1_XTAL_FREQ_COUNT(params.xtal_freq_count);
// Remove power downs from UTMIP PLL control bits
    value &= ~UTMIP_PLL_CFG1_FORCE_PLL_ENABLE_POWERDOWN;
    value &= ~UTMIP_PLL_CFG1_FORCE_PLL_ACTIVE_POWERDOWN;
    value &= ~UTMIP_PLL_CFG1_FORCE_PLLU_POWERUP;
    value &= ~UTMIP_PLL_CFG1_FORCE_PLLU_POWERDOWN;
    writel_relaxed(value, pll.clk_base + UTMIP_PLL_CFG1);
// Setup HW control of UTMIPLL
    value = readl_relaxed(pll.clk_base + UTMIPLL_HW_PWRDN_CFG0);
    value |= UTMIPLL_HW_PWRDN_CFG0_USE_LOCKDET;
    value &= ~UTMIPLL_HW_PWRDN_CFG0_CLK_ENABLE_SWCTL;
    value |= UTMIPLL_HW_PWRDN_CFG0_SEQ_START_STATE;
    writel_relaxed(value, pll.clk_base + UTMIPLL_HW_PWRDN_CFG0);
    value = readl_relaxed(pll.clk_base + UTMIP_PLL_CFG1);
    value &= ~UTMIP_PLL_CFG1_FORCE_PLL_ENABLE_POWERUP;
    value &= ~UTMIP_PLL_CFG1_FORCE_PLL_ENABLE_POWERDOWN;
    writel_relaxed(value, pll.clk_base + UTMIP_PLL_CFG1);
    udelay(1);
//
// Setup SW override of UTMIPLL assuming USB2.0 ports are assigned
// to USB2
//
    value = readl_relaxed(pll.clk_base + UTMIPLL_HW_PWRDN_CFG0);
    value |= UTMIPLL_HW_PWRDN_CFG0_IDDQ_SWCTL;
    value &= ~UTMIPLL_HW_PWRDN_CFG0_IDDQ_OVERRIDE;
    writel_relaxed(value, pll.clk_base + UTMIPLL_HW_PWRDN_CFG0);
    udelay(1);
// Enable HW control of UTMIPLL
    value = readl_relaxed(pll.clk_base + UTMIPLL_HW_PWRDN_CFG0);
    value |= UTMIPLL_HW_PWRDN_CFG0_SEQ_ENABLE;
    writel_relaxed(value, pll.clk_base + UTMIPLL_HW_PWRDN_CFG0);
    out:
    if (pll.lock)
    spin_unlock_irqrestore(pll.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn _clk_plle_tegra_init_parent(pll: *mut tegra_clk_pll) {
    static void _clk_plle_tegra_init_parent(struct tegra_clk_pll *pll)
    {
    u32 val, val_aux;
// ensure parent is set to pll_ref
    val = pll_readl_base(pll);
    val_aux = pll_readl(pll.params.aux_reg, pll);
    if (val & PLL_BASE_ENABLE) {
    if ((val_aux & PLLE_AUX_PLLRE_SEL) ||
    (val_aux & PLLE_AUX_PLLP_SEL))
    WARN(1, "pll_e enabled with unsupported parent %s\n",
    (val_aux & PLLE_AUX_PLLP_SEL) ? "pllp_out0" :
    "pll_re_vco");
    } else {
    val_aux &= ~(PLLE_AUX_PLLRE_SEL | PLLE_AUX_PLLP_SEL);
    pll_writel(val_aux, pll.params.aux_reg, pll);
    fence_udelay(1, pll.clk_base);
    }
    }

    static struct tegra_clk_pll *_tegra_init_pll(void __iomem *clk_base,
    void __iomem *pmc, struct tegra_clk_pll_params *pll_params,
    spinlock_t *lock)
    {
    struct tegra_clk_pll *pll;
    pll = kzalloc_obj(*pll);
    if (!pll)
    return ERR_PTR(-ENOMEM);
    pll.clk_base = clk_base;
    pll.pmc = pmc;
    pll.params = pll_params;
    pll.lock = lock;
    if (!pll_params.div_nmp)
    pll_params.div_nmp = &default_nmp;
    return pll;
    }
    static struct clk *_tegra_clk_register_pll(struct tegra_clk_pll *pll,
    const char *name, const char *parent_name, unsigned long flags,
    const struct clk_ops *ops)
    {
    struct clk_init_data init;
    init.name = name;
    init.ops = ops;
    init.flags = flags;
    init.parent_names = (parent_name ? &parent_name : core::ptr::null_mut());
    init.num_parents = (parent_name ? 1 : 0);
// Default to _calc_rate if unspecified
    if (!pll.params.calc_rate) {
    if (pll.params.flags & TEGRA_PLLM)
    pll.params.calc_rate = _calc_dynamic_ramp_rate;
    else
    pll.params.calc_rate = _calc_rate;
    }
    if (pll.params.set_defaults)
    pll.params.set_defaults(pll);
// Data in .init is copied by clk_register(), so stack variable OK
    pll.hw.init = &init;
    return tegra_clk_dev_register(&pll.hw);
    }
    struct clk *tegra_clk_register_pll(const char *name, const char *parent_name,
    void __iomem *clk_base, void __iomem *pmc,
    unsigned long flags, struct tegra_clk_pll_params *pll_params,
    spinlock_t *lock)
    {
    struct tegra_clk_pll *pll;
    struct clk *clk;
    pll_params.flags |= TEGRA_PLL_BYPASS;
    pll = _tegra_init_pll(clk_base, pmc, pll_params, lock);
    if (IS_ERR(pll))
    return ERR_CAST(pll);
    clk = _tegra_clk_register_pll(pll, name, parent_name, flags,
    &tegra_clk_pll_ops);
    if (IS_ERR(clk))
    kfree(pll);
    return clk;
    }
    static struct div_nmp pll_e_nmp = {
    .divn_shift = PLLE_BASE_DIVN_SHIFT,
    .divn_width = PLLE_BASE_DIVN_WIDTH,
    .divm_shift = PLLE_BASE_DIVM_SHIFT,
    .divm_width = PLLE_BASE_DIVM_WIDTH,
    .divp_shift = PLLE_BASE_DIVP_SHIFT,
    .divp_width = PLLE_BASE_DIVP_WIDTH,
    };
    struct clk *tegra_clk_register_plle(const char *name, const char *parent_name,
    void __iomem *clk_base, void __iomem *pmc,
    unsigned long flags, struct tegra_clk_pll_params *pll_params,
    spinlock_t *lock)
    {
    struct tegra_clk_pll *pll;
    struct clk *clk;
    pll_params.flags |= TEGRA_PLL_BYPASS;
    if (!pll_params.div_nmp)
    pll_params.div_nmp = &pll_e_nmp;
    pll = _tegra_init_pll(clk_base, pmc, pll_params, lock);
    if (IS_ERR(pll))
    return ERR_CAST(pll);
    clk = _tegra_clk_register_pll(pll, name, parent_name, flags,
    &tegra_clk_plle_ops);
    if (IS_ERR(clk))
    kfree(pll);
    return clk;
    }
    struct clk *tegra_clk_register_pllu(const char *name, const char *parent_name,
    void __iomem *clk_base, unsigned long flags,
    struct tegra_clk_pll_params *pll_params, spinlock_t *lock)
    {
    struct tegra_clk_pll *pll;
    struct clk *clk;
    pll_params.flags |= TEGRA_PLLU;
    pll = _tegra_init_pll(clk_base, core::ptr::null_mut(), pll_params, lock);
    if (IS_ERR(pll))
    return ERR_CAST(pll);
    clk = _tegra_clk_register_pll(pll, name, parent_name, flags,
    &tegra_clk_pllu_ops);
    if (IS_ERR(clk))
    kfree(pll);
    return clk;
    }

    defined(CONFIG_ARCH_TEGRA_124_SOC) || \
    defined(CONFIG_ARCH_TEGRA_132_SOC) || \
    defined(CONFIG_ARCH_TEGRA_210_SOC)
    static const struct clk_ops tegra_clk_pllxc_ops = {
    .is_enabled = clk_pll_is_enabled,
    .enable = clk_pll_enable,
    .disable = clk_pll_disable,
    .recalc_rate = clk_pll_recalc_rate,
    .determine_rate = clk_pll_ramp_determine_rate,
    .set_rate = clk_pllxc_set_rate,
    };
    static const struct clk_ops tegra_clk_pllc_ops = {
    .is_enabled = clk_pll_is_enabled,
    .enable = clk_pllc_enable,
    .disable = clk_pllc_disable,
    .recalc_rate = clk_pll_recalc_rate,
    .determine_rate = clk_pll_ramp_determine_rate,
    .set_rate = clk_pllc_set_rate,
    };
    static const struct clk_ops tegra_clk_pllre_ops = {
    .is_enabled = clk_pll_is_enabled,
    .enable = clk_pll_enable,
    .disable = clk_pll_disable,
    .recalc_rate = clk_pllre_recalc_rate,
    .determine_rate = clk_pllre_determine_rate,
    .set_rate = clk_pllre_set_rate,
    };
    static const struct clk_ops tegra_clk_plle_tegra114_ops = {
    .is_enabled =  clk_pll_is_enabled,
    .enable = clk_plle_tegra114_enable,
    .disable = clk_plle_tegra114_disable,
    .recalc_rate = clk_pll_recalc_rate,
    };
    static const struct clk_ops tegra_clk_pllu_tegra114_ops = {
    .is_enabled =  clk_pll_is_enabled,
    .enable = clk_pllu_tegra114_enable,
    .disable = clk_pll_disable,
    .recalc_rate = clk_pll_recalc_rate,
    };
    struct clk *tegra_clk_register_pllxc(const char *name, const char *parent_name,
    void __iomem *clk_base, void __iomem *pmc,
    unsigned long flags,
    struct tegra_clk_pll_params *pll_params,
    spinlock_t *lock)
    {
    struct tegra_clk_pll *pll;
    struct clk *clk, *parent;
    unsigned long parent_rate;
    u32 val, val_iddq;
    parent = __clk_lookup(parent_name);
    if (!parent) {
    WARN(1, "parent clk %s of %s must be registered first\n",
    parent_name, name);
    return ERR_PTR(-EINVAL);
    }
    if (!pll_params.pdiv_tohw)
    return ERR_PTR(-EINVAL);
    parent_rate = clk_get_rate(parent);
    pll_params.vco_min = _clip_vco_min(pll_params.vco_min, parent_rate);
    if (pll_params.adjust_vco)
    pll_params.vco_min = pll_params.adjust_vco(pll_params,
    parent_rate);
//
// If the pll has a set_defaults callback, it will take care of
// configuring dynamic ramping and setting IDDQ in that path.
//
    if (!pll_params.set_defaults) {
    int err;
    err = _setup_dynamic_ramp(pll_params, clk_base, parent_rate);
    if (err)
    return ERR_PTR(err);
    val = readl_relaxed(clk_base + pll_params.base_reg);
    val_iddq = readl_relaxed(clk_base + pll_params.iddq_reg);
    if (val & PLL_BASE_ENABLE)
    WARN_ON(val_iddq & BIT(pll_params.iddq_bit_idx));
    else {
    val_iddq |= BIT(pll_params.iddq_bit_idx);
    writel_relaxed(val_iddq,
    clk_base + pll_params.iddq_reg);
    }
    }
    pll = _tegra_init_pll(clk_base, pmc, pll_params, lock);
    if (IS_ERR(pll))
    return ERR_CAST(pll);
    clk = _tegra_clk_register_pll(pll, name, parent_name, flags,
    &tegra_clk_pllxc_ops);
    if (IS_ERR(clk))
    kfree(pll);
    return clk;
    }
    struct clk *tegra_clk_register_pllre(const char *name, const char *parent_name,
    void __iomem *clk_base, void __iomem *pmc,
    unsigned long flags,
    struct tegra_clk_pll_params *pll_params,
    spinlock_t *lock, unsigned long parent_rate)
    {
    u32 val;
    struct tegra_clk_pll *pll;
    struct clk *clk;
    pll_params.vco_min = _clip_vco_min(pll_params.vco_min, parent_rate);
    if (pll_params.adjust_vco)
    pll_params.vco_min = pll_params.adjust_vco(pll_params,
    parent_rate);
    pll = _tegra_init_pll(clk_base, pmc, pll_params, lock);
    if (IS_ERR(pll))
    return ERR_CAST(pll);
// program minimum rate by default
    val = pll_readl_base(pll);
    if (val & PLL_BASE_ENABLE)
    WARN_ON(readl_relaxed(clk_base + pll_params.iddq_reg) &
    BIT(pll_params.iddq_bit_idx));
    else {
    int m;
    m = _pll_fixed_mdiv(pll_params, parent_rate);
    val = m << divm_shift(pll);
    val |= (pll_params.vco_min / parent_rate) << divn_shift(pll);
    pll_writel_base(val, pll);
    }
// disable lock override
    val = pll_readl_misc(pll);
    val &= ~BIT(29);
    pll_writel_misc(val, pll);
    clk = _tegra_clk_register_pll(pll, name, parent_name, flags,
    &tegra_clk_pllre_ops);
    if (IS_ERR(clk))
    kfree(pll);
    return clk;
    }
    struct clk *tegra_clk_register_pllm(const char *name, const char *parent_name,
    void __iomem *clk_base, void __iomem *pmc,
    unsigned long flags,
    struct tegra_clk_pll_params *pll_params,
    spinlock_t *lock)
    {
    struct tegra_clk_pll *pll;
    struct clk *clk, *parent;
    unsigned long parent_rate;
    if (!pll_params.pdiv_tohw)
    return ERR_PTR(-EINVAL);
    parent = __clk_lookup(parent_name);
    if (!parent) {
    WARN(1, "parent clk %s of %s must be registered first\n",
    parent_name, name);
    return ERR_PTR(-EINVAL);
    }
    parent_rate = clk_get_rate(parent);
    pll_params.vco_min = _clip_vco_min(pll_params.vco_min, parent_rate);
    if (pll_params.adjust_vco)
    pll_params.vco_min = pll_params.adjust_vco(pll_params,
    parent_rate);
    pll_params.flags |= TEGRA_PLL_BYPASS;
    pll_params.flags |= TEGRA_PLLM;
    pll = _tegra_init_pll(clk_base, pmc, pll_params, lock);
    if (IS_ERR(pll))
    return ERR_CAST(pll);
    clk = _tegra_clk_register_pll(pll, name, parent_name, flags,
    &tegra_clk_pll_ops);
    if (IS_ERR(clk))
    kfree(pll);
    return clk;
    }
    struct clk *tegra_clk_register_pllc(const char *name, const char *parent_name,
    void __iomem *clk_base, void __iomem *pmc,
    unsigned long flags,
    struct tegra_clk_pll_params *pll_params,
    spinlock_t *lock)
    {
    struct clk *parent, *clk;
    const struct pdiv_map *p_tohw = pll_params.pdiv_tohw;
    struct tegra_clk_pll *pll;
    struct tegra_clk_pll_freq_table cfg;
    unsigned long parent_rate;
    if (!p_tohw)
    return ERR_PTR(-EINVAL);
    parent = __clk_lookup(parent_name);
    if (!parent) {
    WARN(1, "parent clk %s of %s must be registered first\n",
    parent_name, name);
    return ERR_PTR(-EINVAL);
    }
    parent_rate = clk_get_rate(parent);
    pll_params.vco_min = _clip_vco_min(pll_params.vco_min, parent_rate);
    pll_params.flags |= TEGRA_PLL_BYPASS;
    pll = _tegra_init_pll(clk_base, pmc, pll_params, lock);
    if (IS_ERR(pll))
    return ERR_CAST(pll);
//
// Most of PLLC register fields are shadowed, and can not be read
// directly from PLL h/w. Hence, actual PLLC boot state is unknown.
// Initialize PLL to default state: disabled, reset; shadow registers
// loaded with default parameters; dividers are preset for half of
// minimum VCO rate (the latter assured that shadowed divider settings
// are within supported range).
//
    cfg.m = _pll_fixed_mdiv(pll_params, parent_rate);
    cfg.n = cfg.m * pll_params.vco_min / parent_rate;
    while (p_tohw.pdiv) {
    if (p_tohw.pdiv == 2) {
    cfg.p = p_tohw.hw_val;
    break;
    }
    p_tohw++;
    }
    if (!p_tohw.pdiv) {
    WARN_ON(1);
    return ERR_PTR(-EINVAL);
    }
    pll_writel_base(0, pll);
    _update_pll_mnp(pll, &cfg);
    pll_writel_misc(PLLCX_MISC_DEFAULT, pll);
    pll_writel(PLLCX_MISC1_DEFAULT, pll_params.ext_misc_reg[0], pll);
    pll_writel(PLLCX_MISC2_DEFAULT, pll_params.ext_misc_reg[1], pll);
    pll_writel(PLLCX_MISC3_DEFAULT, pll_params.ext_misc_reg[2], pll);
    _pllcx_update_dynamic_coef(pll, parent_rate, cfg.n);
    clk = _tegra_clk_register_pll(pll, name, parent_name, flags,
    &tegra_clk_pllc_ops);
    if (IS_ERR(clk))
    kfree(pll);
    return clk;
    }
    struct clk *tegra_clk_register_plle_tegra114(const char *name,
    const char *parent_name,
    void __iomem *clk_base, unsigned long flags,
    struct tegra_clk_pll_params *pll_params,
    spinlock_t *lock)
    {
    struct tegra_clk_pll *pll;
    struct clk *clk;
    pll = _tegra_init_pll(clk_base, core::ptr::null_mut(), pll_params, lock);
    if (IS_ERR(pll))
    return ERR_CAST(pll);
    _clk_plle_tegra_init_parent(pll);
    clk = _tegra_clk_register_pll(pll, name, parent_name, flags,
    &tegra_clk_plle_tegra114_ops);
    if (IS_ERR(clk))
    kfree(pll);
    return clk;
    }
    struct clk *
    tegra_clk_register_pllu_tegra114(const char *name, const char *parent_name,
    void __iomem *clk_base, unsigned long flags,
    struct tegra_clk_pll_params *pll_params,
    spinlock_t *lock)
    {
    struct tegra_clk_pll *pll;
    struct clk *clk;
    pll_params.flags |= TEGRA_PLLU;
    pll = _tegra_init_pll(clk_base, core::ptr::null_mut(), pll_params, lock);
    if (IS_ERR(pll))
    return ERR_CAST(pll);
    clk = _tegra_clk_register_pll(pll, name, parent_name, flags,
    &tegra_clk_pllu_tegra114_ops);
    if (IS_ERR(clk))
    kfree(pll);
    return clk;
    }

    static const struct clk_ops tegra_clk_pllss_ops = {
    .is_enabled = clk_pll_is_enabled,
    .enable = clk_pll_enable,
    .disable = clk_pll_disable,
    .recalc_rate = clk_pll_recalc_rate,
    .determine_rate = clk_pll_ramp_determine_rate,
    .set_rate = clk_pllxc_set_rate,
    .restore_context = tegra_clk_pll_restore_context,
    };
    struct clk *tegra_clk_register_pllss(const char *name, const char *parent_name,
    void __iomem *clk_base, unsigned long flags,
    struct tegra_clk_pll_params *pll_params,
    spinlock_t *lock)
    {
    struct tegra_clk_pll *pll;
    struct clk *clk, *parent;
    struct tegra_clk_pll_freq_table cfg;
    unsigned long parent_rate;
    u32 val, val_iddq;
    int i;
    if (!pll_params.div_nmp)
    return ERR_PTR(-EINVAL);
    parent = __clk_lookup(parent_name);
    if (!parent) {
    WARN(1, "parent clk %s of %s must be registered first\n",
    parent_name, name);
    return ERR_PTR(-EINVAL);
    }
    pll = _tegra_init_pll(clk_base, core::ptr::null_mut(), pll_params, lock);
    if (IS_ERR(pll))
    return ERR_CAST(pll);
    val = pll_readl_base(pll);
    val &= ~PLLSS_REF_SRC_SEL_MASK;
    pll_writel_base(val, pll);
    parent_rate = clk_get_rate(parent);
    pll_params.vco_min = _clip_vco_min(pll_params.vco_min, parent_rate);
// initialize PLL to minimum rate
    cfg.m = _pll_fixed_mdiv(pll_params, parent_rate);
    cfg.n = cfg.m * pll_params.vco_min / parent_rate;
    for (i = 0; pll_params.pdiv_tohw[i].pdiv; i++)
    ;
    if (!i) {
    kfree(pll);
    return ERR_PTR(-EINVAL);
    }
    cfg.p = pll_params.pdiv_tohw[i-1].hw_val;
    _update_pll_mnp(pll, &cfg);
    pll_writel_misc(PLLSS_MISC_DEFAULT, pll);
    pll_writel(PLLSS_CFG_DEFAULT, pll_params.ext_misc_reg[0], pll);
    pll_writel(PLLSS_CTRL1_DEFAULT, pll_params.ext_misc_reg[1], pll);
    pll_writel(PLLSS_CTRL1_DEFAULT, pll_params.ext_misc_reg[2], pll);
    val = pll_readl_base(pll);
    val_iddq = readl_relaxed(clk_base + pll_params.iddq_reg);
    if (val & PLL_BASE_ENABLE) {
    if (val_iddq & BIT(pll_params.iddq_bit_idx)) {
    WARN(1, "%s is on but IDDQ set\n", name);
    kfree(pll);
    return ERR_PTR(-EINVAL);
    }
    } else {
    val_iddq |= BIT(pll_params.iddq_bit_idx);
    writel_relaxed(val_iddq, clk_base + pll_params.iddq_reg);
    }
    val &= ~PLLSS_LOCK_OVERRIDE;
    pll_writel_base(val, pll);
    clk = _tegra_clk_register_pll(pll, name, parent_name, flags,
    &tegra_clk_pllss_ops);
    if (IS_ERR(clk))
    kfree(pll);
    return clk;
    }

    struct clk *tegra_clk_register_pllre_tegra210(const char *name,
    const char *parent_name, void __iomem *clk_base,
    void __iomem *pmc, unsigned long flags,
    struct tegra_clk_pll_params *pll_params,
    spinlock_t *lock, unsigned long parent_rate)
    {
    struct tegra_clk_pll *pll;
    struct clk *clk;
    pll_params.vco_min = _clip_vco_min(pll_params.vco_min, parent_rate);
    if (pll_params.adjust_vco)
    pll_params.vco_min = pll_params.adjust_vco(pll_params,
    parent_rate);
    pll = _tegra_init_pll(clk_base, pmc, pll_params, lock);
    if (IS_ERR(pll))
    return ERR_CAST(pll);
    clk = _tegra_clk_register_pll(pll, name, parent_name, flags,
    &tegra_clk_pll_ops);
    if (IS_ERR(clk))
    kfree(pll);
    return clk;
    }
#[no_mangle]
unsafe extern "C" fn clk_plle_tegra210_is_enabled(hw: *mut clk_hw) -> c_int {
    static int clk_plle_tegra210_is_enabled(struct clk_hw *hw)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    u32 val;
    val = pll_readl_base(pll);
    return val & PLLE_BASE_ENABLE ? 1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_plle_tegra210_enable(hw: *mut clk_hw) -> c_int {
    static int clk_plle_tegra210_enable(struct clk_hw *hw)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    struct tegra_clk_pll_freq_table sel;
    u32 val;
    let mut ret: c_int = 0;
    let mut flags: c_ulong = 0;
    unsigned long input_rate;
    if (clk_plle_tegra210_is_enabled(hw))
    return 0;
    input_rate = clk_hw_get_rate(clk_hw_get_parent(hw));
    if (_get_table_rate(hw, &sel, pll.params.fixed_rate, input_rate))
    return -EINVAL;
    if (pll.lock)
    spin_lock_irqsave(pll.lock, flags);
    val = pll_readl(pll.params.aux_reg, pll);
    if (val & PLLE_AUX_SEQ_ENABLE)
    goto out;
    val = pll_readl_base(pll);
    val &= ~BIT(30); /* Disable lock override */
    pll_writel_base(val, pll);
    val = pll_readl_misc(pll);
    val |= PLLE_MISC_LOCK_ENABLE;
    val |= PLLE_MISC_IDDQ_SW_CTRL;
    val &= ~PLLE_MISC_IDDQ_SW_VALUE;
    val |= PLLE_MISC_PLLE_PTS;
    val &= ~(PLLE_MISC_VREG_BG_CTRL_MASK | PLLE_MISC_VREG_CTRL_MASK);
    pll_writel_misc(val, pll);
    udelay(5);
    val = pll_readl(PLLE_SS_CTRL, pll);
    val |= PLLE_SS_DISABLE;
    pll_writel(val, PLLE_SS_CTRL, pll);
    val = pll_readl_base(pll);
    val &= ~(divp_mask_shifted(pll) | divn_mask_shifted(pll) |
    divm_mask_shifted(pll));
    val &= ~(PLLE_BASE_DIVCML_MASK << PLLE_BASE_DIVCML_SHIFT);
    val |= sel.m << divm_shift(pll);
    val |= sel.n << divn_shift(pll);
    val |= sel.cpcon << PLLE_BASE_DIVCML_SHIFT;
    pll_writel_base(val, pll);
    udelay(1);
    val = pll_readl_base(pll);
    val |= PLLE_BASE_ENABLE;
    pll_writel_base(val, pll);
    ret = clk_pll_wait_for_lock(pll);
    if (ret < 0)
    goto out;
    val = pll_readl(PLLE_SS_CTRL, pll);
    val &= ~(PLLE_SS_CNTL_CENTER | PLLE_SS_CNTL_INVERT);
    val &= ~PLLE_SS_COEFFICIENTS_MASK;
    val |= PLLE_SS_COEFFICIENTS_VAL_TEGRA210;
    pll_writel(val, PLLE_SS_CTRL, pll);
    val &= ~(PLLE_SS_CNTL_SSC_BYP | PLLE_SS_CNTL_BYPASS_SS);
    pll_writel(val, PLLE_SS_CTRL, pll);
    udelay(1);
    val &= ~PLLE_SS_CNTL_INTERP_RESET;
    pll_writel(val, PLLE_SS_CTRL, pll);
    udelay(1);
    out:
    if (pll.lock)
    spin_unlock_irqrestore(pll.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn clk_plle_tegra210_disable(hw: *mut clk_hw) {
    static void clk_plle_tegra210_disable(struct clk_hw *hw)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    let mut flags: c_ulong = 0;
    u32 val;
    if (pll.lock)
    spin_lock_irqsave(pll.lock, flags);
// If PLLE HW sequencer is enabled, SW should not disable PLLE
    val = pll_readl(pll.params.aux_reg, pll);
    if (val & PLLE_AUX_SEQ_ENABLE)
    goto out;
    val = pll_readl_base(pll);
    val &= ~PLLE_BASE_ENABLE;
    pll_writel_base(val, pll);
    val = pll_readl(pll.params.aux_reg, pll);
    val |= PLLE_AUX_ENABLE_SWCTL | PLLE_AUX_SS_SWCTL;
    pll_writel(val, pll.params.aux_reg, pll);
    val = pll_readl_misc(pll);
    val |= PLLE_MISC_IDDQ_SW_CTRL | PLLE_MISC_IDDQ_SW_VALUE;
    pll_writel_misc(val, pll);
    udelay(1);
    out:
    if (pll.lock)
    spin_unlock_irqrestore(pll.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn tegra_clk_plle_t210_restore_context(hw: *mut clk_hw) {
    static void tegra_clk_plle_t210_restore_context(struct clk_hw *hw)
    {
    struct tegra_clk_pll *pll = to_clk_pll(hw);
    _clk_plle_tegra_init_parent(pll);
    }
    static const struct clk_ops tegra_clk_plle_tegra210_ops = {
    .is_enabled =  clk_plle_tegra210_is_enabled,
    .enable = clk_plle_tegra210_enable,
    .disable = clk_plle_tegra210_disable,
    .recalc_rate = clk_pll_recalc_rate,
    .restore_context = tegra_clk_plle_t210_restore_context,
    };
    struct clk *tegra_clk_register_plle_tegra210(const char *name,
    const char *parent_name,
    void __iomem *clk_base, unsigned long flags,
    struct tegra_clk_pll_params *pll_params,
    spinlock_t *lock)
    {
    struct tegra_clk_pll *pll;
    struct clk *clk;
    pll = _tegra_init_pll(clk_base, core::ptr::null_mut(), pll_params, lock);
    if (IS_ERR(pll))
    return ERR_CAST(pll);
    _clk_plle_tegra_init_parent(pll);
    clk = _tegra_clk_register_pll(pll, name, parent_name, flags,
    &tegra_clk_plle_tegra210_ops);
    if (IS_ERR(clk))
    kfree(pll);
    return clk;
    }
    struct clk *tegra_clk_register_pllc_tegra210(const char *name,
    const char *parent_name, void __iomem *clk_base,
    void __iomem *pmc, unsigned long flags,
    struct tegra_clk_pll_params *pll_params,
    spinlock_t *lock)
    {
    struct clk *parent, *clk;
    const struct pdiv_map *p_tohw = pll_params.pdiv_tohw;
    struct tegra_clk_pll *pll;
    unsigned long parent_rate;
    if (!p_tohw)
    return ERR_PTR(-EINVAL);
    parent = __clk_lookup(parent_name);
    if (!parent) {
    WARN(1, "parent clk %s of %s must be registered first\n",
    name, parent_name);
    return ERR_PTR(-EINVAL);
    }
    parent_rate = clk_get_rate(parent);
    pll_params.vco_min = _clip_vco_min(pll_params.vco_min, parent_rate);
    if (pll_params.adjust_vco)
    pll_params.vco_min = pll_params.adjust_vco(pll_params,
    parent_rate);
    pll_params.flags |= TEGRA_PLL_BYPASS;
    pll = _tegra_init_pll(clk_base, pmc, pll_params, lock);
    if (IS_ERR(pll))
    return ERR_CAST(pll);
    clk = _tegra_clk_register_pll(pll, name, parent_name, flags,
    &tegra_clk_pll_ops);
    if (IS_ERR(clk))
    kfree(pll);
    return clk;
    }
    struct clk *tegra_clk_register_pllss_tegra210(const char *name,
    const char *parent_name, void __iomem *clk_base,
    unsigned long flags,
    struct tegra_clk_pll_params *pll_params,
    spinlock_t *lock)
    {
    struct tegra_clk_pll *pll;
    struct clk *clk, *parent;
    unsigned long parent_rate;
    u32 val;
    if (!pll_params.div_nmp)
    return ERR_PTR(-EINVAL);
    parent = __clk_lookup(parent_name);
    if (!parent) {
    WARN(1, "parent clk %s of %s must be registered first\n",
    name, parent_name);
    return ERR_PTR(-EINVAL);
    }
    val = readl_relaxed(clk_base + pll_params.base_reg);
    if (val & PLLSS_REF_SRC_SEL_MASK) {
    WARN(1, "not supported reference clock for %s\n", name);
    return ERR_PTR(-EINVAL);
    }
    parent_rate = clk_get_rate(parent);
    pll_params.vco_min = _clip_vco_min(pll_params.vco_min, parent_rate);
    if (pll_params.adjust_vco)
    pll_params.vco_min = pll_params.adjust_vco(pll_params,
    parent_rate);
    pll_params.flags |= TEGRA_PLL_BYPASS;
    pll = _tegra_init_pll(clk_base, core::ptr::null_mut(), pll_params, lock);
    if (IS_ERR(pll))
    return ERR_CAST(pll);
    clk = _tegra_clk_register_pll(pll, name, parent_name, flags,
    &tegra_clk_pll_ops);
    if (IS_ERR(clk))
    kfree(pll);
    return clk;
    }
    struct clk *tegra_clk_register_pllmb(const char *name, const char *parent_name,
    void __iomem *clk_base, void __iomem *pmc,
    unsigned long flags,
    struct tegra_clk_pll_params *pll_params,
    spinlock_t *lock)
    {
    struct tegra_clk_pll *pll;
    struct clk *clk, *parent;
    unsigned long parent_rate;
    if (!pll_params.pdiv_tohw)
    return ERR_PTR(-EINVAL);
    parent = __clk_lookup(parent_name);
    if (!parent) {
    WARN(1, "parent clk %s of %s must be registered first\n",
    parent_name, name);
    return ERR_PTR(-EINVAL);
    }
    parent_rate = clk_get_rate(parent);
    pll_params.vco_min = _clip_vco_min(pll_params.vco_min, parent_rate);
    if (pll_params.adjust_vco)
    pll_params.vco_min = pll_params.adjust_vco(pll_params,
    parent_rate);
    pll_params.flags |= TEGRA_PLL_BYPASS;
    pll_params.flags |= TEGRA_PLLMB;
    pll = _tegra_init_pll(clk_base, pmc, pll_params, lock);
    if (IS_ERR(pll))
    return ERR_CAST(pll);
    clk = _tegra_clk_register_pll(pll, name, parent_name, flags,
    &tegra_clk_pll_ops);
    if (IS_ERR(clk))
    kfree(pll);
    return clk;
    }
