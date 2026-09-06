//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/msm/hdmi/hdmi_phy_8998.c
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
// Copyright (c) 2024 Freebox SAS
//

pub const HDMI_CORECLK_DIV: c_int = 5;
pub const HDMI_DEFAULT_REF_CLOCK: c_int = 19200000;
pub const HDMI_PLL_CMP_CNT: c_int = 1024;
pub const HDMI_PLL_POLL_MAX_READS: c_int = 100;
pub const HDMI_PLL_POLL_TIMEOUT_US: c_int = 150;
pub const HDMI_NUM_TX_CHANNEL: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_pll_8998 {
    pub pdev: *mut platform_device,
    pub clk_hw: clk_hw,
    pub rate: c_ulong,
// pll mmio base
    pub mmio_qserdes_com: *mut void __iomem,
// tx channel base
    pub mmio_qserdes_tx: [*mut void __iomem; HDMI_NUM_TX_CHANNEL],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_8998_phy_pll_reg_cfg {
    pub com_svs_mode_clk_sel: u32,
    pub com_hsclk_sel: u32,
    pub com_pll_cctrl_mode0: u32,
    pub com_pll_rctrl_mode0: u32,
    pub com_cp_ctrl_mode0: u32,
    pub com_dec_start_mode0: u32,
    pub com_div_frac_start1_mode0: u32,
    pub com_div_frac_start2_mode0: u32,
    pub com_div_frac_start3_mode0: u32,
    pub com_integloop_gain0_mode0: u32,
    pub com_integloop_gain1_mode0: u32,
    pub com_lock_cmp_en: u32,
    pub com_lock_cmp1_mode0: u32,
    pub com_lock_cmp2_mode0: u32,
    pub com_lock_cmp3_mode0: u32,
    pub com_core_clk_en: u32,
    pub com_coreclk_div_mode0: u32,
    pub tx_lx_tx_band: [u32; HDMI_NUM_TX_CHANNEL],
    pub tx_lx_tx_drv_lvl: [u32; HDMI_NUM_TX_CHANNEL],
    pub tx_lx_tx_emp_post1_lvl: [u32; HDMI_NUM_TX_CHANNEL],
    pub tx_lx_pre_driver_1: [u32; HDMI_NUM_TX_CHANNEL],
    pub tx_lx_pre_driver_2: [u32; HDMI_NUM_TX_CHANNEL],
    pub tx_lx_res_code_offset: [u32; HDMI_NUM_TX_CHANNEL],
    pub phy_mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_8998_post_divider {
    pub vco_freq: u64,
    pub hsclk_divsel: c_int,
    pub vco_ratio: c_int,
    pub tx_band_sel: c_int,
    pub half_rate_mode: c_int,
}

    static inline struct hdmi_phy *pll_get_phy(struct hdmi_pll_8998 *pll)
    {
    return platform_get_drvdata(pll.pdev);
    }
    static inline void hdmi_pll_write(struct hdmi_pll_8998 *pll, int offset,
    u32 data)
    {
    writel(data, pll.mmio_qserdes_com + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn hdmi_pll_read(pll: *mut hdmi_pll_8998, offset: c_int) -> u32 {
    static inline u32 hdmi_pll_read(struct hdmi_pll_8998 *pll, int offset)
    {
    return readl(pll.mmio_qserdes_com + offset);
    }
    static inline void hdmi_tx_chan_write(struct hdmi_pll_8998 *pll, int channel,
    int offset, int data)
    {
    writel(data, pll.mmio_qserdes_tx[channel] + offset);
    }
    static inline u32 pll_get_cpctrl(u64 frac_start, unsigned long ref_clk,
    bool gen_ssc)
    {
    if ((frac_start != 0) || gen_ssc)
    return 0x8;
    return 0x30;
    }
#[no_mangle]
pub unsafe extern "C" fn pll_get_rctrl(frac_start: u64, gen_ssc: bool) -> u32 {
    static inline u32 pll_get_rctrl(u64 frac_start, bool gen_ssc)
    {
    if ((frac_start != 0) || gen_ssc)
    return 0x16;
    return 0x18;
    }
#[no_mangle]
pub unsafe extern "C" fn pll_get_cctrl(frac_start: u64, gen_ssc: bool) -> u32 {
    static inline u32 pll_get_cctrl(u64 frac_start, bool gen_ssc)
    {
    if ((frac_start != 0) || gen_ssc)
    return 0x34;
    return 0x2;
    }
    static inline u32 pll_get_integloop_gain(u64 frac_start, u64 bclk, u32 ref_clk,
    bool gen_ssc)
    {
    let mut digclk_divsel: c_int = bclk > HDMI_DIG_FREQ_BIT_CLK_THRESHOLD ? 1 : 2;
    u64 base;
    if ((frac_start != 0) || gen_ssc)
    base = 0x3F;
    else
    base = 0xC4;
    base <<= (digclk_divsel == 2 ? 1 : 0);
    return base;
    }
#[no_mangle]
pub unsafe extern "C" fn pll_get_pll_cmp(fdata: u64, ref_clk: c_ulong) -> u32 {
    static inline u32 pll_get_pll_cmp(u64 fdata, unsigned long ref_clk)
    {
    let mut dividend: u64 = HDMI_PLL_CMP_CNT * fdata;
    let mut divisor: u32 = ref_clk * 10;
    u32 rem;
    rem = do_div(dividend, divisor);
    if (rem > (divisor >> 1))
    dividend++;
    return dividend - 1;
    }

#[no_mangle]
unsafe extern "C" fn pll_get_post_div(pd: *mut hdmi_8998_post_divider, bclk: u64) -> c_int {
    static int pll_get_post_div(struct hdmi_8998_post_divider *pd, u64 bclk)
    {
    static const u32 ratio_list[] = {1, 2, 3, 4, 5, 6, 9, 10, 12, 15, 25};
    static const u32 band_list[] = {0, 1, 2, 3};
    let mut sz_ratio: u32 const = ARRAY_SIZE(ratio_list);
    let mut sz_band: u32 const = ARRAY_SIZE(band_list);
    let mut cmp_cnt: u32 const = 1024;
    let mut th_min: u32 const = 500, th_max = 1000;
    let mut half_rate_mode: u32 = 0;
    u32 list_elements;
    int optimal_index;
    u32 i, j, k;
    let mut found_hsclk_divsel: u32 = 0, found_vco_ratio;
    u32 found_tx_band_sel;
    let mut min_freq: u64 const = HDMI_VCO_MIN_FREQ, max_freq = HDMI_VCO_MAX_FREQ;
    u64 freq_list[ARRAY_SIZE(ratio_list) * ARRAY_SIZE(band_list)];
    u64 found_vco_freq;
    u64 freq_optimal;
    find_optimal_index:
    freq_optimal = max_freq;
    optimal_index = -1;
    list_elements = 0;
    for (i = 0; i < sz_ratio; i++) {
    for (j = 0; j < sz_band; j++) {
    let mut freq: u64 = div_u64(bclk, (1 << half_rate_mode));
    freq *= (ratio_list[i] * (1 << band_list[j]));
    freq_list[list_elements++] = freq;
    }
    }
    for (k = 0; k < ARRAY_SIZE(freq_list); k++) {
    let mut clks_pll_div: u32 const = 2, core_clk_div = 5;
    let mut rng1: u32 const = 16, rng2 = 8;
    u32 th1, th2;
    u64 core_clk, rvar1, rem;
    core_clk = div_u64(freq_list[k],
    ratio_list[k / sz_band] * clks_pll_div *
    core_clk_div);
    rvar1 = HDMI_REF_CLOCK_HZ * rng1 * HDMI_MHZ_TO_HZ;
    rvar1 = div64_u64_rem(rvar1, (cmp_cnt * core_clk), &rem);
    if (rem > ((cmp_cnt * core_clk) >> 1))
    rvar1++;
    th1 = rvar1;
    rvar1 = HDMI_REF_CLOCK_HZ * rng2 * HDMI_MHZ_TO_HZ;
    rvar1 = div64_u64_rem(rvar1, (cmp_cnt * core_clk), &rem);
    if (rem > ((cmp_cnt * core_clk) >> 1))
    rvar1++;
    th2 = rvar1;
    if (freq_list[k] >= min_freq &&
    freq_list[k] <= max_freq) {
    if ((th1 >= th_min && th1 <= th_max) ||
    (th2 >= th_min && th2 <= th_max)) {
    if (freq_list[k] <= freq_optimal) {
    freq_optimal = freq_list[k];
    optimal_index = k;
    }
    }
    }
    }
    if (optimal_index == -1) {
    if (!half_rate_mode) {
    half_rate_mode = 1;
    goto find_optimal_index;
    } else {
    return -EINVAL;
    }
    } else {
    found_vco_ratio = ratio_list[optimal_index / sz_band];
    found_tx_band_sel = band_list[optimal_index % sz_band];
    found_vco_freq = freq_optimal;
    }
    switch (found_vco_ratio) {
    case 1:
    found_hsclk_divsel = 15;
    break;
    case 2:
    found_hsclk_divsel = 0;
    break;
    case 3:
    found_hsclk_divsel = 4;
    break;
    case 4:
    found_hsclk_divsel = 8;
    break;
    case 5:
    found_hsclk_divsel = 12;
    break;
    case 6:
    found_hsclk_divsel = 1;
    break;
    case 9:
    found_hsclk_divsel = 5;
    break;
    case 10:
    found_hsclk_divsel = 2;
    break;
    case 12:
    found_hsclk_divsel = 9;
    break;
    case 15:
    found_hsclk_divsel = 13;
    break;
    case 25:
    found_hsclk_divsel = 14;
    break;
    }
    pd.vco_freq = found_vco_freq;
    pd.tx_band_sel = found_tx_band_sel;
    pd.vco_ratio = found_vco_ratio;
    pd.hsclk_divsel = found_hsclk_divsel;
    return 0;
    }
    static int pll_calculate(unsigned long pix_clk, unsigned long ref_clk,
    struct hdmi_8998_phy_pll_reg_cfg *cfg)
    {
    struct hdmi_8998_post_divider pd;
    u64 bclk;
    u64 dec_start;
    u64 frac_start;
    u64 fdata;
    u32 pll_divisor;
    u32 rem;
    u32 cpctrl;
    u32 rctrl;
    u32 cctrl;
    u32 integloop_gain;
    u32 pll_cmp;
    int i, ret;
// bit clk = 10 * pix_clk
    bclk = ((u64)pix_clk) * 10;
    ret = pll_get_post_div(&pd, bclk);
    if (ret)
    return ret;
    dec_start = pd.vco_freq;
    pll_divisor = 4 * ref_clk;
    do_div(dec_start, pll_divisor);
    frac_start = pd.vco_freq * (1 << 20);
    rem = do_div(frac_start, pll_divisor);
    frac_start -= dec_start * (1 << 20);
    if (rem > (pll_divisor >> 1))
    frac_start++;
    cpctrl = pll_get_cpctrl(frac_start, ref_clk, false);
    rctrl = pll_get_rctrl(frac_start, false);
    cctrl = pll_get_cctrl(frac_start, false);
    integloop_gain = pll_get_integloop_gain(frac_start, bclk,
    ref_clk, false);
    fdata = pd.vco_freq;
    do_div(fdata, pd.vco_ratio);
    pll_cmp = pll_get_pll_cmp(fdata, ref_clk);
// Convert these values to register specific values
    if (bclk > HDMI_DIG_FREQ_BIT_CLK_THRESHOLD)
    cfg.com_svs_mode_clk_sel = 1;
    else
    cfg.com_svs_mode_clk_sel = 2;
    cfg.com_hsclk_sel = (0x20 | pd.hsclk_divsel);
    cfg.com_pll_cctrl_mode0 = cctrl;
    cfg.com_pll_rctrl_mode0 = rctrl;
    cfg.com_cp_ctrl_mode0 = cpctrl;
    cfg.com_dec_start_mode0 = dec_start;
    cfg.com_div_frac_start1_mode0 = (frac_start & 0xff);
    cfg.com_div_frac_start2_mode0 = ((frac_start & 0xff00) >> 8);
    cfg.com_div_frac_start3_mode0 = ((frac_start & 0xf0000) >> 16);
    cfg.com_integloop_gain0_mode0 = (integloop_gain & 0xff);
    cfg.com_integloop_gain1_mode0 = ((integloop_gain & 0xf00) >> 8);
    cfg.com_lock_cmp1_mode0 = (pll_cmp & 0xff);
    cfg.com_lock_cmp2_mode0 = ((pll_cmp & 0xff00) >> 8);
    cfg.com_lock_cmp3_mode0 = ((pll_cmp & 0x30000) >> 16);
    cfg.com_lock_cmp_en = 0x0;
    cfg.com_core_clk_en = 0x2c;
    cfg.com_coreclk_div_mode0 = HDMI_CORECLK_DIV;
    cfg.phy_mode = (bclk > HDMI_HIGH_FREQ_BIT_CLK_THRESHOLD) ? 0x5 : 0x4;
    for (i = 0; i < HDMI_NUM_TX_CHANNEL; i++)
    cfg.tx_lx_tx_band[i] = pd.tx_band_sel;
    if (bclk > HDMI_HIGH_FREQ_BIT_CLK_THRESHOLD) {
    cfg.tx_lx_tx_drv_lvl[0] = 0x0f;
    cfg.tx_lx_tx_drv_lvl[1] = 0x0f;
    cfg.tx_lx_tx_drv_lvl[2] = 0x0f;
    cfg.tx_lx_tx_drv_lvl[3] = 0x0f;
    cfg.tx_lx_tx_emp_post1_lvl[0] = 0x03;
    cfg.tx_lx_tx_emp_post1_lvl[1] = 0x02;
    cfg.tx_lx_tx_emp_post1_lvl[2] = 0x03;
    cfg.tx_lx_tx_emp_post1_lvl[3] = 0x00;
    cfg.tx_lx_pre_driver_1[0] = 0x00;
    cfg.tx_lx_pre_driver_1[1] = 0x00;
    cfg.tx_lx_pre_driver_1[2] = 0x00;
    cfg.tx_lx_pre_driver_1[3] = 0x00;
    cfg.tx_lx_pre_driver_2[0] = 0x1C;
    cfg.tx_lx_pre_driver_2[1] = 0x1C;
    cfg.tx_lx_pre_driver_2[2] = 0x1C;
    cfg.tx_lx_pre_driver_2[3] = 0x00;
    cfg.tx_lx_res_code_offset[0] = 0x03;
    cfg.tx_lx_res_code_offset[1] = 0x00;
    cfg.tx_lx_res_code_offset[2] = 0x00;
    cfg.tx_lx_res_code_offset[3] = 0x03;
    } else if (bclk > HDMI_DIG_FREQ_BIT_CLK_THRESHOLD) {
    cfg.tx_lx_tx_drv_lvl[0] = 0x0f;
    cfg.tx_lx_tx_drv_lvl[1] = 0x0f;
    cfg.tx_lx_tx_drv_lvl[2] = 0x0f;
    cfg.tx_lx_tx_drv_lvl[3] = 0x0f;
    cfg.tx_lx_tx_emp_post1_lvl[0] = 0x03;
    cfg.tx_lx_tx_emp_post1_lvl[1] = 0x03;
    cfg.tx_lx_tx_emp_post1_lvl[2] = 0x03;
    cfg.tx_lx_tx_emp_post1_lvl[3] = 0x00;
    cfg.tx_lx_pre_driver_1[0] = 0x00;
    cfg.tx_lx_pre_driver_1[1] = 0x00;
    cfg.tx_lx_pre_driver_1[2] = 0x00;
    cfg.tx_lx_pre_driver_1[3] = 0x00;
    cfg.tx_lx_pre_driver_2[0] = 0x16;
    cfg.tx_lx_pre_driver_2[1] = 0x16;
    cfg.tx_lx_pre_driver_2[2] = 0x16;
    cfg.tx_lx_pre_driver_2[3] = 0x18;
    cfg.tx_lx_res_code_offset[0] = 0x03;
    cfg.tx_lx_res_code_offset[1] = 0x00;
    cfg.tx_lx_res_code_offset[2] = 0x00;
    cfg.tx_lx_res_code_offset[3] = 0x00;
    } else if (bclk > HDMI_MID_FREQ_BIT_CLK_THRESHOLD) {
    cfg.tx_lx_tx_drv_lvl[0] = 0x0f;
    cfg.tx_lx_tx_drv_lvl[1] = 0x0f;
    cfg.tx_lx_tx_drv_lvl[2] = 0x0f;
    cfg.tx_lx_tx_drv_lvl[3] = 0x0f;
    cfg.tx_lx_tx_emp_post1_lvl[0] = 0x05;
    cfg.tx_lx_tx_emp_post1_lvl[1] = 0x05;
    cfg.tx_lx_tx_emp_post1_lvl[2] = 0x05;
    cfg.tx_lx_tx_emp_post1_lvl[3] = 0x00;
    cfg.tx_lx_pre_driver_1[0] = 0x00;
    cfg.tx_lx_pre_driver_1[1] = 0x00;
    cfg.tx_lx_pre_driver_1[2] = 0x00;
    cfg.tx_lx_pre_driver_1[3] = 0x00;
    cfg.tx_lx_pre_driver_2[0] = 0x0E;
    cfg.tx_lx_pre_driver_2[1] = 0x0E;
    cfg.tx_lx_pre_driver_2[2] = 0x0E;
    cfg.tx_lx_pre_driver_2[3] = 0x0E;
    cfg.tx_lx_res_code_offset[0] = 0x00;
    cfg.tx_lx_res_code_offset[1] = 0x00;
    cfg.tx_lx_res_code_offset[2] = 0x00;
    cfg.tx_lx_res_code_offset[3] = 0x00;
    } else {
    cfg.tx_lx_tx_drv_lvl[0] = 0x01;
    cfg.tx_lx_tx_drv_lvl[1] = 0x01;
    cfg.tx_lx_tx_drv_lvl[2] = 0x01;
    cfg.tx_lx_tx_drv_lvl[3] = 0x00;
    cfg.tx_lx_tx_emp_post1_lvl[0] = 0x00;
    cfg.tx_lx_tx_emp_post1_lvl[1] = 0x00;
    cfg.tx_lx_tx_emp_post1_lvl[2] = 0x00;
    cfg.tx_lx_tx_emp_post1_lvl[3] = 0x00;
    cfg.tx_lx_pre_driver_1[0] = 0x00;
    cfg.tx_lx_pre_driver_1[1] = 0x00;
    cfg.tx_lx_pre_driver_1[2] = 0x00;
    cfg.tx_lx_pre_driver_1[3] = 0x00;
    cfg.tx_lx_pre_driver_2[0] = 0x16;
    cfg.tx_lx_pre_driver_2[1] = 0x16;
    cfg.tx_lx_pre_driver_2[2] = 0x16;
    cfg.tx_lx_pre_driver_2[3] = 0x18;
    cfg.tx_lx_res_code_offset[0] = 0x00;
    cfg.tx_lx_res_code_offset[1] = 0x00;
    cfg.tx_lx_res_code_offset[2] = 0x00;
    cfg.tx_lx_res_code_offset[3] = 0x00;
    }
    return 0;
    }
    static int hdmi_8998_pll_set_clk_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct hdmi_pll_8998 *pll = hw_clk_to_pll(hw);
    struct hdmi_phy *phy = pll_get_phy(pll);
    let mut cfg: hdmi_8998_phy_pll_reg_cfg = {};
    int i, ret;
    ret = pll_calculate(rate, parent_rate, &cfg);
    if (ret) {
    DRM_ERROR("PLL calculation failed\n");
    return ret;
    }
// Initially shut down PHY
    hdmi_phy_write(phy, REG_HDMI_8998_PHY_PD_CTL, 0x0);
    udelay(500);
// Power up sequence
    hdmi_phy_write(phy, REG_HDMI_8998_PHY_PD_CTL, 0x1);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_RESETSM_CNTRL, 0x20);
    hdmi_phy_write(phy, REG_HDMI_8998_PHY_CMN_CTRL, 0x6);
    for (i = 0; i < HDMI_NUM_TX_CHANNEL; i++) {
    hdmi_tx_chan_write(pll, i,
    REG_HDMI_8998_PHY_TXn_INTERFACE_SELECT_TX_BAND,
    cfg.tx_lx_tx_band[i]);
    hdmi_tx_chan_write(pll, i,
    REG_HDMI_8998_PHY_TXn_CLKBUF_TERM_ENABLE,
    0x1);
    hdmi_tx_chan_write(pll, i,
    REG_HDMI_8998_PHY_TXn_LANE_MODE,
    0x20);
    }
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_SYSCLK_BUF_ENABLE, 0x02);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_BIAS_EN_CLKBUFLR_EN, 0x0B);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_SYSCLK_EN_SEL, 0x37);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_SYS_CLK_CTRL, 0x02);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_CLK_ENABLE1, 0x0E);
// Bypass VCO calibration
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_SVS_MODE_CLK_SEL,
    cfg.com_svs_mode_clk_sel);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_PLL_IVCO, 0x07);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_VCO_TUNE_CTRL, 0x00);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_CLK_SEL, 0x30);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_HSCLK_SEL,
    cfg.com_hsclk_sel);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_LOCK_CMP_EN,
    cfg.com_lock_cmp_en);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_PLL_CCTRL_MODE0,
    cfg.com_pll_cctrl_mode0);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_PLL_RCTRL_MODE0,
    cfg.com_pll_rctrl_mode0);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_CP_CTRL_MODE0,
    cfg.com_cp_ctrl_mode0);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_DEC_START_MODE0,
    cfg.com_dec_start_mode0);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_DIV_FRAC_START1_MODE0,
    cfg.com_div_frac_start1_mode0);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_DIV_FRAC_START2_MODE0,
    cfg.com_div_frac_start2_mode0);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_DIV_FRAC_START3_MODE0,
    cfg.com_div_frac_start3_mode0);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_INTEGLOOP_GAIN0_MODE0,
    cfg.com_integloop_gain0_mode0);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_INTEGLOOP_GAIN1_MODE0,
    cfg.com_integloop_gain1_mode0);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_LOCK_CMP1_MODE0,
    cfg.com_lock_cmp1_mode0);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_LOCK_CMP2_MODE0,
    cfg.com_lock_cmp2_mode0);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_LOCK_CMP3_MODE0,
    cfg.com_lock_cmp3_mode0);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_VCO_TUNE_MAP, 0x00);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_CORE_CLK_EN,
    cfg.com_core_clk_en);
    hdmi_pll_write(pll, REG_HDMI_8998_PHY_QSERDES_COM_CORECLK_DIV_MODE0,
    cfg.com_coreclk_div_mode0);
