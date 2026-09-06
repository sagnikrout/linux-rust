//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-sp7021.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (C) Sunplus Technology Co., Ltd.
// All rights reserved.
//

// special div_width values for PLLTV/PLLA
pub const DIV_TV: c_int = 33;
pub const DIV_A: c_int = 34;
// PLLTV parameters
    enum {
    SEL_FRA,
    SDM_MOD,
    PH_SEL,
    NFRA,
    DIVR,
    DIVN,
    DIVM,
    P_MAX
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sp_pll {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub /: *mut *mut spinlock_t lock; / lock for reg,
    pub div_shift: c_int,
    pub div_width: c_int,
    pub /: *mut *mut int pd_bit; / power down bit idx,
    pub /: *mut *mut int bp_bit; / bypass bit idx,
    pub /: *mut *mut unsigned long brate; / base rate, TODO: replace brate with muldiv,
    pub /: *mut *mut u32 p[P_MAX]; / for hold PLLTV/PLLA parameters,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sp_clk_gate_info {
    pub /: *mut *mut u16 reg; / reg_index_shift,
    pub /: *mut *mut u16 ext_parent; / parent is extclk,
}

    static const struct sp_clk_gate_info sp_clk_gates[] = {
    { 0x02 },
    { 0x05 },
    { 0x06 },
    { 0x07 },
    { 0x09 },
    { 0x0b, 1 },
    { 0x0f, 1 },
    { 0x14 },
    { 0x15 },
    { 0x16 },
    { 0x17 },
    { 0x18, 1 },
    { 0x19, 1 },
    { 0x1a, 1 },
    { 0x1b, 1 },
    { 0x1c, 1 },
    { 0x1d, 1 },
    { 0x1e },
    { 0x1f, 1 },
    { 0x20 },
    { 0x21 },
    { 0x22 },
    { 0x23 },
    { 0x24 },
    { 0x25 },
    { 0x26 },
    { 0x2a },
    { 0x2b },
    { 0x2d },
    { 0x2e },
    { 0x30 },
    { 0x31 },
    { 0x32 },
    { 0x33 },
    { 0x3d },
    { 0x3e },
    { 0x3f },
    { 0x42 },
    { 0x44 },
    { 0x4b },
    { 0x4c },
    { 0x4d },
    { 0x4e },
    { 0x4f },
    { 0x50 },
    { 0x55 },
    { 0x60 },
    { 0x61 },
    { 0x6a },
    { 0x73 },
    { 0x86 },
    { 0x8a },
    { 0x8b },
    { 0x8d },
    { 0x8e },
    { 0x8f },
    { 0x90 },
    { 0x92 },
    { 0x93 },
    { 0x95 },
    { 0x96 },
    { 0x97 },
    { 0x98 },
    { 0x99 },
    };

// PLL_TV
// TODO: set proper FVCO range

#[no_mangle]
unsafe extern "C" fn plltv_integer_div(clk: *mut sp_pll, freq: c_ulong) -> c_long {
    static long plltv_integer_div(struct sp_pll *clk, unsigned long freq)
    {
// valid m values: 27M must be divisible by m
    static const u32 m_table[] = {
    1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 16, 18, 20, 24, 25, 27, 30, 32
    };
    u32 m, n, r;
    unsigned long fvco, nf;
    long ret;
    freq = clamp(freq, F_MIN, F_MAX);
// DIVR 0~3
    for (r = 0; r <= 3; r++) {
    fvco = freq << r;
    if (fvco <= FVCO_MAX)
    break;
    }
// DIVM
    for (m = 0; m < ARRAY_SIZE(m_table); m++) {
    nf = fvco * m_table[m];
    n = nf / F_27M;
    if ((n * F_27M) == nf)
    break;
    }
    if (m >= ARRAY_SIZE(m_table)) {
    ret = -EINVAL;
    goto err_not_found;
    }
// save parameters
    clk.p[SEL_FRA] = 0;
    clk.p[DIVR]    = r;
    clk.p[DIVN]    = n;
    clk.p[DIVM]    = m_table[m];
    return freq;
    err_not_found:
    pr_err("%s: %s freq:%lu not found a valid setting\n",
    __func__, clk_hw_get_name(&clk.hw), freq);
    return ret;
    }
// parameters for PLLTV fractional divider
    static const u32 pt[][5] = {
// conventional fractional
    {
    1,			/* factor */
    5,			/* 5 * p0 (nint) */
    1,			/* 1 * p0 */
    F_27M,			/* F_27M / p0 */
    1,			/* p0 / p2 */
    },
// phase rotation
    {
    10,			/* factor */
    54,			/* 5.4 * p0 (nint) */
    2,			/* 0.2 * p0 */
    F_27M / 10,		/* F_27M / p0 */
    5,			/* p0 / p2 */
    },
    };
    static const u32 sdm_mod_vals[] = { 91, 55 };
#[no_mangle]
unsafe extern "C" fn plltv_fractional_div(clk: *mut sp_pll, freq: c_ulong) -> c_long {
    static long plltv_fractional_div(struct sp_pll *clk, unsigned long freq)
    {
    u32 m, r;
    u32 nint, nfra;
    let mut df_quotient_min: u32 = 210000000;
    let mut df_remainder_min: u32 = 0;
    unsigned long fvco, nf, f, fout = 0;
    int sdm, ph;
    freq = clamp(freq, F_MIN, F_MAX);
// DIVR 0~3
    for (r = 0; r <= 3; r++) {
    fvco = freq << r;
    if (fvco <= FVCO_MAX)
    break;
    }
    f = F_27M >> r;
// PH_SEL
    for (ph = ARRAY_SIZE(pt) - 1; ph >= 0; ph--) {
    const u32 *pp = pt[ph];
// SDM_MOD
    for (sdm = 0; sdm < ARRAY_SIZE(sdm_mod_vals); sdm++) {
    let mut mod: u32 = sdm_mod_vals[sdm];
// DIVM 1~32
    for (m = 1; m <= 32; m++) {
    u32 df; /* diff freq */
    u32 df_quotient, df_remainder;
    nf = fvco * m;
    nint = nf / pp[3];
    if (nint < pp[1])
    continue;
    if (nint > pp[1])
    break;
    nfra = (((nf % pp[3]) * mod * pp[4]) + (F_27M / 2)) / F_27M;
    if (nfra) {
    let mut df0: u32 = f * (nint + pp[2]) / pp[0];
    let mut df1: u32 = f * (mod - nfra) / mod / pp[4];
    df = df0 - df1;
    } else {
    df = f * (nint) / pp[0];
    }
    df_quotient  = df / m;
    df_remainder = ((df % m) * 1000) / m;
    if (freq > df_quotient) {
    df_quotient  = freq - df_quotient - 1;
    df_remainder = 1000 - df_remainder;
    } else {
    df_quotient = df_quotient - freq;
    }
    if (df_quotient_min > df_quotient ||
    (df_quotient_min == df_quotient &&
    df_remainder_min > df_remainder)) {
// found a closer freq, save parameters
    clk.p[SEL_FRA] = 1;
    clk.p[SDM_MOD] = sdm;
    clk.p[PH_SEL]  = ph;
    clk.p[NFRA]    = nfra;
    clk.p[DIVR]    = r;
    clk.p[DIVM]    = m;
    fout = df / m;
    df_quotient_min = df_quotient;
    df_remainder_min = df_remainder;
    }
    }
    }
    }
    if (!fout) {
    pr_err("%s: %s freq:%lu not found a valid setting\n",
    __func__, clk_hw_get_name(&clk.hw), freq);
    return -EINVAL;
    }
    return fout;
    }
#[no_mangle]
unsafe extern "C" fn plltv_div(clk: *mut sp_pll, freq: c_ulong) -> c_long {
    static long plltv_div(struct sp_pll *clk, unsigned long freq)
    {
    if (freq % 100)
    return plltv_fractional_div(clk, freq);
    return plltv_integer_div(clk, freq);
    }
#[no_mangle]
unsafe extern "C" fn plltv_set_rate(clk: *mut sp_pll) -> c_int {
    static int plltv_set_rate(struct sp_pll *clk)
    {
    unsigned long flags;
    u32 r0, r1, r2;
    r0  = BIT(clk.bp_bit + 16);
    r0 |= FIELD_PREP_WM16(MASK_SEL_FRA, clk.p[SEL_FRA]);
    r0 |= FIELD_PREP_WM16(MASK_SDM_MOD, clk.p[SDM_MOD]);
    r0 |= FIELD_PREP_WM16(MASK_PH_SEL, clk.p[PH_SEL]);
    r0 |= FIELD_PREP_WM16(MASK_NFRA, clk.p[NFRA]);
    r1  = FIELD_PREP_WM16(MASK_DIVR, clk.p[DIVR]);
    r2  = FIELD_PREP_WM16(MASK_DIVN, clk.p[DIVN] - 1);
    r2 |= FIELD_PREP_WM16(MASK_DIVM, clk.p[DIVM] - 1);
    spin_lock_irqsave(&clk.lock, flags);
    writel(r0, clk.reg);
    writel(r1, clk.reg + 4);
    writel(r2, clk.reg + 8);
    spin_unlock_irqrestore(&clk.lock, flags);
    return 0;
    }
// PLL_A
// from Q628_PLLs_REG_setting.xlsx
    static const struct {
    u32 rate;
    u32 regs[5];
    } pa[] = {
    {
    .rate = 135475200,
    .regs = {
    0x4801,
    0x02df,
    0x248f,
    0x0211,
    0x33e9
    }
    },
    {
    .rate = 147456000,
    .regs = {
    0x4801,
    0x1adf,
    0x2490,
    0x0349,
    0x33e9
    }
    },
    {
    .rate = 196608000,
    .regs = {
    0x4801,
    0x42ef,
    0x2495,
    0x01c6,
    0x33e9
    }
    },
    };
#[no_mangle]
unsafe extern "C" fn plla_set_rate(clk: *mut sp_pll) -> c_int {
    static int plla_set_rate(struct sp_pll *clk)
    {
    const u32 *pp = pa[clk.p[0]].regs;
    unsigned long flags;
    int i;
    spin_lock_irqsave(&clk.lock, flags);
    for (i = 0; i < ARRAY_SIZE(pa.regs); i++)
    writel(0xffff0000 | pp[i], clk.reg + (i * 4));
    spin_unlock_irqrestore(&clk.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn plla_round_rate(clk: *mut sp_pll, rate: c_ulong) -> c_long {
    static long plla_round_rate(struct sp_pll *clk, unsigned long rate)
    {
    let mut i: c_int = ARRAY_SIZE(pa);
    while (--i) {
    if (rate >= pa[i].rate)
    break;
    }
    clk.p[0] = i;
    return pa[i].rate;
    }
// SP_PLL
#[no_mangle]
unsafe extern "C" fn sp_pll_calc_div(clk: *mut sp_pll, rate: c_ulong) -> c_long {
    static long sp_pll_calc_div(struct sp_pll *clk, unsigned long rate)
    {
    u32 fbdiv;
    let mut max: u32 = 1 << clk.div_width;
    fbdiv = DIV_ROUND_CLOSEST(rate, clk.brate);
    if (fbdiv > max)
    fbdiv = max;
    return fbdiv;
    }
    static int sp_pll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct sp_pll *clk = to_sp_pll(hw);
    long ret;
    if (req.rate == req.best_parent_rate) {
    ret = req.best_parent_rate; /* bypass */
    } else if (clk.div_width == DIV_A) {
    ret = plla_round_rate(clk, req.rate);
    } else if (clk.div_width == DIV_TV) {
    ret = plltv_div(clk, req.rate);
    if (ret < 0)
    ret = req.best_parent_rate;
    } else {
    ret = sp_pll_calc_div(clk, req.rate) * clk.brate;
    }
    req.rate = ret;
    return 0;
    }
    static unsigned long sp_pll_recalc_rate(struct clk_hw *hw,
    unsigned long prate)
    {
    struct sp_pll *clk = to_sp_pll(hw);
    let mut reg: u32 = readl(clk.reg);
    unsigned long ret;
    if (reg & BIT(clk.bp_bit)) {
    ret = prate; /* bypass */
    } else if (clk.div_width == DIV_A) {
    ret = pa[clk.p[0]].rate;
    } else if (clk.div_width == DIV_TV) {
    u32 m, r, reg2;
    r = FIELD_GET(MASK_DIVR, readl(clk.reg + 4));
    reg2 = readl(clk.reg + 8);
    m = FIELD_GET(MASK_DIVM, reg2) + 1;
    if (reg & MASK_SEL_FRA) {
// fractional divider
    let mut sdm: u32 = FIELD_GET(MASK_SDM_MOD, reg);
    let mut ph: u32 = FIELD_GET(MASK_PH_SEL, reg);
    let mut nfra: u32 = FIELD_GET(MASK_NFRA, reg);
    const u32 *pp = pt[ph];
    unsigned long r0, r1;
    ret = prate >> r;
    r0  = ret * (pp[1] + pp[2]) / pp[0];
    r1  = ret * (sdm_mod_vals[sdm] - nfra) / sdm_mod_vals[sdm] / pp[4];
    ret = (r0 - r1) / m;
    } else {
// integer divider
    let mut n: u32 = FIELD_GET(MASK_DIVN, reg2) + 1;
    ret = (prate / m * n) >> r;
    }
    } else {
    let mut fbdiv: u32 = ((reg >> clk.div_shift) & ((1 << clk.div_width) - 1)) + 1;
    ret = clk.brate * fbdiv;
    }
    return ret;
    }
    static int sp_pll_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long prate)
    {
    struct sp_pll *clk = to_sp_pll(hw);
    unsigned long flags;
    u32 reg;
    reg = BIT(clk.bp_bit + 16); /* HIWORD_MASK */
    if (rate == prate) {
    reg |= BIT(clk.bp_bit); /* bypass */
    } else if (clk.div_width == DIV_A) {
    return plla_set_rate(clk);
    } else if (clk.div_width == DIV_TV) {
    return plltv_set_rate(clk);
    } else if (clk.div_width) {
    let mut fbdiv: u32 = sp_pll_calc_div(clk, rate);
    let mut mask: u32 = GENMASK(clk.div_shift + clk.div_width - 1, clk.div_shift);
    reg |= mask << 16;
    reg |= ((fbdiv - 1) << clk.div_shift) & mask;
    }
    spin_lock_irqsave(&clk.lock, flags);
    writel(reg, clk.reg);
    spin_unlock_irqrestore(&clk.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sp_pll_enable(hw: *mut clk_hw) -> c_int {
    static int sp_pll_enable(struct clk_hw *hw)
    {
    struct sp_pll *clk = to_sp_pll(hw);
    writel(BIT(clk.pd_bit + 16) | BIT(clk.pd_bit), clk.reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sp_pll_disable(hw: *mut clk_hw) {
    static void sp_pll_disable(struct clk_hw *hw)
    {
    struct sp_pll *clk = to_sp_pll(hw);
    writel(BIT(clk.pd_bit + 16), clk.reg);
    }
#[no_mangle]
unsafe extern "C" fn sp_pll_is_enabled(hw: *mut clk_hw) -> c_int {
    static int sp_pll_is_enabled(struct clk_hw *hw)
    {
    struct sp_pll *clk = to_sp_pll(hw);
    return readl(clk.reg) & BIT(clk.pd_bit);
    }
    static const struct clk_ops sp_pll_ops = {
    .enable = sp_pll_enable,
    .disable = sp_pll_disable,
    .is_enabled = sp_pll_is_enabled,
    .determine_rate = sp_pll_determine_rate,
    .recalc_rate = sp_pll_recalc_rate,
    .set_rate = sp_pll_set_rate
    };
    static const struct clk_ops sp_pll_sub_ops = {
    .enable = sp_pll_enable,
    .disable = sp_pll_disable,
    .is_enabled = sp_pll_is_enabled,
    .recalc_rate = sp_pll_recalc_rate,
    };
    static struct clk_hw *sp_pll_register(struct device *dev, const char *name,
    const struct clk_parent_data *parent_data,
    void __iomem *reg, int pd_bit, int bp_bit,
    unsigned long brate, int shift, int width,
    unsigned long flags)
    {
    struct sp_pll *pll;
    struct clk_hw *hw;
    struct clk_init_data initd = {
    .name = name,
    .parent_data = parent_data,
    .ops = (bp_bit >= 0) ? &sp_pll_ops : &sp_pll_sub_ops,
    .num_parents = 1,
    .flags = flags,
    };
    int ret;
    pll = devm_kzalloc(dev, sizeof(*pll), GFP_KERNEL);
    if (!pll)
    return ERR_PTR(-ENOMEM);
    pll.hw.init = &initd;
    pll.reg = reg;
    pll.pd_bit = pd_bit;
    pll.bp_bit = bp_bit;
    pll.brate = brate;
    pll.div_shift = shift;
    pll.div_width = width;
    spin_lock_init(&pll.lock);
    hw = &pll.hw;
    ret = devm_clk_hw_register(dev, hw);
    if (ret)
    return ERR_PTR(ret);
    return hw;
    }

#[no_mangle]
unsafe extern "C" fn sp7021_clk_probe(pdev: *mut platform_device) -> c_int {
    static int sp7021_clk_probe(struct platform_device *pdev)
    {
    static const u32 sp_clken[] = {
    0x67ef, 0x03ff, 0xff03, 0xfff0, 0x0004, /* G0.1~5  */
    0x0000, 0x8000, 0xffff, 0x0040, 0x0000, /* G0.6~10 */
    };
    static struct clk_parent_data pd_ext, pd_sys, pd_e;
    struct device *dev = &pdev.dev;
    void __iomem *clk_base, *pll_base, *sys_base;
    struct clk_hw_onecell_data *clk_data;
    struct clk_hw **hws;
    int i;
    clk_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(clk_base))
    return PTR_ERR(clk_base);
    pll_base = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(pll_base))
    return PTR_ERR(pll_base);
    sys_base = devm_platform_ioremap_resource(pdev, 2);
    if (IS_ERR(sys_base))
    return PTR_ERR(sys_base);
// enable default clks
    for (i = 0; i < ARRAY_SIZE(sp_clken); i++)
    writel((sp_clken[i] << 16) | sp_clken[i], clk_base + i * 4);
    clk_data = devm_kzalloc(dev, struct_size(clk_data, hws, CLK_MAX),
    GFP_KERNEL);
    if (!clk_data)
    return -ENOMEM;
    clk_data.num = CLK_MAX;
    hws = clk_data.hws;
    pd_ext.index = 0;
// PLLs
    hws[PLL_A] = sp_pll_register(dev, "plla", &pd_ext, PLLA_CTL,
    11, 12, 27000000, 0, DIV_A, 0);
    if (IS_ERR(hws[PLL_A]))
    return PTR_ERR(hws[PLL_A]);
    hws[PLL_E] = sp_pll_register(dev, "plle", &pd_ext, PLLE_CTL,
    6, 2, 50000000, 0, 0, 0);
    if (IS_ERR(hws[PLL_E]))
    return PTR_ERR(hws[PLL_E]);
    pd_e.hw = hws[PLL_E];
    hws[PLL_E_2P5] = sp_pll_register(dev, "plle_2p5", &pd_e, PLLE_CTL,
    13, -1, 2500000, 0, 0, 0);
    if (IS_ERR(hws[PLL_E_2P5]))
    return PTR_ERR(hws[PLL_E_2P5]);
    hws[PLL_E_25] = sp_pll_register(dev, "plle_25", &pd_e, PLLE_CTL,
    12, -1, 25000000, 0, 0, 0);
    if (IS_ERR(hws[PLL_E_25]))
    return PTR_ERR(hws[PLL_E_25]);
    hws[PLL_E_112P5] = sp_pll_register(dev, "plle_112p5", &pd_e, PLLE_CTL,
    11, -1, 112500000, 0, 0, 0);
    if (IS_ERR(hws[PLL_E_112P5]))
    return PTR_ERR(hws[PLL_E_112P5]);
    hws[PLL_F] = sp_pll_register(dev, "pllf", &pd_ext, PLLF_CTL,
    0, 10, 13500000, 1, 4, 0);
    if (IS_ERR(hws[PLL_F]))
    return PTR_ERR(hws[PLL_F]);
    hws[PLL_TV] = sp_pll_register(dev, "plltv", &pd_ext, PLLTV_CTL,
    0, 15, 27000000, 0, DIV_TV, 0);
    if (IS_ERR(hws[PLL_TV]))
    return PTR_ERR(hws[PLL_TV]);
    hws[PLL_TV_A] = devm_clk_hw_register_divider(dev, "plltv_a", "plltv", 0,
    PLLTV_CTL + 4, 5, 1,
    CLK_DIVIDER_POWER_OF_TWO,
    &to_sp_pll(hws[PLL_TV]).lock);
    if (IS_ERR(hws[PLL_TV_A]))
    return PTR_ERR(hws[PLL_TV_A]);
// system clock, should not be disabled
    hws[PLL_SYS] = sp_pll_register(dev, "pllsys", &pd_ext, sys_base,
    10, 9, 13500000, 0, 4, CLK_IS_CRITICAL);
    if (IS_ERR(hws[PLL_SYS]))
    return PTR_ERR(hws[PLL_SYS]);
    pd_sys.hw = hws[PLL_SYS];
// gates
    for (i = 0; i < ARRAY_SIZE(sp_clk_gates); i++) {
    char name[10];
    let mut j: u32 = sp_clk_gates[i].reg;
    struct clk_parent_data *pd = sp_clk_gates[i].ext_parent ? &pd_ext : &pd_sys;
    sprintf(name, "%02d_0x%02x", i, j);
    hws[i] = devm_clk_hw_register_gate_parent_data(dev, name, pd, 0,
    clk_base + (j >> 4) * 4,
    j & 0x0f,
    CLK_GATE_HIWORD_MASK,
    core::ptr::null_mut());
    if (IS_ERR(hws[i]))
    return PTR_ERR(hws[i]);
    }
    return devm_of_clk_add_hw_provider(dev, of_clk_hw_onecell_get, clk_data);
    }
    static const struct of_device_id sp7021_clk_dt_ids[] = {
    { .compatible = "sunplus,sp7021-clkc" },
    { }
    };
    MODULE_DEVICE_TABLE(of, sp7021_clk_dt_ids);
    static struct platform_driver sp7021_clk_driver = {
    .probe  = sp7021_clk_probe,
    .driver = {
    .name = "sp7021-clk",
    .of_match_table = sp7021_clk_dt_ids,
    },
    };
    module_platform_driver(sp7021_clk_driver);
    MODULE_AUTHOR("Sunplus Technology");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Clock driver for Sunplus SP7021 SoC");
