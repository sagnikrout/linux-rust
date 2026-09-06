//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-rp1.c
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
// Copyright (C) 2023 Raspberry Pi Ltd.
//
// Clock driver for RP1 PCIe multifunction chip.
//

pub const PLL_SYS_OFFSET: c_uint = 0x08000;

pub const PLL_AUDIO_OFFSET: c_uint = 0x0c000;

pub const PLL_VIDEO_OFFSET: c_uint = 0x10000;

pub const GPCLK_OE_CTRL: c_uint = 0x00000;
pub const CLK_SYS_OFFSET: c_uint = 0x00014;

pub const CLK_SLOW_OFFSET: c_uint = 0x00024;

pub const CLK_DMA_OFFSET: c_uint = 0x00044;

pub const CLK_UART_OFFSET: c_uint = 0x00054;

pub const CLK_ETH_OFFSET: c_uint = 0x00064;

pub const CLK_PWM0_OFFSET: c_uint = 0x00074;

pub const CLK_PWM1_OFFSET: c_uint = 0x00084;

pub const CLK_AUDIO_IN_OFFSET: c_uint = 0x00094;

pub const CLK_AUDIO_OUT_OFFSET: c_uint = 0x000a4;

pub const CLK_I2S_OFFSET: c_uint = 0x000b4;

pub const CLK_MIPI0_CFG_OFFSET: c_uint = 0x000c4;

pub const CLK_MIPI1_CFG_OFFSET: c_uint = 0x000d4;

pub const CLK_PCIE_AUX_OFFSET: c_uint = 0x000e4;

pub const CLK_USBH0_MICROFRAME_OFFSET: c_uint = 0x000f4;

pub const CLK_USBH1_MICROFRAME_OFFSET: c_uint = 0x00104;

pub const CLK_USBH0_SUSPEND_OFFSET: c_uint = 0x00114;

pub const CLK_USBH1_SUSPEND_OFFSET: c_uint = 0x00124;

pub const CLK_ETH_TSU_OFFSET: c_uint = 0x00134;

pub const CLK_ADC_OFFSET: c_uint = 0x00144;

pub const CLK_SDIO_TIMER_OFFSET: c_uint = 0x00154;

pub const CLK_SDIO_ALT_SRC_OFFSET: c_uint = 0x00164;

pub const CLK_GP0_OFFSET: c_uint = 0x00174;

pub const CLK_GP1_OFFSET: c_uint = 0x00184;

pub const CLK_GP2_OFFSET: c_uint = 0x00194;

pub const CLK_GP3_OFFSET: c_uint = 0x001a4;

pub const CLK_GP4_OFFSET: c_uint = 0x001b4;

pub const CLK_GP5_OFFSET: c_uint = 0x001c4;

pub const CLK_SYS_RESUS_CTRL: c_uint = 0x0020c;
pub const CLK_SLOW_SYS_RESUS_CTRL: c_uint = 0x00214;
pub const FC0_OFFSET: c_uint = 0x0021c;

pub const FC_SIZE: c_uint = 0x20;
pub const FC_COUNT: c_int = 8;

pub const AUX_SEL: c_int = 1;
pub const VIDEO_CLOCKS_OFFSET: c_uint = 0x4000;

pub const FC0_RESULT_FRAC_SHIFT: c_int = 5;

// PLL phase output for both PRI and SEC

pub const PLL_PH_PHASE_SHIFT: c_int = 0;
pub const RP1_PLL_PHASE_0: c_int = 0;
pub const RP1_PLL_PHASE_90: c_int = 1;
pub const RP1_PLL_PHASE_180: c_int = 2;
pub const RP1_PLL_PHASE_270: c_int = 3;
// Clock fields for all clocks

