//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-vt8500.c
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
// Clock implementation for VIA/Wondermedia SoC's
// Copyright (C) 2012 Tony Prisk <linux@prisktech.co.nz>
//

pub const LEGACY_PMC_BASE: c_uint = 0xD8130000;
// All clocks share the same lock as none can be changed concurrently
    static DEFINE_SPINLOCK(_lock);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_device {
    pub hw: clk_hw,
    pub div_reg: *mut void __iomem,
    pub div_mask: c_uint,
    pub en_reg: *mut void __iomem,
    pub en_bit: c_int,
    pub lock: *mut spinlock_t,
}

//
// Add new PLL_TYPE_x definitions here as required. Use the first known model
// to support the new type as the name.
// Add case statements to vtwm_pll_recalc_rate(), vtwm_pll_round_round() and
// vtwm_pll_set_rate() to handle the new PLL_TYPE_x
//
pub const PLL_TYPE_VT8500: c_int = 0;
pub const PLL_TYPE_WM8650: c_int = 1;
pub const PLL_TYPE_WM8750: c_int = 2;
pub const PLL_TYPE_WM8850: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_pll {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub lock: *mut spinlock_t,
    pub type: c_int,
}

    static void __iomem *pmc_base;
#[no_mangle]
unsafe extern "C" fn vtwm_set_pmc_base() -> __init void {
    static __init void vtwm_set_pmc_base(void)
    {
    struct device_node *np =
    of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "via,vt8500-pmc");
    if (np)
    pmc_base = of_iomap(np, 0);
    else
    pmc_base = ioremap(LEGACY_PMC_BASE, 0x1000);
    of_node_put(np);
    if (!pmc_base)
    pr_err("%s:of_iomap(pmc) failed\n", __func__);
    }