// TX lanes setup (TX 0/1/2/3)
    for (i = 0; i < HDMI_NUM_TX_CHANNEL; i++) {
    hdmi_tx_chan_write(pll, i,
    REG_HDMI_8998_PHY_TXn_DRV_LVL,
    cfg.tx_lx_tx_drv_lvl[i]);
    hdmi_tx_chan_write(pll, i,
    REG_HDMI_8998_PHY_TXn_EMP_POST1_LVL,
    cfg.tx_lx_tx_emp_post1_lvl[i]);
    hdmi_tx_chan_write(pll, i,
    REG_HDMI_8998_PHY_TXn_PRE_DRIVER_1,
    cfg.tx_lx_pre_driver_1[i]);
    hdmi_tx_chan_write(pll, i,
    REG_HDMI_8998_PHY_TXn_PRE_DRIVER_2,
    cfg.tx_lx_pre_driver_2[i]);
    hdmi_tx_chan_write(pll, i,
    REG_HDMI_8998_PHY_TXn_DRV_LVL_RES_CODE_OFFSET,
    cfg.tx_lx_res_code_offset[i]);
    }
    hdmi_phy_write(phy, REG_HDMI_8998_PHY_MODE, cfg.phy_mode);
    for (i = 0; i < HDMI_NUM_TX_CHANNEL; i++) {
    hdmi_tx_chan_write(pll, i,
    REG_HDMI_8998_PHY_TXn_LANE_CONFIG,
    0x10);
    }