pub const CLK_CTRL_SRC_SHIFT: c_int = 0;
pub const CLK_DIV_FRAC_BITS: c_int = 16;
pub const LOCK_TIMEOUT_US: c_int = 100000;
pub const LOCK_POLL_DELAY_US: c_int = 5;
pub const MAX_CLK_PARENTS: c_int = 16;
pub const PLL_DIV_INVALID: c_int = 19;
//
// Secondary PLL channel output divider table.
// Divider values range from 8 to 19, where
// 19 means invalid.
//
    static const struct clk_div_table pll_sec_div_table[] = {
    { 0x00, PLL_DIV_INVALID },
    { 0x01, PLL_DIV_INVALID },
    { 0x02, PLL_DIV_INVALID },
    { 0x03, PLL_DIV_INVALID },
    { 0x04, PLL_DIV_INVALID },
    { 0x05, PLL_DIV_INVALID },
    { 0x06, PLL_DIV_INVALID },
    { 0x07, PLL_DIV_INVALID },
    { 0x08,  8 },
    { 0x09,  9 },
    { 0x0a, 10 },
    { 0x0b, 11 },
    { 0x0c, 12 },
    { 0x0d, 13 },
    { 0x0e, 14 },
    { 0x0f, 15 },
    { 0x10, 16 },
    { 0x11, 17 },
    { 0x12, 18 },
    { 0x13, PLL_DIV_INVALID },
    { 0x14, PLL_DIV_INVALID },
    { 0x15, PLL_DIV_INVALID },
    { 0x16, PLL_DIV_INVALID },
    { 0x17, PLL_DIV_INVALID },
    { 0x18, PLL_DIV_INVALID },
    { 0x19, PLL_DIV_INVALID },
    { 0x1a, PLL_DIV_INVALID },
    { 0x1b, PLL_DIV_INVALID },
    { 0x1c, PLL_DIV_INVALID },
    { 0x1d, PLL_DIV_INVALID },
    { 0x1e, PLL_DIV_INVALID },
    { 0x1f, PLL_DIV_INVALID },
    { 0 }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rp1_clockman {
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub regmap: *mut regmap,
    pub /: *mut *mut spinlock_t regs_lock; / spinlock for all clocks,
// Must be last
    pub onecell: clk_hw_onecell_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rp1_pll_core_data {
    pub cs_reg: u32,
    pub pwr_reg: u32,
    pub fbdiv_int_reg: u32,
    pub fbdiv_frac_reg: u32,
    pub fc0_src: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rp1_pll_data {
    pub ctrl_reg: u32,
    pub fc0_src: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rp1_pll_ph_data {
    pub phase: c_uint,
    pub fixed_divider: c_uint,
    pub ph_reg: u32,
    pub fc0_src: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rp1_pll_divider_data {
    pub sec_reg: u32,
    pub fc0_src: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rp1_clock_data {
    pub num_std_parents: c_int,
    pub num_aux_parents: c_int,
    pub oe_mask: u32,
    pub clk_src_mask: u32,
    pub ctrl_reg: u32,
    pub div_int_reg: u32,
    pub div_frac_reg: u32,
    pub sel_reg: u32,
    pub div_int_max: u32,
    pub max_freq: c_ulong,
    pub fc0_src: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rp1_clk_desc {
    struct clk_hw *(*clk_register)(struct rp1_clockman *clockman,
    pub desc): *mut rp1_clk_desc,
    pub data: *const c_void,
    pub hw: clk_hw,
    pub clockman: *mut rp1_clockman,
    pub cached_rate: c_ulong,
    pub div: clk_divider,
}

    static struct rp1_clk_desc *clk_audio_core;
    static struct rp1_clk_desc *clk_audio;
    static struct rp1_clk_desc *clk_i2s;
    static struct clk_hw *clk_xosc;
    static inline
#[no_mangle]
pub unsafe extern "C" fn clockman_write(clockman: *mut rp1_clockman, reg: u32, val: u32) {
    void clockman_write(struct rp1_clockman *clockman, u32 reg, u32 val)
    {
    regmap_write(clockman.regmap, reg, val);
    }
#[no_mangle]
pub unsafe extern "C" fn clockman_read(clockman: *mut rp1_clockman, reg: u32) -> u32 {
    static inline u32 clockman_read(struct rp1_clockman *clockman, u32 reg)
    {
    u32 val;
    regmap_read(clockman.regmap, reg, &val);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn rp1_pll_core_is_on(hw: *mut clk_hw) -> c_int {
    static int rp1_pll_core_is_on(struct clk_hw *hw)
    {
    struct rp1_clk_desc *pll_core = container_of(hw, struct rp1_clk_desc, hw);
    struct rp1_clockman *clockman = pll_core.clockman;
    const struct rp1_pll_core_data *data = pll_core.data;
    let mut pwr: u32 = clockman_read(clockman, data.pwr_reg);
    return (pwr & PLL_PWR_PD) || (pwr & PLL_PWR_POSTDIVPD);
    }
#[no_mangle]
unsafe extern "C" fn rp1_pll_core_on(hw: *mut clk_hw) -> c_int {
    static int rp1_pll_core_on(struct clk_hw *hw)
    {
    struct rp1_clk_desc *pll_core = container_of(hw, struct rp1_clk_desc, hw);
    struct rp1_clockman *clockman = pll_core.clockman;
    const struct rp1_pll_core_data *data = pll_core.data;
    u32 fbdiv_frac, val;
    int ret;
    spin_lock(&clockman.regs_lock);
    if (!(clockman_read(clockman, data.cs_reg) & PLL_CS_LOCK)) {
// Reset to a known state.
    clockman_write(clockman, data.pwr_reg, PLL_PWR_MASK);
    clockman_write(clockman, data.fbdiv_int_reg, 20);
    clockman_write(clockman, data.fbdiv_frac_reg, 0);
    clockman_write(clockman, data.cs_reg, PLL_CS_REFDIV_MASK);
    }
// Come out of reset.
    fbdiv_frac = clockman_read(clockman, data.fbdiv_frac_reg);
    clockman_write(clockman, data.pwr_reg, fbdiv_frac ? 0 : PLL_PWR_DSMPD);
    spin_unlock(&clockman.regs_lock);
// Wait for the PLL to lock.
    ret = regmap_read_poll_timeout(clockman.regmap, data.cs_reg, val,
    val & PLL_CS_LOCK,
    LOCK_POLL_DELAY_US, LOCK_TIMEOUT_US);
    if (ret)
    dev_err(clockman.dev, "%s: can't lock PLL\n",
    clk_hw_get_name(hw));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rp1_pll_core_off(hw: *mut clk_hw) {
    static void rp1_pll_core_off(struct clk_hw *hw)
    {
    struct rp1_clk_desc *pll_core = container_of(hw, struct rp1_clk_desc, hw);
    struct rp1_clockman *clockman = pll_core.clockman;
    const struct rp1_pll_core_data *data = pll_core.data;
    spin_lock(&clockman.regs_lock);
    clockman_write(clockman, data.pwr_reg, 0);
    spin_unlock(&clockman.regs_lock);
    }
    static inline unsigned long get_pll_core_divider(struct clk_hw *hw,
    unsigned long rate,
    unsigned long parent_rate,
    u32 *div_int, u32 *div_frac)
    {
    u32 fbdiv_int, fbdiv_frac;
    unsigned long calc_rate;
    u64 shifted_fbdiv_int;
    u64 div_fp64; /* 32.32 fixed point fraction. */
// Factor of reference clock to VCO frequency.
    div_fp64 = (u64)(rate) << 32;
    div_fp64 = DIV_ROUND_CLOSEST_ULL(div_fp64, parent_rate);
// Round the fractional component at 24 bits.
    div_fp64 += 1 << (32 - 24 - 1);
    fbdiv_int = div_fp64 >> 32;
    fbdiv_frac = (div_fp64 >> (32 - 24)) & 0xffffff;
    shifted_fbdiv_int = (u64)fbdiv_int << 24;
    calc_rate = (u64)parent_rate * (shifted_fbdiv_int + fbdiv_frac);
    calc_rate += BIT(23);
    calc_rate >>= 24;
// div_int = fbdiv_int;
// div_frac = fbdiv_frac;
    return calc_rate;
    }
    static int rp1_pll_core_set_rate(struct clk_hw *hw,
    unsigned long rate, unsigned long parent_rate)
    {
    struct rp1_clk_desc *pll_core = container_of(hw, struct rp1_clk_desc, hw);
    struct rp1_clockman *clockman = pll_core.clockman;
    const struct rp1_pll_core_data *data = pll_core.data;
    u32 fbdiv_int, fbdiv_frac;
// Disable dividers to start with.
    spin_lock(&clockman.regs_lock);
    clockman_write(clockman, data.fbdiv_int_reg, 0);
    clockman_write(clockman, data.fbdiv_frac_reg, 0);
    spin_unlock(&clockman.regs_lock);
    get_pll_core_divider(hw, rate, parent_rate,
    &fbdiv_int, &fbdiv_frac);
    spin_lock(&clockman.regs_lock);
    clockman_write(clockman, data.pwr_reg, fbdiv_frac ? 0 : PLL_PWR_DSMPD);
    clockman_write(clockman, data.fbdiv_int_reg, fbdiv_int);
    clockman_write(clockman, data.fbdiv_frac_reg, fbdiv_frac);
    spin_unlock(&clockman.regs_lock);
// Check that reference frequency is no greater than VCO / 16.
    if (WARN_ON_ONCE(parent_rate > (rate / 16)))
    return -ERANGE;
    spin_lock(&clockman.regs_lock);
// Don't need to divide ref unless parent_rate > (output freq / 16)
    clockman_write(clockman, data.cs_reg,
    clockman_read(clockman, data.cs_reg) |
    PLL_CS_REFDIV_MASK);
    spin_unlock(&clockman.regs_lock);
    return 0;
    }
    static unsigned long rp1_pll_core_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct rp1_clk_desc *pll_core = container_of(hw, struct rp1_clk_desc, hw);
    struct rp1_clockman *clockman = pll_core.clockman;
    const struct rp1_pll_core_data *data = pll_core.data;
    u32 fbdiv_int, fbdiv_frac;
    unsigned long calc_rate;
    u64 shifted_fbdiv_int;
    fbdiv_int = clockman_read(clockman, data.fbdiv_int_reg);
    fbdiv_frac = clockman_read(clockman, data.fbdiv_frac_reg);
    shifted_fbdiv_int = (u64)fbdiv_int << 24;
    calc_rate = (u64)parent_rate * (shifted_fbdiv_int + fbdiv_frac);
    calc_rate += BIT(23);
    calc_rate >>= 24;
    return calc_rate;
    }
    static int rp1_pll_core_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    u32 fbdiv_int, fbdiv_frac;
    req.rate = get_pll_core_divider(hw, req.rate, req.best_parent_rate,
    &fbdiv_int,
    &fbdiv_frac);
    return 0;
    }
    static void get_pll_prim_dividers(unsigned long rate, unsigned long parent_rate,
    u32 *divider1, u32 *divider2)
    {
    unsigned int div1, div2;
    let mut best_div1: c_uint = 7, best_div2 = 7;
    unsigned long best_rate_diff =
    abs_diff(DIV_ROUND_CLOSEST(parent_rate, best_div1 * best_div2), rate);
    unsigned long rate_diff, calc_rate;
    for (div1 = 1; div1 <= 7; div1++) {
    for (div2 = 1; div2 <= div1; div2++) {
    calc_rate = DIV_ROUND_CLOSEST(parent_rate, div1 * div2);
    rate_diff = abs_diff(calc_rate, rate);
    if (calc_rate == rate) {
    best_div1 = div1;
    best_div2 = div2;
    goto done;
    } else if (rate_diff < best_rate_diff) {
    best_div1 = div1;
    best_div2 = div2;
    best_rate_diff = rate_diff;
    }
    }
    }
    done:
// divider1 = best_div1;
// divider2 = best_div2;
    }
    static int rp1_pll_set_rate(struct clk_hw *hw,
    unsigned long rate, unsigned long parent_rate)
    {
    struct rp1_clk_desc *pll = container_of(hw, struct rp1_clk_desc, hw);
    struct rp1_clockman *clockman = pll.clockman;
    const struct rp1_pll_data *data = pll.data;
    u32 prim, prim_div1, prim_div2;
    get_pll_prim_dividers(rate, parent_rate, &prim_div1, &prim_div2);
    spin_lock(&clockman.regs_lock);
    prim = clockman_read(clockman, data.ctrl_reg);
    prim &= ~PLL_PRIM_DIV1_MASK;
    prim |= FIELD_PREP(PLL_PRIM_DIV1_MASK, prim_div1);
    prim &= ~PLL_PRIM_DIV2_MASK;
    prim |= FIELD_PREP(PLL_PRIM_DIV2_MASK, prim_div2);
    clockman_write(clockman, data.ctrl_reg, prim);
    spin_unlock(&clockman.regs_lock);
    return 0;
    }
    static unsigned long rp1_pll_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct rp1_clk_desc *pll = container_of(hw, struct rp1_clk_desc, hw);
    struct rp1_clockman *clockman = pll.clockman;
    const struct rp1_pll_data *data = pll.data;
    u32 prim, prim_div1, prim_div2;
    prim = clockman_read(clockman, data.ctrl_reg);
    prim_div1 = FIELD_GET(PLL_PRIM_DIV1_MASK, prim);
    prim_div2 = FIELD_GET(PLL_PRIM_DIV2_MASK, prim);
    if (!prim_div1 || !prim_div2) {
    dev_err(clockman.dev, "%s: (%s) zero divider value\n",
    __func__, clk_hw_get_name(hw));
    return 0;
    }
    return DIV_ROUND_CLOSEST(parent_rate, prim_div1 * prim_div2);
    }
    static int rp1_pll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_hw *clk_audio_hw = &clk_audio.hw;
    u32 div1, div2;
    if (hw == clk_audio_hw && clk_audio.cached_rate == req.rate)
    req.best_parent_rate = clk_audio_core.cached_rate;
    get_pll_prim_dividers(req.rate, req.best_parent_rate, &div1, &div2);
    req.rate = DIV_ROUND_CLOSEST(req.best_parent_rate, div1 * div2);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rp1_pll_ph_is_on(hw: *mut clk_hw) -> c_int {
    static int rp1_pll_ph_is_on(struct clk_hw *hw)
    {
    struct rp1_clk_desc *pll_ph = container_of(hw, struct rp1_clk_desc, hw);
    struct rp1_clockman *clockman = pll_ph.clockman;
    const struct rp1_pll_ph_data *data = pll_ph.data;
    return !!(clockman_read(clockman, data.ph_reg) & PLL_PH_EN);
    }
#[no_mangle]
unsafe extern "C" fn rp1_pll_ph_on(hw: *mut clk_hw) -> c_int {
    static int rp1_pll_ph_on(struct clk_hw *hw)
    {
    struct rp1_clk_desc *pll_ph = container_of(hw, struct rp1_clk_desc, hw);
    struct rp1_clockman *clockman = pll_ph.clockman;
    const struct rp1_pll_ph_data *data = pll_ph.data;
    u32 ph_reg;
    spin_lock(&clockman.regs_lock);
    ph_reg = clockman_read(clockman, data.ph_reg);
    ph_reg |= data.phase << PLL_PH_PHASE_SHIFT;
    ph_reg |= PLL_PH_EN;
    clockman_write(clockman, data.ph_reg, ph_reg);
    spin_unlock(&clockman.regs_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rp1_pll_ph_off(hw: *mut clk_hw) {
    static void rp1_pll_ph_off(struct clk_hw *hw)
    {
    struct rp1_clk_desc *pll_ph = container_of(hw, struct rp1_clk_desc, hw);
    struct rp1_clockman *clockman = pll_ph.clockman;
    const struct rp1_pll_ph_data *data = pll_ph.data;
    spin_lock(&clockman.regs_lock);
    clockman_write(clockman, data.ph_reg,
    clockman_read(clockman, data.ph_reg) & ~PLL_PH_EN);
    spin_unlock(&clockman.regs_lock);
    }
    static unsigned long rp1_pll_ph_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct rp1_clk_desc *pll_ph = container_of(hw, struct rp1_clk_desc, hw);
    const struct rp1_pll_ph_data *data = pll_ph.data;
    return parent_rate / data.fixed_divider;
    }
    static int rp1_pll_ph_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct rp1_clk_desc *pll_ph = container_of(hw, struct rp1_clk_desc, hw);
    const struct rp1_pll_ph_data *data = pll_ph.data;
    req.rate = req.best_parent_rate / data.fixed_divider;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rp1_pll_divider_is_on(hw: *mut clk_hw) -> c_int {
    static int rp1_pll_divider_is_on(struct clk_hw *hw)
    {
    struct rp1_clk_desc *divider = container_of(hw, struct rp1_clk_desc, div.hw);
    struct rp1_clockman *clockman = divider.clockman;
    const struct rp1_pll_data *data = divider.data;
    return !(clockman_read(clockman, data.ctrl_reg) & PLL_SEC_RST);
    }
#[no_mangle]
unsafe extern "C" fn rp1_pll_divider_on(hw: *mut clk_hw) -> c_int {
    static int rp1_pll_divider_on(struct clk_hw *hw)
    {
    struct rp1_clk_desc *divider = container_of(hw, struct rp1_clk_desc, div.hw);
    struct rp1_clockman *clockman = divider.clockman;
    const struct rp1_pll_data *data = divider.data;
    spin_lock(&clockman.regs_lock);
// Check the implementation bit is set!
    WARN_ON(!(clockman_read(clockman, data.ctrl_reg) & PLL_SEC_IMPL));
    clockman_write(clockman, data.ctrl_reg,
    clockman_read(clockman, data.ctrl_reg) & ~PLL_SEC_RST);
    spin_unlock(&clockman.regs_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rp1_pll_divider_off(hw: *mut clk_hw) {
    static void rp1_pll_divider_off(struct clk_hw *hw)
    {
    struct rp1_clk_desc *divider = container_of(hw, struct rp1_clk_desc, div.hw);
    struct rp1_clockman *clockman = divider.clockman;
    const struct rp1_pll_data *data = divider.data;
    spin_lock(&clockman.regs_lock);
    clockman_write(clockman, data.ctrl_reg,
    clockman_read(clockman, data.ctrl_reg) | PLL_SEC_RST);
    spin_unlock(&clockman.regs_lock);
    }
    static int rp1_pll_divider_set_rate(struct clk_hw *hw,
    unsigned long rate,
    unsigned long parent_rate)
    {
    struct rp1_clk_desc *divider = container_of(hw, struct rp1_clk_desc, div.hw);
    struct rp1_clockman *clockman = divider.clockman;
    const struct rp1_pll_data *data = divider.data;
    u32 div, sec;
    div = DIV_ROUND_UP_ULL(parent_rate, rate);
    div = clamp(div, 8u, 19u);
    spin_lock(&clockman.regs_lock);
    sec = clockman_read(clockman, data.ctrl_reg);
    sec &= ~PLL_SEC_DIV_MASK;
    sec |= FIELD_PREP(PLL_SEC_DIV_MASK, div);
// Must keep the divider in reset to change the value.
    sec |= PLL_SEC_RST;
    clockman_write(clockman, data.ctrl_reg, sec);
// must sleep 10 pll vco cycles
    ndelay(div64_ul(10ULL * div * NSEC_PER_SEC, parent_rate));
    sec &= ~PLL_SEC_RST;
    clockman_write(clockman, data.ctrl_reg, sec);
    spin_unlock(&clockman.regs_lock);
    return 0;
    }
    static unsigned long rp1_pll_divider_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    return clk_divider_ops.recalc_rate(hw, parent_rate);
    }
    static int rp1_pll_divider_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    req.rate = clk_divider_ops.determine_rate(hw, req);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rp1_clock_is_on(hw: *mut clk_hw) -> c_int {
    static int rp1_clock_is_on(struct clk_hw *hw)
    {
    struct rp1_clk_desc *clock = container_of(hw, struct rp1_clk_desc, hw);
    struct rp1_clockman *clockman = clock.clockman;
    const struct rp1_clock_data *data = clock.data;
    return !!(clockman_read(clockman, data.ctrl_reg) & CLK_CTRL_ENABLE);
    }
    static unsigned long rp1_clock_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct rp1_clk_desc *clock = container_of(hw, struct rp1_clk_desc, hw);
    struct rp1_clockman *clockman = clock.clockman;
    const struct rp1_clock_data *data = clock.data;
    u64 calc_rate;
    u64 div;
    u32 frac;
    div = clockman_read(clockman, data.div_int_reg);
    frac = (data.div_frac_reg != 0) ?
    clockman_read(clockman, data.div_frac_reg) : 0;
// If the integer portion of the divider is 0, treat it as 2^16
    if (!div)
    div = 1 << 16;
    div = (div << CLK_DIV_FRAC_BITS) | (frac >> (32 - CLK_DIV_FRAC_BITS));
    calc_rate = (u64)parent_rate << CLK_DIV_FRAC_BITS;
    calc_rate = div64_u64(calc_rate, div);
    return calc_rate;
    }
#[no_mangle]
unsafe extern "C" fn rp1_clock_on(hw: *mut clk_hw) -> c_int {
    static int rp1_clock_on(struct clk_hw *hw)
    {
    struct rp1_clk_desc *clock = container_of(hw, struct rp1_clk_desc, hw);
    struct rp1_clockman *clockman = clock.clockman;
    const struct rp1_clock_data *data = clock.data;
    spin_lock(&clockman.regs_lock);
    clockman_write(clockman, data.ctrl_reg,
    clockman_read(clockman, data.ctrl_reg) | CLK_CTRL_ENABLE);
// If this is a GPCLK, turn on the output-enable
    if (data.oe_mask)
    clockman_write(clockman, GPCLK_OE_CTRL,
    clockman_read(clockman, GPCLK_OE_CTRL) | data.oe_mask);
    spin_unlock(&clockman.regs_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rp1_clock_off(hw: *mut clk_hw) {
    static void rp1_clock_off(struct clk_hw *hw)
    {
    struct rp1_clk_desc *clock = container_of(hw, struct rp1_clk_desc, hw);
    struct rp1_clockman *clockman = clock.clockman;
    const struct rp1_clock_data *data = clock.data;
    spin_lock(&clockman.regs_lock);
    clockman_write(clockman, data.ctrl_reg,
    clockman_read(clockman, data.ctrl_reg) & ~CLK_CTRL_ENABLE);
// If this is a GPCLK, turn off the output-enable
    if (data.oe_mask)
    clockman_write(clockman, GPCLK_OE_CTRL,
    clockman_read(clockman, GPCLK_OE_CTRL) & ~data.oe_mask);
    spin_unlock(&clockman.regs_lock);
    }
    static u32 rp1_clock_choose_div(unsigned long rate, unsigned long parent_rate,
    const struct rp1_clock_data *data)
    {
    u64 div;
//
// Due to earlier rounding, calculated parent_rate may differ from
// expected value. Don't fail on a small discrepancy near unity divide.
//
    if (!rate || rate > parent_rate + (parent_rate >> CLK_DIV_FRAC_BITS))
    return 0;
//
// Always express div in fixed-point format for fractional division;
// If no fractional divider is present, the fraction part will be zero.
//
    if (data.div_frac_reg) {
    div = (u64)parent_rate << CLK_DIV_FRAC_BITS;
    div = DIV_ROUND_CLOSEST_ULL(div, rate);
    } else {
    div = DIV_ROUND_CLOSEST_ULL(parent_rate, rate);
    div <<= CLK_DIV_FRAC_BITS;
    }
    div = clamp(div,
    1ull << CLK_DIV_FRAC_BITS,
    (u64)data.div_int_max << CLK_DIV_FRAC_BITS);
    return div;
    }
#[no_mangle]
unsafe extern "C" fn rp1_clock_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 rp1_clock_get_parent(struct clk_hw *hw)
    {
    struct rp1_clk_desc *clock = container_of(hw, struct rp1_clk_desc, hw);
    struct rp1_clockman *clockman = clock.clockman;
    const struct rp1_clock_data *data = clock.data;
    u32 sel, ctrl;
    u8 parent;
// Sel is one-hot, so find the first bit set
    sel = clockman_read(clockman, data.sel_reg);
    parent = ffs(sel) - 1;
// sel == 0 implies the parent clock is not enabled yet.
    if (!sel) {
// Read the clock src from the CTRL register instead
    ctrl = clockman_read(clockman, data.ctrl_reg);
    parent = (ctrl & data.clk_src_mask) >> CLK_CTRL_SRC_SHIFT;
    }
    if (parent >= data.num_std_parents)
    parent = AUX_SEL;
    if (parent == AUX_SEL) {
//
// Clock parent is an auxiliary source, so get the parent from
// the AUXSRC register field.
//
    ctrl = clockman_read(clockman, data.ctrl_reg);
    parent = FIELD_GET(CLK_CTRL_AUXSRC_MASK, ctrl);
    parent += data.num_std_parents;
    }
    return parent;
    }
#[no_mangle]
unsafe extern "C" fn rp1_clock_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int rp1_clock_set_parent(struct clk_hw *hw, u8 index)
    {
    struct rp1_clk_desc *clock = container_of(hw, struct rp1_clk_desc, hw);
    struct rp1_clockman *clockman = clock.clockman;
    const struct rp1_clock_data *data = clock.data;
    u32 ctrl, sel;
    spin_lock(&clockman.regs_lock);
    ctrl = clockman_read(clockman, data.ctrl_reg);
    if (index >= data.num_std_parents) {
// This is an aux source request
    if (index >= data.num_std_parents + data.num_aux_parents) {
    spin_unlock(&clockman.regs_lock);
    return -EINVAL;
    }
// Select parent from aux list
    ctrl &= ~CLK_CTRL_AUXSRC_MASK;
    ctrl |= FIELD_PREP(CLK_CTRL_AUXSRC_MASK, index - data.num_std_parents);
// Set src to aux list
    ctrl &= ~data.clk_src_mask;
    ctrl |= (AUX_SEL << CLK_CTRL_SRC_SHIFT) & data.clk_src_mask;
    } else {
    ctrl &= ~data.clk_src_mask;
    ctrl |= (index << CLK_CTRL_SRC_SHIFT) & data.clk_src_mask;
    }
    clockman_write(clockman, data.ctrl_reg, ctrl);
    spin_unlock(&clockman.regs_lock);
    sel = rp1_clock_get_parent(hw);
    if (sel != index)
    return -EINVAL;
    return 0;
    }
    static int rp1_clock_set_rate_and_parent(struct clk_hw *hw,
    unsigned long rate,
    unsigned long parent_rate,
    u8 parent)
    {
    struct rp1_clk_desc *clock = container_of(hw, struct rp1_clk_desc, hw);
    struct rp1_clockman *clockman = clock.clockman;
    const struct rp1_clock_data *data = clock.data;
    let mut div: u32 = rp1_clock_choose_div(rate, parent_rate, data);
    spin_lock(&clockman.regs_lock);
    clockman_write(clockman, data.div_int_reg, div >> CLK_DIV_FRAC_BITS);
    if (data.div_frac_reg)
    clockman_write(clockman, data.div_frac_reg, div << (32 - CLK_DIV_FRAC_BITS));
    spin_unlock(&clockman.regs_lock);
    if (parent != 0xff)
    return rp1_clock_set_parent(hw, parent);
    return 0;
    }
    static int rp1_clock_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    return rp1_clock_set_rate_and_parent(hw, rate, parent_rate, 0xff);
    }
    static unsigned long calc_core_pll_rate(struct clk_hw *pll_hw,
    unsigned long target_rate,
    int *pdiv_prim, int *pdiv_clk)
    {
    static const int prim_divs[] = {
    2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 15, 16,
    18, 20, 21, 24, 25, 28, 30, 35, 36, 42, 49,
    };
    let mut xosc_rate: c_ulong = clk_hw_get_rate(clk_xosc);
    let mut core_min: c_ulong = xosc_rate * 16;
    let mut core_max: c_ulong = 2400000000;
    let mut best_div_prim: c_int = 1, best_div_clk = 1;
    let mut best_rate: c_ulong = core_max + 1;
    let mut core_rate: c_ulong = 0;
    int div_int, div_frac;
    u64 div;
    int i;
// Given the target rate, choose a set of divisors/multipliers
    for (i = 0; i < ARRAY_SIZE(prim_divs); i++) {
    let mut div_prim: c_int = prim_divs[i];
    int div_clk;
    for (div_clk = 1; div_clk <= 256; div_clk++) {
    core_rate = target_rate * div_clk * div_prim;
    if (core_rate >= core_min) {
    if (core_rate < best_rate) {
    best_rate = core_rate;
    best_div_prim = div_prim;
    best_div_clk = div_clk;
    }
    break;
    }
    }
    }
    if (best_rate < core_max) {
    div = ((best_rate << 24) + xosc_rate / 2) / xosc_rate;
    div_int = div >> 24;
    div_frac = div % (1 << 24);
    core_rate = (xosc_rate * ((div_int << 24) + div_frac) + (1 << 23)) >> 24;
    } else {
    core_rate = 0;
    }
    if (pdiv_prim)
// pdiv_prim = best_div_prim;
    if (pdiv_clk)
// pdiv_clk = best_div_clk;
    return core_rate;
    }
    static void rp1_clock_choose_div_and_prate(struct clk_hw *hw,
    int parent_idx,
    unsigned long rate,
    unsigned long *prate,
    unsigned long *calc_rate)
    {
    struct rp1_clk_desc *clock = container_of(hw, struct rp1_clk_desc, hw);
    const struct rp1_clock_data *data = clock.data;
    struct clk_hw *clk_audio_hw = &clk_audio.hw;
    struct clk_hw *clk_i2s_hw = &clk_i2s.hw;
    struct clk_hw *parent;
    u32 div;
    u64 tmp;
    parent = clk_hw_get_parent_by_index(hw, parent_idx);
    if (hw == clk_i2s_hw && clk_i2s.cached_rate == rate && parent == clk_audio_hw) {
// prate = clk_audio->cached_rate;
// calc_rate = rate;
    return;
    }
    if (hw == clk_i2s_hw && parent == clk_audio_hw) {
    unsigned long core_rate, audio_rate, i2s_rate;
    int div_prim, div_clk;
    core_rate = calc_core_pll_rate(parent, rate, &div_prim, &div_clk);
    audio_rate = DIV_ROUND_CLOSEST(core_rate, div_prim);
    i2s_rate = DIV_ROUND_CLOSEST(audio_rate, div_clk);
    clk_audio_core.cached_rate = core_rate;
    clk_audio.cached_rate = audio_rate;
    clk_i2s.cached_rate = i2s_rate;
// prate = audio_rate;
// calc_rate = i2s_rate;
    return;
    }
// prate = clk_hw_get_rate(parent);
    div = rp1_clock_choose_div(rate, *prate, data);
    if (!div) {
// calc_rate = 0;
    return;
    }
// Recalculate to account for rounding errors
    tmp = (u64)*prate << CLK_DIV_FRAC_BITS;
    tmp = div_u64(tmp, div);
//
// Prevent overclocks - if all parent choices result in
// a downstream clock in excess of the maximum, then the
// call to set the clock will fail.
//
    if (tmp > data.max_freq)
// calc_rate = 0;
    else
// calc_rate = tmp;
    }
    static int rp1_clock_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_hw *parent, *best_parent = core::ptr::null_mut();
    let mut best_rate: c_ulong = 0;
    let mut best_prate: c_ulong = 0;
    let mut best_rate_diff: c_ulong = ULONG_MAX;
    unsigned long prate, calc_rate;
    size_t i;
//
// If the NO_REPARENT flag is set, try to use existing parent.
//
    if ((clk_hw_get_flags(hw) & CLK_SET_RATE_NO_REPARENT)) {
    i = rp1_clock_get_parent(hw);
    parent = clk_hw_get_parent_by_index(hw, i);
    if (parent) {
    rp1_clock_choose_div_and_prate(hw, i, req.rate, &prate,
    &calc_rate);
    if (calc_rate > 0) {
    req.best_parent_hw = parent;
    req.best_parent_rate = prate;
    req.rate = calc_rate;
    return 0;
    }
    }
    }
//
// Select parent clock that results in the closest rate (lower or
// higher)
//
    for (i = 0; i < clk_hw_get_num_parents(hw); i++) {
    parent = clk_hw_get_parent_by_index(hw, i);
    if (!parent)
    continue;
    rp1_clock_choose_div_and_prate(hw, i, req.rate, &prate,
    &calc_rate);
    if (abs_diff(calc_rate, req.rate) < best_rate_diff) {
    best_parent = parent;
    best_prate = prate;
    best_rate = calc_rate;
    best_rate_diff = abs_diff(calc_rate, req.rate);
    if (best_rate_diff == 0)
    break;
    }
    }
    if (best_rate == 0)
    return -EINVAL;
    req.best_parent_hw = best_parent;
    req.best_parent_rate = best_prate;
    req.rate = best_rate;
    return 0;
    }
    static int rp1_varsrc_set_rate(struct clk_hw *hw,
    unsigned long rate, unsigned long parent_rate)
    {
    struct rp1_clk_desc *clock = container_of(hw, struct rp1_clk_desc, hw);
//
// "varsrc" exists purely to let clock dividers know the frequency
// of an externally-managed clock source (such as MIPI DSI byte-clock)
// which may change at run-time as a side-effect of some other driver.
//
    clock.cached_rate = rate;
    return 0;
    }
    static unsigned long rp1_varsrc_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct rp1_clk_desc *clock = container_of(hw, struct rp1_clk_desc, hw);
    return clock.cached_rate;
    }
    static const struct clk_ops rp1_pll_core_ops = {
    .is_prepared = rp1_pll_core_is_on,
    .prepare = rp1_pll_core_on,
    .unprepare = rp1_pll_core_off,
    .set_rate = rp1_pll_core_set_rate,
    .recalc_rate = rp1_pll_core_recalc_rate,
    .determine_rate = rp1_pll_core_determine_rate,
    };
    static const struct clk_ops rp1_pll_ops = {
    .set_rate = rp1_pll_set_rate,
    .recalc_rate = rp1_pll_recalc_rate,
    .determine_rate = rp1_pll_determine_rate,
    };
    static const struct clk_ops rp1_pll_ph_ops = {
    .is_prepared = rp1_pll_ph_is_on,
    .prepare = rp1_pll_ph_on,
    .unprepare = rp1_pll_ph_off,
    .recalc_rate = rp1_pll_ph_recalc_rate,
    .determine_rate = rp1_pll_ph_determine_rate,
    };
    static const struct clk_ops rp1_pll_divider_ops = {
    .is_prepared = rp1_pll_divider_is_on,
    .prepare = rp1_pll_divider_on,
    .unprepare = rp1_pll_divider_off,
    .set_rate = rp1_pll_divider_set_rate,
    .recalc_rate = rp1_pll_divider_recalc_rate,
    .determine_rate = rp1_pll_divider_determine_rate,
    };
    static const struct clk_ops rp1_clk_ops = {
    .is_prepared = rp1_clock_is_on,
    .prepare = rp1_clock_on,
    .unprepare = rp1_clock_off,
    .recalc_rate = rp1_clock_recalc_rate,
    .get_parent = rp1_clock_get_parent,
    .set_parent = rp1_clock_set_parent,
    .set_rate_and_parent = rp1_clock_set_rate_and_parent,
    .set_rate = rp1_clock_set_rate,
    .determine_rate = rp1_clock_determine_rate,
    };
    static const struct clk_ops rp1_varsrc_ops = {
    .set_rate = rp1_varsrc_set_rate,
    .recalc_rate = rp1_varsrc_recalc_rate,
    .determine_rate = clk_determine_rate_noop,
    };
    static struct clk_hw *rp1_register_pll(struct rp1_clockman *clockman,
    struct rp1_clk_desc *desc)
    {
    int ret;
    desc.clockman = clockman;
    ret = devm_clk_hw_register(clockman.dev, &desc.hw);
    if (ret)
    return ERR_PTR(ret);
    return &desc.hw;
    }
    static struct clk_hw *rp1_register_pll_divider(struct rp1_clockman *clockman,
    struct rp1_clk_desc *desc)
    {
    const struct rp1_pll_data *divider_data = desc.data;
    int ret;
    desc.div.reg = clockman.regs + divider_data.ctrl_reg;
    desc.div.shift = __ffs(PLL_SEC_DIV_MASK);
    desc.div.width = __ffs(~(PLL_SEC_DIV_MASK >> desc.div.shift));
    desc.div.flags = CLK_DIVIDER_ROUND_CLOSEST;
    desc.div.lock = &clockman.regs_lock;
    desc.div.hw.init = desc.hw.init;
    desc.div.table = pll_sec_div_table;
    desc.clockman = clockman;
    ret = devm_clk_hw_register(clockman.dev, &desc.div.hw);
    if (ret)
    return ERR_PTR(ret);
    return &desc.div.hw;
    }
    static struct clk_hw *rp1_register_clock(struct rp1_clockman *clockman,
    struct rp1_clk_desc *desc)
    {
    const struct rp1_clock_data *clock_data = desc.data;
    int ret;
    if (WARN_ON_ONCE(MAX_CLK_PARENTS <
    clock_data.num_std_parents + clock_data.num_aux_parents))
    return ERR_PTR(-EINVAL);
// There must be a gap for the AUX selector
    if (WARN_ON_ONCE(clock_data.num_std_parents > AUX_SEL &&
    desc.hw.init.parent_data[AUX_SEL].index != -1))
    return ERR_PTR(-EINVAL);
    desc.clockman = clockman;
    ret = devm_clk_hw_register(clockman.dev, &desc.hw);
    if (ret)
    return ERR_PTR(ret);
    return &desc.hw;
    }
// Assignment helper macros for different clock types.

    __VA_ARGS__)

    __VA_ARGS__)

    __VA_ARGS__)
    static struct rp1_clk_desc pll_sys_core_desc = REGISTER_PLL(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "pll_sys_core",
    (const struct clk_parent_data[]) { { .index = 0 } },
    &rp1_pll_core_ops,
    CLK_IS_CRITICAL
    ),
    CLK_DATA(rp1_pll_core_data,
    .cs_reg = PLL_SYS_CS,
    .pwr_reg = PLL_SYS_PWR,
    .fbdiv_int_reg = PLL_SYS_FBDIV_INT,
    .fbdiv_frac_reg = PLL_SYS_FBDIV_FRAC,
    )
    );
    static struct rp1_clk_desc pll_audio_core_desc = REGISTER_PLL(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "pll_audio_core",
    (const struct clk_parent_data[]) { { .index = 0 } },
    &rp1_pll_core_ops,
    CLK_IS_CRITICAL
    ),
    CLK_DATA(rp1_pll_core_data,
    .cs_reg = PLL_AUDIO_CS,
    .pwr_reg = PLL_AUDIO_PWR,
    .fbdiv_int_reg = PLL_AUDIO_FBDIV_INT,
    .fbdiv_frac_reg = PLL_AUDIO_FBDIV_FRAC,
    )
    );
    static struct rp1_clk_desc pll_video_core_desc = REGISTER_PLL(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "pll_video_core",
    (const struct clk_parent_data[]) { { .index = 0 } },
    &rp1_pll_core_ops,
    CLK_IS_CRITICAL
    ),
    CLK_DATA(rp1_pll_core_data,
    .cs_reg = PLL_VIDEO_CS,
    .pwr_reg = PLL_VIDEO_PWR,
    .fbdiv_int_reg = PLL_VIDEO_FBDIV_INT,
    .fbdiv_frac_reg = PLL_VIDEO_FBDIV_FRAC,
    )
    );
    static struct rp1_clk_desc pll_sys_desc = REGISTER_PLL(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "pll_sys",
    (const struct clk_parent_data[]) {
    { .hw = &pll_sys_core_desc.hw }
    },
    &rp1_pll_ops,
    0
    ),
    CLK_DATA(rp1_pll_data,
    .ctrl_reg = PLL_SYS_PRIM,
    .fc0_src = FC_NUM(0, 2),
    )
    );
    static struct rp1_clk_desc pll_audio_desc = REGISTER_PLL(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "pll_audio",
    (const struct clk_parent_data[]) {
    { .hw = &pll_audio_core_desc.hw }
    },
    &rp1_pll_ops,
    CLK_SET_RATE_PARENT
    ),
    CLK_DATA(rp1_pll_data,
    .ctrl_reg = PLL_AUDIO_PRIM,
    .fc0_src = FC_NUM(4, 2),
    )
    );
    static struct rp1_clk_desc pll_video_desc = REGISTER_PLL(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "pll_video",
    (const struct clk_parent_data[]) {
    { .hw = &pll_video_core_desc.hw }
    },
    &rp1_pll_ops,
    0
    ),
    CLK_DATA(rp1_pll_data,
    .ctrl_reg = PLL_VIDEO_PRIM,
    .fc0_src = FC_NUM(3, 2),
    )
    );
    static struct rp1_clk_desc pll_sys_sec_desc = REGISTER_PLL_DIV(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "pll_sys_sec",
    (const struct clk_parent_data[]) {
    { .hw = &pll_sys_core_desc.hw }
    },
    &rp1_pll_divider_ops,
    0
    ),
    CLK_DATA(rp1_pll_data,
    .ctrl_reg = PLL_SYS_SEC,
    .fc0_src = FC_NUM(2, 2),
    )
    );
    static struct rp1_clk_desc pll_video_sec_desc = REGISTER_PLL_DIV(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "pll_video_sec",
    (const struct clk_parent_data[]) {
    { .hw = &pll_video_core_desc.hw }
    },
    &rp1_pll_divider_ops,
    0
    ),
    CLK_DATA(rp1_pll_data,
    .ctrl_reg = PLL_VIDEO_SEC,
    .fc0_src = FC_NUM(5, 3),
    )
    );
    static const struct clk_parent_data clk_eth_tsu_parents[] = {
    { .index = 0 },
    { .hw = &pll_video_sec_desc.hw },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    };
    static struct rp1_clk_desc clk_eth_tsu_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_eth_tsu",
    clk_eth_tsu_parents,
    &rp1_clk_ops,
    0
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 8,
    .ctrl_reg = CLK_ETH_TSU_CTRL,
    .div_int_reg = CLK_ETH_TSU_DIV_INT,
    .sel_reg = CLK_ETH_TSU_SEL,
    .div_int_max = DIV_INT_8BIT_MAX,
    .max_freq = 50 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(5, 7),
    )
    );
    static const struct clk_parent_data clk_eth_parents[] = {
    { .hw = &pll_sys_sec_desc.div.hw },
    { .hw = &pll_sys_desc.hw },
    { .hw = &pll_video_sec_desc.hw },
    };
    static struct rp1_clk_desc clk_eth_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_eth",
    clk_eth_parents,
    &rp1_clk_ops,
    0
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 3,
    .ctrl_reg = CLK_ETH_CTRL,
    .div_int_reg = CLK_ETH_DIV_INT,
    .sel_reg = CLK_ETH_SEL,
    .div_int_max = DIV_INT_8BIT_MAX,
    .max_freq = 125 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(4, 6),
    )
    );
    static const struct clk_parent_data clk_sys_parents[] = {
    { .index = 0 },
    { .index = -1 },
    { .hw = &pll_sys_desc.hw },
    };
    static struct rp1_clk_desc clk_sys_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_sys",
    clk_sys_parents,
    &rp1_clk_ops,
    CLK_IS_CRITICAL
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 3,
    .num_aux_parents = 0,
    .ctrl_reg = CLK_SYS_CTRL,
    .div_int_reg = CLK_SYS_DIV_INT,
    .sel_reg = CLK_SYS_SEL,
    .div_int_max = DIV_INT_24BIT_MAX,
    .max_freq = 200 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(0, 4),
    .clk_src_mask = 0x3,
    )
    );
    static struct rp1_clk_desc pll_sys_pri_ph_desc = REGISTER_PLL(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "pll_sys_pri_ph",
    (const struct clk_parent_data[]) {
    { .hw = &pll_sys_desc.hw }
    },
    &rp1_pll_ph_ops,
    0
    ),
    CLK_DATA(rp1_pll_ph_data,
    .ph_reg = PLL_SYS_PRIM,
    .fixed_divider = 2,
    .phase = RP1_PLL_PHASE_0,
    .fc0_src = FC_NUM(1, 2),
    )
    );
    static struct rp1_clk_desc pll_audio_pri_ph_desc = REGISTER_PLL(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "pll_audio_pri_ph",
    (const struct clk_parent_data[]) {
    { .hw = &pll_audio_desc.hw }
    },
    &rp1_pll_ph_ops,
    0
    ),
    CLK_DATA(rp1_pll_ph_data,
    .ph_reg = PLL_AUDIO_PRIM,
    .fixed_divider = 2,
    .phase = RP1_PLL_PHASE_0,
    .fc0_src = FC_NUM(5, 1),
    )
    );
    static struct rp1_clk_desc pll_video_pri_ph_desc = REGISTER_PLL(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "pll_video_pri_ph",
    (const struct clk_parent_data[]) {
    { .hw = &pll_video_desc.hw }
    },
    &rp1_pll_ph_ops,
    0
    ),
    CLK_DATA(rp1_pll_ph_data,
    .ph_reg = PLL_VIDEO_PRIM,
    .fixed_divider = 2,
    .phase = RP1_PLL_PHASE_0,
    .fc0_src = FC_NUM(4, 3),
    )
    );
    static struct rp1_clk_desc pll_audio_sec_desc = REGISTER_PLL_DIV(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "pll_audio_sec",
    (const struct clk_parent_data[]) {
    { .hw = &pll_audio_core_desc.hw }
    },
    &rp1_pll_divider_ops,
    0
    ),
    CLK_DATA(rp1_pll_data,
    .ctrl_reg = PLL_AUDIO_SEC,
    .fc0_src = FC_NUM(6, 2),
    )
    );
    static struct rp1_clk_desc pll_audio_tern_desc = REGISTER_PLL_DIV(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "pll_audio_tern",
    (const struct clk_parent_data[]) {
    { .hw = &pll_audio_core_desc.hw }
    },
    &rp1_pll_divider_ops,
    0
    ),
    CLK_DATA(rp1_pll_data,
    .ctrl_reg = PLL_AUDIO_TERN,
    .fc0_src = FC_NUM(6, 2),
    )
    );
    static struct rp1_clk_desc clk_slow_sys_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_slow_sys",
    (const struct clk_parent_data[]) { { .index = 0 } },
    &rp1_clk_ops,
    CLK_IS_CRITICAL
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 1,
    .num_aux_parents = 0,
    .ctrl_reg = CLK_SLOW_SYS_CTRL,
    .div_int_reg = CLK_SLOW_SYS_DIV_INT,
    .sel_reg = CLK_SLOW_SYS_SEL,
    .div_int_max = DIV_INT_8BIT_MAX,
    .max_freq = 50 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(1, 4),
    .clk_src_mask = 0x1,
    )
    );
    static const struct clk_parent_data clk_dma_parents[] = {
    { .hw = &pll_sys_pri_ph_desc.hw },
    { .hw = &pll_video_desc.hw },
    { .index = 0 },
    };
    static struct rp1_clk_desc clk_dma_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_dma",
    clk_dma_parents,
    &rp1_clk_ops,
    0
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 3,
    .ctrl_reg = CLK_DMA_CTRL,
    .div_int_reg = CLK_DMA_DIV_INT,
    .sel_reg = CLK_DMA_SEL,
    .div_int_max = DIV_INT_8BIT_MAX,
    .max_freq = 100 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(2, 2),
    )
    );
    static const struct clk_parent_data clk_uart_parents[] = {
    { .hw = &pll_sys_pri_ph_desc.hw },
    { .hw = &pll_video_desc.hw },
    { .index = 0 },
    };
    static struct rp1_clk_desc clk_uart_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_uart",
    clk_uart_parents,
    &rp1_clk_ops,
    0
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 3,
    .ctrl_reg = CLK_UART_CTRL,
    .div_int_reg = CLK_UART_DIV_INT,
    .sel_reg = CLK_UART_SEL,
    .div_int_max = DIV_INT_8BIT_MAX,
    .max_freq = 100 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(6, 7),
    )
    );
    static const struct clk_parent_data clk_pwm0_parents[] = {
    { .index = -1 },
    { .hw = &pll_video_sec_desc.hw },
    { .index = 0 },
    };
    static struct rp1_clk_desc clk_pwm0_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_pwm0",
    clk_pwm0_parents,
    &rp1_clk_ops,
    0
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 3,
    .ctrl_reg = CLK_PWM0_CTRL,
    .div_int_reg = CLK_PWM0_DIV_INT,
    .div_frac_reg = CLK_PWM0_DIV_FRAC,
    .sel_reg = CLK_PWM0_SEL,
    .div_int_max = DIV_INT_16BIT_MAX,
    .max_freq = 76800 * HZ_PER_KHZ,
    .fc0_src = FC_NUM(0, 5),
    )
    );
    static const struct clk_parent_data clk_pwm1_parents[] = {
    { .index = -1 },
    { .hw = &pll_video_sec_desc.hw },
    { .index = 0 },
    };
    static struct rp1_clk_desc clk_pwm1_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_pwm1",
    clk_pwm1_parents,
    &rp1_clk_ops,
    0
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 3,
    .ctrl_reg = CLK_PWM1_CTRL,
    .div_int_reg = CLK_PWM1_DIV_INT,
    .div_frac_reg = CLK_PWM1_DIV_FRAC,
    .sel_reg = CLK_PWM1_SEL,
    .div_int_max = DIV_INT_16BIT_MAX,
    .max_freq = 76800 * HZ_PER_KHZ,
    .fc0_src = FC_NUM(1, 5),
    )
    );
    static const struct clk_parent_data clk_audio_in_parents[] = {
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .hw = &pll_video_sec_desc.hw },
    { .index = 0 },
    };
    static struct rp1_clk_desc clk_audio_in_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_audio_in",
    clk_audio_in_parents,
    &rp1_clk_ops,
    0
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 5,
    .ctrl_reg = CLK_AUDIO_IN_CTRL,
    .div_int_reg = CLK_AUDIO_IN_DIV_INT,
    .sel_reg = CLK_AUDIO_IN_SEL,
    .div_int_max = DIV_INT_8BIT_MAX,
    .max_freq = 76800 * HZ_PER_KHZ,
    .fc0_src = FC_NUM(2, 5),
    )
    );
    static const struct clk_parent_data clk_audio_out_parents[] = {
    { .index = -1 },
    { .index = -1 },
    { .hw = &pll_video_sec_desc.hw },
    { .index = 0 },
    };
    static struct rp1_clk_desc clk_audio_out_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_audio_out",
    clk_audio_out_parents,
    &rp1_clk_ops,
    0
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 4,
    .ctrl_reg = CLK_AUDIO_OUT_CTRL,
    .div_int_reg = CLK_AUDIO_OUT_DIV_INT,
    .sel_reg = CLK_AUDIO_OUT_SEL,
    .div_int_max = DIV_INT_8BIT_MAX,
    .max_freq = 153600 * HZ_PER_KHZ,
    .fc0_src = FC_NUM(3, 5),
    )
    );
    static const struct clk_parent_data clk_i2s_parents[] = {
    { .index = 0 },
    { .hw = &pll_audio_desc.hw },
    { .hw = &pll_audio_sec_desc.hw },
    };
    static struct rp1_clk_desc clk_i2s_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_i2s",
    clk_i2s_parents,
    &rp1_clk_ops,
    CLK_SET_RATE_PARENT
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 3,
    .ctrl_reg = CLK_I2S_CTRL,
    .div_int_reg = CLK_I2S_DIV_INT,
    .sel_reg = CLK_I2S_SEL,
    .div_int_max = DIV_INT_8BIT_MAX,
    .max_freq = 50 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(4, 4),
    )
    );
    static struct rp1_clk_desc clk_mipi0_cfg_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_mipi0_cfg",
    (const struct clk_parent_data[]) { { .index = 0 } },
    &rp1_clk_ops,
    0
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 1,
    .ctrl_reg = CLK_MIPI0_CFG_CTRL,
    .div_int_reg = CLK_MIPI0_CFG_DIV_INT,
    .sel_reg = CLK_MIPI0_CFG_SEL,
    .div_int_max = DIV_INT_8BIT_MAX,
    .max_freq = 50 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(4, 5),
    )
    );
    static struct rp1_clk_desc clk_mipi1_cfg_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_mipi1_cfg",
    (const struct clk_parent_data[]) { { .index = 0 } },
    &rp1_clk_ops,
    0
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 1,
    .ctrl_reg = CLK_MIPI1_CFG_CTRL,
    .div_int_reg = CLK_MIPI1_CFG_DIV_INT,
    .sel_reg = CLK_MIPI1_CFG_SEL,
    .div_int_max = DIV_INT_8BIT_MAX,
    .max_freq = 50 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(5, 6),
    .clk_src_mask = 0x1,
    )
    );
    static struct rp1_clk_desc clk_adc_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_adc",
    (const struct clk_parent_data[]) { { .index = 0 } },
    &rp1_clk_ops,
    0
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 1,
    .ctrl_reg = CLK_ADC_CTRL,
    .div_int_reg = CLK_ADC_DIV_INT,
    .sel_reg = CLK_ADC_SEL,
    .div_int_max = DIV_INT_8BIT_MAX,
    .max_freq = 50 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(5, 5),
    )
    );
    static struct rp1_clk_desc clk_sdio_timer_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_sdio_timer",
    (const struct clk_parent_data[]) { { .index = 0 } },
    &rp1_clk_ops,
    0
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 1,
    .ctrl_reg = CLK_SDIO_TIMER_CTRL,
    .div_int_reg = CLK_SDIO_TIMER_DIV_INT,
    .sel_reg = CLK_SDIO_TIMER_SEL,
    .div_int_max = DIV_INT_8BIT_MAX,
    .max_freq = 50 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(3, 4),
    )
    );
    static struct rp1_clk_desc clk_sdio_alt_src_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_sdio_alt_src",
    (const struct clk_parent_data[]) {
    { .hw = &pll_sys_desc.hw }
    },
    &rp1_clk_ops,
    0
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 1,
    .ctrl_reg = CLK_SDIO_ALT_SRC_CTRL,
    .div_int_reg = CLK_SDIO_ALT_SRC_DIV_INT,
    .sel_reg = CLK_SDIO_ALT_SRC_SEL,
    .div_int_max = DIV_INT_8BIT_MAX,
    .max_freq = 200 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(5, 4),
    )
    );
    static const struct clk_parent_data clk_dpi_parents[] = {
    { .hw = &pll_sys_desc.hw },
    { .hw = &pll_video_sec_desc.hw },
    { .hw = &pll_video_desc.hw },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    };
    static struct rp1_clk_desc clk_dpi_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_dpi",
    clk_dpi_parents,
    &rp1_clk_ops,
    CLK_SET_RATE_NO_REPARENT /* Let DPI driver set parent */
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 8,
    .ctrl_reg = VIDEO_CLK_DPI_CTRL,
    .div_int_reg = VIDEO_CLK_DPI_DIV_INT,
    .sel_reg = VIDEO_CLK_DPI_SEL,
    .div_int_max = DIV_INT_8BIT_MAX,
    .max_freq = 200 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(1, 6),
    )
    );
    static const struct clk_parent_data clk_gp0_parents[] = {
    { .index = 0 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .hw = &pll_sys_desc.hw },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .hw = &clk_i2s_desc.hw },
    { .hw = &clk_adc_desc.hw },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .hw = &clk_sys_desc.hw },
    };
    static struct rp1_clk_desc clk_gp0_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_gp0",
    clk_gp0_parents,
    &rp1_clk_ops,
    0
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 16,
    .oe_mask = BIT(0),
    .ctrl_reg = CLK_GP0_CTRL,
    .div_int_reg = CLK_GP0_DIV_INT,
    .div_frac_reg = CLK_GP0_DIV_FRAC,
    .sel_reg = CLK_GP0_SEL,
    .div_int_max = DIV_INT_16BIT_MAX,
    .max_freq = 100 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(0, 1),
    )
    );
    static const struct clk_parent_data clk_gp1_parents[] = {
    { .hw = &clk_sdio_timer_desc.hw },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .hw = &pll_sys_pri_ph_desc.hw },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .hw = &clk_adc_desc.hw },
    { .hw = &clk_dpi_desc.hw },
    { .hw = &clk_pwm0_desc.hw },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    };
    static struct rp1_clk_desc clk_gp1_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_gp1",
    clk_gp1_parents,
    &rp1_clk_ops,
    0
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 16,
    .oe_mask = BIT(1),
    .ctrl_reg = CLK_GP1_CTRL,
    .div_int_reg = CLK_GP1_DIV_INT,
    .div_frac_reg = CLK_GP1_DIV_FRAC,
    .sel_reg = CLK_GP1_SEL,
    .div_int_max = DIV_INT_16BIT_MAX,
    .max_freq = 100 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(1, 1),
    )
    );
    static struct rp1_clk_desc clksrc_mipi0_dsi_byteclk_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clksrc_mipi0_dsi_byteclk",
    (const struct clk_parent_data[]) { { .index = 0 } },
    &rp1_varsrc_ops,
    0
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 1,
    .num_aux_parents = 0,
    )
    );
    static struct rp1_clk_desc clksrc_mipi1_dsi_byteclk_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clksrc_mipi1_dsi_byteclk",
    (const struct clk_parent_data[]) { { .index = 0 } },
    &rp1_varsrc_ops,
    0
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 1,
    .num_aux_parents = 0,
    )
    );
    static const struct clk_parent_data clk_mipi0_dpi_parents[] = {
    { .hw = &pll_sys_desc.hw },
    { .hw = &pll_video_sec_desc.hw },
    { .hw = &pll_video_desc.hw },
    { .hw = &clksrc_mipi0_dsi_byteclk_desc.hw },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    };
    static struct rp1_clk_desc clk_mipi0_dpi_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_mipi0_dpi",
    clk_mipi0_dpi_parents,
    &rp1_clk_ops,
    CLK_SET_RATE_NO_REPARENT /* Let DSI driver set parent */
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 8,
    .ctrl_reg = VIDEO_CLK_MIPI0_DPI_CTRL,
    .div_int_reg = VIDEO_CLK_MIPI0_DPI_DIV_INT,
    .div_frac_reg = VIDEO_CLK_MIPI0_DPI_DIV_FRAC,
    .sel_reg = VIDEO_CLK_MIPI0_DPI_SEL,
    .div_int_max = DIV_INT_8BIT_MAX,
    .max_freq = 200 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(2, 6),
    )
    );
    static const struct clk_parent_data clk_mipi1_dpi_parents[] = {
    { .hw = &pll_sys_desc.hw },
    { .hw = &pll_video_sec_desc.hw },
    { .hw = &pll_video_desc.hw },
    { .hw = &clksrc_mipi1_dsi_byteclk_desc.hw },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    };
    static struct rp1_clk_desc clk_mipi1_dpi_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_mipi1_dpi",
    clk_mipi1_dpi_parents,
    &rp1_clk_ops,
    CLK_SET_RATE_NO_REPARENT /* Let DSI driver set parent */
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 8,
    .ctrl_reg = VIDEO_CLK_MIPI1_DPI_CTRL,
    .div_int_reg = VIDEO_CLK_MIPI1_DPI_DIV_INT,
    .div_frac_reg = VIDEO_CLK_MIPI1_DPI_DIV_FRAC,
    .sel_reg = VIDEO_CLK_MIPI1_DPI_SEL,
    .div_int_max = DIV_INT_8BIT_MAX,
    .max_freq = 200 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(3, 6),
    )
    );
    static const struct clk_parent_data clk_gp2_parents[] = {
    { .hw = &clk_sdio_alt_src_desc.hw },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .hw = &pll_sys_sec_desc.hw },
    { .index = -1 },
    { .hw = &pll_video_desc.hw },
    { .hw = &clk_audio_in_desc.hw },
    { .hw = &clk_dpi_desc.hw },
    { .hw = &clk_pwm0_desc.hw },
    { .hw = &clk_pwm1_desc.hw },
    { .hw = &clk_mipi0_dpi_desc.hw },
    { .hw = &clk_mipi1_cfg_desc.hw },
    { .hw = &clk_sys_desc.hw },
    };
    static struct rp1_clk_desc clk_gp2_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_gp2",
    clk_gp2_parents,
    &rp1_clk_ops,
    0
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 16,
    .oe_mask = BIT(2),
    .ctrl_reg = CLK_GP2_CTRL,
    .div_int_reg = CLK_GP2_DIV_INT,
    .div_frac_reg = CLK_GP2_DIV_FRAC,
    .sel_reg = CLK_GP2_SEL,
    .div_int_max = DIV_INT_16BIT_MAX,
    .max_freq = 100 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(2, 1),
    )
    );
    static const struct clk_parent_data clk_gp3_parents[] = {
    { .index = 0 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .hw = &pll_video_pri_ph_desc.hw },
    { .hw = &clk_audio_out_desc.hw },
    { .index = -1 },
    { .index = -1 },
    { .hw = &clk_mipi1_dpi_desc.hw },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    };
    static struct rp1_clk_desc clk_gp3_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_gp3",
    clk_gp3_parents,
    &rp1_clk_ops,
    0
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 16,
    .oe_mask = BIT(3),
    .ctrl_reg = CLK_GP3_CTRL,
    .div_int_reg = CLK_GP3_DIV_INT,
    .div_frac_reg = CLK_GP3_DIV_FRAC,
    .sel_reg = CLK_GP3_SEL,
    .div_int_max = DIV_INT_16BIT_MAX,
    .max_freq = 100 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(3, 1),
    )
    );
    static const struct clk_parent_data clk_gp4_parents[] = {
    { .index = 0 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .hw = &pll_video_sec_desc.hw },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .hw = &clk_mipi0_cfg_desc.hw },
    { .hw = &clk_uart_desc.hw },
    { .index = -1 },
    { .index = -1 },
    { .hw = &clk_sys_desc.hw },
    };
    static struct rp1_clk_desc clk_gp4_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_gp4",
    clk_gp4_parents,
    &rp1_clk_ops,
    0
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 16,
    .oe_mask = BIT(4),
    .ctrl_reg = CLK_GP4_CTRL,
    .div_int_reg = CLK_GP4_DIV_INT,
    .div_frac_reg = CLK_GP4_DIV_FRAC,
    .sel_reg = CLK_GP4_SEL,
    .div_int_max = DIV_INT_16BIT_MAX,
    .max_freq = 100 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(4, 1),
    )
    );
    static const struct clk_parent_data clk_vec_parents[] = {
    { .hw = &pll_sys_pri_ph_desc.hw },
    { .hw = &pll_video_sec_desc.hw },
    { .hw = &pll_video_desc.hw },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    };
    static struct rp1_clk_desc clk_vec_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_vec",
    clk_vec_parents,
    &rp1_clk_ops,
    CLK_SET_RATE_NO_REPARENT /* Let VEC driver set parent */
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 8,
    .ctrl_reg = VIDEO_CLK_VEC_CTRL,
    .div_int_reg = VIDEO_CLK_VEC_DIV_INT,
    .sel_reg = VIDEO_CLK_VEC_SEL,
    .div_int_max = DIV_INT_8BIT_MAX,
    .max_freq = 108 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(0, 6),
    )
    );
    static const struct clk_parent_data clk_gp5_parents[] = {
    { .index = 0 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .hw = &pll_video_sec_desc.hw },
    { .hw = &clk_eth_tsu_desc.hw },
    { .index = -1 },
    { .hw = &clk_vec_desc.hw },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    { .index = -1 },
    };
    static struct rp1_clk_desc clk_gp5_desc = REGISTER_CLK(
    .hw.init = CLK_HW_INIT_PARENTS_DATA(
    "clk_gp5",
    clk_gp5_parents,
    &rp1_clk_ops,
    0
    ),
    CLK_DATA(rp1_clock_data,
    .num_std_parents = 0,
    .num_aux_parents = 16,
    .oe_mask = BIT(5),
    .ctrl_reg = CLK_GP5_CTRL,
    .div_int_reg = CLK_GP5_DIV_INT,
    .div_frac_reg = CLK_GP5_DIV_FRAC,
    .sel_reg = CLK_GP5_SEL,
    .div_int_max = DIV_INT_16BIT_MAX,
    .max_freq = 100 * HZ_PER_MHZ,
    .fc0_src = FC_NUM(5, 1),
    )
    );
    static struct rp1_clk_desc *const clk_desc_array[] = {
    [RP1_PLL_SYS_CORE] = &pll_sys_core_desc,
    [RP1_PLL_AUDIO_CORE] = &pll_audio_core_desc,
    [RP1_PLL_VIDEO_CORE] = &pll_video_core_desc,
    [RP1_PLL_SYS] = &pll_sys_desc,
    [RP1_CLK_ETH_TSU] = &clk_eth_tsu_desc,
    [RP1_CLK_ETH] = &clk_eth_desc,
    [RP1_CLK_SYS] = &clk_sys_desc,
    [RP1_PLL_SYS_PRI_PH] = &pll_sys_pri_ph_desc,
    [RP1_PLL_SYS_SEC] = &pll_sys_sec_desc,
    [RP1_PLL_AUDIO] = &pll_audio_desc,
    [RP1_PLL_VIDEO] = &pll_video_desc,
    [RP1_PLL_AUDIO_PRI_PH] = &pll_audio_pri_ph_desc,
    [RP1_PLL_VIDEO_PRI_PH] = &pll_video_pri_ph_desc,
    [RP1_PLL_AUDIO_SEC] = &pll_audio_sec_desc,
    [RP1_PLL_VIDEO_SEC] = &pll_video_sec_desc,
    [RP1_PLL_AUDIO_TERN] = &pll_audio_tern_desc,
    [RP1_CLK_SLOW_SYS] = &clk_slow_sys_desc,
    [RP1_CLK_DMA] = &clk_dma_desc,
    [RP1_CLK_UART] = &clk_uart_desc,
    [RP1_CLK_PWM0] = &clk_pwm0_desc,
    [RP1_CLK_PWM1] = &clk_pwm1_desc,
    [RP1_CLK_AUDIO_IN] = &clk_audio_in_desc,
    [RP1_CLK_AUDIO_OUT] = &clk_audio_out_desc,
    [RP1_CLK_I2S] = &clk_i2s_desc,
    [RP1_CLK_MIPI0_CFG] = &clk_mipi0_cfg_desc,
    [RP1_CLK_MIPI1_CFG] = &clk_mipi1_cfg_desc,
    [RP1_CLK_ADC] = &clk_adc_desc,
    [RP1_CLK_SDIO_TIMER] = &clk_sdio_timer_desc,
    [RP1_CLK_SDIO_ALT_SRC] = &clk_sdio_alt_src_desc,
    [RP1_CLK_GP0] = &clk_gp0_desc,
    [RP1_CLK_GP1] = &clk_gp1_desc,
    [RP1_CLK_GP2] = &clk_gp2_desc,
    [RP1_CLK_GP3] = &clk_gp3_desc,
    [RP1_CLK_GP4] = &clk_gp4_desc,
    [RP1_CLK_GP5] = &clk_gp5_desc,
    [RP1_CLK_VEC] = &clk_vec_desc,
    [RP1_CLK_DPI] = &clk_dpi_desc,
    [RP1_CLK_MIPI0_DPI] = &clk_mipi0_dpi_desc,
    [RP1_CLK_MIPI1_DPI] = &clk_mipi1_dpi_desc,
    [RP1_CLK_MIPI0_DSI_BYTECLOCK] = &clksrc_mipi0_dsi_byteclk_desc,
    [RP1_CLK_MIPI1_DSI_BYTECLOCK] = &clksrc_mipi1_dsi_byteclk_desc,
    };
    static const struct regmap_range rp1_reg_ranges[] = {
    regmap_reg_range(PLL_SYS_CS, PLL_SYS_SEC),
    regmap_reg_range(PLL_AUDIO_CS, PLL_AUDIO_TERN),
    regmap_reg_range(PLL_VIDEO_CS, PLL_VIDEO_SEC),
    regmap_reg_range(GPCLK_OE_CTRL, GPCLK_OE_CTRL),
    regmap_reg_range(CLK_SYS_CTRL, CLK_SYS_DIV_INT),
    regmap_reg_range(CLK_SYS_SEL, CLK_SYS_SEL),
    regmap_reg_range(CLK_SLOW_SYS_CTRL, CLK_SLOW_SYS_DIV_INT),
    regmap_reg_range(CLK_SLOW_SYS_SEL, CLK_SLOW_SYS_SEL),
    regmap_reg_range(CLK_DMA_CTRL, CLK_DMA_DIV_INT),
    regmap_reg_range(CLK_DMA_SEL, CLK_DMA_SEL),
    regmap_reg_range(CLK_UART_CTRL, CLK_UART_DIV_INT),
    regmap_reg_range(CLK_UART_SEL, CLK_UART_SEL),
    regmap_reg_range(CLK_ETH_CTRL, CLK_ETH_DIV_INT),
    regmap_reg_range(CLK_ETH_SEL, CLK_ETH_SEL),
    regmap_reg_range(CLK_PWM0_CTRL, CLK_PWM0_SEL),
    regmap_reg_range(CLK_PWM1_CTRL, CLK_PWM1_SEL),
    regmap_reg_range(CLK_AUDIO_IN_CTRL, CLK_AUDIO_IN_DIV_INT),
    regmap_reg_range(CLK_AUDIO_IN_SEL, CLK_AUDIO_IN_SEL),
    regmap_reg_range(CLK_AUDIO_OUT_CTRL, CLK_AUDIO_OUT_DIV_INT),
    regmap_reg_range(CLK_AUDIO_OUT_SEL, CLK_AUDIO_OUT_SEL),
    regmap_reg_range(CLK_I2S_CTRL, CLK_I2S_DIV_INT),
    regmap_reg_range(CLK_I2S_SEL, CLK_I2S_SEL),
    regmap_reg_range(CLK_MIPI0_CFG_CTRL, CLK_MIPI0_CFG_DIV_INT),
    regmap_reg_range(CLK_MIPI0_CFG_SEL, CLK_MIPI0_CFG_SEL),
    regmap_reg_range(CLK_MIPI1_CFG_CTRL, CLK_MIPI1_CFG_DIV_INT),
    regmap_reg_range(CLK_MIPI1_CFG_SEL, CLK_MIPI1_CFG_SEL),
    regmap_reg_range(CLK_PCIE_AUX_CTRL, CLK_PCIE_AUX_DIV_INT),
    regmap_reg_range(CLK_PCIE_AUX_SEL, CLK_PCIE_AUX_SEL),
    regmap_reg_range(CLK_USBH0_MICROFRAME_CTRL, CLK_USBH0_MICROFRAME_DIV_INT),
    regmap_reg_range(CLK_USBH0_MICROFRAME_SEL, CLK_USBH0_MICROFRAME_SEL),
    regmap_reg_range(CLK_USBH1_MICROFRAME_CTRL, CLK_USBH1_MICROFRAME_DIV_INT),
    regmap_reg_range(CLK_USBH1_MICROFRAME_SEL, CLK_USBH1_MICROFRAME_SEL),
    regmap_reg_range(CLK_USBH0_SUSPEND_CTRL, CLK_USBH0_SUSPEND_DIV_INT),
    regmap_reg_range(CLK_USBH0_SUSPEND_SEL, CLK_USBH0_SUSPEND_SEL),
    regmap_reg_range(CLK_USBH1_SUSPEND_CTRL, CLK_USBH1_SUSPEND_DIV_INT),
    regmap_reg_range(CLK_USBH1_SUSPEND_SEL, CLK_USBH1_SUSPEND_SEL),
    regmap_reg_range(CLK_ETH_TSU_CTRL, CLK_ETH_TSU_DIV_INT),
    regmap_reg_range(CLK_ETH_TSU_SEL, CLK_ETH_TSU_SEL),
    regmap_reg_range(CLK_ADC_CTRL, CLK_ADC_DIV_INT),
    regmap_reg_range(CLK_ADC_SEL, CLK_ADC_SEL),
    regmap_reg_range(CLK_SDIO_TIMER_CTRL, CLK_SDIO_TIMER_DIV_INT),
    regmap_reg_range(CLK_SDIO_TIMER_SEL, CLK_SDIO_TIMER_SEL),
    regmap_reg_range(CLK_SDIO_ALT_SRC_CTRL, CLK_SDIO_ALT_SRC_DIV_INT),
    regmap_reg_range(CLK_SDIO_ALT_SRC_SEL, CLK_SDIO_ALT_SRC_SEL),
    regmap_reg_range(CLK_GP0_CTRL, CLK_GP0_SEL),
    regmap_reg_range(CLK_GP1_CTRL, CLK_GP1_SEL),
    regmap_reg_range(CLK_GP2_CTRL, CLK_GP2_SEL),
    regmap_reg_range(CLK_GP3_CTRL, CLK_GP3_SEL),
    regmap_reg_range(CLK_GP4_CTRL, CLK_GP4_SEL),
    regmap_reg_range(CLK_GP5_CTRL, CLK_GP5_SEL),
    regmap_reg_range(CLK_SYS_RESUS_CTRL, CLK_SYS_RESUS_CTRL),
    regmap_reg_range(CLK_SLOW_SYS_RESUS_CTRL, CLK_SLOW_SYS_RESUS_CTRL),
    regmap_reg_range(FC0_REF_KHZ, FC0_RESULT),
    regmap_reg_range(VIDEO_CLK_VEC_CTRL, VIDEO_CLK_VEC_DIV_INT),
    regmap_reg_range(VIDEO_CLK_VEC_SEL, VIDEO_CLK_DPI_DIV_INT),
    regmap_reg_range(VIDEO_CLK_DPI_SEL, VIDEO_CLK_MIPI1_DPI_SEL),
    };
    static const struct regmap_access_table rp1_reg_table = {
    .yes_ranges = rp1_reg_ranges,
    .n_yes_ranges = ARRAY_SIZE(rp1_reg_ranges),
    };
    static const struct regmap_config rp1_clk_regmap_cfg = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = PLL_VIDEO_SEC,
    .name = "rp1-clk",
    .rd_table = &rp1_reg_table,
    .disable_locking = true,
    };
