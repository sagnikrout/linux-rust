//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-si544.c
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
// Driver for Silicon Labs Si544/Si549 Programmable Oscillator
// Copyright (C) 2018 Topic Embedded Products
// Author: Mike Looijmans <mike.looijmans@topic.nl>
//

// I2C registers (decimal as in datasheet)
pub const SI544_REG_CONTROL: c_int = 7;
pub const SI544_REG_OE_STATE: c_int = 17;
pub const SI544_REG_HS_DIV: c_int = 23;
pub const SI544_REG_LS_HS_DIV: c_int = 24;
pub const SI544_REG_FBDIV0: c_int = 26;
pub const SI544_REG_FBDIV8: c_int = 27;
pub const SI544_REG_FBDIV16: c_int = 28;
pub const SI544_REG_FBDIV24: c_int = 29;
pub const SI544_REG_FBDIV32: c_int = 30;
pub const SI544_REG_FBDIV40: c_int = 31;
pub const SI544_REG_FCAL_OVR: c_int = 69;
pub const SI544_REG_ADPLL_DELTA_M0: c_int = 231;
pub const SI544_REG_ADPLL_DELTA_M8: c_int = 232;
pub const SI544_REG_ADPLL_DELTA_M16: c_int = 233;
pub const SI544_REG_PAGE_SELECT: c_int = 255;
// Register values

// Max freq depends on speed grade

// Si544 Internal oscillator runs at 55.05 MHz

// Si549 Internal oscilator runs at 152.60 MHz

// VCO range is 10.8 .. 12.1 GHz, max depends on speed grade

pub const HS_DIV_MAX: c_int = 2046;
pub const HS_DIV_MAX_ODD: c_int = 33;
// Lowest frequency synthesizeable using only the HS divider

// Range and interpretation of the adjustment value
pub const DELTA_M_MAX: c_int = 8161512;
pub const DELTA_M_FRAC_NUM: c_int = 19;
pub const DELTA_M_FRAC_DEN: c_int = 20000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si544_clk_desc {
    pub max_freq: c_ulong,
    pub xo_freq: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_si544 {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub i2c_client: *mut i2c_client,
    pub chip_info: *const si544_clk_desc,
}

//
// struct clk_si544_muldiv - Multiplier/divider settings
// @fb_div_frac:	integer part of feedback divider (32 bits)
// @fb_div_int:		fractional part of feedback divider (11 bits)
// @hs_div:		1st divider, 5..2046, must be even when >33
// @ls_div_bits:	2nd divider, as 2^x, range 0..5
// If ls_div_bits is non-zero, hs_div must be even
// @delta_m:		Frequency shift for small -950..+950 ppm changes, 24 bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_si544_muldiv {
    pub fb_div_frac: u32,
    pub fb_div_int: u16,
    pub hs_div: u16,
    pub ls_div_bits: u8,
    pub delta_m: i32,
    pub xo_freq: u32,
}

