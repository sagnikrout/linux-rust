//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-si514.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Driver for Silicon Labs Si514 Programmable Oscillator
//
// Copyright (C) 2015 Topic Embedded Products
//
// Author: Mike Looijmans <mike.looijmans@topic.nl>
//

// I2C registers
pub const SI514_REG_LP: c_int = 0;
pub const SI514_REG_M_FRAC1: c_int = 5;
pub const SI514_REG_M_FRAC2: c_int = 6;
pub const SI514_REG_M_FRAC3: c_int = 7;
pub const SI514_REG_M_INT_FRAC: c_int = 8;
pub const SI514_REG_M_INT: c_int = 9;
pub const SI514_REG_HS_DIV: c_int = 10;
pub const SI514_REG_LS_HS_DIV: c_int = 11;
pub const SI514_REG_OE_STATE: c_int = 14;
pub const SI514_REG_RESET: c_int = 128;
pub const SI514_REG_CONTROL: c_int = 132;
// Register values

pub const HS_DIV_MAX: c_int = 1022;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_si514 {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub i2c_client: *mut i2c_client,
}

// Multiplier/divider settings
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_si514_muldiv {
    pub /: *mut *mut u32 m_frac; / 29-bit Fractional part of multiplier M,
    pub /: *mut *mut u8 m_int; / Integer part of multiplier M, 65..78,
    pub /: *mut *mut u8 ls_div_bits; / 2nd divider, as 2^x,
    pub /: *mut *mut u16 hs_div; / 1st divider, must be even and 10<=x<=1022,
}

