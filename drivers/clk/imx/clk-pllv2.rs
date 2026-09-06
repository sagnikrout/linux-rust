//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-pllv2.c
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

// PLL Register Offsets
pub const MXC_PLL_DP_CTL: c_uint = 0x00;
pub const MXC_PLL_DP_CONFIG: c_uint = 0x04;
pub const MXC_PLL_DP_OP: c_uint = 0x08;
pub const MXC_PLL_DP_MFD: c_uint = 0x0C;
pub const MXC_PLL_DP_MFN: c_uint = 0x10;
pub const MXC_PLL_DP_MFNMINUS: c_uint = 0x14;
pub const MXC_PLL_DP_MFNPLUS: c_uint = 0x18;
pub const MXC_PLL_DP_HFS_OP: c_uint = 0x1C;
pub const MXC_PLL_DP_HFS_MFD: c_uint = 0x20;
pub const MXC_PLL_DP_HFS_MFN: c_uint = 0x24;
pub const MXC_PLL_DP_MFN_TOGC: c_uint = 0x28;
pub const MXC_PLL_DP_DESTAT: c_uint = 0x2c;
// PLL Register Bit definitions
pub const MXC_PLL_DP_CTL_MUL_CTRL: c_uint = 0x2000;
pub const MXC_PLL_DP_CTL_DPDCK0_2_EN: c_uint = 0x1000;
pub const MXC_PLL_DP_CTL_DPDCK0_2_OFFSET: c_int = 12;
pub const MXC_PLL_DP_CTL_ADE: c_uint = 0x800;
pub const MXC_PLL_DP_CTL_REF_CLK_DIV: c_uint = 0x400;

pub const MXC_PLL_DP_CTL_REF_CLK_SEL_OFFSET: c_int = 8;
pub const MXC_PLL_DP_CTL_HFSM: c_uint = 0x80;
pub const MXC_PLL_DP_CTL_PRE: c_uint = 0x40;
pub const MXC_PLL_DP_CTL_UPEN: c_uint = 0x20;
pub const MXC_PLL_DP_CTL_RST: c_uint = 0x10;
pub const MXC_PLL_DP_CTL_RCP: c_uint = 0x8;
pub const MXC_PLL_DP_CTL_PLM: c_uint = 0x4;
pub const MXC_PLL_DP_CTL_BRM0: c_uint = 0x2;
pub const MXC_PLL_DP_CTL_LRF: c_uint = 0x1;
pub const MXC_PLL_DP_CONFIG_BIST: c_uint = 0x8;
pub const MXC_PLL_DP_CONFIG_SJC_CE: c_uint = 0x4;
pub const MXC_PLL_DP_CONFIG_AREN: c_uint = 0x2;
pub const MXC_PLL_DP_CONFIG_LDREQ: c_uint = 0x1;
pub const MXC_PLL_DP_OP_MFI_OFFSET: c_int = 4;

pub const MXC_PLL_DP_OP_PDF_OFFSET: c_int = 0;
pub const MXC_PLL_DP_OP_PDF_MASK: c_uint = 0xF;
pub const MXC_PLL_DP_MFD_OFFSET: c_int = 0;
pub const MXC_PLL_DP_MFD_MASK: c_uint = 0x07FFFFFF;
pub const MXC_PLL_DP_MFN_OFFSET: c_uint = 0x0;
pub const MXC_PLL_DP_MFN_MASK: c_uint = 0x07FFFFFF;

pub const MXC_PLL_DP_MFN_TOGC_CNT_OFFSET: c_uint = 0x0;
pub const MXC_PLL_DP_MFN_TOGC_CNT_MASK: c_uint = 0xFFFF;