// Enables or disables the output driver
#[no_mangle]
unsafe extern "C" fn si544_enable_output(data: *mut clk_si544, enable: bool) -> c_int {
    static int si544_enable_output(struct clk_si544 *data, bool enable)
    {
    return regmap_update_bits(data.regmap, SI544_REG_OE_STATE,
    SI544_OE_STATE_ODC_OE, enable ? SI544_OE_STATE_ODC_OE : 0);
    }
#[no_mangle]
unsafe extern "C" fn si544_prepare(hw: *mut clk_hw) -> c_int {
    static int si544_prepare(struct clk_hw *hw)
    {
    struct clk_si544 *data = to_clk_si544(hw);
    return si544_enable_output(data, true);
    }
#[no_mangle]
unsafe extern "C" fn si544_unprepare(hw: *mut clk_hw) {
    static void si544_unprepare(struct clk_hw *hw)
    {
    struct clk_si544 *data = to_clk_si544(hw);
    si544_enable_output(data, false);
    }
#[no_mangle]
unsafe extern "C" fn si544_is_prepared(hw: *mut clk_hw) -> c_int {
    static int si544_is_prepared(struct clk_hw *hw)
    {
    struct clk_si544 *data = to_clk_si544(hw);
    unsigned int val;
    int err;
    err = regmap_read(data.regmap, SI544_REG_OE_STATE, &val);
    if (err < 0)
    return err;
    return !!(val & SI544_OE_STATE_ODC_OE);
    }
// Retrieve clock multiplier and dividers from hardware
    static int si544_get_muldiv(struct clk_si544 *data,
    struct clk_si544_muldiv *settings)
    {
    int err;
    u8 reg[6];
    err = regmap_bulk_read(data.regmap, SI544_REG_HS_DIV, reg, 2);
    if (err)
    return err;
    settings.ls_div_bits = (reg[1] >> 4) & 0x07;
    settings.hs_div = (reg[1] & 0x07) << 8 | reg[0];
    err = regmap_bulk_read(data.regmap, SI544_REG_FBDIV0, reg, 6);
    if (err)
    return err;
    settings.fb_div_int = reg[4] | (reg[5] & 0x07) << 8;
    settings.fb_div_frac = reg[0] | reg[1] << 8 | reg[2] << 16 |
    reg[3] << 24;
    err = regmap_bulk_read(data.regmap, SI544_REG_ADPLL_DELTA_M0, reg, 3);
    if (err)
    return err;
// Interpret as 24-bit signed number
    settings.delta_m = reg[0] << 8 | reg[1] << 16 | reg[2] << 24;
    settings.delta_m >>= 8;
    settings.xo_freq = data.chip_info.xo_freq;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn si544_set_delta_m(data: *mut clk_si544, delta_m: i32) -> c_int {
    static int si544_set_delta_m(struct clk_si544 *data, s32 delta_m)
    {
    u8 reg[3];
    reg[0] = delta_m;
    reg[1] = delta_m >> 8;
    reg[2] = delta_m >> 16;
    return regmap_bulk_write(data.regmap, SI544_REG_ADPLL_DELTA_M0,
    reg, 3);
    }
    static int si544_set_muldiv(struct clk_si544 *data,
    struct clk_si544_muldiv *settings)
    {
    int err;
    u8 reg[6];
    reg[0] = settings.hs_div;
    reg[1] = settings.hs_div >> 8 | settings.ls_div_bits << 4;
    err = regmap_bulk_write(data.regmap, SI544_REG_HS_DIV, reg, 2);
    if (err < 0)
    return err;
    reg[0] = settings.fb_div_frac;
    reg[1] = settings.fb_div_frac >> 8;
    reg[2] = settings.fb_div_frac >> 16;
    reg[3] = settings.fb_div_frac >> 24;
    reg[4] = settings.fb_div_int;
    reg[5] = settings.fb_div_int >> 8;
//
// Writing to SI544_REG_FBDIV40 triggers the clock change, so that
// must be written last
//
    return regmap_bulk_write(data.regmap, SI544_REG_FBDIV0, reg, 6);
    }
    static bool is_valid_frequency(const struct clk_si544 *data,
    unsigned long frequency)
    {
    if (frequency < SI544_MIN_FREQ)
    return false;
    return frequency <= data.chip_info.max_freq;
    }
// Calculate divider settings for a given frequency
    static int si544_calc_muldiv(struct clk_si544_muldiv *settings,
    unsigned long frequency)
    {
    u64 vco;
    let mut fxo: u32 = settings.xo_freq;
    u32 ls_freq;
    u32 tmp;
    u8 res;
// Determine the minimum value of LS_DIV and resulting target freq.
    ls_freq = frequency;
    settings.ls_div_bits = 0;
    if (frequency >= MIN_HSDIV_FREQ) {
    settings.ls_div_bits = 0;
    } else {
    res = 1;
    tmp = 2 * HS_DIV_MAX;
    while (tmp <= (HS_DIV_MAX * 32)) {
    if (((u64)frequency * tmp) >= FVCO_MIN)
    break;
    ++res;
    tmp <<= 1;
    }
    settings.ls_div_bits = res;
    ls_freq = frequency << res;
    }
// Determine minimum HS_DIV by rounding up
    vco = FVCO_MIN + ls_freq - 1;
    do_div(vco, ls_freq);
    settings.hs_div = vco;
// round up to even number when required
    if ((settings.hs_div & 1) &&
    (settings.hs_div > HS_DIV_MAX_ODD || settings.ls_div_bits))
    ++settings.hs_div;
// Calculate VCO frequency (in 10..12GHz range)
    vco = (u64)ls_freq * settings.hs_div;
// Calculate the integer part of the feedback divider
    tmp = do_div(vco, fxo);
    settings.fb_div_int = vco;
// And the fractional bits using the remainder
    vco = (u64)tmp << 32;
    vco += fxo / 2; /* Round to nearest multiple */
    do_div(vco, fxo);
    settings.fb_div_frac = vco;
// Reset the frequency adjustment
    settings.delta_m = 0;
    return 0;
    }
// Calculate resulting frequency given the register settings
    static unsigned long si544_calc_center_rate(
    const struct clk_si544_muldiv *settings)
    {
    let mut d: u32 = settings.hs_div * BIT(settings.ls_div_bits);
    let mut fxo: u32 = settings.xo_freq;
    u64 vco;
// Calculate VCO from the fractional part
    vco = (u64)settings.fb_div_frac * fxo;
    vco += (fxo / 2);
    vco >>= 32;
// Add the integer part of the VCO frequency
    vco += (u64)settings.fb_div_int * fxo;
// Apply divider to obtain the generated frequency
    do_div(vco, d);
    return vco;
    }
#[no_mangle]
unsafe extern "C" fn si544_calc_rate(settings: *const clk_si544_muldiv) -> c_ulong {
    static unsigned long si544_calc_rate(const struct clk_si544_muldiv *settings)
    {
    let mut rate: c_ulong = si544_calc_center_rate(settings);
    let mut delta: i64 = (s64)rate * (DELTA_M_FRAC_NUM * settings.delta_m);
//
// The clock adjustment is much smaller than 1 Hz, round to the
// nearest multiple. Apparently div64_s64 rounds towards zero, hence
// check the sign and adjust into the proper direction.
//
    if (settings.delta_m < 0)
    delta -= ((s64)DELTA_M_MAX * DELTA_M_FRAC_DEN) / 2;
    else
    delta += ((s64)DELTA_M_MAX * DELTA_M_FRAC_DEN) / 2;
    delta = div64_s64(delta, ((s64)DELTA_M_MAX * DELTA_M_FRAC_DEN));
    return rate + delta;
    }
    static unsigned long si544_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_si544 *data = to_clk_si544(hw);
    struct clk_si544_muldiv settings;
    int err;
    err = si544_get_muldiv(data, &settings);
    if (err)
    return 0;
    return si544_calc_rate(&settings);
    }
    static int si544_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_si544 *data = to_clk_si544(hw);
    if (!is_valid_frequency(data, req.rate))
    return -EINVAL;
// The accuracy is less than 1 Hz, so any rate is possible
    return 0;
    }
// Calculates the maximum "small" change, 950 * rate / 1000000
#[no_mangle]
unsafe extern "C" fn si544_max_delta(rate: c_ulong) -> c_ulong {
    static unsigned long si544_max_delta(unsigned long rate)
    {
    let mut num: u64 = rate;
    num *= DELTA_M_FRAC_NUM;
    do_div(num, DELTA_M_FRAC_DEN);
    return num;
    }
#[no_mangle]
unsafe extern "C" fn si544_calc_delta(delta: i32, max_delta: i32) -> i32 {
    static s32 si544_calc_delta(s32 delta, s32 max_delta)
    {
    let mut n: i64 = (s64)delta * DELTA_M_MAX;
    return div_s64(n, max_delta);
    }
    static int si544_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_si544 *data = to_clk_si544(hw);
    struct clk_si544_muldiv settings;
    unsigned long center;
    long max_delta;
    long delta;
    unsigned int old_oe_state;
    int err;
    if (!is_valid_frequency(data, rate))
    return -EINVAL;
// Try using the frequency adjustment feature for a <= 950ppm change
    err = si544_get_muldiv(data, &settings);
    if (err)
    return err;
    center = si544_calc_center_rate(&settings);
    max_delta = si544_max_delta(center);
    delta = rate - center;
    if (abs(delta) <= max_delta)
    return si544_set_delta_m(data,
    si544_calc_delta(delta, max_delta));
// Too big for the delta adjustment, need to reprogram
    err = si544_calc_muldiv(&settings, rate);
    if (err)
    return err;
    err = regmap_read(data.regmap, SI544_REG_OE_STATE, &old_oe_state);
    if (err)
    return err;
    si544_enable_output(data, false);
// Allow FCAL for this frequency update
    err = regmap_write(data.regmap, SI544_REG_FCAL_OVR, 0);
    if (err < 0)
    return err;
    err = si544_set_delta_m(data, settings.delta_m);
    if (err < 0)
    return err;
    err = si544_set_muldiv(data, &settings);
    if (err < 0)
    return err; /* Undefined state now, best to leave disabled */
// Trigger calibration
    err = regmap_write(data.regmap, SI544_REG_CONTROL,
    SI544_CONTROL_MS_ICAL2);
    if (err < 0)
    return err;
// Applying a new frequency can take up to 10ms
    usleep_range(10000, 12000);
    if (old_oe_state & SI544_OE_STATE_ODC_OE)
    si544_enable_output(data, true);
    return err;
    }
    static const struct clk_ops si544_clk_ops = {
    .prepare = si544_prepare,
    .unprepare = si544_unprepare,
    .is_prepared = si544_is_prepared,
    .recalc_rate = si544_recalc_rate,
    .determine_rate = si544_determine_rate,
    .set_rate = si544_set_rate,
    };
#[no_mangle]
unsafe extern "C" fn si544_regmap_is_volatile(dev: *mut device, reg: c_uint) -> bool {
    static bool si544_regmap_is_volatile(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case SI544_REG_CONTROL:
    case SI544_REG_FCAL_OVR:
    return true;
    default:
    return false;
    }
    }
    static const struct regmap_config si544_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .cache_type = REGCACHE_MAPLE,
    .max_register = SI544_REG_PAGE_SELECT,
    .volatile_reg = si544_regmap_is_volatile,
    };
#[no_mangle]
unsafe extern "C" fn si544_probe(client: *mut i2c_client) -> c_int {
    static int si544_probe(struct i2c_client *client)
    {
    struct clk_si544 *data;
    struct clk_init_data init;
    int err;
    data = devm_kzalloc(&client.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    init.ops = &si544_clk_ops;
    init.flags = 0;
    init.num_parents = 0;
    data.hw.init = &init;
    data.i2c_client = client;
    data.chip_info = i2c_get_match_data(client);
    if (of_property_read_string(client.dev.of_node, "clock-output-names",
    &init.name))
    init.name = client.dev.of_node.name;
    data.regmap = devm_regmap_init_i2c(client, &si544_regmap_config);
    if (IS_ERR(data.regmap))
    return PTR_ERR(data.regmap);
    i2c_set_clientdata(client, data);
// Select page 0, just to be sure, there appear to be no more
    err = regmap_write(data.regmap, SI544_REG_PAGE_SELECT, 0);
    if (err < 0)
    return err;
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
    static const struct si544_clk_desc clk_si544a_info = {
    .xo_freq = SI544_XO_FREQ,
    .max_freq = 1500000000,
    };
    static const struct si544_clk_desc clk_si544b_info = {
    .xo_freq = SI544_XO_FREQ,
    .max_freq = 800000000,
    };
    static const struct si544_clk_desc clk_si544c_info = {
    .xo_freq = SI544_XO_FREQ,
    .max_freq = 325000000,
    };
    static const struct si544_clk_desc clk_si549a_info = {
    .xo_freq = SI549_XO_FREQ,
    .max_freq = 1500000000,
    };
    static const struct si544_clk_desc clk_si549b_info = {
    .xo_freq = SI549_XO_FREQ,
    .max_freq = 800000000,
    };
    static const struct si544_clk_desc clk_si549c_info = {
    .xo_freq = SI549_XO_FREQ,
    .max_freq = 325000000,
    };
    static const struct i2c_device_id si544_id[] = {
    { .name = "si544a", .driver_data = (kernel_ulong_t)&clk_si544a_info },
    { .name = "si544b", .driver_data = (kernel_ulong_t)&clk_si544b_info },
    { .name = "si544c", .driver_data = (kernel_ulong_t)&clk_si544c_info },
    { .name = "si549a", .driver_data = (kernel_ulong_t)&clk_si549a_info },
    { .name = "si549b", .driver_data = (kernel_ulong_t)&clk_si549b_info },
    { .name = "si549c", .driver_data = (kernel_ulong_t)&clk_si549c_info },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, si544_id);
    static const struct of_device_id clk_si544_of_match[] = {
    { .compatible = "silabs,si544a", .data = &clk_si544a_info },
    { .compatible = "silabs,si544b", .data = &clk_si544b_info },
    { .compatible = "silabs,si544c", .data = &clk_si544c_info },
    { .compatible = "silabs,si549a", .data = &clk_si549a_info },
    { .compatible = "silabs,si549b", .data = &clk_si549b_info },
    { .compatible = "silabs,si549c", .data = &clk_si549c_info },
    { }
    };
    MODULE_DEVICE_TABLE(of, clk_si544_of_match);
    static struct i2c_driver si544_driver = {
    .driver = {
    .name = "si544",
    .of_match_table = clk_si544_of_match,
    },
    .probe		= si544_probe,
    .id_table	= si544_id,
    };
    module_i2c_driver(si544_driver);
    MODULE_AUTHOR("Mike Looijmans <mike.looijmans@topic.nl>");
    MODULE_DESCRIPTION("Si544 driver");
    MODULE_LICENSE("GPL");