// Enables or disables the output driver
#[no_mangle]
unsafe extern "C" fn si514_enable_output(data: *mut clk_si514, enable: bool) -> c_int {
    static int si514_enable_output(struct clk_si514 *data, bool enable)
    {
    return regmap_update_bits(data.regmap, SI514_REG_CONTROL,
    SI514_CONTROL_OE, enable ? SI514_CONTROL_OE : 0);
    }
#[no_mangle]
unsafe extern "C" fn si514_prepare(hw: *mut clk_hw) -> c_int {
    static int si514_prepare(struct clk_hw *hw)
    {
    struct clk_si514 *data = to_clk_si514(hw);
    return si514_enable_output(data, true);
    }
#[no_mangle]
unsafe extern "C" fn si514_unprepare(hw: *mut clk_hw) {
    static void si514_unprepare(struct clk_hw *hw)
    {
    struct clk_si514 *data = to_clk_si514(hw);
    si514_enable_output(data, false);
    }
#[no_mangle]
unsafe extern "C" fn si514_is_prepared(hw: *mut clk_hw) -> c_int {
    static int si514_is_prepared(struct clk_hw *hw)
    {
    struct clk_si514 *data = to_clk_si514(hw);
    unsigned int val;
    int err;
    err = regmap_read(data.regmap, SI514_REG_CONTROL, &val);
    if (err < 0)
    return err;
    return !!(val & SI514_CONTROL_OE);
    }
// Retrieve clock multiplier and dividers from hardware
    static int si514_get_muldiv(struct clk_si514 *data,
    struct clk_si514_muldiv *settings)
    {
    int err;
    u8 reg[7];
    err = regmap_bulk_read(data.regmap, SI514_REG_M_FRAC1,
    reg, ARRAY_SIZE(reg));
    if (err)
    return err;
    settings.m_frac = reg[0] | reg[1] << 8 | reg[2] << 16 |
    (reg[3] & 0x1F) << 24;
    settings.m_int = (reg[4] & 0x3f) << 3 | reg[3] >> 5;
    settings.ls_div_bits = (reg[6] >> 4) & 0x07;
    settings.hs_div = (reg[6] & 0x03) << 8 | reg[5];
    return 0;
    }
    static int si514_set_muldiv(struct clk_si514 *data,
    struct clk_si514_muldiv *settings)
    {
    u8 lp;
    u8 reg[7];
    int err;
// Calculate LP1/LP2 according to table 13 in the datasheet
// 65.259980246
    if (settings.m_int < 65 ||
    (settings.m_int == 65 && settings.m_frac <= 139575831))
    lp = 0x22;
// 67.859763463
    else if (settings.m_int < 67 ||
    (settings.m_int == 67 && settings.m_frac <= 461581994))
    lp = 0x23;
// 72.937624981
    else if (settings.m_int < 72 ||
    (settings.m_int == 72 && settings.m_frac <= 503383578))
    lp = 0x33;
// 75.843265046
    else if (settings.m_int < 75 ||
    (settings.m_int == 75 && settings.m_frac <= 452724474))
    lp = 0x34;
    else
    lp = 0x44;
    err = regmap_write(data.regmap, SI514_REG_LP, lp);
    if (err < 0)
    return err;
    reg[0] = settings.m_frac;
    reg[1] = settings.m_frac >> 8;
    reg[2] = settings.m_frac >> 16;
    reg[3] = settings.m_frac >> 24 | settings.m_int << 5;
    reg[4] = settings.m_int >> 3;
    reg[5] = settings.hs_div;
    reg[6] = (settings.hs_div >> 8) | (settings.ls_div_bits << 4);
    err = regmap_bulk_write(data.regmap, SI514_REG_HS_DIV, reg + 5, 2);
    if (err < 0)
    return err;
//
// Writing to SI514_REG_M_INT_FRAC triggers the clock change, so that
// must be written last
//
    return regmap_bulk_write(data.regmap, SI514_REG_M_FRAC1, reg, 5);
    }
// Calculate divider settings for a given frequency
    static int si514_calc_muldiv(struct clk_si514_muldiv *settings,
    unsigned long frequency)
    {
    u64 m;
    u32 ls_freq;
    u32 tmp;
    u8 res;
    if ((frequency < SI514_MIN_FREQ) || (frequency > SI514_MAX_FREQ))
    return -EINVAL;
// Determine the minimum value of LS_DIV and resulting target freq.
    ls_freq = frequency;
    if (frequency >= (FVCO_MIN / HS_DIV_MAX))
    settings.ls_div_bits = 0;
    else {
    res = 1;
    tmp = 2 * HS_DIV_MAX;
    while (tmp <= (HS_DIV_MAX * 32)) {
    if ((frequency * tmp) >= FVCO_MIN)
    break;
    ++res;
    tmp <<= 1;
    }
    settings.ls_div_bits = res;
    ls_freq = frequency << res;
    }
// Determine minimum HS_DIV, round up to even number
    settings.hs_div = DIV_ROUND_UP(FVCO_MIN >> 1, ls_freq) << 1;
// M = LS_DIV x HS_DIV x frequency / F_XO (in fixed-point)
    m = ((u64)(ls_freq * settings.hs_div) << 29) + (FXO / 2);
    do_div(m, FXO);
    settings.m_frac = (u32)m & (BIT(29) - 1);
    settings.m_int = (u32)(m >> 29);
    return 0;
    }
// Calculate resulting frequency given the register settings
#[no_mangle]
unsafe extern "C" fn si514_calc_rate(settings: *mut clk_si514_muldiv) -> c_ulong {
    static unsigned long si514_calc_rate(struct clk_si514_muldiv *settings)
    {
    let mut m: u64 = settings.m_frac | ((u64)settings.m_int << 29);
    let mut d: u32 = settings.hs_div * BIT(settings.ls_div_bits);
    return ((u32)(((m * FXO) + (FXO / 2)) >> 29)) / d;
    }
    static unsigned long si514_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_si514 *data = to_clk_si514(hw);
    struct clk_si514_muldiv settings;
    int err;
    err = si514_get_muldiv(data, &settings);
    if (err) {
    dev_err(&data.i2c_client.dev, "unable to retrieve settings\n");
    return 0;
    }
    return si514_calc_rate(&settings);
    }
    static int si514_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_si514_muldiv settings;
    int err;
    if (!req.rate) {
    req.rate = 0;
    return 0;
    }
    err = si514_calc_muldiv(&settings, req.rate);
    if (err) {
    req.rate = err;
    return 0;
    }
    req.rate = si514_calc_rate(&settings);
    return 0;
    }
//
// Update output frequency for big frequency changes (> 1000 ppm).
// The chip supports <1000ppm changes "on the fly", we haven't implemented
// that here.
//
    static int si514_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_si514 *data = to_clk_si514(hw);
    struct clk_si514_muldiv settings;
    unsigned int old_oe_state;
    int err;
    err = si514_calc_muldiv(&settings, rate);
    if (err)
    return err;
    err = regmap_read(data.regmap, SI514_REG_CONTROL, &old_oe_state);
    if (err)
    return err;
    si514_enable_output(data, false);
    err = si514_set_muldiv(data, &settings);
    if (err < 0)
    return err; /* Undefined state now, best to leave disabled */
// Trigger calibration
    err = regmap_write(data.regmap, SI514_REG_CONTROL, SI514_CONTROL_FCAL);
    if (err < 0)
    return err;
// Applying a new frequency can take up to 10ms
    usleep_range(10000, 12000);
    if (old_oe_state & SI514_CONTROL_OE)
    si514_enable_output(data, true);
    return err;
    }
    static const struct clk_ops si514_clk_ops = {
    .prepare = si514_prepare,
    .unprepare = si514_unprepare,
    .is_prepared = si514_is_prepared,
    .recalc_rate = si514_recalc_rate,
    .determine_rate = si514_determine_rate,
    .set_rate = si514_set_rate,
    };
#[no_mangle]
unsafe extern "C" fn si514_regmap_is_volatile(dev: *mut device, reg: c_uint) -> bool {
    static bool si514_regmap_is_volatile(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case SI514_REG_CONTROL:
    case SI514_REG_RESET:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn si514_regmap_is_writeable(dev: *mut device, reg: c_uint) -> bool {
    static bool si514_regmap_is_writeable(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case SI514_REG_LP:
    case SI514_REG_M_FRAC1 ... SI514_REG_LS_HS_DIV:
    case SI514_REG_OE_STATE:
    case SI514_REG_RESET:
    case SI514_REG_CONTROL:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_config si514_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .cache_type = REGCACHE_MAPLE,
    .max_register = SI514_REG_CONTROL,
    .writeable_reg = si514_regmap_is_writeable,
    .volatile_reg = si514_regmap_is_volatile,
    };
#[no_mangle]
unsafe extern "C" fn si514_probe(client: *mut i2c_client) -> c_int {
    static int si514_probe(struct i2c_client *client)
    {
    struct clk_si514 *data;
    struct clk_init_data init;
    int err;
    data = devm_kzalloc(&client.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    init.ops = &si514_clk_ops;
    init.flags = 0;
    init.num_parents = 0;
    data.hw.init = &init;
    data.i2c_client = client;
    if (of_property_read_string(client.dev.of_node, "clock-output-names",
    &init.name))
    init.name = client.dev.of_node.name;
    data.regmap = devm_regmap_init_i2c(client, &si514_regmap_config);
    if (IS_ERR(data.regmap)) {
    dev_err(&client.dev, "failed to allocate register map\n");
    return PTR_ERR(data.regmap);
    }
    i2c_set_clientdata(client, data);
    err = devm_clk_hw_register(&client.dev, &data.hw);
    if (err) {
    dev_err(&client.dev, "clock registration failed\n");
    return err;
    }
    err = devm_of_clk_add_hw_provider(&client.dev, of_clk_hw_simple_get,
    &data.hw);
    if (err) {
    dev_err(&client.dev, "unable to add clk provider\n");
    return err;
    }
    return 0;
    }
    static const struct i2c_device_id si514_id[] = {
    { .name = "si514" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, si514_id);
    static const struct of_device_id clk_si514_of_match[] = {
    { .compatible = "silabs,si514" },
    { },
    };
    MODULE_DEVICE_TABLE(of, clk_si514_of_match);
    static struct i2c_driver si514_driver = {
    .driver = {
    .name = "si514",
    .of_match_table = clk_si514_of_match,
    },
    .probe		= si514_probe,
    .id_table	= si514_id,
    };
    module_i2c_driver(si514_driver);
    MODULE_AUTHOR("Mike Looijmans <mike.looijmans@topic.nl>");
    MODULE_DESCRIPTION("Si514 driver");
    MODULE_LICENSE("GPL");