pub const MXC_PLL_DP_DESTAT_MFN: c_uint = 0x07FFFFFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_pllv2 {
    pub hw: clk_hw,
    pub base: *mut void __iomem,
}

    static unsigned long __clk_pllv2_recalc_rate(unsigned long parent_rate,
    u32 dp_ctl, u32 dp_op, u32 dp_mfd, u32 dp_mfn)
    {
    long mfi, mfn, mfd, pdf, ref_clk;
    unsigned long dbl;
    u64 temp;
    dbl = dp_ctl & MXC_PLL_DP_CTL_DPDCK0_2_EN;
    pdf = dp_op & MXC_PLL_DP_OP_PDF_MASK;
    mfi = (dp_op & MXC_PLL_DP_OP_MFI_MASK) >> MXC_PLL_DP_OP_MFI_OFFSET;
    mfi = (mfi <= 5) ? 5 : mfi;
    mfd = dp_mfd & MXC_PLL_DP_MFD_MASK;
    mfn = dp_mfn & MXC_PLL_DP_MFN_MASK;
    mfn = sign_extend32(mfn, 26);
    ref_clk = 2 * parent_rate;
    if (dbl != 0)
    ref_clk *= 2;
    ref_clk /= (pdf + 1);
    temp = (u64) ref_clk * abs(mfn);
    do_div(temp, mfd + 1);
    if (mfn < 0)
    temp = (ref_clk * mfi) - temp;
    else
    temp = (ref_clk * mfi) + temp;
    return temp;
    }
    static unsigned long clk_pllv2_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    u32 dp_op, dp_mfd, dp_mfn, dp_ctl;
    void __iomem *pllbase;
    struct clk_pllv2 *pll = to_clk_pllv2(hw);
    pllbase = pll.base;
    dp_ctl = __raw_readl(pllbase + MXC_PLL_DP_CTL);
    dp_op = __raw_readl(pllbase + MXC_PLL_DP_OP);
    dp_mfd = __raw_readl(pllbase + MXC_PLL_DP_MFD);
    dp_mfn = __raw_readl(pllbase + MXC_PLL_DP_MFN);
    return __clk_pllv2_recalc_rate(parent_rate, dp_ctl, dp_op, dp_mfd, dp_mfn);
    }
    static int __clk_pllv2_set_rate(unsigned long rate, unsigned long parent_rate,
    u32 *dp_op, u32 *dp_mfd, u32 *dp_mfn)
    {
    u32 reg;
    long mfi, pdf, mfn, mfd = 999999;
    u64 temp64;
    unsigned long quad_parent_rate;
    quad_parent_rate = 4 * parent_rate;
    pdf = mfi = -1;
    while (++pdf < 16 && mfi < 5)
    mfi = rate * (pdf+1) / quad_parent_rate;
    if (mfi > 15)
    return -EINVAL;
    pdf--;
    temp64 = rate * (pdf + 1) - quad_parent_rate * mfi;
    do_div(temp64, quad_parent_rate / 1000000);
    mfn = (long)temp64;
    reg = mfi << 4 | pdf;
// dp_op = reg;
// dp_mfd = mfd;
// dp_mfn = mfn;
    return 0;
    }
    static int clk_pllv2_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_pllv2 *pll = to_clk_pllv2(hw);
    void __iomem *pllbase;
    u32 dp_ctl, dp_op, dp_mfd, dp_mfn;
    int ret;
    pllbase = pll.base;
    ret = __clk_pllv2_set_rate(rate, parent_rate, &dp_op, &dp_mfd, &dp_mfn);
    if (ret)
    return ret;
    dp_ctl = __raw_readl(pllbase + MXC_PLL_DP_CTL);
// use dpdck0_2
    __raw_writel(dp_ctl | 0x1000L, pllbase + MXC_PLL_DP_CTL);
    __raw_writel(dp_op, pllbase + MXC_PLL_DP_OP);
    __raw_writel(dp_mfd, pllbase + MXC_PLL_DP_MFD);
    __raw_writel(dp_mfn, pllbase + MXC_PLL_DP_MFN);
    return 0;
    }
    static int clk_pllv2_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    u32 dp_op, dp_mfd, dp_mfn;
    int ret;
    ret = __clk_pllv2_set_rate(req.rate, req.best_parent_rate, &dp_op,
    &dp_mfd, &dp_mfn);
    if (ret) {
    req.rate = ret;
    return 0;
    }
    req.rate = __clk_pllv2_recalc_rate(req.best_parent_rate,
    MXC_PLL_DP_CTL_DPDCK0_2_EN, dp_op,
    dp_mfd, dp_mfn);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_pllv2_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_pllv2_prepare(struct clk_hw *hw)
    {
    struct clk_pllv2 *pll = to_clk_pllv2(hw);
    u32 reg;
    void __iomem *pllbase;
    let mut i: c_int = 0;
    pllbase = pll.base;
    reg = __raw_readl(pllbase + MXC_PLL_DP_CTL) | MXC_PLL_DP_CTL_UPEN;
    __raw_writel(reg, pllbase + MXC_PLL_DP_CTL);
// Wait for lock
    do {
    reg = __raw_readl(pllbase + MXC_PLL_DP_CTL);
    if (reg & MXC_PLL_DP_CTL_LRF)
    break;
    udelay(1);
    } while (++i < MAX_DPLL_WAIT_TRIES);
    if (i == MAX_DPLL_WAIT_TRIES) {
    pr_err("MX5: pll locking failed\n");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_pllv2_unprepare(hw: *mut clk_hw) {
    static void clk_pllv2_unprepare(struct clk_hw *hw)
    {
    struct clk_pllv2 *pll = to_clk_pllv2(hw);
    u32 reg;
    void __iomem *pllbase;
    pllbase = pll.base;
    reg = __raw_readl(pllbase + MXC_PLL_DP_CTL) & ~MXC_PLL_DP_CTL_UPEN;
    __raw_writel(reg, pllbase + MXC_PLL_DP_CTL);
    }
    static const struct clk_ops clk_pllv2_ops = {
    .prepare = clk_pllv2_prepare,
    .unprepare = clk_pllv2_unprepare,
    .recalc_rate = clk_pllv2_recalc_rate,
    .determine_rate = clk_pllv2_determine_rate,
    .set_rate = clk_pllv2_set_rate,
    };
    struct clk_hw *imx_clk_hw_pllv2(const char *name, const char *parent,
    void __iomem *base)
    {
    struct clk_pllv2 *pll;
    struct clk_hw *hw;
    struct clk_init_data init;
    int ret;
    pll = kzalloc_obj(*pll);
    if (!pll)
    return ERR_PTR(-ENOMEM);
    pll.base = base;
    init.name = name;
    init.ops = &clk_pllv2_ops;
    init.flags = 0;
    init.parent_names = &parent;
    init.num_parents = 1;
    pll.hw.init = &init;
    hw = &pll.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if (ret) {
    kfree(pll);
    return ERR_PTR(ret);
    }
    return hw;
    }