//
// Ensure that vco configuration gets flushed to hardware before
// enabling the PLL
//
    wmb();
    pll.rate = rate;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hdmi_8998_phy_ready_status(phy: *mut hdmi_phy) -> c_int {
    static int hdmi_8998_phy_ready_status(struct hdmi_phy *phy)
    {
    let mut nb_tries: u32 = HDMI_PLL_POLL_MAX_READS;
    let mut timeout: c_ulong = HDMI_PLL_POLL_TIMEOUT_US;
    u32 status;
    let mut phy_ready: c_int = 0;
    while (nb_tries--) {
    status = hdmi_phy_read(phy, REG_HDMI_8998_PHY_STATUS);
    phy_ready = status & BIT(0);
    if (phy_ready)
    break;
    udelay(timeout);
    }
    return phy_ready;
    }
#[no_mangle]
unsafe extern "C" fn hdmi_8998_pll_lock_status(pll: *mut hdmi_pll_8998) -> c_int {
    static int hdmi_8998_pll_lock_status(struct hdmi_pll_8998 *pll)
    {
    u32 status;
    let mut nb_tries: c_int = HDMI_PLL_POLL_MAX_READS;
    let mut timeout: c_ulong = HDMI_PLL_POLL_TIMEOUT_US;
    let mut pll_locked: c_int = 0;
    while (nb_tries--) {
    status = hdmi_pll_read(pll,
    REG_HDMI_8998_PHY_QSERDES_COM_C_READY_STATUS);
    pll_locked = status & BIT(0);
    if (pll_locked)
    break;
    udelay(timeout);
    }
    return pll_locked;
    }
#[no_mangle]
unsafe extern "C" fn hdmi_8998_pll_prepare(hw: *mut clk_hw) -> c_int {
    static int hdmi_8998_pll_prepare(struct clk_hw *hw)
    {
    struct hdmi_pll_8998 *pll = hw_clk_to_pll(hw);
    struct hdmi_phy *phy = pll_get_phy(pll);
    int i, ret = 0;
    hdmi_phy_write(phy, REG_HDMI_8998_PHY_CFG, 0x1);
    udelay(100);
    hdmi_phy_write(phy, REG_HDMI_8998_PHY_CFG, 0x59);
    udelay(100);
    ret = hdmi_8998_pll_lock_status(pll);
    if (!ret)
    return ret;
    for (i = 0; i < HDMI_NUM_TX_CHANNEL; i++) {
    hdmi_tx_chan_write(pll, i,
    REG_HDMI_8998_PHY_TXn_LANE_CONFIG, 0x1F);
    }
// Ensure all registers are flushed to hardware
    wmb();
    ret = hdmi_8998_phy_ready_status(phy);
    if (!ret)
    return ret;
// Restart the retiming buffer
    hdmi_phy_write(phy, REG_HDMI_8998_PHY_CFG, 0x58);
    udelay(1);
    hdmi_phy_write(phy, REG_HDMI_8998_PHY_CFG, 0x59);
// Ensure all registers are flushed to hardware
    wmb();
    return 0;
    }
    static int hdmi_8998_pll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    req.rate = clamp_t(unsigned long, req.rate, HDMI_TMDS_CHAR_RATE_MIN_HZ,
    HDMI_2_0_TMDS_CHAR_RATE_MAX_HZ);
    return 0;
    }
    static unsigned long hdmi_8998_pll_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct hdmi_pll_8998 *pll = hw_clk_to_pll(hw);
    return pll.rate;
    }
#[no_mangle]
unsafe extern "C" fn hdmi_8998_pll_unprepare(hw: *mut clk_hw) {
    static void hdmi_8998_pll_unprepare(struct clk_hw *hw)
    {
    struct hdmi_pll_8998 *pll = hw_clk_to_pll(hw);
    struct hdmi_phy *phy = pll_get_phy(pll);
    hdmi_phy_write(phy, REG_HDMI_8998_PHY_PD_CTL, 0);
    usleep_range(100, 150);
    }
#[no_mangle]
unsafe extern "C" fn hdmi_8998_pll_is_enabled(hw: *mut clk_hw) -> c_int {
    static int hdmi_8998_pll_is_enabled(struct clk_hw *hw)
    {
    struct hdmi_pll_8998 *pll = hw_clk_to_pll(hw);
    u32 status;
    int pll_locked;
    status = hdmi_pll_read(pll, REG_HDMI_8998_PHY_QSERDES_COM_C_READY_STATUS);
    pll_locked = status & BIT(0);
    return pll_locked;
    }
    static const struct clk_ops hdmi_8998_pll_ops = {
    .set_rate = hdmi_8998_pll_set_clk_rate,
    .determine_rate = hdmi_8998_pll_determine_rate,
    .recalc_rate = hdmi_8998_pll_recalc_rate,
    .prepare = hdmi_8998_pll_prepare,
    .unprepare = hdmi_8998_pll_unprepare,
    .is_enabled = hdmi_8998_pll_is_enabled,
    };
    static const struct clk_init_data pll_init = {
    .name = "hdmipll",
    .ops = &hdmi_8998_pll_ops,
    .parent_data = (const struct clk_parent_data[]){
    { .fw_name = "xo", .name = "xo_board" },
    },
    .num_parents = 1,
    .flags = CLK_IGNORE_UNUSED,
    };
#[no_mangle]
pub unsafe extern "C" fn msm_hdmi_pll_8998_init(pdev: *mut platform_device) -> c_int {
    int msm_hdmi_pll_8998_init(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct hdmi_pll_8998 *pll;
    int ret, i;
    pll = devm_kzalloc(dev, sizeof(*pll), GFP_KERNEL);
    if (!pll)
    return -ENOMEM;
    pll.pdev = pdev;
    pll.mmio_qserdes_com = msm_ioremap(pdev, "hdmi_pll");
    if (IS_ERR(pll.mmio_qserdes_com)) {
    DRM_DEV_ERROR(dev, "failed to map pll base\n");
    return -ENOMEM;
    }
    for (i = 0; i < HDMI_NUM_TX_CHANNEL; i++) {
    char name[32];
    snprintf(name, sizeof(name), "hdmi_tx_l%d", i);
    pll.mmio_qserdes_tx[i] = msm_ioremap(pdev, name);
    if (IS_ERR(pll.mmio_qserdes_tx[i])) {
    DRM_DEV_ERROR(dev, "failed to map pll base\n");
    return -ENOMEM;
    }
    }
    pll.clk_hw.init = &pll_init;
    ret = devm_clk_hw_register(dev, &pll.clk_hw);
    if (ret) {
    DRM_DEV_ERROR(dev, "failed to register pll clock\n");
    return ret;
    }
    ret = devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get, &pll.clk_hw);
    if (ret) {
    DRM_DEV_ERROR(dev, "failed to register clk provider: %d\n", ret);
    return ret;
    }
    return 0;
    }
    static const char * const hdmi_phy_8998_reg_names[] = {
    "vddio",
    "vcca",
    };
    static const char * const hdmi_phy_8998_clk_names[] = {
    "iface", "ref", "xo",
    };
    const struct hdmi_phy_cfg msm_hdmi_phy_8998_cfg = {
    .type = MSM_HDMI_PHY_8998,
    .reg_names = hdmi_phy_8998_reg_names,
    .num_regs = ARRAY_SIZE(hdmi_phy_8998_reg_names),
    .clk_names = hdmi_phy_8998_clk_names,
    .num_clks = ARRAY_SIZE(hdmi_phy_8998_clk_names),
    };