pub const VT8500_PMC_BUSY_MASK: c_uint = 0x18;
#[no_mangle]
unsafe extern "C" fn vt8500_pmc_wait_busy() {
    static void vt8500_pmc_wait_busy(void)
    {
    while (readl(pmc_base) & VT8500_PMC_BUSY_MASK)
    cpu_relax();
    }
#[no_mangle]
unsafe extern "C" fn vt8500_dclk_enable(hw: *mut clk_hw) -> c_int {
    static int vt8500_dclk_enable(struct clk_hw *hw)
    {
    struct clk_device *cdev = to_clk_device(hw);
    u32 en_val;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(cdev.lock, flags);
    en_val = readl(cdev.en_reg);
    en_val |= BIT(cdev.en_bit);
    writel(en_val, cdev.en_reg);
    spin_unlock_irqrestore(cdev.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vt8500_dclk_disable(hw: *mut clk_hw) {
    static void vt8500_dclk_disable(struct clk_hw *hw)
    {
    struct clk_device *cdev = to_clk_device(hw);
    u32 en_val;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(cdev.lock, flags);
    en_val = readl(cdev.en_reg);
    en_val &= ~BIT(cdev.en_bit);
    writel(en_val, cdev.en_reg);
    spin_unlock_irqrestore(cdev.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn vt8500_dclk_is_enabled(hw: *mut clk_hw) -> c_int {
    static int vt8500_dclk_is_enabled(struct clk_hw *hw)
    {
    struct clk_device *cdev = to_clk_device(hw);
    let mut en_val: u32 = (readl(cdev.en_reg) & BIT(cdev.en_bit));
    return en_val ? 1 : 0;
    }
    static unsigned long vt8500_dclk_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_device *cdev = to_clk_device(hw);
    let mut div: u32 = readl(cdev.div_reg) & cdev.div_mask;
// Special case for SDMMC devices
    if ((cdev.div_mask == 0x3F) && (div & BIT(5)))
    div = 64 * (div & 0x1f);
// div == 0 is actually the highest divisor
    if (div == 0)
    div = (cdev.div_mask + 1);
    return parent_rate / div;
    }
    static int vt8500_dclk_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_device *cdev = to_clk_device(hw);
    u32 divisor;
    if (req.rate == 0)
    return 0;
    divisor = req.best_parent_rate / req.rate;
// If prate / rate would be decimal, incr the divisor
    if (req.rate * divisor < req.best_parent_rate)
    divisor++;
//
// If this is a request for SDMMC we have to adjust the divisor
// when >31 to use the fixed predivisor
//
    if ((cdev.div_mask == 0x3F) && (divisor > 31))
    divisor = 64 * ((divisor / 64) + 1);
    req.rate = req.best_parent_rate / divisor;
    return 0;
    }
    static int vt8500_dclk_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_device *cdev = to_clk_device(hw);
    u32 divisor;
    let mut flags: c_ulong = 0;
    if (rate == 0)
    return 0;
    divisor =  parent_rate / rate;
    if (divisor == cdev.div_mask + 1)
    divisor = 0;
// SDMMC mask may need to be corrected before testing if its valid
    if ((cdev.div_mask == 0x3F) && (divisor > 31)) {
//
// Bit 5 is a fixed /64 predivisor. If the requested divisor
// is >31 then correct for the fixed divisor being required.
//
    divisor = 0x20 + (divisor / 64);
    }
    if (divisor > cdev.div_mask) {
    pr_err("%s: invalid divisor for clock\n", __func__);
    return -EINVAL;
    }
    spin_lock_irqsave(cdev.lock, flags);
    vt8500_pmc_wait_busy();
    writel(divisor, cdev.div_reg);
    vt8500_pmc_wait_busy();
    spin_unlock_irqrestore(cdev.lock, flags);
    return 0;
    }
    static const struct clk_ops vt8500_gated_clk_ops = {
    .enable = vt8500_dclk_enable,
    .disable = vt8500_dclk_disable,
    .is_enabled = vt8500_dclk_is_enabled,
    };
    static const struct clk_ops vt8500_divisor_clk_ops = {
    .determine_rate = vt8500_dclk_determine_rate,
    .set_rate = vt8500_dclk_set_rate,
    .recalc_rate = vt8500_dclk_recalc_rate,
    };
    static const struct clk_ops vt8500_gated_divisor_clk_ops = {
    .enable = vt8500_dclk_enable,
    .disable = vt8500_dclk_disable,
    .is_enabled = vt8500_dclk_is_enabled,
    .determine_rate = vt8500_dclk_determine_rate,
    .set_rate = vt8500_dclk_set_rate,
    .recalc_rate = vt8500_dclk_recalc_rate,
    };

#[no_mangle]
unsafe extern "C" fn vtwm_device_clk_init(node: *mut device_node) -> __init void {
    static __init void vtwm_device_clk_init(struct device_node *node)
    {
    u32 en_reg, div_reg;
    struct clk_hw *hw;
    struct clk_device *dev_clk;
    const char *clk_name = node.name;
    const char *parent_name;
    struct clk_init_data init;
    int rc;
    let mut clk_init_flags: c_int = 0;
    if (!pmc_base)
    vtwm_set_pmc_base();
    dev_clk = kzalloc_obj(*dev_clk);
    if (WARN_ON(!dev_clk))
    return;
    dev_clk.lock = &_lock;
    rc = of_property_read_u32(node, "enable-reg", &en_reg);
    if (!rc) {
    dev_clk.en_reg = pmc_base + en_reg;
    rc = of_property_read_u32(node, "enable-bit", &dev_clk.en_bit);
    if (rc) {
    pr_err("%s: enable-bit property required for gated clock\n",
    __func__);
    return;
    }
    clk_init_flags |= CLK_INIT_GATED;
    }
    rc = of_property_read_u32(node, "divisor-reg", &div_reg);
    if (!rc) {
    dev_clk.div_reg = pmc_base + div_reg;
//
// use 0x1f as the default mask since it covers
// almost all the clocks and reduces dts properties
//
    dev_clk.div_mask = 0x1f;
    of_property_read_u32(node, "divisor-mask", &dev_clk.div_mask);
    clk_init_flags |= CLK_INIT_DIVISOR;
    }
    of_property_read_string(node, "clock-output-names", &clk_name);
    switch (clk_init_flags) {
    case CLK_INIT_GATED:
    init.ops = &vt8500_gated_clk_ops;
    break;
    case CLK_INIT_DIVISOR:
    init.ops = &vt8500_divisor_clk_ops;
    break;
    case CLK_INIT_GATED_DIVISOR:
    init.ops = &vt8500_gated_divisor_clk_ops;
    break;
    default:
    pr_err("%s: Invalid clock description in device tree\n",
    __func__);
    kfree(dev_clk);
    return;
    }
    init.name = clk_name;
    init.flags = 0;
    parent_name = of_clk_get_parent_name(node, 0);
    init.parent_names = &parent_name;
    init.num_parents = 1;
    dev_clk.hw.init = &init;
    hw = &dev_clk.hw;
    rc = clk_hw_register(core::ptr::null_mut(), hw);
    if (WARN_ON(rc)) {
    kfree(dev_clk);
    return;
    }
    rc = of_clk_add_hw_provider(node, of_clk_hw_simple_get, hw);
    clk_hw_register_clkdev(hw, clk_name, core::ptr::null_mut());
    }
    CLK_OF_DECLARE(vt8500_device, "via,vt8500-device-clock", vtwm_device_clk_init);
// PLL clock related functions

// Helper macros for PLL_VT8500

    ((r / d) * m)

    ((d == 2 ? 0 : 0x100) | ((m >> 1) & 0x1F))
// Helper macros for PLL_WM8650

    (r * m / (d1 * (1 << d2)))

    ((d2 << 13) | (d1 << 10) | (m & 0x3FF))
// Helper macros for PLL_WM8750

    (r * (m+1) / ((d1+1) * (1 << d2)))

    ((f << 24) | ((m - 1) << 16) | ((d1 - 1) << 8) | d2)
// Helper macros for PLL_WM8850

    (r * ((m + 1) * 2) / ((d1+1) * (1 << d2)))

    ((((m / 2) - 1) << 16) | ((d1 - 1) << 8) | d2)
    static int vt8500_find_pll_bits(unsigned long rate, unsigned long parent_rate,
    u32 *multiplier, u32 *prediv)
    {
    unsigned long tclk;
// sanity check
    if ((rate < parent_rate * 4) || (rate > parent_rate * 62)) {
    pr_err("%s: requested rate out of range\n", __func__);
// multiplier = 0;
// prediv = 1;
    return -EINVAL;
    }
    if (rate <= parent_rate * 31)
// use the prediv to double the resolution
// prediv = 2;
    else
// prediv = 1;
// multiplier = rate / (parent_rate / *prediv);
    tclk = (parent_rate / *prediv) * *multiplier;
    if (tclk != rate)
    pr_warn("%s: requested rate %lu, found rate %lu\n", __func__,
    rate, tclk);
    return 0;
    }
//
// M * parent [O1] => / P [O2] => / D [O3]
// Where O1 is 900MHz...3GHz;
// O2 is 600MHz >= (M * parent) / P >= 300MHz;
// M is 36...120 [25MHz parent]; D is 1 or 2 or 4 or 8.
// Possible ranges (O3):
// D = 8: 37,5MHz...75MHz
// D = 4: 75MHz...150MHz
// D = 2: 150MHz...300MHz
// D = 1: 300MHz...600MHz
//
    static int wm8650_find_pll_bits(unsigned long rate,
    unsigned long parent_rate, u32 *multiplier, u32 *divisor1,
    u32 *divisor2)
    {
    unsigned long O1, min_err, rate_err;
    if (!parent_rate || (rate < 37500000) || (rate > 600000000))
    return -EINVAL;
// divisor2 = rate <= 75000000 ? 3 : rate <= 150000000 ? 2 :
    rate <= 300000000 ? 1 : 0;
//
// Divisor P cannot be calculated. Test all divisors and find where M
// will be as close as possible to the requested rate.
//
    min_err = ULONG_MAX;
    for (*divisor1 = 5; *divisor1 >= 3; (*divisor1)--) {
    O1 = rate * *divisor1 * (1 << (*divisor2));
    rate_err = O1 % parent_rate;
    if (rate_err < min_err) {
// multiplier = O1 / parent_rate;
    if (rate_err == 0)
    return 0;
    min_err = rate_err;
    }
    }
    if ((*multiplier < 3) || (*multiplier > 1023))
    return -EINVAL;
    pr_warn("%s: rate error is %lu\n", __func__, min_err);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wm8750_get_filter(parent_rate: u32, divisor1: u32) -> u32 {
    static u32 wm8750_get_filter(u32 parent_rate, u32 divisor1)
    {
// calculate frequency (MHz) after pre-divisor
    let mut freq: u32 = (parent_rate / 1000000) / (divisor1 + 1);
    if ((freq < 10) || (freq > 200))
    pr_warn("%s: PLL recommended input frequency 10..200Mhz (requested %d Mhz)\n",
    __func__, freq);
    if (freq >= 166)
    return 7;
#[no_mangle]
pub unsafe extern "C" fn if(104: freq >=) -> else {
    else if (freq >= 104)
    return 6;
#[no_mangle]
pub unsafe extern "C" fn if(65: freq >=) -> else {
    else if (freq >= 65)
    return 5;
#[no_mangle]
pub unsafe extern "C" fn if(42: freq >=) -> else {
    else if (freq >= 42)
    return 4;
#[no_mangle]
pub unsafe extern "C" fn if(26: freq >=) -> else {
    else if (freq >= 26)
    return 3;
#[no_mangle]
pub unsafe extern "C" fn if(16: freq >=) -> else {
    else if (freq >= 16)
    return 2;
#[no_mangle]
pub unsafe extern "C" fn if(10: freq >=) -> else {
    else if (freq >= 10)
    return 1;
    return 0;
    }
    static int wm8750_find_pll_bits(unsigned long rate, unsigned long parent_rate,
    u32 *filter, u32 *multiplier, u32 *divisor1, u32 *divisor2)
    {
    u32 mul;
    int div1, div2;
    unsigned long tclk, rate_err, best_err;
    best_err = (unsigned long)-1;
// Find the closest match (lower or equal to requested)
    for (div1 = 1; div1 >= 0; div1--)
    for (div2 = 7; div2 >= 0; div2--)
    for (mul = 0; mul <= 255; mul++) {
    tclk = parent_rate * (mul + 1) / ((div1 + 1) * (1 << div2));
    if (tclk > rate)
    continue;
// error will always be +ve
    rate_err = rate - tclk;
    if (rate_err == 0) {
// filter = wm8750_get_filter(parent_rate, div1);
// multiplier = mul;
// divisor1 = div1;
// divisor2 = div2;
    return 0;
    }
    if (rate_err < best_err) {
    best_err = rate_err;
// multiplier = mul;
// divisor1 = div1;
// divisor2 = div2;
    }
    }
    if (best_err == (unsigned long)-1) {
    pr_warn("%s: impossible rate %lu\n", __func__, rate);
    return -EINVAL;
    }
// if we got here, it wasn't an exact match
    pr_warn("%s: requested rate %lu, found rate %lu\n", __func__, rate,
    rate - best_err);
// filter = wm8750_get_filter(parent_rate, *divisor1);
    return 0;
    }
    static int wm8850_find_pll_bits(unsigned long rate, unsigned long parent_rate,
    u32 *multiplier, u32 *divisor1, u32 *divisor2)
    {
    u32 mul;
    int div1, div2;
    unsigned long tclk, rate_err, best_err;
    best_err = (unsigned long)-1;
// Find the closest match (lower or equal to requested)
    for (div1 = 1; div1 >= 0; div1--)
    for (div2 = 3; div2 >= 0; div2--)
    for (mul = 0; mul <= 127; mul++) {
    tclk = parent_rate * ((mul + 1) * 2) /
    ((div1 + 1) * (1 << div2));
    if (tclk > rate)
    continue;
// error will always be +ve
    rate_err = rate - tclk;
    if (rate_err == 0) {
// multiplier = mul;
// divisor1 = div1;
// divisor2 = div2;
    return 0;
    }
    if (rate_err < best_err) {
    best_err = rate_err;
// multiplier = mul;
// divisor1 = div1;
// divisor2 = div2;
    }
    }
    if (best_err == (unsigned long)-1) {
    pr_warn("%s: impossible rate %lu\n", __func__, rate);
    return -EINVAL;
    }
// if we got here, it wasn't an exact match
    pr_warn("%s: requested rate %lu, found rate %lu\n", __func__, rate,
    rate - best_err);
    return 0;
    }
    static int vtwm_pll_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_pll *pll = to_clk_pll(hw);
    u32 filter, mul, div1, div2;
    u32 pll_val;
    let mut flags: c_ulong = 0;
    int ret;
// sanity check
    switch (pll.type) {
    case PLL_TYPE_VT8500:
    ret = vt8500_find_pll_bits(rate, parent_rate, &mul, &div1);
    if (!ret)
    pll_val = VT8500_BITS_TO_VAL(mul, div1);
    break;
    case PLL_TYPE_WM8650:
    ret = wm8650_find_pll_bits(rate, parent_rate, &mul, &div1, &div2);
    if (!ret)
    pll_val = WM8650_BITS_TO_VAL(mul, div1, div2);
    break;
    case PLL_TYPE_WM8750:
    ret = wm8750_find_pll_bits(rate, parent_rate, &filter, &mul, &div1, &div2);
    if (!ret)
    pll_val = WM8750_BITS_TO_VAL(filter, mul, div1, div2);
    break;
    case PLL_TYPE_WM8850:
    ret = wm8850_find_pll_bits(rate, parent_rate, &mul, &div1, &div2);
    if (!ret)
    pll_val = WM8850_BITS_TO_VAL(mul, div1, div2);
    break;
    default:
    pr_err("%s: invalid pll type\n", __func__);
    ret = -EINVAL;
    }
    if (ret)
    return ret;
    spin_lock_irqsave(pll.lock, flags);
    vt8500_pmc_wait_busy();
    writel(pll_val, pll.reg);
    vt8500_pmc_wait_busy();
    spin_unlock_irqrestore(pll.lock, flags);
    return 0;
    }
    static int vtwm_pll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_pll *pll = to_clk_pll(hw);
    u32 filter, mul, div1, div2;
    long round_rate;
    int ret;
    switch (pll.type) {
    case PLL_TYPE_VT8500:
    ret = vt8500_find_pll_bits(req.rate, req.best_parent_rate,
    &mul, &div1);
    if (!ret)
    round_rate = VT8500_BITS_TO_FREQ(req.best_parent_rate,
    mul, div1);
    break;
    case PLL_TYPE_WM8650:
    ret = wm8650_find_pll_bits(req.rate, req.best_parent_rate,
    &mul, &div1, &div2);
    if (!ret)
    round_rate = WM8650_BITS_TO_FREQ(req.best_parent_rate,
    mul, div1, div2);
    break;
    case PLL_TYPE_WM8750:
    ret = wm8750_find_pll_bits(req.rate, req.best_parent_rate,
    &filter, &mul, &div1, &div2);
    if (!ret)
    round_rate = WM8750_BITS_TO_FREQ(req.best_parent_rate,
    mul, div1, div2);
    break;
    case PLL_TYPE_WM8850:
    ret = wm8850_find_pll_bits(req.rate, req.best_parent_rate,
    &mul, &div1, &div2);
    if (!ret)
    round_rate = WM8850_BITS_TO_FREQ(req.best_parent_rate,
    mul, div1, div2);
    break;
    default:
    return -EINVAL;
    }
    if (ret)
    req.rate = ret;
    else
    req.rate = round_rate;
    return 0;
    }
    static unsigned long vtwm_pll_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_pll *pll = to_clk_pll(hw);
    let mut pll_val: u32 = readl(pll.reg);
    unsigned long pll_freq;
    switch (pll.type) {
    case PLL_TYPE_VT8500:
    pll_freq = parent_rate * VT8500_PLL_MUL(pll_val);
    pll_freq /= VT8500_PLL_DIV(pll_val);
    break;
    case PLL_TYPE_WM8650:
    pll_freq = parent_rate * WM8650_PLL_MUL(pll_val);
    pll_freq /= WM8650_PLL_DIV(pll_val);
    break;
    case PLL_TYPE_WM8750:
    pll_freq = parent_rate * WM8750_PLL_MUL(pll_val);
    pll_freq /= WM8750_PLL_DIV(pll_val);
    break;
    case PLL_TYPE_WM8850:
    pll_freq = parent_rate * WM8850_PLL_MUL(pll_val);
    pll_freq /= WM8850_PLL_DIV(pll_val);
    break;
    default:
    pll_freq = 0;
    }
    return pll_freq;
    }
    static const struct clk_ops vtwm_pll_ops = {
    .determine_rate = vtwm_pll_determine_rate,
    .set_rate = vtwm_pll_set_rate,
    .recalc_rate = vtwm_pll_recalc_rate,
    };
#[no_mangle]
unsafe extern "C" fn vtwm_pll_clk_init(node: *mut device_node, pll_type: c_int) -> __init void {
    static __init void vtwm_pll_clk_init(struct device_node *node, int pll_type)
    {
    u32 reg;
    struct clk_hw *hw;
    struct clk_pll *pll_clk;
    const char *clk_name = node.name;
    const char *parent_name;
    struct clk_init_data init;
    int rc;
    if (!pmc_base)
    vtwm_set_pmc_base();
    rc = of_property_read_u32(node, "reg", &reg);
    if (WARN_ON(rc))
    return;
    pll_clk = kzalloc_obj(*pll_clk);
    if (WARN_ON(!pll_clk))
    return;
    pll_clk.reg = pmc_base + reg;
    pll_clk.lock = &_lock;
    pll_clk.type = pll_type;
    of_property_read_string(node, "clock-output-names", &clk_name);
    init.name = clk_name;
    init.ops = &vtwm_pll_ops;
    init.flags = 0;
    parent_name = of_clk_get_parent_name(node, 0);
    init.parent_names = &parent_name;
    init.num_parents = 1;
    pll_clk.hw.init = &init;
    hw = &pll_clk.hw;
    rc = clk_hw_register(core::ptr::null_mut(), &pll_clk.hw);
    if (WARN_ON(rc)) {
    kfree(pll_clk);
    return;
    }
    rc = of_clk_add_hw_provider(node, of_clk_hw_simple_get, hw);
    clk_hw_register_clkdev(hw, clk_name, core::ptr::null_mut());
    }
// Wrappers for initialization functions
#[no_mangle]
unsafe extern "C" fn vt8500_pll_init(node: *mut device_node) -> void __init {
    static void __init vt8500_pll_init(struct device_node *node)
    {
    vtwm_pll_clk_init(node, PLL_TYPE_VT8500);
    }
    CLK_OF_DECLARE(vt8500_pll, "via,vt8500-pll-clock", vt8500_pll_init);
#[no_mangle]
unsafe extern "C" fn wm8650_pll_init(node: *mut device_node) -> void __init {
    static void __init wm8650_pll_init(struct device_node *node)
    {
    vtwm_pll_clk_init(node, PLL_TYPE_WM8650);
    }
    CLK_OF_DECLARE(wm8650_pll, "wm,wm8650-pll-clock", wm8650_pll_init);
#[no_mangle]
unsafe extern "C" fn wm8750_pll_init(node: *mut device_node) -> void __init {
    static void __init wm8750_pll_init(struct device_node *node)
    {
    vtwm_pll_clk_init(node, PLL_TYPE_WM8750);
    }
    CLK_OF_DECLARE(wm8750_pll, "wm,wm8750-pll-clock", wm8750_pll_init);
#[no_mangle]
unsafe extern "C" fn wm8850_pll_init(node: *mut device_node) -> void __init {
    static void __init wm8850_pll_init(struct device_node *node)
    {
    vtwm_pll_clk_init(node, PLL_TYPE_WM8850);
    }
    CLK_OF_DECLARE(wm8850_pll, "wm,wm8850-pll-clock", wm8850_pll_init);