#[no_mangle]
unsafe extern "C" fn rp1_clk_probe(pdev: *mut platform_device) -> c_int {
    static int rp1_clk_probe(struct platform_device *pdev)
    {
    let mut asize: usize = ARRAY_SIZE(clk_desc_array);
    struct rp1_clk_desc *desc;
    struct device *dev = &pdev.dev;
    struct rp1_clockman *clockman;
    struct clk_hw **hws;
    unsigned int i;
    clockman = devm_kzalloc(dev, struct_size(clockman, onecell.hws, asize),
    GFP_KERNEL);
    if (!clockman)
    return -ENOMEM;
    spin_lock_init(&clockman.regs_lock);
    clockman.dev = dev;
    clockman.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(clockman.regs))
    return PTR_ERR(clockman.regs);
    clockman.regmap = devm_regmap_init_mmio(dev, clockman.regs,
    &rp1_clk_regmap_cfg);
    if (IS_ERR(clockman.regmap)) {
    dev_err_probe(dev, PTR_ERR(clockman.regmap),
    "could not init clock regmap\n");
    return PTR_ERR(clockman.regmap);
    }
    clockman.onecell.num = asize;
    hws = clockman.onecell.hws;
    for (i = 0; i < asize; i++) {
    desc = clk_desc_array[i];
    if (desc && desc.clk_register && desc.data)
    hws[i] = desc.clk_register(clockman, desc);
    }
    clk_audio_core = &pll_audio_core_desc;
    clk_audio = &pll_audio_desc;
    clk_i2s = &clk_i2s_desc;
    clk_xosc = clk_hw_get_parent_by_index(&clk_i2s.hw, 0);
    platform_set_drvdata(pdev, clockman);
    return devm_of_clk_add_hw_provider(dev, of_clk_hw_onecell_get,
    &clockman.onecell);
    }
    static const struct of_device_id rp1_clk_of_match[] = {
    { .compatible = "raspberrypi,rp1-clocks" },
    {}
    };
    MODULE_DEVICE_TABLE(of, rp1_clk_of_match);
    static struct platform_driver rp1_clk_driver = {
    .driver = {
    .name = "rp1-clk",
    .of_match_table = rp1_clk_of_match,
    },
    .probe = rp1_clk_probe,
    };
    module_platform_driver(rp1_clk_driver);
    MODULE_AUTHOR("Naushir Patuck <naush@raspberrypi.com>");
    MODULE_AUTHOR("Andrea della Porta <andrea.porta@suse.com>");
    MODULE_DESCRIPTION("RP1 clock driver");
    MODULE_LICENSE("GPL");
