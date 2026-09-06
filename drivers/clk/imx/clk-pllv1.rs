//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-pllv1.c
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
// struct clk_pllv1 - IMX PLLv1 clock descriptor
//
// @hw:		clock source
// @base:	base address of pll registers
// @type:	type of IMX_PLLV1
//
// PLL clock version 1, found on i.MX1/21/25/27/31/35
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_pllv1 {
    pub hw: clk_hw,
    pub base: *mut void __iomem,
    pub type: enum imx_pllv1_type,
}

#[no_mangle]
pub unsafe extern "C" fn is_imx1_pllv1(pll: *mut clk_pllv1) -> bool {
    static inline bool is_imx1_pllv1(struct clk_pllv1 *pll)
    {
    return pll.type == IMX_PLLV1_IMX1;
    }
#[no_mangle]
pub unsafe extern "C" fn is_imx21_pllv1(pll: *mut clk_pllv1) -> bool {
    static inline bool is_imx21_pllv1(struct clk_pllv1 *pll)
    {
    return pll.type == IMX_PLLV1_IMX21;
    }
#[no_mangle]
pub unsafe extern "C" fn is_imx27_pllv1(pll: *mut clk_pllv1) -> bool {
    static inline bool is_imx27_pllv1(struct clk_pllv1 *pll)
    {
    return pll.type == IMX_PLLV1_IMX27;
    }
#[no_mangle]
pub unsafe extern "C" fn mfn_is_negative(pll: *mut clk_pllv1, mfn: c_uint) -> bool {
    static inline bool mfn_is_negative(struct clk_pllv1 *pll, unsigned int mfn)
    {
    return !is_imx1_pllv1(pll) && !is_imx21_pllv1(pll) && (mfn & MFN_SIGN);
    }
    static unsigned long clk_pllv1_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_pllv1 *pll = to_clk_pllv1(hw);
    unsigned long long ull;
    int mfn_abs;
    unsigned int mfi, mfn, mfd, pd;
    u32 reg;
    unsigned long rate;
    reg = readl(pll.base);
//
// Get the resulting clock rate from a PLL register value and the input
// frequency. PLLs with this register layout can be found on i.MX1,
// i.MX21, i.MX27 and i,MX31
//
// mfi + mfn / (mfd + 1)
// f = 2 * f_ref * --------------------
// pd + 1
//
    mfi = (reg >> 10) & 0xf;
    mfn = reg & 0x3ff;
    mfd = (reg >> 16) & 0x3ff;
    pd =  (reg >> 26) & 0xf;
    mfi = mfi <= 5 ? 5 : mfi;
    mfn_abs = mfn;
//
// On all i.MXs except i.MX1 and i.MX21 mfn is a 10bit
// 2's complements number.
// On i.MX27 the bit 9 is the sign bit.
//
    if (mfn_is_negative(pll, mfn)) {
    if (is_imx27_pllv1(pll))
    mfn_abs = mfn & MFN_MASK;
    else
    mfn_abs = BIT(MFN_BITS) - mfn;
    }
    rate = parent_rate * 2;
    rate /= pd + 1;
    ull = (unsigned long long)rate * mfn_abs;
    do_div(ull, mfd + 1);
    if (mfn_is_negative(pll, mfn))
    ull = (rate * mfi) - ull;
    else
    ull = (rate * mfi) + ull;
    return ull;
    }
    static const struct clk_ops clk_pllv1_ops = {
    .recalc_rate = clk_pllv1_recalc_rate,
    };
    struct clk_hw *imx_clk_hw_pllv1(enum imx_pllv1_type type, const char *name,
    const char *parent, void __iomem *base)
    {
    struct clk_pllv1 *pll;
    struct clk_hw *hw;
    struct clk_init_data init;
    int ret;
    pll = kmalloc_obj(*pll);
    if (!pll)
    return ERR_PTR(-ENOMEM);
    pll.base = base;
    pll.type = type;
    init.name = name;
    init.ops = &clk_pllv1_ops;
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
