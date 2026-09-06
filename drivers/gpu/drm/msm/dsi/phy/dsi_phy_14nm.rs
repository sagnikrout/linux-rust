//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/msm/dsi/phy/dsi_phy_14nm.c
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
// Copyright (c) 2016, The Linux Foundation. All rights reserved.
//

pub const PHY_14NM_CKLN_IDX: c_int = 4;
//
// DSI PLL 14nm - clock diagram (eg: DSI0):
//
// dsi0n1_postdiv_clk
// |
// +----+  |  +----+
// dsi0vco_clk ---| n1 |--o--| /8 |-- dsi0pllbyte
// +----+  |  +----+
// |           dsi0n1_postdivby2_clk
// |   +----+  |
// o---| /2 |--o--|\
// |   +----+     | \   +----+
// |              |  |--| n2 |-- dsi0pll
// o--------------| /   +----+
// |
//
pub const POLL_MAX_READS: c_int = 15;
pub const POLL_TIMEOUT_US: c_int = 1000;
pub const VCO_REF_CLK_RATE: c_int = 19200000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsi_pll_config {
    pub vco_current_rate: u64,
    pub /: *mut *mut u32 ssc_en; / SSC enable/disable,
// fixed params
    pub plllock_cnt: u32,
    pub ssc_center: u32,
    pub ssc_adj_period: u32,
    pub ssc_spread: u32,
    pub ssc_freq: u32,
// calculated
    pub dec_start: u32,
    pub div_frac_start: u32,
    pub ssc_period: u32,
    pub ssc_step_size: u32,
    pub plllock_cmp: u32,
    pub pll_vco_div_ref: u32,
    pub pll_vco_count: u32,
    pub pll_kvco_div_ref: u32,
    pub pll_kvco_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_14nm_cached_state {
    pub vco_rate: c_ulong,
    pub n2postdiv: u8,
    pub n1postdiv: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsi_pll_14nm {
    pub clk_hw: clk_hw,
    pub phy: *mut msm_dsi_phy,
// protects REG_DSI_14nm_PHY_CMN_CLK_CFG0 register
    pub postdiv_lock: spinlock_t,
    pub cached_state: pll_14nm_cached_state,
    pub slave: *mut dsi_pll_14nm,
}

//
// Private struct for N1/N2 post-divider clocks. These clocks are similar to
// the generic clk_divider class of clocks. The only difference is that it
// also sets the slave DSI PLL's post-dividers if in bonded DSI mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsi_pll_14nm_postdiv {
    pub hw: clk_hw,
// divider params
    pub shift: u8,
    pub width: u8,
    pub /: *mut *mut u8 flags; / same flags as used by clk_divider struct,
    pub pll: *mut dsi_pll_14nm,
}

//
// Global list of private DSI PLL struct pointers. We need this for bonded DSI
// mode, where the master PLL's clk_ops needs access the slave's private data
//
    static struct dsi_pll_14nm *pll_14nm_list[DSI_MAX];
    static bool pll_14nm_poll_for_ready(struct dsi_pll_14nm *pll_14nm,
    u32 nb_tries, u32 timeout_us)
    {
    let mut pll_locked: bool = false, pll_ready = false;
    void __iomem *base = pll_14nm.phy.pll_base;
    u32 tries, val;
    tries = nb_tries;
    while (tries--) {
    val = readl(base + REG_DSI_14nm_PHY_PLL_RESET_SM_READY_STATUS);
    pll_locked = !!(val & BIT(5));
    if (pll_locked)
    break;
    udelay(timeout_us);
    }
    if (!pll_locked)
    goto out;
    tries = nb_tries;
    while (tries--) {
    val = readl(base + REG_DSI_14nm_PHY_PLL_RESET_SM_READY_STATUS);
    pll_ready = !!(val & BIT(0));
    if (pll_ready)
    break;
    udelay(timeout_us);
    }
    out:
    DBG("DSI PLL is %slocked, %sready", pll_locked ? "" : "*not* ", pll_ready ? "" : "*not* ");
    return pll_locked && pll_ready;
    }
#[no_mangle]
unsafe extern "C" fn dsi_pll_14nm_config_init(pconf: *mut dsi_pll_config) {
    static void dsi_pll_14nm_config_init(struct dsi_pll_config *pconf)
    {
// fixed input
    pconf.plllock_cnt = 1;
//
// SSC is enabled by default. We might need DT props for configuring
// some SSC params like PPM and center/down spread etc.
//
    pconf.ssc_en = 1;
    pconf.ssc_center = 0;		/* down spread by default */
    pconf.ssc_spread = 5;		/* PPM / 1000 */
    pconf.ssc_freq = 31500;	/* default recommended */
    pconf.ssc_adj_period = 37;
    }

#[no_mangle]
unsafe extern "C" fn pll_14nm_ssc_calc(pll: *mut dsi_pll_14nm, pconf: *mut dsi_pll_config) {
    static void pll_14nm_ssc_calc(struct dsi_pll_14nm *pll, struct dsi_pll_config *pconf)
    {
    u32 period, ssc_period;
    u32 ref, rem;
    u64 step_size;
    DBG("vco=%lld ref=%d", pconf.vco_current_rate, VCO_REF_CLK_RATE);
    ssc_period = pconf.ssc_freq / 500;
    period = (u32)VCO_REF_CLK_RATE / 1000;
    ssc_period  = CEIL(period, ssc_period);
    ssc_period -= 1;
    pconf.ssc_period = ssc_period;
    DBG("ssc freq=%d spread=%d period=%d", pconf.ssc_freq,
    pconf.ssc_spread, pconf.ssc_period);
    step_size = (u32)pconf.vco_current_rate;
    ref = VCO_REF_CLK_RATE;
    ref /= 1000;
    step_size = div_u64(step_size, ref);
    step_size <<= 20;
    step_size = div_u64(step_size, 1000);
    step_size *= pconf.ssc_spread;
    step_size = div_u64(step_size, 1000);
    step_size *= (pconf.ssc_adj_period + 1);
    rem = 0;
    step_size = div_u64_rem(step_size, ssc_period + 1, &rem);
    if (rem)
    step_size++;
    DBG("step_size=%lld", step_size);
    step_size &= 0x0ffff;	/* take lower 16 bits */
    pconf.ssc_step_size = step_size;
    }
#[no_mangle]
unsafe extern "C" fn pll_14nm_dec_frac_calc(pll: *mut dsi_pll_14nm, pconf: *mut dsi_pll_config) {
    static void pll_14nm_dec_frac_calc(struct dsi_pll_14nm *pll, struct dsi_pll_config *pconf)
    {
    let mut multiplier: u64 = BIT(20);
    u64 dec_start_multiple, dec_start, pll_comp_val;
    u32 duration, div_frac_start;
    let mut vco_clk_rate: u64 = pconf.vco_current_rate;
    let mut fref: u64 = VCO_REF_CLK_RATE;
    DBG("vco_clk_rate=%lld ref_clk_rate=%lld", vco_clk_rate, fref);
    dec_start_multiple = div_u64(vco_clk_rate * multiplier, fref);
    dec_start = div_u64_rem(dec_start_multiple, multiplier, &div_frac_start);
    pconf.dec_start = (u32)dec_start;
    pconf.div_frac_start = div_frac_start;
    if (pconf.plllock_cnt == 0)
    duration = 1024;
#[no_mangle]
pub unsafe extern "C" fn if(1: pconf->plllock_cnt ==) -> else {
    else if (pconf.plllock_cnt == 1)
    duration = 256;
#[no_mangle]
pub unsafe extern "C" fn if(2: pconf->plllock_cnt ==) -> else {
    else if (pconf.plllock_cnt == 2)
    duration = 128;
    else
    duration = 32;
    pll_comp_val = duration * dec_start_multiple;
    pll_comp_val = div_u64(pll_comp_val, multiplier);
    do_div(pll_comp_val, 10);
    pconf.plllock_cmp = (u32)pll_comp_val;
    }
#[no_mangle]
unsafe extern "C" fn pll_14nm_kvco_slop(vrate: u32) -> u32 {
    static u32 pll_14nm_kvco_slop(u32 vrate)
    {
    let mut slop: u32 = 0;
    if (vrate > VCO_MIN_RATE && vrate <= 1800000000UL)
    slop =  600;
#[no_mangle]
pub unsafe extern "C" fn if(2300000000UL: vrate > 1800000000UL && vrate <) -> else {
    else if (vrate > 1800000000UL && vrate < 2300000000UL)
    slop = 400;
#[no_mangle]
pub unsafe extern "C" fn if(VCO_MAX_RATE: vrate > 2300000000UL && vrate <) -> else {
    else if (vrate > 2300000000UL && vrate < VCO_MAX_RATE)
    slop = 280;
    return slop;
    }
#[no_mangle]
unsafe extern "C" fn pll_14nm_calc_vco_count(pll: *mut dsi_pll_14nm, pconf: *mut dsi_pll_config) {
    static void pll_14nm_calc_vco_count(struct dsi_pll_14nm *pll, struct dsi_pll_config *pconf)
    {
    let mut vco_clk_rate: u64 = pconf.vco_current_rate;
    let mut fref: u64 = VCO_REF_CLK_RATE;
    let mut vco_measure_time: u32 = 5;
    let mut kvco_measure_time: u32 = 5;
    u64 data;
    u32 cnt;
    data = fref * vco_measure_time;
    do_div(data, 1000000);
    data &= 0x03ff;	/* 10 bits */
    data -= 2;
    pconf.pll_vco_div_ref = data;
    data = div_u64(vco_clk_rate, 1000000);	/* unit is Mhz */
    data *= vco_measure_time;
    do_div(data, 10);
    pconf.pll_vco_count = data;
    data = fref * kvco_measure_time;
    do_div(data, 1000000);
    data &= 0x03ff;	/* 10 bits */
    data -= 1;
    pconf.pll_kvco_div_ref = data;
    cnt = pll_14nm_kvco_slop(vco_clk_rate);
    cnt *= 2;
    cnt /= 100;
    cnt *= kvco_measure_time;
    pconf.pll_kvco_count = cnt;
    }
#[no_mangle]
unsafe extern "C" fn pll_db_commit_ssc(pll: *mut dsi_pll_14nm, pconf: *mut dsi_pll_config) {
    static void pll_db_commit_ssc(struct dsi_pll_14nm *pll, struct dsi_pll_config *pconf)
    {
    void __iomem *base = pll.phy.pll_base;
    u8 data;
    data = pconf.ssc_adj_period;
    data &= 0x0ff;
    writel(data, base + REG_DSI_14nm_PHY_PLL_SSC_ADJ_PER1);
    data = (pconf.ssc_adj_period >> 8);
    data &= 0x03;
    writel(data, base + REG_DSI_14nm_PHY_PLL_SSC_ADJ_PER2);
    data = pconf.ssc_period;
    data &= 0x0ff;
    writel(data, base + REG_DSI_14nm_PHY_PLL_SSC_PER1);
    data = (pconf.ssc_period >> 8);
    data &= 0x0ff;
    writel(data, base + REG_DSI_14nm_PHY_PLL_SSC_PER2);
    data = pconf.ssc_step_size;
    data &= 0x0ff;
    writel(data, base + REG_DSI_14nm_PHY_PLL_SSC_STEP_SIZE1);
    data = (pconf.ssc_step_size >> 8);
    data &= 0x0ff;
    writel(data, base + REG_DSI_14nm_PHY_PLL_SSC_STEP_SIZE2);
    data = (pconf.ssc_center & 0x01);
    data <<= 1;
    data |= 0x01; /* enable */
    writel(data, base + REG_DSI_14nm_PHY_PLL_SSC_EN_CENTER);
    wmb();	/* make sure register committed */
    }
    static void pll_db_commit_common(struct dsi_pll_14nm *pll,
    struct dsi_pll_config *pconf)
    {
    void __iomem *base = pll.phy.pll_base;
    u8 data;
// confgiure the non frequency dependent pll registers
    data = 0;
    writel(data, base + REG_DSI_14nm_PHY_PLL_SYSCLK_EN_RESET);
    writel(1, base + REG_DSI_14nm_PHY_PLL_TXCLK_EN);
    writel(48, base + REG_DSI_14nm_PHY_PLL_RESETSM_CNTRL);
// bandgap_timer
    writel(4 << 3, base + REG_DSI_14nm_PHY_PLL_RESETSM_CNTRL2);
// pll_wakeup_timer
    writel(5, base + REG_DSI_14nm_PHY_PLL_RESETSM_CNTRL5);
    data = pconf.pll_vco_div_ref & 0xff;
    writel(data, base + REG_DSI_14nm_PHY_PLL_VCO_DIV_REF1);
    data = (pconf.pll_vco_div_ref >> 8) & 0x3;
    writel(data, base + REG_DSI_14nm_PHY_PLL_VCO_DIV_REF2);
    data = pconf.pll_kvco_div_ref & 0xff;
    writel(data, base + REG_DSI_14nm_PHY_PLL_KVCO_DIV_REF1);
    data = (pconf.pll_kvco_div_ref >> 8) & 0x3;
    writel(data, base + REG_DSI_14nm_PHY_PLL_KVCO_DIV_REF2);
    writel(16, base + REG_DSI_14nm_PHY_PLL_PLL_MISC1);
    writel(4, base + REG_DSI_14nm_PHY_PLL_IE_TRIM);
    writel(4, base + REG_DSI_14nm_PHY_PLL_IP_TRIM);
    writel(1 << 3 | 1, base + REG_DSI_14nm_PHY_PLL_CP_SET_CUR);
    writel(0 << 3 | 0, base + REG_DSI_14nm_PHY_PLL_PLL_ICPCSET);
    writel(0 << 3 | 0, base + REG_DSI_14nm_PHY_PLL_PLL_ICPMSET);
    writel(4 << 3 | 4, base + REG_DSI_14nm_PHY_PLL_PLL_ICP_SET);
    writel(1 << 4 | 11, base + REG_DSI_14nm_PHY_PLL_PLL_LPF1);
    writel(7, base + REG_DSI_14nm_PHY_PLL_IPTAT_TRIM);
    writel(1 << 4 | 2, base + REG_DSI_14nm_PHY_PLL_PLL_CRCTRL);
    }
#[no_mangle]
unsafe extern "C" fn pll_14nm_software_reset(pll_14nm: *mut dsi_pll_14nm) {
    static void pll_14nm_software_reset(struct dsi_pll_14nm *pll_14nm)
    {
    void __iomem *cmn_base = pll_14nm.phy.base;
// de assert pll start and apply pll sw reset
// stop pll
    writel(0, cmn_base + REG_DSI_14nm_PHY_CMN_PLL_CNTRL);
// pll sw reset
    writel(0x20, cmn_base + REG_DSI_14nm_PHY_CMN_CTRL_1);
    udelay(10);
    wmb();	/* make sure register committed */
    writel(0, cmn_base + REG_DSI_14nm_PHY_CMN_CTRL_1);
    wmb();	/* make sure register committed */
    }
    static void pll_db_commit_14nm(struct dsi_pll_14nm *pll,
    struct dsi_pll_config *pconf)
    {
    void __iomem *base = pll.phy.pll_base;
    void __iomem *cmn_base = pll.phy.base;
    u8 data;
    DBG("DSI%d PLL", pll.phy.id);
    writel(0x3c, cmn_base + REG_DSI_14nm_PHY_CMN_LDO_CNTRL);
    pll_db_commit_common(pll, pconf);
    pll_14nm_software_reset(pll);
// Use the /2 path in Mux
    writel(1, cmn_base + REG_DSI_14nm_PHY_CMN_CLK_CFG1);
    data = 0xff; /* data, clk, pll normal operation */
    writel(data, cmn_base + REG_DSI_14nm_PHY_CMN_CTRL_0);
// configure the frequency dependent pll registers
    data = pconf.dec_start;
    writel(data, base + REG_DSI_14nm_PHY_PLL_DEC_START);
    data = pconf.div_frac_start & 0xff;
    writel(data, base + REG_DSI_14nm_PHY_PLL_DIV_FRAC_START1);
    data = (pconf.div_frac_start >> 8) & 0xff;
    writel(data, base + REG_DSI_14nm_PHY_PLL_DIV_FRAC_START2);
    data = (pconf.div_frac_start >> 16) & 0xf;
    writel(data, base + REG_DSI_14nm_PHY_PLL_DIV_FRAC_START3);
    data = pconf.plllock_cmp & 0xff;
    writel(data, base + REG_DSI_14nm_PHY_PLL_PLLLOCK_CMP1);
    data = (pconf.plllock_cmp >> 8) & 0xff;
    writel(data, base + REG_DSI_14nm_PHY_PLL_PLLLOCK_CMP2);
    data = (pconf.plllock_cmp >> 16) & 0x3;
    writel(data, base + REG_DSI_14nm_PHY_PLL_PLLLOCK_CMP3);
    data = pconf.plllock_cnt << 1 | 0 << 3; /* plllock_rng */
    writel(data, base + REG_DSI_14nm_PHY_PLL_PLLLOCK_CMP_EN);
    data = pconf.pll_vco_count & 0xff;
    writel(data, base + REG_DSI_14nm_PHY_PLL_VCO_COUNT1);
    data = (pconf.pll_vco_count >> 8) & 0xff;
    writel(data, base + REG_DSI_14nm_PHY_PLL_VCO_COUNT2);
    data = pconf.pll_kvco_count & 0xff;
    writel(data, base + REG_DSI_14nm_PHY_PLL_KVCO_COUNT1);
    data = (pconf.pll_kvco_count >> 8) & 0x3;
    writel(data, base + REG_DSI_14nm_PHY_PLL_KVCO_COUNT2);
//
// High nibble configures the post divider internal to the VCO. It's
// fixed to divide by 1 for now.
//
// 0: divided by 1
// 1: divided by 2
// 2: divided by 4
// 3: divided by 8
//
    writel(0 << 4 | 3, base + REG_DSI_14nm_PHY_PLL_PLL_LPF2_POSTDIV);
    if (pconf.ssc_en)
    pll_db_commit_ssc(pll, pconf);
    wmb();	/* make sure register committed */
    }
//
// VCO clock Callbacks
//
    static int dsi_pll_14nm_vco_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct dsi_pll_14nm *pll_14nm = to_pll_14nm(hw);
    struct dsi_pll_config conf;
    DBG("DSI PLL%d rate=%lu, parent's=%lu", pll_14nm.phy.id, rate,
    parent_rate);
    dsi_pll_14nm_config_init(&conf);
    conf.vco_current_rate = rate;
    pll_14nm_dec_frac_calc(pll_14nm, &conf);
    if (conf.ssc_en)
    pll_14nm_ssc_calc(pll_14nm, &conf);
    pll_14nm_calc_vco_count(pll_14nm, &conf);
// commit the slave DSI PLL registers if we're master. Note that we
// don't lock the slave PLL. We just ensure that the PLL/PHY registers
// of the master and slave are identical
//
    if (pll_14nm.phy.usecase == MSM_DSI_PHY_MASTER) {
    struct dsi_pll_14nm *pll_14nm_slave = pll_14nm.slave;
    pll_db_commit_14nm(pll_14nm_slave, &conf);
    }
    pll_db_commit_14nm(pll_14nm, &conf);
    return 0;
    }
    static unsigned long dsi_pll_14nm_vco_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct dsi_pll_14nm *pll_14nm = to_pll_14nm(hw);
    void __iomem *base = pll_14nm.phy.pll_base;
    u64 vco_rate, multiplier = BIT(20);
    u32 div_frac_start;
    u32 dec_start;
    let mut ref_clk: u64 = parent_rate;
    dec_start = readl(base + REG_DSI_14nm_PHY_PLL_DEC_START);
    dec_start &= 0x0ff;
    DBG("dec_start = %x", dec_start);
    div_frac_start = (readl(base + REG_DSI_14nm_PHY_PLL_DIV_FRAC_START3)
    & 0xf) << 16;
    div_frac_start |= (readl(base + REG_DSI_14nm_PHY_PLL_DIV_FRAC_START2)
    & 0xff) << 8;
    div_frac_start |= readl(base + REG_DSI_14nm_PHY_PLL_DIV_FRAC_START1)
    & 0xff;
    DBG("div_frac_start = %x", div_frac_start);
    vco_rate = ref_clk * dec_start;
    vco_rate += ((ref_clk * div_frac_start) / multiplier);
//
// Recalculating the rate from dec_start and frac_start doesn't end up
// the rate we originally set. Convert the freq to KHz, round it up and
// convert it back to MHz.
//
    vco_rate = DIV_ROUND_UP_ULL(vco_rate, 1000) * 1000;
    DBG("returning vco rate = %lu", (unsigned long)vco_rate);
    return (unsigned long)vco_rate;
    }
#[no_mangle]
unsafe extern "C" fn dsi_pll_14nm_vco_prepare(hw: *mut clk_hw) -> c_int {
    static int dsi_pll_14nm_vco_prepare(struct clk_hw *hw)
    {
    struct dsi_pll_14nm *pll_14nm = to_pll_14nm(hw);
    void __iomem *base = pll_14nm.phy.pll_base;
    void __iomem *cmn_base = pll_14nm.phy.base;
    bool locked;
    DBG("");
    if (unlikely(pll_14nm.phy.pll_on))
    return 0;
    if (dsi_pll_14nm_vco_recalc_rate(hw, VCO_REF_CLK_RATE) == 0)
    dsi_pll_14nm_vco_set_rate(hw, pll_14nm.phy.cfg.min_pll_rate, VCO_REF_CLK_RATE);
    writel(0x10, base + REG_DSI_14nm_PHY_PLL_VREF_CFG1);
    writel(1, cmn_base + REG_DSI_14nm_PHY_CMN_PLL_CNTRL);
    locked = pll_14nm_poll_for_ready(pll_14nm, POLL_MAX_READS,
    POLL_TIMEOUT_US);
    if (unlikely(!locked)) {
    DRM_DEV_ERROR(&pll_14nm.phy.pdev.dev, "DSI PLL lock failed\n");
    return -EINVAL;
    }
    DBG("DSI PLL lock success");
    pll_14nm.phy.pll_on = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dsi_pll_14nm_vco_unprepare(hw: *mut clk_hw) {
    static void dsi_pll_14nm_vco_unprepare(struct clk_hw *hw)
    {
    struct dsi_pll_14nm *pll_14nm = to_pll_14nm(hw);
    void __iomem *cmn_base = pll_14nm.phy.base;
    DBG("");
    if (unlikely(!pll_14nm.phy.pll_on))
    return;
    writel(0, cmn_base + REG_DSI_14nm_PHY_CMN_PLL_CNTRL);
    pll_14nm.phy.pll_on = false;
    }
    static int dsi_pll_14nm_clk_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct dsi_pll_14nm *pll_14nm = to_pll_14nm(hw);
    req.rate = clamp_t(unsigned long, req.rate,
    pll_14nm.phy.cfg.min_pll_rate, pll_14nm.phy.cfg.max_pll_rate);
    return 0;
    }
    static const struct clk_ops clk_ops_dsi_pll_14nm_vco = {
    .determine_rate = dsi_pll_14nm_clk_determine_rate,
    .set_rate = dsi_pll_14nm_vco_set_rate,
    .recalc_rate = dsi_pll_14nm_vco_recalc_rate,
    .prepare = dsi_pll_14nm_vco_prepare,
    .unprepare = dsi_pll_14nm_vco_unprepare,
    };
//
// N1 and N2 post-divider clock callbacks
//

    static unsigned long dsi_pll_14nm_postdiv_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct dsi_pll_14nm_postdiv *postdiv = to_pll_14nm_postdiv(hw);
    struct dsi_pll_14nm *pll_14nm = postdiv.pll;
    void __iomem *base = pll_14nm.phy.base;
    let mut shift: u8 = postdiv.shift;
    let mut width: u8 = postdiv.width;
    u32 val;
    DBG("DSI%d PLL parent rate=%lu", pll_14nm.phy.id, parent_rate);
    val = readl(base + REG_DSI_14nm_PHY_CMN_CLK_CFG0) >> shift;
    val &= div_mask(width);
    return divider_recalc_rate(hw, parent_rate, val, core::ptr::null_mut(),
    postdiv.flags, width);
    }
    static int dsi_pll_14nm_postdiv_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct dsi_pll_14nm_postdiv *postdiv = to_pll_14nm_postdiv(hw);
    struct dsi_pll_14nm *pll_14nm = postdiv.pll;
    DBG("DSI%d PLL parent rate=%lu", pll_14nm.phy.id, req.rate);
    return divider_determine_rate(hw, req, core::ptr::null_mut(), postdiv.width, postdiv.flags);
    }
    static int dsi_pll_14nm_postdiv_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct dsi_pll_14nm_postdiv *postdiv = to_pll_14nm_postdiv(hw);
    struct dsi_pll_14nm *pll_14nm = postdiv.pll;
    void __iomem *base = pll_14nm.phy.base;
    spinlock_t *lock = &pll_14nm.postdiv_lock;
    let mut shift: u8 = postdiv.shift;
    let mut width: u8 = postdiv.width;
    unsigned int value;
    let mut flags: c_ulong = 0;
    u32 val;
    DBG("DSI%d PLL parent rate=%lu parent rate %lu", pll_14nm.phy.id, rate,
    parent_rate);
    value = divider_get_val(rate, parent_rate, core::ptr::null_mut(), postdiv.width,
    postdiv.flags);
    spin_lock_irqsave(lock, flags);
    val = readl(base + REG_DSI_14nm_PHY_CMN_CLK_CFG0);
    val &= ~(div_mask(width) << shift);
    val |= value << shift;
    writel(val, base + REG_DSI_14nm_PHY_CMN_CLK_CFG0);
// If we're master in bonded DSI mode, then the slave PLL's post-dividers
// follow the master's post dividers
//
    if (pll_14nm.phy.usecase == MSM_DSI_PHY_MASTER) {
    struct dsi_pll_14nm *pll_14nm_slave = pll_14nm.slave;
    void __iomem *slave_base = pll_14nm_slave.phy.base;
    writel(val, slave_base + REG_DSI_14nm_PHY_CMN_CLK_CFG0);
    }
    spin_unlock_irqrestore(lock, flags);
    return 0;
    }
    static const struct clk_ops clk_ops_dsi_pll_14nm_postdiv = {
    .recalc_rate = dsi_pll_14nm_postdiv_recalc_rate,
    .determine_rate = dsi_pll_14nm_postdiv_determine_rate,
    .set_rate = dsi_pll_14nm_postdiv_set_rate,
    };
//
// PLL Callbacks
//
#[no_mangle]
unsafe extern "C" fn dsi_14nm_pll_save_state(phy: *mut msm_dsi_phy) {
    static void dsi_14nm_pll_save_state(struct msm_dsi_phy *phy)
    {
    struct dsi_pll_14nm *pll_14nm = to_pll_14nm(phy.vco_hw);
    struct pll_14nm_cached_state *cached_state = &pll_14nm.cached_state;
    void __iomem *cmn_base = pll_14nm.phy.base;
    u32 data;
    data = readl(cmn_base + REG_DSI_14nm_PHY_CMN_CLK_CFG0);
    cached_state.n1postdiv = data & 0xf;
    cached_state.n2postdiv = (data >> 4) & 0xf;
    DBG("DSI%d PLL save state %x %x", pll_14nm.phy.id,
    cached_state.n1postdiv, cached_state.n2postdiv);
    cached_state.vco_rate = clk_hw_get_rate(phy.vco_hw);
    }
#[no_mangle]
unsafe extern "C" fn dsi_14nm_pll_restore_state(phy: *mut msm_dsi_phy) -> c_int {
    static int dsi_14nm_pll_restore_state(struct msm_dsi_phy *phy)
    {
    struct dsi_pll_14nm *pll_14nm = to_pll_14nm(phy.vco_hw);
    struct pll_14nm_cached_state *cached_state = &pll_14nm.cached_state;
    void __iomem *cmn_base = pll_14nm.phy.base;
    u32 data;
    int ret;
    ret = dsi_pll_14nm_vco_set_rate(phy.vco_hw,
    cached_state.vco_rate, 0);
    if (ret) {
    DRM_DEV_ERROR(&pll_14nm.phy.pdev.dev,
    "restore vco rate failed. ret=%d\n", ret);
    return ret;
    }
    data = cached_state.n1postdiv | (cached_state.n2postdiv << 4);
    DBG("DSI%d PLL restore state %x %x", pll_14nm.phy.id,
    cached_state.n1postdiv, cached_state.n2postdiv);
    writel(data, cmn_base + REG_DSI_14nm_PHY_CMN_CLK_CFG0);
// also restore post-dividers for slave DSI PLL
    if (phy.usecase == MSM_DSI_PHY_MASTER) {
    struct dsi_pll_14nm *pll_14nm_slave = pll_14nm.slave;
    void __iomem *slave_base = pll_14nm_slave.phy.base;
    writel(data, slave_base + REG_DSI_14nm_PHY_CMN_CLK_CFG0);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dsi_14nm_set_usecase(phy: *mut msm_dsi_phy) -> c_int {
    static int dsi_14nm_set_usecase(struct msm_dsi_phy *phy)
    {
    struct dsi_pll_14nm *pll_14nm = to_pll_14nm(phy.vco_hw);
    void __iomem *base = phy.pll_base;
    u32 clkbuflr_en, bandgap = 0;
    switch (phy.usecase) {
    case MSM_DSI_PHY_STANDALONE:
    clkbuflr_en = 0x1;
    break;
    case MSM_DSI_PHY_MASTER:
    clkbuflr_en = 0x3;
    pll_14nm.slave = pll_14nm_list[(pll_14nm.phy.id + 1) % DSI_MAX];
    break;
    case MSM_DSI_PHY_SLAVE:
    clkbuflr_en = 0x0;
    bandgap = 0x3;
    break;
    default:
    return -EINVAL;
    }
    writel(clkbuflr_en, base + REG_DSI_14nm_PHY_PLL_CLKBUFLR_EN);
    if (bandgap)
    writel(bandgap, base + REG_DSI_14nm_PHY_PLL_PLL_BANDGAP);
    return 0;
    }
    static struct clk_hw *pll_14nm_postdiv_register(struct dsi_pll_14nm *pll_14nm,
    const char *name,
    const struct clk_hw *parent_hw,
    unsigned long flags,
    u8 shift)
    {
    struct dsi_pll_14nm_postdiv *pll_postdiv;
    struct device *dev = &pll_14nm.phy.pdev.dev;
    struct clk_init_data postdiv_init = {
    .parent_hws = (const struct clk_hw *[]) { parent_hw },
    .num_parents = 1,
    .name = name,
    .flags = flags,
    .ops = &clk_ops_dsi_pll_14nm_postdiv,
    };
    int ret;
    pll_postdiv = devm_kzalloc(dev, sizeof(*pll_postdiv), GFP_KERNEL);
    if (!pll_postdiv)
    return ERR_PTR(-ENOMEM);
    pll_postdiv.pll = pll_14nm;
    pll_postdiv.shift = shift;
// both N1 and N2 postdividers are 4 bits wide
    pll_postdiv.width = 4;
// range of each divider is from 1 to 15
    pll_postdiv.flags = CLK_DIVIDER_ONE_BASED;
    pll_postdiv.hw.init = &postdiv_init;
    ret = devm_clk_hw_register(dev, &pll_postdiv.hw);
    if (ret)
    return ERR_PTR(ret);
    return &pll_postdiv.hw;
    }
#[no_mangle]
unsafe extern "C" fn pll_14nm_register(pll_14nm: *mut dsi_pll_14nm, provided_clocks: *mut clk_hw) -> c_int {
    static int pll_14nm_register(struct dsi_pll_14nm *pll_14nm, struct clk_hw **provided_clocks)
    {
    char clk_name[32];
    struct clk_init_data vco_init = {
    .parent_data = &(const struct clk_parent_data) {
    .fw_name = "ref",
    },
    .num_parents = 1,
    .name = clk_name,
    .flags = CLK_IGNORE_UNUSED,
    .ops = &clk_ops_dsi_pll_14nm_vco,
    };
    struct device *dev = &pll_14nm.phy.pdev.dev;
    struct clk_hw *hw, *n1_postdiv, *n1_postdivby2;
    int ret;
    DBG("DSI%d", pll_14nm.phy.id);
    snprintf(clk_name, sizeof(clk_name), "dsi%dvco_clk", pll_14nm.phy.id);
    pll_14nm.clk_hw.init = &vco_init;
    ret = devm_clk_hw_register(dev, &pll_14nm.clk_hw);
    if (ret)
    return ret;
    snprintf(clk_name, sizeof(clk_name), "dsi%dn1_postdiv_clk", pll_14nm.phy.id);
// N1 postdiv, bits 0-3 in REG_DSI_14nm_PHY_CMN_CLK_CFG0
    n1_postdiv = pll_14nm_postdiv_register(pll_14nm, clk_name,
    &pll_14nm.clk_hw, CLK_SET_RATE_PARENT, 0);
    if (IS_ERR(n1_postdiv))
    return PTR_ERR(n1_postdiv);
    snprintf(clk_name, sizeof(clk_name), "dsi%dpllbyte", pll_14nm.phy.id);
// DSI Byte clock = VCO_CLK / N1 / 8
    hw = devm_clk_hw_register_fixed_factor_parent_hw(dev, clk_name,
    n1_postdiv, CLK_SET_RATE_PARENT, 1, 8);
    if (IS_ERR(hw))
    return PTR_ERR(hw);
    provided_clocks[DSI_BYTE_PLL_CLK] = hw;
    snprintf(clk_name, sizeof(clk_name), "dsi%dn1_postdivby2_clk", pll_14nm.phy.id);
//
// Skip the mux for now, force DSICLK_SEL to 1, Add a /2 divider
// on the way. Don't let it set parent.
//
    n1_postdivby2 = devm_clk_hw_register_fixed_factor_parent_hw(dev,
    clk_name, n1_postdiv, 0, 1, 2);
    if (IS_ERR(n1_postdivby2))
    return PTR_ERR(n1_postdivby2);
    snprintf(clk_name, sizeof(clk_name), "dsi%dpll", pll_14nm.phy.id);
// DSI pixel clock = VCO_CLK / N1 / 2 / N2
// This is the output of N2 post-divider, bits 4-7 in
// REG_DSI_14nm_PHY_CMN_CLK_CFG0. Don't let it set parent.
//
    hw = pll_14nm_postdiv_register(pll_14nm, clk_name, n1_postdivby2,
    0, 4);
    if (IS_ERR(hw))
    return PTR_ERR(hw);
    provided_clocks[DSI_PIXEL_PLL_CLK] = hw;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dsi_pll_14nm_init(phy: *mut msm_dsi_phy) -> c_int {
    static int dsi_pll_14nm_init(struct msm_dsi_phy *phy)
    {
    struct platform_device *pdev = phy.pdev;
    struct dsi_pll_14nm *pll_14nm;
    int ret;
    if (!pdev)
    return -ENODEV;
    pll_14nm = devm_kzalloc(&pdev.dev, sizeof(*pll_14nm), GFP_KERNEL);
    if (!pll_14nm)
    return -ENOMEM;
    DBG("PLL%d", phy.id);
    pll_14nm_list[phy.id] = pll_14nm;
    spin_lock_init(&pll_14nm.postdiv_lock);
    pll_14nm.phy = phy;
    ret = pll_14nm_register(pll_14nm, phy.provided_clocks.hws);
    if (ret) {
    DRM_DEV_ERROR(&pdev.dev, "failed to register PLL: %d\n", ret);
    return ret;
    }
    phy.vco_hw = &pll_14nm.clk_hw;
    return 0;
    }
    static void dsi_14nm_dphy_set_timing(struct msm_dsi_phy *phy,
    struct msm_dsi_dphy_timing *timing,
    int lane_idx)
    {
    void __iomem *base = phy.lane_base;
    let mut clk_ln: bool = (lane_idx == PHY_14NM_CKLN_IDX);
    let mut zero: u32 = clk_ln ? timing.clk_zero : timing.hs_zero;
    let mut prepare: u32 = clk_ln ? timing.clk_prepare : timing.hs_prepare;
    let mut trail: u32 = clk_ln ? timing.clk_trail : timing.hs_trail;
    let mut rqst: u32 = clk_ln ? timing.hs_rqst_ckln : timing.hs_rqst;
    let mut prep_dly: u32 = clk_ln ? timing.hs_prep_dly_ckln : timing.hs_prep_dly;
    u32 halfbyte_en = clk_ln ? timing.hs_halfbyte_en_ckln :
    timing.hs_halfbyte_en;
    writel(DSI_14nm_PHY_LN_TIMING_CTRL_4_HS_EXIT(timing.hs_exit),
    base + REG_DSI_14nm_PHY_LN_TIMING_CTRL_4(lane_idx));
    writel(DSI_14nm_PHY_LN_TIMING_CTRL_5_HS_ZERO(zero),
    base + REG_DSI_14nm_PHY_LN_TIMING_CTRL_5(lane_idx));
    writel(DSI_14nm_PHY_LN_TIMING_CTRL_6_HS_PREPARE(prepare),
    base + REG_DSI_14nm_PHY_LN_TIMING_CTRL_6(lane_idx));
    writel(DSI_14nm_PHY_LN_TIMING_CTRL_7_HS_TRAIL(trail),
    base + REG_DSI_14nm_PHY_LN_TIMING_CTRL_7(lane_idx));
    writel(DSI_14nm_PHY_LN_TIMING_CTRL_8_HS_RQST(rqst),
    base + REG_DSI_14nm_PHY_LN_TIMING_CTRL_8(lane_idx));
    writel(DSI_14nm_PHY_LN_CFG0_PREPARE_DLY(prep_dly),
    base + REG_DSI_14nm_PHY_LN_CFG0(lane_idx));
    writel(halfbyte_en ? DSI_14nm_PHY_LN_CFG1_HALFBYTECLK_EN : 0,
    base + REG_DSI_14nm_PHY_LN_CFG1(lane_idx));
    writel(DSI_14nm_PHY_LN_TIMING_CTRL_9_TA_GO(timing.ta_go) |
    DSI_14nm_PHY_LN_TIMING_CTRL_9_TA_SURE(timing.ta_sure),
    base + REG_DSI_14nm_PHY_LN_TIMING_CTRL_9(lane_idx));
    writel(DSI_14nm_PHY_LN_TIMING_CTRL_10_TA_GET(timing.ta_get),
    base + REG_DSI_14nm_PHY_LN_TIMING_CTRL_10(lane_idx));
    writel(DSI_14nm_PHY_LN_TIMING_CTRL_11_TRIG3_CMD(0xa0),
    base + REG_DSI_14nm_PHY_LN_TIMING_CTRL_11(lane_idx));
    }
    static int dsi_14nm_phy_enable(struct msm_dsi_phy *phy,
    struct msm_dsi_phy_clk_request *clk_req)
    {
    struct msm_dsi_dphy_timing *timing = &phy.timing;
    u32 data;
    int i;
    int ret;
    void __iomem *base = phy.base;
    void __iomem *lane_base = phy.lane_base;
    u32 glbl_test_ctrl;
    if (msm_dsi_dphy_timing_calc_v2(timing, clk_req)) {
    DRM_DEV_ERROR(&phy.pdev.dev,
    "%s: D-PHY timing calculation failed\n",
    __func__);
    return -EINVAL;
    }
    data = 0x1c;
    if (phy.usecase != MSM_DSI_PHY_STANDALONE)
    data |= DSI_14nm_PHY_CMN_LDO_CNTRL_VREG_CTRL(32);
    writel(data, base + REG_DSI_14nm_PHY_CMN_LDO_CNTRL);
    writel(0x1, base + REG_DSI_14nm_PHY_CMN_GLBL_TEST_CTRL);
// 4 data lanes + 1 clk lane configuration
    for (i = 0; i < 5; i++) {
    writel(0x1d, lane_base + REG_DSI_14nm_PHY_LN_VREG_CNTRL(i));
    writel(0xff, lane_base + REG_DSI_14nm_PHY_LN_STRENGTH_CTRL_0(i));
    writel(i == PHY_14NM_CKLN_IDX ? 0x00 : 0x06,
    lane_base + REG_DSI_14nm_PHY_LN_STRENGTH_CTRL_1(i));
    writel(i == PHY_14NM_CKLN_IDX ? 0x8f : 0x0f,
    lane_base + REG_DSI_14nm_PHY_LN_CFG3(i));
    writel(0x10, lane_base + REG_DSI_14nm_PHY_LN_CFG2(i));
    writel(0, lane_base + REG_DSI_14nm_PHY_LN_TEST_DATAPATH(i));
    writel(0x88, lane_base + REG_DSI_14nm_PHY_LN_TEST_STR(i));
    dsi_14nm_dphy_set_timing(phy, timing, i);
    }
// Make sure PLL is not start
    writel(0x00, base + REG_DSI_14nm_PHY_CMN_PLL_CNTRL);
    wmb(); /* make sure everything is written before reset and enable */
// reset digital block
    writel(0x80, base + REG_DSI_14nm_PHY_CMN_CTRL_1);
    wmb(); /* ensure reset is asserted */
    udelay(100);
    writel(0x00, base + REG_DSI_14nm_PHY_CMN_CTRL_1);
    glbl_test_ctrl = readl(base + REG_DSI_14nm_PHY_CMN_GLBL_TEST_CTRL);
    if (phy.id == DSI_1 && phy.usecase == MSM_DSI_PHY_SLAVE)
    glbl_test_ctrl |= DSI_14nm_PHY_CMN_GLBL_TEST_CTRL_BITCLK_HS_SEL;
    else
    glbl_test_ctrl &= ~DSI_14nm_PHY_CMN_GLBL_TEST_CTRL_BITCLK_HS_SEL;
    writel(glbl_test_ctrl, base + REG_DSI_14nm_PHY_CMN_GLBL_TEST_CTRL);
    ret = dsi_14nm_set_usecase(phy);
    if (ret) {
    DRM_DEV_ERROR(&phy.pdev.dev, "%s: set pll usecase failed, %d\n",
    __func__, ret);
    return ret;
    }
// Remove power down from PLL and all lanes
    writel(0xff, base + REG_DSI_14nm_PHY_CMN_CTRL_0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dsi_14nm_phy_disable(phy: *mut msm_dsi_phy) {
    static void dsi_14nm_phy_disable(struct msm_dsi_phy *phy)
    {
    writel(0, phy.base + REG_DSI_14nm_PHY_CMN_GLBL_TEST_CTRL);
    writel(0, phy.base + REG_DSI_14nm_PHY_CMN_CTRL_0);
// ensure that the phy is completely disabled
    wmb();
    }
    static const struct regulator_bulk_data dsi_phy_14nm_17mA_regulators[] = {
    { .supply = "vcca", .init_load_uA = 17000 },
    };
    static const struct regulator_bulk_data dsi_phy_14nm_73p4mA_regulators[] = {
    { .supply = "vcca", .init_load_uA = 73400 },
    };
    static const struct regulator_bulk_data dsi_phy_14nm_36mA_regulators[] = {
    { .supply = "vdda", .init_load_uA = 36000 },
    };
    const struct msm_dsi_phy_cfg dsi_phy_14nm_cfgs = {
    .has_phy_lane = true,
    .regulator_data = dsi_phy_14nm_17mA_regulators,
    .num_regulators = ARRAY_SIZE(dsi_phy_14nm_17mA_regulators),
    .ops = {
    .enable = dsi_14nm_phy_enable,
    .disable = dsi_14nm_phy_disable,
    .pll_init = dsi_pll_14nm_init,
    .save_pll_state = dsi_14nm_pll_save_state,
    .restore_pll_state = dsi_14nm_pll_restore_state,
    },
    .min_pll_rate = VCO_MIN_RATE,
    .max_pll_rate = VCO_MAX_RATE,
    .io_start = { 0x994400, 0x996400 },
    .num_dsi_phy = 2,
    };
    const struct msm_dsi_phy_cfg dsi_phy_14nm_660_cfgs = {
    .has_phy_lane = true,
    .regulator_data = dsi_phy_14nm_73p4mA_regulators,
    .num_regulators = ARRAY_SIZE(dsi_phy_14nm_73p4mA_regulators),
    .ops = {
    .enable = dsi_14nm_phy_enable,
    .disable = dsi_14nm_phy_disable,
    .pll_init = dsi_pll_14nm_init,
    .save_pll_state = dsi_14nm_pll_save_state,
    .restore_pll_state = dsi_14nm_pll_restore_state,
    },
    .min_pll_rate = VCO_MIN_RATE,
    .max_pll_rate = VCO_MAX_RATE,
    .io_start = { 0xc994400, 0xc996400 },
    .num_dsi_phy = 2,
    };
    const struct msm_dsi_phy_cfg dsi_phy_14nm_8953_cfgs = {
    .has_phy_lane = true,
    .regulator_data = dsi_phy_14nm_17mA_regulators,
    .num_regulators = ARRAY_SIZE(dsi_phy_14nm_17mA_regulators),
    .ops = {
    .enable = dsi_14nm_phy_enable,
    .disable = dsi_14nm_phy_disable,
    .pll_init = dsi_pll_14nm_init,
    .save_pll_state = dsi_14nm_pll_save_state,
    .restore_pll_state = dsi_14nm_pll_restore_state,
    },
    .min_pll_rate = VCO_MIN_RATE,
    .max_pll_rate = VCO_MAX_RATE,
    .io_start = { 0x1a94400, 0x1a96400 },
    .num_dsi_phy = 2,
    };
    const struct msm_dsi_phy_cfg dsi_phy_14nm_2290_cfgs = {
    .has_phy_lane = true,
    .ops = {
    .enable = dsi_14nm_phy_enable,
    .disable = dsi_14nm_phy_disable,
    .pll_init = dsi_pll_14nm_init,
    .save_pll_state = dsi_14nm_pll_save_state,
    .restore_pll_state = dsi_14nm_pll_restore_state,
    },
    .min_pll_rate = VCO_MIN_RATE,
    .max_pll_rate = VCO_MAX_RATE,
    .io_start = { 0x5e94400 },
    .num_dsi_phy = 1,
    };
    const struct msm_dsi_phy_cfg dsi_phy_14nm_6150_cfgs = {
    .has_phy_lane = true,
    .regulator_data = dsi_phy_14nm_36mA_regulators,
    .num_regulators = ARRAY_SIZE(dsi_phy_14nm_36mA_regulators),
    .ops = {
    .enable = dsi_14nm_phy_enable,
    .disable = dsi_14nm_phy_disable,
    .pll_init = dsi_pll_14nm_init,
    .save_pll_state = dsi_14nm_pll_save_state,
    .restore_pll_state = dsi_14nm_pll_restore_state,
    },
    .min_pll_rate = VCO_MIN_RATE,
    .max_pll_rate = VCO_MAX_RATE,
    .io_start = { 0xae94400 },
    .num_dsi_phy = 1,
    };
